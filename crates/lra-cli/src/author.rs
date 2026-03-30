//! LRA Author Mission: Unified workflow for content creation.
//! 
//! Commands: init, build, serve, build-docs.

use crate::init;
use crate::build;
use crate::serve;
use crate::build_docs;

pub fn execute_author_mission(cmd: &str, args: Vec<String>) -> i32 {
    match cmd {
        "init" => {
            if let Some(name) = args.get(0) {
                init::run(name)
            } else {
                eprintln!("Error: Mission 'init' requires a project name.");
                1
            }
        }
        "build" => {
            let input = args.get(0).map(|s| s.as_str()).unwrap_or("paper.md");
            let output = args.get(1).map(|s| s.as_str()).unwrap_or("index.html");
            build::run(input, output)
        }
        "serve" => {
            let port = args.get(0).and_then(|s| s.parse::<u16>().ok()).unwrap_or(8080);
            serve::run(port)
        }
        "build-docs" => {
            let input = args.get(0).map(|s| s.as_str()).unwrap_or("docs");
            let output = args.get(1).map(|s| s.as_str()).unwrap_or("_site");
            let css = args.get(2).map(|s| s.as_str()).unwrap_or("style.css");
            build_docs::run(input, output, css)
        }
        _ => {
            eprintln!("Unknown author mission: {}", cmd);
            1
        }
    }
}
