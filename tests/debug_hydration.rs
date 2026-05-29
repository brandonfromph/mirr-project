use nasa_rust_project::parser::parse_mirr;
use nasa_rust_project::pipeline::{run_pipeline_on_program, PipelineConfig};

fn run_test(source: &str) -> Result<(), String> {
    let program = match parse_mirr(source) {
        Ok(p) => p,
        Err(e) => return Err(format!("{:?}", e)),
    };
    let config = PipelineConfig { bootstrap_mode: true, ..Default::default() };
    match run_pipeline_on_program(program, &config) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:?}", e)),
    }
}

#[test]
fn test_hydration_bug() {
    let source = "
    module test {
        signal s1: u5;
        signal b: bool;
        reflex r { on always { b = s1[0]; } }
    }
    ";
    match run_test(source) {
        Ok(_) => println!("PASS"),
        Err(e) => panic!("FAIL: {}", e),
    }
}

fn main() {}
