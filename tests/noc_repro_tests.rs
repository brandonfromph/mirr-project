use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};

#[test]
fn test_rspu_top_noc_repro() {
    let source = r#"
    def noc_router(
        clk: signal in bool, rst_n: signal in bool,
        tx_v0: signal in bool, tx_d0: signal in u64,
        rx_v0: signal out bool, rx_d0: signal out u64,
        tx_v1: signal in bool, tx_d1: signal in u64,
        rx_v1: signal out bool, rx_d1: signal out u64
    ) {
        reflect {
            guard g { when true for 1 cycles; }
            reflex r { on g { ${rx_d0} = ${tx_d0}; } }
        }
    }

    module rspu_top {
        signals {
            sys_clk: in bool;
            sys_rst_n: in bool;
            tx_valid_0: in bool;
            tx_data_0: in u64;
            rx_valid_0: out bool;
            rx_data_0: out u64;
            tx_valid_1: in bool;
            tx_data_1: in u64;
            rx_valid_1: out bool;
            rx_data_1: out u64;
        }

        noc_router(
            sys_clk, sys_rst_n,
            tx_valid_0, tx_data_0, rx_valid_0, rx_data_0,
            tx_valid_1, tx_data_1, rx_valid_1, rx_data_1
        );
    }
    "#;
    let config = PipelineConfig::default();
    let result = run_pipeline(source, &config);

    if let Err(e) = &result {
        for err in &e.errors {
            println!("Error: {:?}", err);
        }
    }

    assert!(result.is_ok(), "Should parse multi-line NoC router call");
}
