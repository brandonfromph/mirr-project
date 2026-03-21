use super::*;

#[test]
fn test_alu_unary_on_uninitialized_errors() {
    let mut sim = RspuSimulator::new();
    // R192 is uninitialized -- unary op should error with E708.
    let program = make_program(vec![
        RspuInstruction::AluUnary { op: AluUnaryOp::Not, dst: 193, src: 192 },
        RspuInstruction::Halt,
    ]);
    let err = sim.step(&program).expect_err("Unary on uninitialized must error");
    let msg = err.to_string();
    assert!(msg.contains("E708"), "Error must contain E708 tag violation code, got: {msg}");
}

// ---------------------------------------------------------------------------
// 11. Temporal tier: Shift register
// ---------------------------------------------------------------------------

#[test]
fn test_sr_init_activates_guard_on_nonzero_cond() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 8 },
        RspuInstruction::SrInit { guard: 0, length: 4, cond: 192 },
        RspuInstruction::SrQuery { dst: 193, guard: 0 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("SrInit must succeed");
    assert_eq!(
        sim.registers.read(193).value,
        1,
        "Guard 0 must be active after SrInit with nonzero cond"
    );
}

#[test]
fn test_sr_init_inactive_on_zero_cond() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 0, width: 8 },
        RspuInstruction::SrInit { guard: 0, length: 4, cond: 192 },
        RspuInstruction::SrQuery { dst: 193, guard: 0 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("SrInit zero cond must succeed");
    assert_eq!(
        sim.registers.read(193).value,
        0,
        "Guard 0 must be inactive after SrInit with zero cond"
    );
}

#[test]
fn test_sr_tick_is_noop() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 8 },
        RspuInstruction::SrInit { guard: 0, length: 4, cond: 192 },
        RspuInstruction::SrTick { guard: 0 },
        RspuInstruction::SrQuery { dst: 193, guard: 0 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("SrTick must succeed");
    assert_eq!(
        sim.registers.read(193).value,
        1,
        "Guard remains active after SrTick (no-op in single-tick model)"
    );
}

// ---------------------------------------------------------------------------
// 12. Temporal tier: Counter
// ---------------------------------------------------------------------------

#[test]
fn test_ctr_init_activates_guard_on_nonzero_cond() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 5, width: 8 },
        RspuInstruction::CtrInit { guard: 1, target: 10, cond: 192 },
        RspuInstruction::CtrQuery { dst: 193, guard: 1 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("CtrInit must succeed");
    assert_eq!(
        sim.registers.read(193).value,
        1,
        "Guard 1 must be active after CtrInit with nonzero cond"
    );
}

#[test]
fn test_ctr_tick_is_noop() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 8 },
        RspuInstruction::CtrInit { guard: 2, target: 5, cond: 192 },
        RspuInstruction::CtrTick { guard: 2 },
        RspuInstruction::CtrQuery { dst: 193, guard: 2 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("CtrTick must succeed");
    assert_eq!(
        sim.registers.read(193).value,
        1,
        "Guard remains active after CtrTick (no-op in single-tick model)"
    );
}

// ---------------------------------------------------------------------------
// 13. Guard combinators
// ---------------------------------------------------------------------------

#[test]
fn test_guard_and_both_true() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 8 },
        RspuInstruction::SrInit { guard: 0, length: 1, cond: 192 },
        RspuInstruction::SrInit { guard: 1, length: 1, cond: 192 },
        RspuInstruction::GuardAnd { dst: 2, a: 0, b: 1 },
        RspuInstruction::SrQuery { dst: 193, guard: 2 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("GuardAnd must succeed");
    assert_eq!(sim.registers.read(193).value, 1, "AND(true, true) must be true");
}

#[test]
fn test_guard_and_one_false() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 0, width: 8 },
        RspuInstruction::SrInit { guard: 0, length: 1, cond: 192 },
        RspuInstruction::SrInit { guard: 1, length: 1, cond: 193 },
        RspuInstruction::GuardAnd { dst: 2, a: 0, b: 1 },
        RspuInstruction::SrQuery { dst: 194, guard: 2 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("GuardAnd one false must succeed");
    assert_eq!(sim.registers.read(194).value, 0, "AND(true, false) must be false");
}

#[test]
fn test_guard_or_one_true() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 0, width: 8 },
        RspuInstruction::SrInit { guard: 0, length: 1, cond: 192 },
        RspuInstruction::SrInit { guard: 1, length: 1, cond: 193 },
        RspuInstruction::GuardOr { dst: 2, a: 0, b: 1 },
        RspuInstruction::SrQuery { dst: 194, guard: 2 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("GuardOr must succeed");
    assert_eq!(sim.registers.read(194).value, 1, "OR(true, false) must be true");
}

#[test]
fn test_guard_or_both_false() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 0, width: 8 },
        RspuInstruction::SrInit { guard: 0, length: 1, cond: 192 },
        RspuInstruction::SrInit { guard: 1, length: 1, cond: 192 },
        RspuInstruction::GuardOr { dst: 2, a: 0, b: 1 },
        RspuInstruction::SrQuery { dst: 193, guard: 2 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("GuardOr both false must succeed");
    assert_eq!(sim.registers.read(193).value, 0, "OR(false, false) must be false");
}

// ---------------------------------------------------------------------------
// 14. ReflexIf instruction
// ---------------------------------------------------------------------------

#[test]
fn test_reflex_if_guard_active() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 8 },
        RspuInstruction::SrInit { guard: 0, length: 1, cond: 192 },
        RspuInstruction::LoadImm { dst: 193, value: 99, width: 8 },
        RspuInstruction::ReflexIf { guard: 0, dst: 194, src: 193 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("ReflexIf active must succeed");
    assert_eq!(sim.registers.read(194).value, 99, "ReflexIf must copy when guard is active");
}

#[test]
fn test_reflex_if_guard_inactive() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 0, width: 8 },
        RspuInstruction::SrInit { guard: 0, length: 1, cond: 192 },
        RspuInstruction::LoadImm { dst: 193, value: 99, width: 8 },
        RspuInstruction::ReflexIf { guard: 0, dst: 194, src: 193 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("ReflexIf inactive must succeed");
    assert_eq!(
        sim.registers.read(194).tag,
        TypeTag::Uninitialized,
        "ReflexIf must NOT copy when guard is inactive"
    );
}

// ---------------------------------------------------------------------------
// 15. Prev instruction
// ---------------------------------------------------------------------------

#[test]
fn test_prev_copies_signal() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 77, width: 8 },
        RspuInstruction::Prev { dst: 193, signal: 192, delay: 1 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("Prev must succeed");
    assert_eq!(
        sim.registers.read(193).value,
        77,
        "Prev must copy signal to dst in single-tick model"
    );
}

// ---------------------------------------------------------------------------
// 16. AssertAlways / AssertNever
// ---------------------------------------------------------------------------

#[test]
fn test_assert_always_no_violation() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 2 },
        RspuInstruction::AssertAlways { cond: 192, property_id: 10 },
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 100).expect("AssertAlways no violation must succeed");
    assert!(result.property_violations.is_empty(), "No violations when cond is nonzero");
}

#[test]
fn test_assert_always_violation() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 0, width: 2 },
        RspuInstruction::AssertAlways { cond: 192, property_id: 7 },
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 100).expect("AssertAlways violation must succeed");
    assert_eq!(result.property_violations, vec![7], "Property 7 must be violated when cond is 0");
}

#[test]
fn test_assert_never_no_violation() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 0, width: 2 },
        RspuInstruction::AssertNever { cond: 192, property_id: 20 },
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 100).expect("AssertNever no violation must succeed");
    assert!(result.property_violations.is_empty(), "No violations when cond is 0 for AssertNever");
}

#[test]
fn test_assert_never_violation() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 2 },
        RspuInstruction::AssertNever { cond: 192, property_id: 33 },
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 100).expect("AssertNever violation must succeed");
    assert_eq!(
        result.property_violations,
        vec![33],
        "Property 33 must be violated when cond is nonzero for AssertNever"
    );
}

#[test]
fn test_multiple_property_violations() {
    // MEGA-4: AssertAlways now raises PropertyFail on first violation,
    // stopping execution. We verify the first violation is recorded.
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 0, width: 2 },
        RspuInstruction::LoadImm { dst: 193, value: 1, width: 2 },
        RspuInstruction::AssertAlways { cond: 192, property_id: 0 },
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 1000).expect("Property violation must succeed");
    assert_eq!(result.property_violations, vec![0], "First violation must be recorded");
    assert_eq!(
        result.exception,
        Some(ExceptionCode::PropertyFail),
        "PropertyFail exception must be raised"
    );
}

// ---------------------------------------------------------------------------
// 17. Trap and TrapIf
// ---------------------------------------------------------------------------

#[test]
fn test_trap_raises_software_trap() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![RspuInstruction::Trap { code: 1 }, RspuInstruction::Halt]);
    let result = sim.run(&program, 100).expect("Trap must succeed");
    assert_eq!(
        result.exception,
        Some(ExceptionCode::SoftwareTrap),
        "Trap must produce SoftwareTrap exception"
    );
}

#[test]
fn test_trap_if_condition_true() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 8 },
        RspuInstruction::TrapIf { cond: 192, code: 2 },
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 100).expect("TrapIf cond true must succeed");
    assert_eq!(
        result.exception,
        Some(ExceptionCode::SoftwareTrap),
        "TrapIf with nonzero cond must trap"
    );
}

#[test]
fn test_trap_if_condition_false() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 0, width: 8 },
        RspuInstruction::TrapIf { cond: 192, code: 2 },
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 100).expect("TrapIf cond false must succeed");
    assert!(result.halted, "TrapIf with zero cond must continue to Halt");
    assert!(result.exception.is_none(), "TrapIf with zero cond must not produce exception");
}

// ---------------------------------------------------------------------------
// 18. ModeSwitch
// ---------------------------------------------------------------------------

#[test]
fn test_mode_switch_to_host() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::ModeSwitch { mode: 1 }, // Host
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("ModeSwitch to Host must succeed");
    assert_eq!(sim.exceptions.mode, ExecMode::Host, "Mode must be Host after ModeSwitch(1)");
}

#[test]
fn test_mode_switch_same_mode_tolerant() {
    let mut sim = RspuSimulator::new();
    // Reflex -> Reflex should be tolerated (no error).
    let program = make_program(vec![
        RspuInstruction::ModeSwitch { mode: 0 }, // Reflex (same as default)
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 100).expect("Same-mode switch must be tolerated");
    assert!(result.halted, "Program must halt normally after same-mode switch");
}

#[test]
fn test_mode_switch_invalid_mode() {
    let mut sim = RspuSimulator::new();
    let program =
        make_program(vec![RspuInstruction::ModeSwitch { mode: 99 }, RspuInstruction::Halt]);
    let err = sim.run(&program, 100).expect_err("Invalid mode must error");
    let msg = err.to_string();
    assert!(msg.contains("E714"), "Invalid mode error must contain E714, got: {msg}");
}

// ---------------------------------------------------------------------------
// 19. TagLoad, TagCheck, TagRead
// ---------------------------------------------------------------------------

#[test]
fn test_tag_load() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 42, width: 8 },
        RspuInstruction::TagLoad { dst: 192, tag: 1 }, // 1 = Bool
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("TagLoad must succeed");
    assert_eq!(sim.registers.read(192).tag, TypeTag::Bool, "TagLoad(1) must set tag to Bool");
    assert_eq!(sim.registers.read(192).value, 42, "TagLoad must preserve value");
}

#[test]
fn test_tag_check_pass() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 10, width: 8 },
        RspuInstruction::TagCheck { src: 192, expected: 8 }, // 8 = Unsigned{width:8}
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 100).expect("TagCheck pass must succeed");
    assert!(result.halted, "TagCheck pass must continue to Halt");
    assert!(result.exception.is_none(), "TagCheck pass must not raise exception");
}

#[test]
fn test_tag_check_fail() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 10, width: 8 },
        RspuInstruction::TagCheck { src: 192, expected: 1 }, // 1 = Bool, actual is Unsigned(8)
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 100).expect("TagCheck fail must succeed (returns exception)");
    assert_eq!(
        result.exception,
        Some(ExceptionCode::TagViolation),
        "TagCheck mismatch must raise TagViolation"
    );
}

#[test]
fn test_tag_read() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 0, width: 1 }, // Bool
        RspuInstruction::TagRead { dst: 193, src: 192 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("TagRead must succeed");
    // Bool encodes as u8 = 1.
    assert_eq!(sim.registers.read(193).value, 1, "TagRead of Bool must produce 1");
    assert_eq!(
        sim.registers.read(193).tag,
        TypeTag::Unsigned { width: 8 },
        "TagRead result must be Unsigned(8)"
    );
}

// ---------------------------------------------------------------------------
// 20. DeadlineSet
// ---------------------------------------------------------------------------

#[test]
fn test_deadline_set_no_miss() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::DeadlineSet { cycles: 100 },
        RspuInstruction::Nop,
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 200).expect("Deadline no miss must succeed");
    assert!(result.halted, "Program must halt before deadline");
    assert!(result.exception.is_none(), "No exception when deadline not reached");
}

#[test]
fn test_deadline_miss() {
    let mut sim = RspuSimulator::new();
    // Deadline at cycle 2; we execute 3 Nops before Halt so cycle will reach 2.
    let program = make_program(vec![
        RspuInstruction::DeadlineSet { cycles: 2 },
        RspuInstruction::Nop,
        RspuInstruction::Nop, // After this step, cycle=3 >= deadline(2)
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 100).expect("Deadline miss must succeed (returns exception)");
    assert_eq!(
        result.exception,
        Some(ExceptionCode::DeadlineMiss),
        "Must report DeadlineMiss when cycle reaches deadline"
    );
}

// ---------------------------------------------------------------------------
// 21. Fence instruction
// ---------------------------------------------------------------------------

#[test]
fn test_fence_is_noop() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![RspuInstruction::Fence, RspuInstruction::Halt]);
    let result = sim.run(&program, 100).expect("Fence must succeed");
    assert!(result.halted, "Fence must not prevent program from halting");
    assert_eq!(result.cycles, 2, "Fence + Halt = 2 cycles");
}

// ---------------------------------------------------------------------------
// 22. Program counter behavior
// ---------------------------------------------------------------------------

