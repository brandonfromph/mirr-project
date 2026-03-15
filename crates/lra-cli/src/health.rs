#![forbid(unsafe_code)]

/// Maximum response size for health check (1 MB, NASA Power-of-10 bound).
const MAX_RESPONSE_SIZE: usize = 1024 * 1024;

/// Maximum connection timeout (10 seconds).
const MAX_TIMEOUT_SECS: u64 = 10;

/// Maximum checks to evaluate (NASA Power-of-10 bound).
const MAX_CHECKS: usize = 10;

pub fn run(url: &str) -> i32 {
    println!("LRA Health Check \u{2014} {}\n", url);

    let agent = ureq::AgentBuilder::new()
        .timeout_connect(std::time::Duration::from_secs(MAX_TIMEOUT_SECS))
        .timeout_read(std::time::Duration::from_secs(MAX_TIMEOUT_SECS))
        .build();

    let response = match agent.get(url).call() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("  [FAIL] Cannot reach: {}", e);
            return 1;
        }
    };

    let status = response.status();
    if status != 200 {
        eprintln!("  [FAIL] HTTP {}", status);
        return 1;
    }

    let body = match response.into_string() {
        Ok(b) => {
            if b.len() > MAX_RESPONSE_SIZE {
                eprintln!("  [FAIL] Response too large ({} bytes)", b.len());
                return 1;
            }
            b
        }
        Err(e) => {
            eprintln!("  [FAIL] Cannot read response: {}", e);
            return 1;
        }
    };

    // Check for LRA markers (bounded check list)
    let checks: [(&str, &str, bool); 4] = [
        ("LRA version tag", "lra:version", body.contains("lra:version")),
        (
            "Service Worker reference",
            "sw.js",
            body.contains("sw.js") || body.contains("serviceWorker"),
        ),
        ("Claims markup", "data-lra-claim", body.contains("data-lra-claim")),
        ("Capability tag", "lra:capability", body.contains("lra:capability")),
    ];

    let mut pass_count = 0;
    let mut i = 0;
    while i < checks.len() && i < MAX_CHECKS {
        let (label, _pattern, pass) = checks[i];
        let tag = if pass { "PASS" } else { "FAIL" };
        println!("  [{}] {}", tag, label);
        if pass {
            pass_count += 1;
        }
        i += 1;
    }

    let total = checks.len();
    println!("\n  Result: {}/{} headless checks passed", pass_count, total);

    if pass_count == total {
        0
    } else {
        1
    }
}
