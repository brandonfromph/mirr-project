#![forbid(unsafe_code)]
//! Extended SystemVerilog emitter tests.
//!
//! Comprehensive coverage of `src/emit/verilog.rs` (~666 lines):
//! - Module header and boilerplate
//! - Signal declarations (input, output, internal, signed, widths)
//! - Guard logic (shift register, counter, 1-cycle combinational, complex)
//! - Reflex assignments (single guard, multi-guard AND join, defaults)
//! - SVA property generation (all 6 formula variants x 3 directives)
//! - Bind file generation (`emit_sva_bind_file`)
//! - Synthesis-clean mode (`emit_sv_synthesis`)
//! - Prev register handling (`_dN` delay suffix)
//! - Synchronizer chain emission (`emit_synchronizer_chains`)
//! - SVA-only mode (`emit_sva_only`)
//! - Edge cases (empty modules, no guards, no reflexes, no properties)
//!
//! NASA Power-of-10 compliance:
//! - `#![forbid(unsafe_code)]`
//! - Bounded iteration with `MAX_*` constants
//! - No recursion in test helpers
//! - All `assert!` with descriptive messages

use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::ast::program::{Assignment, Guard, MirrProgram, Module, Reflex, SignalDecl};
use nasa_rust_project::ast::property::{PropertyDecl, PropertyDirective, PropertyFormula};
use nasa_rust_project::ast::types::{BinaryOp, ExtendedType, LiteralValue, SignalKind, SignalType};
use nasa_rust_project::emit::verilog;
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig, PipelineResult};
use nasa_rust_project::temporal::low_level_ir::{
    CompiledGuard, ComplexGuard, ConditionKind, ShiftRegisterGuard, TemporalNetlist,
};

// ---------------------------------------------------------------------------
// Bounded iteration constants (NASA Power-of-10)
// ---------------------------------------------------------------------------

/// Maximum lines to scan in emitted output.
const MAX_OUTPUT_LINES: usize = 2048;

/// Maximum ports to verify in a module declaration.
const MAX_PORTS_CHECK: usize = 64;

/// Maximum number of property variants to iterate over.
const MAX_PROPERTY_VARIANTS: usize = 16;

// ---------------------------------------------------------------------------
// Helper functions (no recursion)
// ---------------------------------------------------------------------------

fn default_config() -> PipelineConfig {
    PipelineConfig::default()
}

fn sig(name: &str) -> Expr {
    Expr::Signal(name.to_string())
}

fn lit_int(n: u64) -> Expr {
    Expr::Literal(LiteralValue::Integer(n))
}

fn lit_bool(b: bool) -> Expr {
    Expr::Literal(LiteralValue::Bool(b))
}

fn gt_expr(lhs: Expr, rhs: u64) -> Expr {
    Expr::Binary {
        op: BinaryOp::Gt,
        left: Box::new(lhs),
        right: Box::new(Expr::Literal(LiteralValue::Integer(rhs))),
    }
}

fn add_expr(lhs: Expr, rhs: Expr) -> Expr {
    Expr::Binary { op: BinaryOp::Add, left: Box::new(lhs), right: Box::new(rhs) }
}

fn sub_expr(lhs: Expr, rhs: Expr) -> Expr {
    Expr::Binary { op: BinaryOp::Sub, left: Box::new(lhs), right: Box::new(rhs) }
}

fn mul_expr(lhs: Expr, rhs: Expr) -> Expr {
    Expr::Binary { op: BinaryOp::Mul, left: Box::new(lhs), right: Box::new(rhs) }
}

fn not_expr(operand: Expr) -> Expr {
    Expr::Unary { op: nasa_rust_project::ast::types::UnaryOp::Not, operand: Box::new(operand) }
}

fn prev_expr(signal: &str, delay: u64) -> Expr {
    Expr::Prev { signal: signal.to_string(), delay }
}

fn signal_decl(name: &str, kind: SignalKind, ty: SignalType) -> SignalDecl {
    SignalDecl {
        name: name.to_string(),
        kind,
        ty: ExtendedType::from_core(ty),
        origin: None,
        span: None,
    }
}

fn make_guard(name: &str, condition: Expr, cycles: u64) -> Guard {
    Guard { name: name.to_string(), condition, cycles, origin: None, span: None }
}

fn make_reflex(name: &str, guard_names: Vec<&str>, assignments: Vec<Assignment>) -> Reflex {
    Reflex {
        name: name.to_string(),
        guard_names: guard_names.into_iter().map(|s| s.to_string()).collect(),
        assignments,
        origin: None,
        span: None,
    }
}

fn make_assignment(target: &str, value: Expr) -> Assignment {
    Assignment { target: target.to_string(), value, span: None }
}

fn make_property(
    name: &str,
    directive: PropertyDirective,
    formula: PropertyFormula,
) -> PropertyDecl {
    PropertyDecl { name: name.to_string(), directive, formula, origin: None, span: None }
}

/// Build a PipelineResult from a programmatic Module (bypasses parser/validation).
fn result_from_module(module: Module) -> PipelineResult {
    PipelineResult {
        program: MirrProgram { patterns: Vec::new(), module },
        simplify_stats: None,
        width_result: None,
        temporal_netlist: None,
        rspu_program: None,
        type_map: None,
        extended_type_map: None,
        sim_result: None,
        mape_k_result: None,
    }
}

/// Build a PipelineResult from a Module with a custom temporal netlist.
fn result_with_netlist(module: Module, netlist: TemporalNetlist) -> PipelineResult {
    PipelineResult {
        program: MirrProgram { patterns: Vec::new(), module },
        simplify_stats: None,
        width_result: None,
        temporal_netlist: Some(netlist),
        rspu_program: None,
        type_map: None,
        extended_type_map: None,
        sim_result: None,
        mape_k_result: None,
    }
}

/// Count lines in output, bounded by MAX_OUTPUT_LINES.
fn count_lines_bounded(text: &str) -> usize {
    let mut count = 0usize;
    for _line in text.lines() {
        count += 1;
        if count >= MAX_OUTPUT_LINES {
            break;
        }
    }
    count
}

// ---------------------------------------------------------------------------
// MIRR source fixtures
// ---------------------------------------------------------------------------

const MINIMAL_MODULE: &str = r#"
module minimal {
    signal a: in u8;
    signal b: out u8;

    guard g {
        when a > 0
        for 1 cycles;
    }

    reflex r {
        on g {
            b = a;
        }
    }
}
"#;

const MULTI_GUARD_MODULE: &str = r#"
module multi_guard_mod {
    signal x: in bool;
    signal y: in bool;
    signal z: out bool;

    guard g1 {
        when x
        for 3 cycles;
    }

    guard g2 {
        when y
        for 5 cycles;
    }

    reflex join_both {
        on g1 and g2 {
            z = true;
        }
    }
}
"#;

const COUNTER_GUARD_MODULE: &str = r#"
module counter_mod {
    signal enable: in bool;
    signal ready: out bool;

    guard sustained {
        when enable
        for 100 cycles;
    }

    reflex fire {
        on sustained {
            ready = true;
        }
    }
}
"#;

const INTERNAL_SIGNALS_MODULE: &str = r#"
module internals {
    signal sensor: in u16;
    signal result: out u16;
    signal accumulator: internal u16;

    guard active {
        when sensor > 0
        for 2 cycles;
    }

    reflex compute {
        on active {
            accumulator = sensor;
            result = accumulator;
        }
    }
}
"#;

const PROPERTY_ALL_VARIANTS: &str = r#"
module prop_mod {
    signal sensor: in u16;
    signal alarm: out bool;

    guard g {
        when sensor > 100
        for 3 cycles;
    }

    reflex r {
        on g {
            alarm = true;
        }
    }

    property p_always {
        always (sensor < 1000);
    }

    property p_never {
        never (alarm && sensor < 50);
    }

    property p_implies {
        always (sensor > 100 -> alarm);
    }

    property p_never_implies {
        never (sensor > 100 -> alarm);
    }

    property p_eventually {
        eventually within 10 (alarm);
    }

    property p_followed_by {
        always (sensor > 200 followed_by 3 alarm);
    }
}
"#;

const NO_GUARD_MODULE: &str = r#"
module bare {
    signal a: in u8;
    signal b: out u8;
}
"#;

const SIGNED_TYPES_MODULE: &str = r#"
module signed_types {
    signal s_in: in i16;
    signal s_out: out i16;
    signal en: in bool;

    guard g {
        when en
        for 1 cycles;
    }

    reflex r {
        on g {
            s_out = s_in;
        }
    }
}
"#;

// ===========================================================================
// Section 1: Module Header Tests
// ===========================================================================

#[test]
fn header_contains_auto_generated_comment() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(
        sv.contains("Auto-generated by MIRR compiler"),
        "header must contain auto-generated comment"
    );
}

#[test]
fn header_contains_phase6_marker() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("Phase 6"), "header must reference Phase 6");
}

#[test]
fn header_contains_do_not_edit_warning() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("Do not edit"), "header must contain 'Do not edit' warning");
}

#[test]
fn header_contains_target_sv() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("Target: SystemVerilog"), "header must specify SystemVerilog target");
}

#[test]
fn output_ends_with_endmodule() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.trim_end().ends_with("endmodule"), "output must end with endmodule");
}

// ===========================================================================
// Section 2: Module Declaration and Port List
// ===========================================================================

#[test]
fn module_decl_contains_module_name() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("module minimal ("), "must contain module name in declaration");
}

#[test]
fn module_decl_auto_injects_clk_with_guards() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    let decl_start = sv.find("module minimal (").expect("must find module decl");
    let decl_end = sv[decl_start..].find(");").expect("must find );") + decl_start;
    let decl = &sv[decl_start..decl_end];

    assert!(decl.contains("clk"), "clk must be auto-injected when guards exist");
    assert!(decl.contains("rst_n"), "rst_n must be auto-injected when guards exist");
}

#[test]
fn module_decl_no_clk_without_guards() {
    let result = run_pipeline(NO_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    let decl_start = sv.find("module bare (").expect("must find module decl");
    let decl_end = sv[decl_start..].find(");").expect("must find );") + decl_start;
    let decl = &sv[decl_start..decl_end];

    assert!(!decl.contains("clk"), "clk must NOT be injected without guards");
    assert!(!decl.contains("rst_n"), "rst_n must NOT be injected without guards");
}

#[test]
fn module_decl_input_direction() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("input "), "must contain input direction for input signals");
}

#[test]
fn module_decl_output_direction() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("output"), "must contain output direction for output signals");
}

#[test]
fn module_decl_port_commas_correct() {
    let result = run_pipeline(MULTI_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    let decl_start = sv.find("module multi_guard_mod (").expect("must find module decl");
    let decl_end = sv[decl_start..].find(");").expect("must find );") + decl_start;
    let decl = &sv[decl_start..decl_end];

    // Count commas vs ports: N ports should have N-1 commas
    let comma_count = decl.chars().filter(|c| *c == ',').count();
    let port_lines: Vec<&str> =
        decl.lines().filter(|l| l.contains("input") || l.contains("output")).collect();

    assert!(port_lines.len() > 1, "must have multiple ports, found {}", port_lines.len());
    assert_eq!(
        comma_count,
        port_lines.len() - 1,
        "port list comma count ({}) must equal port count ({}) minus 1",
        comma_count,
        port_lines.len()
    );
}

#[test]
fn module_decl_internal_not_in_port_list() {
    let result = run_pipeline(INTERNAL_SIGNALS_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    let decl_start = sv.find("module internals (").expect("must find module decl");
    let decl_end = sv[decl_start..].find(");").expect("must find );") + decl_start;
    let decl = &sv[decl_start..decl_end];

    assert!(!decl.contains("accumulator"), "internal signal must NOT appear in port list");
}

// ===========================================================================
// Section 3: Signal Type Rendering
// ===========================================================================

#[test]
fn sv_type_bool_renders_as_logic() {
    let result = run_pipeline(MULTI_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    // Bool type should render as "logic" with trailing spaces for alignment
    assert!(sv.contains("logic       "), "bool type must render as 'logic' with padding");
}

#[test]
fn sv_type_u8_renders_with_width() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("logic [ 7:0]"), "u8 must render as 'logic [ 7:0]'");
}

#[test]
fn sv_type_u16_renders_with_width() {
    let result = run_pipeline(INTERNAL_SIGNALS_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("logic [15:0]"), "u16 must render as 'logic [15:0]'");
}

#[test]
fn sv_type_signed_renders_correctly() {
    let result = run_pipeline(SIGNED_TYPES_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("logic signed"), "signed type must contain 'logic signed'");
}

#[test]
fn sv_type_u1_renders_as_logic_no_range() {
    let source = r#"
module u1_mod {
    signal f: in u1;
    signal o: out bool;

    guard g {
        when f > 0
        for 1 cycles;
    }

    reflex r {
        on g {
            o = true;
        }
    }
}
"#;
    let result = run_pipeline(source, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    let decl_start = sv.find("module u1_mod").expect("must find module decl");
    let decl_end = sv[decl_start..].find(");").expect("must find );") + decl_start;
    let decl = &sv[decl_start..decl_end];

    assert!(!decl.contains("[0:0]"), "u1 must render as 'logic', not 'logic [0:0]'");
}

// ===========================================================================
// Section 4: Internal Signal Declarations
// ===========================================================================

#[test]
fn internal_signals_section_header() {
    let result = run_pipeline(INTERNAL_SIGNALS_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("// Internal signals"), "must have internal signals section comment");
}

#[test]
fn internal_signal_declared_inside_module() {
    let result = run_pipeline(INTERNAL_SIGNALS_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    // Internal signal should appear as a declaration after the port list
    let endport = sv.find(");").expect("must find );");
    let after_ports = &sv[endport..];

    assert!(
        after_ports.contains("accumulator"),
        "internal signal 'accumulator' must be declared after port list"
    );
}

#[test]
fn no_internal_section_when_none_exist() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(
        !sv.contains("// Internal signals"),
        "must NOT have internal signals section when there are no internals"
    );
}

// ===========================================================================
// Section 5: Temporal Guard Logic (Shift Register)
// ===========================================================================

#[test]
fn shift_register_guard_comment() {
    let result = run_pipeline(MULTI_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("// Guard:"), "must contain guard comment annotations");
}

#[test]
fn shift_register_always_ff_block() {
    let result = run_pipeline(MULTI_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(
        sv.contains("always_ff @(posedge clk or negedge rst_n)"),
        "shift register guard must use always_ff with clk and rst_n"
    );
}

#[test]
fn shift_register_reset_logic() {
    let result = run_pipeline(MULTI_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("if (!rst_n)"), "shift register must have reset condition");
    assert!(sv.contains("<= '0;"), "shift register must reset to zero");
}

#[test]
fn shift_register_sr_declaration() {
    let result = run_pipeline(MULTI_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    // g1 has 3 cycles, so shift register width is [2:0]
    assert!(sv.contains("g1_sr"), "must declare g1 shift register");
    // g2 has 5 cycles, so shift register width is [4:0]
    assert!(sv.contains("g2_sr"), "must declare g2 shift register");
}

#[test]
fn shift_register_condition_wire() {
    let result = run_pipeline(MULTI_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("g1_cond"), "must declare condition wire for g1");
    assert!(sv.contains("g2_cond"), "must declare condition wire for g2");
}

#[test]
fn shift_register_output_and_reduction() {
    let result = run_pipeline(MULTI_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    // Output fires when all stages are 1: assign gN_out = &gN_sr;
    assert!(sv.contains("&g1_sr"), "g1 output must use AND-reduction of shift register");
    assert!(sv.contains("&g2_sr"), "g2 output must use AND-reduction of shift register");
}

// ===========================================================================
// Section 6: Temporal Guard Logic (Counter)
// ===========================================================================

#[test]
fn counter_guard_comment() {
    let result = run_pipeline(COUNTER_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("counter"), "counter guard must have counter-related annotation");
}

#[test]
fn counter_guard_always_ff() {
    let result = run_pipeline(COUNTER_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(
        sv.contains("always_ff @(posedge clk or negedge rst_n)"),
        "counter guard must use always_ff block"
    );
}

#[test]
fn counter_guard_reset_and_count_logic() {
    let result = run_pipeline(COUNTER_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    // Counter resets on !rst_n or when condition is false
    assert!(sv.contains("if (!rst_n)"), "counter must have reset condition");
    assert!(sv.contains("<= '0;"), "counter must reset to zero");
    assert!(sv.contains("+ 1"), "counter must increment by 1");
}

#[test]
fn counter_guard_output_comparison() {
    let result = run_pipeline(COUNTER_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains(">= 100"), "counter output must compare against target count 100");
}

// ===========================================================================
// Section 7: 1-Cycle Combinational Guard
// ===========================================================================

#[test]
fn one_cycle_guard_no_always_ff() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    // A 1-cycle guard is purely combinational
    assert!(!sv.contains("always_ff"), "1-cycle guard must NOT use always_ff (combinational only)");
}

#[test]
fn one_cycle_guard_uses_assign() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(
        sv.contains("assign g_out = g_cond"),
        "1-cycle guard must use combinational assign for output"
    );
}

#[test]
fn one_cycle_guard_condition_wire() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("logic g_cond"), "1-cycle guard must declare condition wire");
    assert!(sv.contains("assign g_cond ="), "1-cycle guard must assign condition wire");
}

// ===========================================================================
// Section 8: Complex Guard (Programmatic)
// ===========================================================================

#[test]
fn complex_guard_assign_block() {
    let module = Module {
        name: "complex_test".to_string(),
        signals: vec![
            signal_decl("a", SignalKind::Input, SignalType::Bool),
            signal_decl("b", SignalKind::Output, SignalType::Bool),
        ],
        guards: vec![],
        reflexes: vec![],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let mut netlist = TemporalNetlist::new();
    let complex = ComplexGuard::new("combo".to_string(), vec![], Expr::Signal("a".to_string()));
    netlist.add_guard(CompiledGuard::Complex(complex));

    let result = result_with_netlist(module, netlist);
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("Complex guard: combo"), "complex guard must have descriptive comment");
    assert!(
        sv.contains("assign combo_out = a"),
        "complex guard must have assign for combination logic"
    );
}

// ===========================================================================
// Section 9: Reflex Logic
// ===========================================================================

#[test]
fn reflex_section_header() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(
        sv.contains("// ── Reflex Assignments ──"),
        "must have reflex assignments section header"
    );
}

#[test]
fn reflex_always_comb_block() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("always_comb begin"), "reflex must use always_comb block");
}

#[test]
fn reflex_default_assignment_prevents_latch() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(
        sv.contains("= '0;"),
        "always_comb must have default assignments to prevent latch inference"
    );
}

#[test]
fn reflex_guard_out_wire_declared() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("logic g_out;"), "guard _out wire must be declared for reflex use");
}

#[test]
fn reflex_single_guard_condition() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("if (g_out)"), "single-guard reflex must use guard_out as condition");
}

#[test]
fn reflex_multi_guard_and_join() {
    let result = run_pipeline(MULTI_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("g1_out && g2_out"), "multi-guard reflex must AND-join guard outputs");
}

#[test]
fn reflex_name_in_comment() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("// Reflex: r"), "reflex name must appear in comment");
}

#[test]
fn no_reflex_section_when_empty() {
    let result = run_pipeline(NO_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(
        !sv.contains("// ── Reflex Assignments ──"),
        "must NOT have reflex section when module has no reflexes"
    );
}

// ===========================================================================
// Section 10: Prev Register Handling
// ===========================================================================

#[test]
fn prev_delay_1_renders_as_d1() {
    let module = Module {
        name: "prev_test".to_string(),
        signals: vec![
            signal_decl("sensor", SignalKind::Input, SignalType::Unsigned(16)),
            signal_decl("delta", SignalKind::Output, SignalType::Unsigned(16)),
        ],
        guards: vec![make_guard("g", gt_expr(sig("sensor"), 0), 1)],
        reflexes: vec![make_reflex(
            "r",
            vec!["g"],
            vec![make_assignment("delta", sub_expr(sig("sensor"), prev_expr("sensor", 1)))],
        )],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let result = result_from_module(module);
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("sensor_d1"), "prev(sensor, 1) must render as sensor_d1");
}

#[test]
fn prev_delay_3_renders_as_d3() {
    let module = Module {
        name: "prev3_test".to_string(),
        signals: vec![
            signal_decl("x", SignalKind::Input, SignalType::Unsigned(8)),
            signal_decl("y", SignalKind::Output, SignalType::Unsigned(8)),
        ],
        guards: vec![make_guard("g", gt_expr(sig("x"), 0), 1)],
        reflexes: vec![make_reflex("r", vec!["g"], vec![make_assignment("y", prev_expr("x", 3))])],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let result = result_from_module(module);
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("x_d3"), "prev(x, 3) must render as x_d3");
}

#[test]
fn prev_in_binary_expression() {
    let module = Module {
        name: "prev_bin".to_string(),
        signals: vec![
            signal_decl("a", SignalKind::Input, SignalType::Unsigned(8)),
            signal_decl("out", SignalKind::Output, SignalType::Unsigned(8)),
        ],
        guards: vec![make_guard("g", gt_expr(sig("a"), 0), 1)],
        reflexes: vec![make_reflex(
            "r",
            vec!["g"],
            vec![make_assignment("out", add_expr(sig("a"), prev_expr("a", 2)))],
        )],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let result = result_from_module(module);
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("(a + a_d2)"), "prev in binary expr must render as (a + a_d2)");
}

// ===========================================================================
// Section 11: Expression Rendering
// ===========================================================================

#[test]
fn expr_literal_bool_true() {
    let module = Module {
        name: "lit_true".to_string(),
        signals: vec![
            signal_decl("x", SignalKind::Input, SignalType::Bool),
            signal_decl("y", SignalKind::Output, SignalType::Bool),
        ],
        guards: vec![make_guard("g", sig("x"), 1)],
        reflexes: vec![make_reflex("r", vec!["g"], vec![make_assignment("y", lit_bool(true))])],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let result = result_from_module(module);
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("1'b1"), "true literal must render as 1'b1");
}

#[test]
fn expr_literal_bool_false() {
    let module = Module {
        name: "lit_false".to_string(),
        signals: vec![
            signal_decl("x", SignalKind::Input, SignalType::Bool),
            signal_decl("y", SignalKind::Output, SignalType::Bool),
        ],
        guards: vec![make_guard("g", sig("x"), 1)],
        reflexes: vec![make_reflex("r", vec!["g"], vec![make_assignment("y", lit_bool(false))])],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let result = result_from_module(module);
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("1'b0"), "false literal must render as 1'b0");
}

#[test]
fn expr_literal_integer() {
    let module = Module {
        name: "lit_int".to_string(),
        signals: vec![
            signal_decl("x", SignalKind::Input, SignalType::Unsigned(8)),
            signal_decl("y", SignalKind::Output, SignalType::Unsigned(8)),
        ],
        guards: vec![make_guard("g", gt_expr(sig("x"), 0), 1)],
        reflexes: vec![make_reflex("r", vec!["g"], vec![make_assignment("y", lit_int(42))])],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let result = result_from_module(module);
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("42"), "integer literal 42 must appear in output");
}

#[test]
fn expr_not_operator() {
    let module = Module {
        name: "not_op".to_string(),
        signals: vec![
            signal_decl("x", SignalKind::Input, SignalType::Bool),
            signal_decl("y", SignalKind::Output, SignalType::Bool),
        ],
        guards: vec![make_guard("g", sig("x"), 1)],
        reflexes: vec![make_reflex("r", vec!["g"], vec![make_assignment("y", not_expr(sig("x")))])],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let result = result_from_module(module);
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("(!x)"), "NOT operator must render as (!x)");
}

#[test]
fn expr_multiply_operator() {
    let module = Module {
        name: "mul_op".to_string(),
        signals: vec![
            signal_decl("a", SignalKind::Input, SignalType::Unsigned(8)),
            signal_decl("b", SignalKind::Input, SignalType::Unsigned(8)),
            signal_decl("out", SignalKind::Output, SignalType::Unsigned(16)),
        ],
        guards: vec![make_guard("g", gt_expr(sig("a"), 0), 1)],
        reflexes: vec![make_reflex(
            "r",
            vec!["g"],
            vec![make_assignment("out", mul_expr(sig("a"), sig("b")))],
        )],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let result = result_from_module(module);
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("(a * b)"), "multiply must render as (a * b)");
}

// ===========================================================================
// Section 12: Condition Expression Rendering
// ===========================================================================

#[test]
fn condition_simple_signal() {
    let source = r#"
module cond_simple {
    signal trigger: in bool;
    signal out: out bool;

    guard g {
        when trigger
        for 3 cycles;
    }

    reflex r {
        on g {
            out = true;
        }
    }
}
"#;
    let result = run_pipeline(source, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(
        sv.contains("assign g_cond = trigger"),
        "simple signal condition must assign directly: got\n{sv}"
    );
}

#[test]
fn condition_negated_signal() {
    let source = r#"
module cond_negated {
    signal active: in bool;
    signal out: out bool;

    guard g {
        when !active
        for 3 cycles;
    }

    reflex r {
        on g {
            out = true;
        }
    }
}
"#;
    let result = run_pipeline(source, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("!active"), "negated signal condition must contain !active");
}

#[test]
fn condition_comparison_gt() {
    let source = r#"
module cond_cmp {
    signal pressure: in u16;
    signal alarm: out bool;

    guard g {
        when pressure > 500
        for 4 cycles;
    }

    reflex r {
        on g {
            alarm = true;
        }
    }
}
"#;
    let result = run_pipeline(source, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("pressure > 500"), "comparison condition must contain 'pressure > 500'");
}

// ===========================================================================
// Section 13: SVA Property Generation — All 6 Formula Variants
// ===========================================================================

#[test]
fn sva_always_formula() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("// ── Safety Properties (SVA) ──"), "must have SVA section header");
    assert!(sv.contains("assert property"), "always formula must use assert property");
}

#[test]
fn sva_never_formula_negation() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    // Never formula wraps in !(...)
    assert!(sv.contains("!("), "never formula must negate the expression");
}

#[test]
fn sva_always_implies_operator() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("|->"), "always-implies formula must contain |-> operator");
}

#[test]
fn sva_never_implies_negated_implication() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    // never-implies has both |-> and !(...)
    let sva_section_start =
        sv.find("// ── Safety Properties (SVA) ──").expect("must find SVA section");
    let sva_section = &sv[sva_section_start..];

    assert!(sva_section.contains("|->"), "never-implies must contain |-> in SVA section");
}

#[test]
fn sva_eventually_within_temporal() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("##[1:10]"), "eventually within 10 must produce ##[1:10]");
}

#[test]
fn sva_always_followed_by_delay() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("|-> ##3"), "always followed_by 3 must produce |-> ##3");
}

#[test]
fn sva_posedge_clk_clock() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("@(posedge clk)"), "SVA properties must be clocked on posedge clk");
}

#[test]
fn sva_no_properties_when_empty() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(
        !sv.contains("// ── Safety Properties (SVA) ──"),
        "must NOT have SVA section when no properties exist"
    );
}

// ===========================================================================
// Section 14: SVA Directives (Assert, Cover, Assume)
// ===========================================================================

#[test]
fn sva_directive_assert_keyword() {
    let module = Module {
        name: "dir_assert".to_string(),
        signals: vec![
            signal_decl("s", SignalKind::Input, SignalType::Unsigned(16)),
            signal_decl("o", SignalKind::Output, SignalType::Bool),
        ],
        guards: vec![make_guard("g", gt_expr(sig("s"), 0), 1)],
        reflexes: vec![make_reflex("r", vec!["g"], vec![make_assignment("o", lit_bool(true))])],
        properties: vec![make_property(
            "p",
            PropertyDirective::Assert,
            PropertyFormula::Always(gt_expr(sig("s"), 0)),
        )],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let result = result_from_module(module);
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("assert property"), "Assert directive must produce 'assert property'");
}

#[test]
fn sva_directive_cover_keyword() {
    let module = Module {
        name: "dir_cover".to_string(),
        signals: vec![
            signal_decl("s", SignalKind::Input, SignalType::Unsigned(16)),
            signal_decl("o", SignalKind::Output, SignalType::Bool),
        ],
        guards: vec![make_guard("g", gt_expr(sig("s"), 0), 1)],
        reflexes: vec![make_reflex("r", vec!["g"], vec![make_assignment("o", lit_bool(true))])],
        properties: vec![make_property(
            "p",
            PropertyDirective::Cover,
            PropertyFormula::Always(gt_expr(sig("s"), 0)),
        )],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let result = result_from_module(module);
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("cover property"), "Cover directive must produce 'cover property'");
}

#[test]
fn sva_directive_assume_keyword() {
    let module = Module {
        name: "dir_assume".to_string(),
        signals: vec![
            signal_decl("s", SignalKind::Input, SignalType::Unsigned(16)),
            signal_decl("o", SignalKind::Output, SignalType::Bool),
        ],
        guards: vec![make_guard("g", gt_expr(sig("s"), 0), 1)],
        reflexes: vec![make_reflex("r", vec!["g"], vec![make_assignment("o", lit_bool(true))])],
        properties: vec![make_property(
            "p",
            PropertyDirective::Assume,
            PropertyFormula::Always(gt_expr(sig("s"), 0)),
        )],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let result = result_from_module(module);
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("assume property"), "Assume directive must produce 'assume property'");
}

// ===========================================================================
// Section 15: SVA Disable Iff (rst_n handling)
// ===========================================================================

#[test]
fn sva_disable_iff_with_rst_n_input() {
    // When module has rst_n as input, properties should have disable iff (!rst_n)
    let module = Module {
        name: "rst_mod".to_string(),
        signals: vec![
            signal_decl("rst_n", SignalKind::Input, SignalType::Bool),
            signal_decl("s", SignalKind::Input, SignalType::Unsigned(16)),
            signal_decl("o", SignalKind::Output, SignalType::Bool),
        ],
        guards: vec![make_guard("g", gt_expr(sig("s"), 0), 1)],
        reflexes: vec![make_reflex("r", vec!["g"], vec![make_assignment("o", lit_bool(true))])],
        properties: vec![make_property(
            "p",
            PropertyDirective::Assert,
            PropertyFormula::Always(gt_expr(sig("s"), 0)),
        )],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let result = result_from_module(module);
    let sv = verilog::emit_sv(&result);

    assert!(
        sv.contains("disable iff (!rst_n)"),
        "SVA must have 'disable iff (!rst_n)' when rst_n is an input signal"
    );
}

#[test]
fn sva_no_disable_iff_without_rst_n() {
    // Without rst_n input, no disable clause
    let module = Module {
        name: "no_rst".to_string(),
        signals: vec![
            signal_decl("s", SignalKind::Input, SignalType::Unsigned(16)),
            signal_decl("o", SignalKind::Output, SignalType::Bool),
        ],
        guards: vec![make_guard("g", gt_expr(sig("s"), 0), 1)],
        reflexes: vec![make_reflex("r", vec!["g"], vec![make_assignment("o", lit_bool(true))])],
        properties: vec![make_property(
            "p",
            PropertyDirective::Assert,
            PropertyFormula::Always(gt_expr(sig("s"), 0)),
        )],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let result = result_from_module(module);
    let sv = verilog::emit_sv(&result);

    assert!(
        !sv.contains("disable iff"),
        "SVA must NOT have 'disable iff' without rst_n input signal"
    );
}

// ===========================================================================
// Section 16: Bind File Generation
// ===========================================================================

#[test]
fn bind_file_empty_without_properties() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let bind = verilog::emit_sva_bind_file(&result);

    assert!(bind.is_empty(), "bind file must be empty when module has no properties");
}

#[test]
fn bind_file_contains_sva_module_name() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let bind = verilog::emit_sva_bind_file(&result);

    assert!(bind.contains("module prop_mod_sva"), "bind file must define _sva wrapper module");
}

#[test]
fn bind_file_contains_bind_statement() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let bind = verilog::emit_sva_bind_file(&result);

    assert!(
        bind.contains("bind prop_mod prop_mod_sva u_sva (.*)"),
        "bind file must contain bind statement targeting original module"
    );
}

#[test]
fn bind_file_has_auto_generated_comment() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let bind = verilog::emit_sva_bind_file(&result);

    assert!(
        bind.contains("Auto-generated SVA bind file"),
        "bind file must have auto-generated header"
    );
}

#[test]
fn bind_file_has_endmodule() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let bind = verilog::emit_sva_bind_file(&result);

    assert!(bind.contains("endmodule"), "bind file must contain endmodule");
}

#[test]
fn bind_file_ports_are_all_inputs() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let bind = verilog::emit_sva_bind_file(&result);

    // In the bind SVA module, all ports should be inputs (observing the DUT)
    let mod_start = bind.find("module prop_mod_sva").expect("must find sva module");
    let mod_end = bind[mod_start..].find(");").expect("must find );") + mod_start;
    let mod_decl = &bind[mod_start..mod_end];

    assert!(
        !mod_decl.contains("output"),
        "bind file ports must all be inputs (observing DUT), found 'output'"
    );
}

#[test]
fn bind_file_contains_assert_property() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let bind = verilog::emit_sva_bind_file(&result);

    assert!(bind.contains("assert property"), "bind file must contain SVA assertions");
}

// ===========================================================================
// Section 17: Synthesis-Clean Mode
// ===========================================================================

#[test]
fn synthesis_mode_strips_sva() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let sv = verilog::emit_sv_synthesis(&result, None, 0);

    assert!(!sv.contains("assert property"), "synthesis mode must strip 'assert property'");
    assert!(!sv.contains("assume property"), "synthesis mode must strip 'assume property'");
    assert!(!sv.contains("cover property"), "synthesis mode must strip 'cover property'");
}

#[test]
fn synthesis_mode_preserves_module() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let sv = verilog::emit_sv_synthesis(&result, None, 0);

    assert!(sv.contains("module prop_mod"), "synthesis mode must preserve module declaration");
    assert!(sv.contains("endmodule"), "synthesis mode must preserve endmodule");
}

#[test]
fn synthesis_mode_preserves_rtl() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let sv = verilog::emit_sv_synthesis(&result, None, 0);

    assert!(sv.contains("always_ff"), "synthesis mode must preserve always_ff blocks");
    assert!(sv.contains("always_comb"), "synthesis mode must preserve always_comb blocks");
}

#[test]
fn synthesis_mode_no_sva_section_header() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let sv = verilog::emit_sv_synthesis(&result, None, 0);

    assert!(
        !sv.contains("Safety Properties (SVA)"),
        "synthesis mode must strip the SVA section header"
    );
}

// ===========================================================================
// Section 18: SVA-Only Mode
// ===========================================================================

#[test]
fn sva_only_no_module_wrapper() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let sva = verilog::emit_sva_only(&result);

    assert!(!sva.contains("module prop_mod ("), "sva_only must NOT contain module declaration");
    assert!(!sva.contains("endmodule"), "sva_only must NOT contain endmodule");
}

#[test]
fn sva_only_contains_assertions() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let sva = verilog::emit_sva_only(&result);

    assert!(sva.contains("assert property"), "sva_only must contain SVA assertions");
}

#[test]
fn sva_only_has_module_comment() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let sva = verilog::emit_sva_only(&result);

    assert!(sva.contains("Module: prop_mod"), "sva_only must reference the module name");
}

#[test]
fn sva_only_empty_for_no_properties() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sva = verilog::emit_sva_only(&result);

    // The header is still emitted but no assertions follow
    assert!(
        !sva.contains("assert property"),
        "sva_only must have no assertions when module has no properties"
    );
}

// ===========================================================================
// Section 19: Synchronizer Chain Emission
// ===========================================================================

#[test]
fn synchronizer_chains_header_comment() {
    let module = Module {
        name: "sync_test".to_string(),
        signals: vec![
            signal_decl("clk", SignalKind::Input, SignalType::Bool),
            signal_decl("rst_n", SignalKind::Input, SignalType::Bool),
            signal_decl("data_in", SignalKind::Input, SignalType::Unsigned(8)),
            signal_decl("data_out", SignalKind::Output, SignalType::Unsigned(8)),
        ],
        guards: vec![],
        reflexes: vec![],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let mut out = String::new();
    let mappings = verilog::emit_synchronizer_chains(&module, 2, &mut out);

    assert!(
        out.contains("// ── Input Synchronizer Chains ──"),
        "must have synchronizer section header"
    );
    assert!(!mappings.is_empty(), "must produce mappings for input signals");
}

#[test]
fn synchronizer_chains_skip_clk_rst() {
    let module = Module {
        name: "sync_skip".to_string(),
        signals: vec![
            signal_decl("clk", SignalKind::Input, SignalType::Bool),
            signal_decl("rst_n", SignalKind::Input, SignalType::Bool),
            signal_decl("data", SignalKind::Input, SignalType::Unsigned(8)),
        ],
        guards: vec![],
        reflexes: vec![],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let mut out = String::new();
    let mappings = verilog::emit_synchronizer_chains(&module, 2, &mut out);

    // Only data should be synchronized, not clk or rst_n
    let mut found_clk_sync = false;
    let mut found_data_sync = false;
    for i in 0..MAX_PORTS_CHECK {
        if i >= mappings.len() {
            break;
        }
        let (orig, _synced) = &mappings[i];
        if orig == "clk" || orig == "rst_n" {
            found_clk_sync = true;
        }
        if orig == "data" {
            found_data_sync = true;
        }
    }

    assert!(!found_clk_sync, "clk and rst_n must NOT be synchronized");
    assert!(found_data_sync, "data signal must be synchronized");
}

#[test]
fn synchronizer_chains_produces_sync_register() {
    let module = Module {
        name: "sync_reg".to_string(),
        signals: vec![
            signal_decl("clk", SignalKind::Input, SignalType::Bool),
            signal_decl("rst_n", SignalKind::Input, SignalType::Bool),
            signal_decl("sig_in", SignalKind::Input, SignalType::Unsigned(4)),
        ],
        guards: vec![],
        reflexes: vec![],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let mut out = String::new();
    verilog::emit_synchronizer_chains(&module, 2, &mut out);

    assert!(out.contains("sig_in_sync"), "must declare synchronizer register");
    assert!(out.contains("sig_in_s"), "must declare synchronized output signal");
    assert!(out.contains("always_ff"), "synchronizer must use always_ff");
}

#[test]
fn synchronizer_chains_zero_stages_returns_empty() {
    let module = Module {
        name: "sync_zero".to_string(),
        signals: vec![signal_decl("data", SignalKind::Input, SignalType::Unsigned(8))],
        guards: vec![],
        reflexes: vec![],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let mut out = String::new();
    let mappings = verilog::emit_synchronizer_chains(&module, 0, &mut out);

    assert!(mappings.is_empty(), "zero sync stages must return empty mappings");
    assert!(out.is_empty(), "zero sync stages must produce no output");
}

#[test]
fn synchronizer_chains_skip_output_signals() {
    let module = Module {
        name: "sync_out".to_string(),
        signals: vec![
            signal_decl("clk", SignalKind::Input, SignalType::Bool),
            signal_decl("rst_n", SignalKind::Input, SignalType::Bool),
            signal_decl("data_in", SignalKind::Input, SignalType::Unsigned(8)),
            signal_decl("data_out", SignalKind::Output, SignalType::Unsigned(8)),
        ],
        guards: vec![],
        reflexes: vec![],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let mut out = String::new();
    let mappings = verilog::emit_synchronizer_chains(&module, 2, &mut out);

    let mut found_output_sync = false;
    for i in 0..MAX_PORTS_CHECK {
        if i >= mappings.len() {
            break;
        }
        if mappings[i].0 == "data_out" {
            found_output_sync = true;
        }
    }

    assert!(!found_output_sync, "output signals must NOT be synchronized (only inputs)");
}

// ===========================================================================
// Section 20: Pattern Origin Annotations
// ===========================================================================

#[test]
fn pattern_origin_comment_in_output() {
    let module = Module {
        name: "pat_mod".to_string(),
        signals: vec![
            signal_decl("a", SignalKind::Input, SignalType::Unsigned(8)),
            signal_decl("b", SignalKind::Output, SignalType::Unsigned(8)),
        ],
        guards: vec![make_guard("g", gt_expr(sig("a"), 0), 1)],
        reflexes: vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![make_assignment("b", sig("a"))],
            origin: Some("watchdog(10, threshold)".to_string()),
            span: None,
        }],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![nasa_rust_project::ast::pattern::PatternOrigin {
            pattern_name: "watchdog".to_string(),
            call_args_summary: "10, threshold".to_string(),
        }],
        span: None,
    };

    let result = result_from_module(module);
    let sv = verilog::emit_sv(&result);

    assert!(
        sv.contains("// Pattern: watchdog(10, threshold)"),
        "must emit pattern expansion annotation"
    );
}

#[test]
fn no_pattern_section_without_origins() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(
        !sv.contains("// ── Pattern Expansions ──"),
        "must NOT have pattern section when no pattern origins"
    );
}

// ===========================================================================
// Section 21: Property with Origin Tag
// ===========================================================================

#[test]
fn property_origin_comment_in_sva() {
    let module = Module {
        name: "prop_origin".to_string(),
        signals: vec![
            signal_decl("s", SignalKind::Input, SignalType::Unsigned(16)),
            signal_decl("o", SignalKind::Output, SignalType::Bool),
        ],
        guards: vec![make_guard("g", gt_expr(sig("s"), 0), 1)],
        reflexes: vec![make_reflex("r", vec!["g"], vec![make_assignment("o", lit_bool(true))])],
        properties: vec![PropertyDecl {
            name: "traceability_prop".to_string(),
            directive: PropertyDirective::Assert,
            formula: PropertyFormula::Always(gt_expr(sig("s"), 0)),
            origin: Some("safety_watchdog".to_string()),
            span: None,
        }],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let result = result_from_module(module);
    let sv = verilog::emit_sv(&result);

    assert!(
        sv.contains("// Pattern: safety_watchdog"),
        "property origin tag must appear as comment in SVA output"
    );
}

// ===========================================================================
// Section 22: Temporal Guard Section Header
// ===========================================================================

#[test]
fn temporal_section_header_present() {
    let result = run_pipeline(MULTI_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("// ── Temporal Guards ──"), "must have temporal guards section header");
}

#[test]
fn no_temporal_section_without_netlist() {
    let mut cfg = default_config();
    cfg.temporal = false;
    let result = run_pipeline(NO_GUARD_MODULE, &cfg).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(
        !sv.contains("// ── Temporal Guards ──"),
        "must NOT have temporal section when temporal stage is disabled"
    );
}

// ===========================================================================
// Section 23: Output Line Count Sanity
// ===========================================================================

#[test]
fn output_has_reasonable_line_count() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);
    let lines = count_lines_bounded(&sv);

    assert!(lines > 5, "minimal module output must have at least 5 lines, got {}", lines);
    assert!(
        lines < MAX_OUTPUT_LINES,
        "output must not exceed {} lines, got {}",
        MAX_OUTPUT_LINES,
        lines
    );
}

#[test]
fn property_heavy_output_bounded() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);
    let lines = count_lines_bounded(&sv);

    assert!(lines > 20, "property-heavy module must have substantial output, got {} lines", lines);
    assert!(lines < MAX_OUTPUT_LINES, "output must stay within bounds, got {} lines", lines);
}

// ===========================================================================
// Section 24: All Binary Operators Render Correctly
// ===========================================================================

#[test]
fn all_binary_ops_in_expression() {
    // Test that all 13 binary operators render with correct SV syntax.
    let ops_and_symbols: [(BinaryOp, &str); 13] = [
        (BinaryOp::And, "&"),
        (BinaryOp::Or, "|"),
        (BinaryOp::Xor, "^"),
        (BinaryOp::Lt, "<"),
        (BinaryOp::Le, "<="),
        (BinaryOp::Gt, ">"),
        (BinaryOp::Ge, ">="),
        (BinaryOp::Eq, "=="),
        (BinaryOp::Ne, "!="),
        (BinaryOp::Add, "+"),
        (BinaryOp::Sub, "-"),
        (BinaryOp::Mul, "*"),
        (BinaryOp::Shl, "<<"),
    ];

    for i in 0..MAX_PROPERTY_VARIANTS {
        if i >= ops_and_symbols.len() {
            break;
        }
        let (op, expected_sym) = ops_and_symbols[i];
        let module = Module {
            name: format!("op_test_{i}"),
            signals: vec![
                signal_decl("a", SignalKind::Input, SignalType::Unsigned(8)),
                signal_decl("b", SignalKind::Input, SignalType::Unsigned(8)),
                signal_decl("out", SignalKind::Output, SignalType::Unsigned(8)),
            ],
            guards: vec![make_guard("g", gt_expr(sig("a"), 0), 1)],
            reflexes: vec![make_reflex(
                "r",
                vec!["g"],
                vec![make_assignment(
                    "out",
                    Expr::Binary { op, left: Box::new(sig("a")), right: Box::new(sig("b")) },
                )],
            )],
            properties: vec![],
            pattern_calls: vec![],
            pattern_origins: vec![],
            span: None,
        };

        let result = result_from_module(module);
        let sv = verilog::emit_sv(&result);

        let expected_expr = format!("(a {expected_sym} b)");
        assert!(
            sv.contains(&expected_expr),
            "binary op {:?} must render as '{}' in SV output, got:\n{}",
            op,
            expected_expr,
            sv
        );
    }
}

// ===========================================================================
// Section 25: Full Pipeline Integration (MIRR source -> SV)
// ===========================================================================

#[test]
fn full_pipeline_round_trip_minimal() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    // Verify all major sections are present
    assert!(sv.contains("Auto-generated"), "missing header");
    assert!(sv.contains("module minimal"), "missing module declaration");
    assert!(sv.contains("always_comb"), "missing always_comb");
    assert!(sv.contains("endmodule"), "missing endmodule");
}

#[test]
fn full_pipeline_counter_guard_module() {
    let result = run_pipeline(COUNTER_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("module counter_mod"), "missing module name");
    assert!(sv.contains("always_ff"), "counter guard must have always_ff");
    assert!(sv.contains("always_comb"), "reflex must have always_comb");
    assert!(sv.contains(">= 100"), "counter must compare against 100");
}

#[test]
fn full_pipeline_multi_guard_module() {
    let result = run_pipeline(MULTI_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("module multi_guard_mod"), "missing module name");
    assert!(sv.contains("g1_sr"), "missing g1 shift register");
    assert!(sv.contains("g2_sr"), "missing g2 shift register");
    assert!(sv.contains("g1_out && g2_out"), "missing AND-joined guard condition");
}

#[test]
fn full_pipeline_all_properties_module() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    // Verify all 6 property names appear as comments
    let property_names =
        ["p_always", "p_never", "p_implies", "p_never_implies", "p_eventually", "p_followed_by"];
    for i in 0..MAX_PROPERTY_VARIANTS {
        if i >= property_names.len() {
            break;
        }
        assert!(
            sv.contains(&format!("// property: {}", property_names[i])),
            "missing property comment for {}",
            property_names[i]
        );
    }
}

// ===========================================================================
// Section 26: Edge Cases
// ===========================================================================

#[test]
fn empty_module_no_crash() {
    // Module with only IO signals, no guards or reflexes
    let result = run_pipeline(NO_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("module bare"), "must contain module name");
    assert!(sv.contains("endmodule"), "must contain endmodule");
    assert!(!sv.contains("always_ff"), "must not contain always_ff");
    assert!(!sv.contains("always_comb"), "must not contain always_comb");
}

#[test]
fn multiple_assignments_in_single_reflex() {
    let result = run_pipeline(INTERNAL_SIGNALS_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    // The reflex has two assignments: accumulator = sensor and result = accumulator
    assert!(sv.contains("accumulator"), "must contain accumulator assignment");
    assert!(sv.contains("result"), "must contain result assignment");
}

#[test]
fn condition_boolean_literal_in_comparison() {
    // Build a netlist with a bool-valued comparison condition
    let module = Module {
        name: "bool_cmp".to_string(),
        signals: vec![
            signal_decl("flag", SignalKind::Input, SignalType::Bool),
            signal_decl("out", SignalKind::Output, SignalType::Bool),
        ],
        guards: vec![],
        reflexes: vec![],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let mut netlist = TemporalNetlist::new();
    let ck = ConditionKind::Comparison {
        signal: "flag".to_string(),
        op: BinaryOp::Eq,
        value: LiteralValue::Bool(true),
    };
    let sr = ShiftRegisterGuard::new("bool_guard".to_string(), "flag".to_string(), 2, ck);
    netlist.add_guard(CompiledGuard::ShiftRegister(sr));

    let result = result_with_netlist(module, netlist);
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("1'b1"), "bool comparison value true must render as 1'b1");
}

#[test]
fn condition_boolean_false_literal() {
    let module = Module {
        name: "bool_false_cmp".to_string(),
        signals: vec![
            signal_decl("flag", SignalKind::Input, SignalType::Bool),
            signal_decl("out", SignalKind::Output, SignalType::Bool),
        ],
        guards: vec![],
        reflexes: vec![],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let mut netlist = TemporalNetlist::new();
    let ck = ConditionKind::Comparison {
        signal: "flag".to_string(),
        op: BinaryOp::Eq,
        value: LiteralValue::Bool(false),
    };
    let sr = ShiftRegisterGuard::new("bool_false_g".to_string(), "flag".to_string(), 2, ck);
    netlist.add_guard(CompiledGuard::ShiftRegister(sr));

    let result = result_with_netlist(module, netlist);
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("1'b0"), "bool comparison value false must render as 1'b0");
}

#[test]
fn negate_expr_renders_correctly() {
    let module = Module {
        name: "negate_test".to_string(),
        signals: vec![
            signal_decl("x", SignalKind::Input, SignalType::Unsigned(8)),
            signal_decl("y", SignalKind::Output, SignalType::Unsigned(8)),
        ],
        guards: vec![make_guard("g", gt_expr(sig("x"), 0), 1)],
        reflexes: vec![make_reflex(
            "r",
            vec!["g"],
            vec![make_assignment(
                "y",
                Expr::Unary {
                    op: nasa_rust_project::ast::types::UnaryOp::Negate,
                    operand: Box::new(sig("x")),
                },
            )],
        )],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let result = result_from_module(module);
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("(-x)"), "negate operator must render as (-x)");
}

// ===========================================================================
// Section 27: Shift Right Operator
// ===========================================================================

#[test]
fn shr_operator_renders() {
    let module = Module {
        name: "shr_test".to_string(),
        signals: vec![
            signal_decl("a", SignalKind::Input, SignalType::Unsigned(16)),
            signal_decl("out", SignalKind::Output, SignalType::Unsigned(16)),
        ],
        guards: vec![make_guard("g", gt_expr(sig("a"), 0), 1)],
        reflexes: vec![make_reflex(
            "r",
            vec!["g"],
            vec![make_assignment(
                "out",
                Expr::Binary {
                    op: BinaryOp::Shr,
                    left: Box::new(sig("a")),
                    right: Box::new(lit_int(2)),
                },
            )],
        )],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let result = result_from_module(module);
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("(a >> 2)"), "SHR operator must render as (a >> 2)");
}
