use mirrc::ast::types::SignalKind;
use mirrc::ecs::components::{EntityKind, KindComponent, TypeComponent};
use mirrc::ecs::Registry;
use mirrc::emit::fpga_target::FpgaTarget;
use mirrc::emit::fpga_scaffold::{emit_constraints, emit_build_script};
use mirrc::pipeline::PipelineResult;
use mirrc::ast::types::{ExtendedType, SignalType};
use mirrc::span::FileTable;

#[test]
fn test_fpga_target_coverage() {
    let targets = vec![
        FpgaTarget::Xilinx7,
        FpgaTarget::XilinxUS,
        FpgaTarget::IntelCyclone,
        FpgaTarget::LatticeIce40,
        FpgaTarget::LatticeEcp5,
        FpgaTarget::LatticeNexus,
        FpgaTarget::Generic,
    ];

    for target in targets {
        assert!(!target.display_name().is_empty());
        assert!(!target.constraint_extension().is_empty());
        
        let build_tool = target.build_tool();
        assert!(!build_tool.is_empty());
        
        if let Some(pack_tool) = target.pack_tool() {
            assert!(!pack_tool.is_empty());
        }
        
        assert!(!target.default_part().is_empty());
        assert!(!target.clock_primitive().is_empty());
        assert!(!target.dsp_primitive().is_empty());
        assert!(!target.dsp_attribute().is_empty());
        assert!(target.dsp_max_input_width() > 0);
        
        if let Some(bin) = target.nextpnr_binary() {
            assert!(!bin.is_empty());
        }
        
        if let Some(dev) = target.icetime_device() {
            assert!(!dev.is_empty());
        }
        
        assert!(!target.yosys_synth_command().is_empty());
    }

    assert_eq!(FpgaTarget::from_str_name("xilinx-7").unwrap(), FpgaTarget::Xilinx7);
    assert_eq!(FpgaTarget::from_str_name("unknown").is_none(), true);
}

#[test]
fn test_fpga_scaffold_coverage() {
    let mut reg = Registry::new();
    let _mod_id = reg.create_entity("test_mod", KindComponent(EntityKind::MODULE));
    
    let sig_in = reg.create_entity("sig_in", KindComponent(EntityKind::SIGNAL(SignalKind::Input)));
    let sig_out = reg.create_entity("sig_out", KindComponent(EntityKind::SIGNAL(SignalKind::Output)));
    let sig_int = reg.create_entity("sig_int", KindComponent(EntityKind::SIGNAL(SignalKind::Internal)));
    let sig_out_wide = reg.create_entity("sig_out_wide", KindComponent(EntityKind::SIGNAL(SignalKind::Output)));
    
    reg.types[sig_in.0 as usize] = Some(TypeComponent(ExtendedType::from_core(SignalType::Unsigned(1))));
    reg.types[sig_out.0 as usize] = Some(TypeComponent(ExtendedType::from_core(SignalType::Unsigned(1))));
    reg.types[sig_int.0 as usize] = Some(TypeComponent(ExtendedType::from_core(SignalType::Unsigned(1))));
    reg.types[sig_out_wide.0 as usize] = Some(TypeComponent(ExtendedType::from_core(SignalType::Unsigned(4))));

    let result = PipelineResult {
        program: None,
        simplify_stats: None,
        sat_stats: None,
        width_stats: None,
        width_diagnostics: vec![],
        temporal_netlist: None,
        rspu_program: None,
        extended_type_map: None,
        sim_result: None,
        mape_k_result: None,
        retiming_stats: None,
        totality_result: None,
        symbolic_result: None,
        mape_k_rtl: None,
        hls_result: None,
        file_table: FileTable::new(),
        ecs_registry: Some(reg),
    };

    let targets = vec![
        FpgaTarget::Xilinx7,
        FpgaTarget::IntelCyclone,
        FpgaTarget::LatticeIce40,
        FpgaTarget::LatticeEcp5,
        FpgaTarget::LatticeNexus,
        FpgaTarget::Generic,
    ];

    for target in targets {
        let constraints = emit_constraints(&result, &target);
        assert!(!constraints.is_empty());
        let build_script = emit_build_script(&result, &target);
        assert!(!build_script.is_empty());
    }

    let missing_result = PipelineResult {
        program: None,
        simplify_stats: None,
        sat_stats: None,
        width_stats: None,
        width_diagnostics: vec![],
        temporal_netlist: None,
        rspu_program: None,
        extended_type_map: None,
        sim_result: None,
        mape_k_result: None,
        retiming_stats: None,
        totality_result: None,
        symbolic_result: None,
        mape_k_rtl: None,
        hls_result: None,
        file_table: FileTable::new(),
        ecs_registry: None,
    };
    assert!(emit_constraints(&missing_result, &FpgaTarget::Generic).contains("No ECS registry"));
    assert!(emit_build_script(&missing_result, &FpgaTarget::Generic).contains("No ECS registry"));
}
