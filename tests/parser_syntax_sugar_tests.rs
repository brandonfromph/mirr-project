#![forbid(unsafe_code)]

use std::fs;

use nasa_rust_project::parser::parse_mirr;
use nasa_rust_project::pipeline::PipelineConfig;
use nasa_rust_project::Workspace;

#[test]
fn signals_block_parses_without_repeating_keyword() {
    let source = r#"
module demo {
    signals {
        clk: in bool
        data_in: in u16
        data_out: out u16;
    }
}
"#;

    let program = parse_mirr(source).expect("signals block should parse");
    assert_eq!(program.module.signals.len(), 3);
    assert_eq!(program.module.signals[0].name, "clk");
    assert_eq!(program.module.signals[1].name, "data_in");
    assert_eq!(program.module.signals[2].name, "data_out");
}

#[test]
fn calls_block_parses_without_repeating_semicolons() {
    let source = r#"
def child_core(clk: signal in bool, out_sig: signal out bool) {
    reflect {
        guard g {
            when clk
            for 1 cycles;
        }

        reflex r {
            on g {
                out_sig = true;
            }
        }
    }
}

module demo {
    signals {
        clk: in bool
        out_sig: out bool
    }

    calls {
        child_core(clk, out_sig)
    }
}
"#;

    let program = parse_mirr(source).expect("calls block should parse");
    assert_eq!(program.module.pattern_calls.len(), 1);
    assert_eq!(program.module.pattern_calls[0].pattern_name, "child_core");
}

#[test]
fn rust_style_namespace_resolution_works() {
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let root = tmp.path();

    let child_path = root.join("child.mirr");
    fs::write(
        &child_path,
        r#"
def child_core(clk: signal in bool, out_sig: signal out bool) {
    reflect {
        guard g {
            when clk
            for 1 cycles;
        }

        reflex r {
            on g {
                out_sig = true;
            }
        }
    }
}

module child {
    signals {
        data: out bool
    }
}
"#,
    )
    .expect("write child");

    let main_path = root.join("top.mirr");
    fs::write(
        &main_path,
        r#"
import "child.mirr" as child;

module top {
    signals {
        clk: in bool
        out_sig: out bool
    }

    calls {
        child::child_core(clk, out_sig)
    }
}
"#,
    )
    .expect("write root");

    let mut workspace = Workspace::new(root);
    let snapshot =
        workspace.compile_snapshot(&main_path, &PipelineConfig::default()).expect("compile");

    assert_eq!(snapshot.pipeline.program.module.name, "top");
    assert_eq!(snapshot.pipeline.program.module.signals.len(), 2);
}
