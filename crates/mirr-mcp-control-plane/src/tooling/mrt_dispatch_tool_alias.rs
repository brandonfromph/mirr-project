#![forbid(unsafe_code)]

pub fn canonical_dispatch_tool_name(raw_name: &str) -> &str {
    match raw_name {
        "mrt_lra_init" => "lra_init",
        "mrt_lra_validate" => "lra_validate",
        "mrt_lra_serve" => "lra_serve",
        "mrt_lra_check" => "lra_check",
        "mrt_lra_sign" => "lra_sign",
        "mrt_lra_verify" => "lra_verify",
        _ => raw_name,
    }
}

#[cfg(test)]
mod tests {
    use super::canonical_dispatch_tool_name;

    #[test]
    fn prefixed_lra_names_are_canonicalized() {
        assert_eq!(canonical_dispatch_tool_name("mrt_lra_init"), "lra_init");
        assert_eq!(canonical_dispatch_tool_name("mrt_lra_validate"), "lra_validate");
        assert_eq!(canonical_dispatch_tool_name("mrt_lra_serve"), "lra_serve");
        assert_eq!(canonical_dispatch_tool_name("mrt_lra_check"), "lra_check");
        assert_eq!(canonical_dispatch_tool_name("mrt_lra_sign"), "lra_sign");
        assert_eq!(canonical_dispatch_tool_name("mrt_lra_verify"), "lra_verify");
    }

    #[test]
    fn canonical_and_unknown_names_are_unchanged() {
        assert_eq!(canonical_dispatch_tool_name("mrt_compile"), "mrt_compile");
        assert_eq!(canonical_dispatch_tool_name("unknown"), "unknown");
    }
}
