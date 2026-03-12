#![forbid(unsafe_code)]

use std::io::Read;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10 MB

/// Start the dev server. Returns exit code.
pub fn run(port: u16) -> i32 {
    let addr = format!("127.0.0.1:{}", port);

    let server = match tiny_http::Server::http(&addr) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: cannot bind to {}: {}", addr, e);
            return 1;
        }
    };

    println!("LRA dev server running at http://localhost:{}", port);
    println!("Watching for changes... (Ctrl+C to stop)\n");

    // Start file watcher in a thread
    let reload_flag = Arc::new(AtomicBool::new(false));
    let flag_clone = Arc::clone(&reload_flag);
    start_watcher(flag_clone);

    // Bounded request loop — server runs until Ctrl+C
    // MAX_REQUESTS prevents hypothetical infinite-loop lockups (NASA Power-of-10)
    const MAX_REQUESTS: usize = 10_000_000;
    for _ in 0..MAX_REQUESTS {
        let request = match server.recv_timeout(std::time::Duration::from_millis(500)) {
            Ok(Some(r)) => r,
            Ok(None) => continue,
            Err(_) => break,
        };

        let url = request.url().to_string();

        // SSE endpoint for live reload
        if url == "/__lra_reload" {
            handle_sse(request, &reload_flag);
            continue;
        }

        let file_path = if url == "/" {
            "index.html".to_string()
        } else {
            url.trim_start_matches('/').to_string()
        };

        // Path traversal guard: reject any request outside the working directory
        let root = match std::fs::canonicalize(".") {
            Ok(p) => p,
            Err(_) => {
                let resp =
                    tiny_http::Response::from_string("500 Internal Error").with_status_code(500);
                let _ = request.respond(resp);
                continue;
            }
        };
        let target = root.join(&file_path);
        let resolved = match std::fs::canonicalize(&target) {
            Ok(p) => p,
            Err(_) => {
                let resp = tiny_http::Response::from_string("404 Not Found").with_status_code(404);
                let _ = request.respond(resp);
                continue;
            }
        };
        if !resolved.starts_with(&root) {
            let resp = tiny_http::Response::from_string("403 Forbidden").with_status_code(403);
            let _ = request.respond(resp);
            continue;
        }

        serve_file(request, &file_path);
    }

    0
}

fn mime_type(path: &str) -> &'static str {
    match Path::new(path).extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "application/javascript; charset=utf-8",
        Some("wasm") => "application/wasm",
        Some("json") => "application/json; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        _ => "application/octet-stream",
    }
}

fn serve_file(request: tiny_http::Request, path: &str) {
    let file_path = Path::new(path);
    if !file_path.exists() || !file_path.is_file() {
        let resp = tiny_http::Response::from_string("404 Not Found").with_status_code(404);
        let _ = request.respond(resp);
        return;
    }

    // Size check (NASA Power-of-10: bounded)
    let metadata = match std::fs::metadata(file_path) {
        Ok(m) => m,
        Err(_) => {
            let resp = tiny_http::Response::from_string("500 Internal Error").with_status_code(500);
            let _ = request.respond(resp);
            return;
        }
    };

    if metadata.len() > MAX_FILE_SIZE {
        let resp = tiny_http::Response::from_string("413 File Too Large").with_status_code(413);
        let _ = request.respond(resp);
        return;
    }

    let mut content = Vec::new();
    let mut file = match std::fs::File::open(file_path) {
        Ok(f) => f,
        Err(_) => {
            let resp = tiny_http::Response::from_string("500 Internal Error").with_status_code(500);
            let _ = request.respond(resp);
            return;
        }
    };

    if file.read_to_end(&mut content).is_err() {
        let resp = tiny_http::Response::from_string("500 Read Error").with_status_code(500);
        let _ = request.respond(resp);
        return;
    }

    // Inject live-reload script for HTML files
    let content_type = mime_type(path);
    if path.ends_with(".html") {
        let html = String::from_utf8_lossy(&content);
        let injected = html.replace(
            "</body>",
            "<script>\n\
             (function(){var es=new EventSource('/__lra_reload');\n\
             es.onmessage=function(){location.reload();};\n\
             es.onerror=function(){es.close();};\n\
             })();\n\
             </script>\n</body>",
        );
        let resp = tiny_http::Response::from_string(injected)
            .with_header(tiny_http::Header::from_bytes("Content-Type", content_type).unwrap());
        let _ = request.respond(resp);
    } else {
        let resp = tiny_http::Response::from_data(content)
            .with_header(tiny_http::Header::from_bytes("Content-Type", content_type).unwrap());
        let _ = request.respond(resp);
    }
}

fn handle_sse(request: tiny_http::Request, reload_flag: &Arc<AtomicBool>) {
    let headers = vec![
        tiny_http::Header::from_bytes("Content-Type", "text/event-stream").unwrap(),
        tiny_http::Header::from_bytes("Cache-Control", "no-cache").unwrap(),
        tiny_http::Header::from_bytes("Connection", "keep-alive").unwrap(),
    ];

    // We send a simple response with SSE data
    // For simplicity, check flag and send reload if needed
    let mut body = String::from("retry: 1000\n\n");
    if reload_flag.swap(false, Ordering::Relaxed) {
        body.push_str("data: reload\n\n");
    }

    let mut resp = tiny_http::Response::from_string(body);
    for h in headers {
        resp.add_header(h);
    }
    let _ = request.respond(resp);
}

fn start_watcher(flag: Arc<AtomicBool>) {
    std::thread::spawn(move || {
        use notify::{RecursiveMode, Watcher};

        let flag_inner = flag;
        let mut watcher =
            match notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
                if let Ok(event) = res {
                    // Only trigger on content changes
                    let dominated_by_modify = event.kind.is_modify() || event.kind.is_create();
                    if dominated_by_modify {
                        flag_inner.store(true, Ordering::Relaxed);
                    }
                }
            }) {
                Ok(w) => w,
                Err(e) => {
                    eprintln!("Warning: file watcher failed to start: {}", e);
                    return;
                }
            };

        if let Err(e) = watcher.watch(Path::new("."), RecursiveMode::Recursive) {
            eprintln!("Warning: cannot watch directory: {}", e);
            return;
        }

        // Keep the watcher alive — bounded sleep loop (NASA Power-of-10)
        const MAX_WATCH_CYCLES: usize = 1_000_000;
        for _ in 0..MAX_WATCH_CYCLES {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}
