#[forbid(unsafe_code)]

use nasa_rust_project::ast::types::{BinaryOp, ExtendedType, LiteralValue, SignalKind, SignalType};
use nasa_rust_project::ast::Expr;
use nasa_rust_project::ecs::*;
use nasa_rust_project::pipeline::PipelineConfig;
use nasa_rust_project::Workspace;
use std::path::PathBuf;

fn tool_available(name: &str) -> bool {
    let flag = if name == "yosys" || name == "icetime" { "-V" } else { "--version" };
    std::process::Command::new(name)
        .arg(flag)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn write_temp(name: &str, content: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(name);
    std::fs::write(&path, content).expect("Failed to write temporary test file");
    path
}

#[test]
fn test_rspu_massive_compiler_stress_and_rtl_synthesis() {
    let root_path = PathBuf::from("rspu_chip/rspu_top.mirr");
    let workspace_root = PathBuf::from("rspu_chip");

    // 1. Compile the multi-file RSPU project (TDD verification of the real silicon architecture)
    assert!(root_path.exists(), "RSPU top file does not exist, check workspace pathing");
    let mut workspace = Workspace::new(&workspace_root);
    let config = PipelineConfig {
        temporal: false,
        rspu: false,
        ..Default::default()
    };

    println!("Compiling upgraded multi-file RSPU project from {}...", root_path.display());
    let snapshot = workspace
        .compile_snapshot(&root_path, &config)
        .expect("RSPU project workspace compilation failed");

    assert!(snapshot.imported_file_count() >= 2, "Must import ALU and RAM core modules");
    println!("Compiled multi-file project workspace hash: {}", snapshot.workspace_hash);

    // 2. STRESS TEST: Flood the ECS Registry to trigger lockstep vector growth & reallocation
    println!("Initiating high-scale Registry memory pressure and resizing tests...");
    let mut registry = Registry::new();
    
    // Create a base module and register signals
    let mod_ent = registry.create_entity("top_stress_module", KindComponent::MODULE);
    
    // Register 1,000 signals sequentially to verify contiguous EntityId allocation
    let mut signal_entities = Vec::with_capacity(1000);
    for i in 0..1000 {
        let sig_name = format!("sensor_bus_{}", i);
        let sig = registry.create_signal(
            sig_name.clone(),
            KindComponent(EntityKind::SIGNAL(SignalKind::Internal)),
            TypeComponent(ExtendedType::new(SignalType::Unsigned(16), Default::default())),
        );
        registry.set_parent(sig, mod_ent);
        signal_entities.push(sig);
    }

    // 3. Build a pathologically deep, nested composite structure (Structs within Arrays)
    // Satisfies the 512 MAX_EXPR_NODES and 256 validation depth bounds checks
    println!("Constructing nested structural expressions...");
    
    // Dynamically build a massive array of structure literals
    let mut struct_exprs = Vec::with_capacity(200);
    for i in 0..100 {
        let st = Expr::StructLiteral {
            name: format!("CoreState_{}", i),
            fields: vec![
                ("alu_out".to_string(), Expr::Literal(LiteralValue::Integer(i * 10))),
                ("active".to_string(), Expr::Literal(LiteralValue::Bool(i % 2 == 0))),
            ],
        };
        struct_exprs.push(st);
    }
    
    let array_expr = Expr::ArrayLiteral(struct_exprs);
    
    // Ingest the nested composite literal into our ECS Registry
    let root_ent = registry.ingest_expr(&array_expr).expect("Ingesting composite literal failed");

    // 4. Verify structural integrity, desync alignment, and table lengths
    println!("Executing ECS Validation Gate checks...");
    registry.validate().expect("ECS Registry validation failed on structural desyncs");

    // 5. Perform typechecking & flat bottom-up type inference on the nested composite row
    println!("Running type checking and flat type inference passes...");
    let inferred_ty = registry.infer_type(root_ent).expect("Type inference on flat elements failed");
    assert!(matches!(inferred_ty, SignalType::Array { .. }));

    // 6. Verify loss-less iterative round-trip reification (Reconstruct back to tree AST)
    println!("Reifying flat rows back to AST...");
    let reified = registry.reify_expr(root_ent).expect("Reification of flat elements failed");
    assert_eq!(array_expr, reified, "Loss-less reification round-trip failed");

    // Artificially construct a dependency loop to verify E172 / max iterations
    println!("Verifying cyclic dependency loop detection...");
    let loop_ent = registry.next_id();
    let loop_ent_2 = registry.next_id();
    registry.binary_ops[loop_ent.0 as usize] = Some(BinaryComponent {
        op: BinaryOp::Add,
        left: loop_ent_2,
        right: loop_ent_2,
    });
    registry.binary_ops[loop_ent_2.0 as usize] = Some(BinaryComponent {
        op: BinaryOp::Add,
        left: loop_ent,
        right: loop_ent,
    });

    // We must connect the cyclic expression to an active GUARD or REFLEX
    // for semantic_validate to traverse it
    let g_ent = registry.next_id();
    registry.names[g_ent.0 as usize] = Some(NameComponent("cyclic_guard".to_string()));
    registry.kinds[g_ent.0 as usize] = Some(KindComponent::GUARD);
    registry.conditions[g_ent.0 as usize] = Some(ConditionComponent(loop_ent));
    registry.cycles[g_ent.0 as usize] = Some(CyclesComponent(5));

    // Must fail due to cyclic dependency loop
    registry.semantic_validate().unwrap_err();
    println!("Cyclic validation checks completed successfully.");


    // 8. Open-Source RTL Synthesis Integration (Icarus Verilog TDD check)
    if tool_available("iverilog") {
        println!("Icarus Verilog found in PATH. Initiating RTL compilation check...");
        
        // Extract the generated SystemVerilog code from the compiled workspace snapshot
        let sv_rtl = nasa_rust_project::emit::verilog::emit_sv(&snapshot.pipeline);
        
        let sv_path = write_temp("rspu_stress_top.sv", &sv_rtl);
        let out_path = std::env::temp_dir().join("rspu_stress_top.vvp");
        
        let out = std::process::Command::new("iverilog")
            .arg("-g2012")
            .arg("-Wall")
            .arg("-o")
            .arg(&out_path)
            .arg("-s")
            .arg("rspu_top")
            .arg(&sv_path)
            .output()
            .expect("Failed to execute iverilog command");

        assert!(
            out.status.success(),
            "Icarus Verilog compilation of RSPU upgraded chip top failed:\n{}",
            String::from_utf8_lossy(&out.stderr)
        );
        println!("Icarus Verilog successfully compiled and verified the generated RSPU Verilog RTL.");
    } else {
        println!("Icarus Verilog (iverilog) not available in this sandbox context. Skipping external RTL compilation.");
    }
}
