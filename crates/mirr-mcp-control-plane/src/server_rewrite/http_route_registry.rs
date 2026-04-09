#![forbid(unsafe_code)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HttpVerb {
    Get,
    Post,
}

impl HttpVerb {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Get => "get",
            Self::Post => "post",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HttpRoute {
    pub verb: HttpVerb,
    pub path: &'static str,
}

impl HttpRoute {
    pub const fn new(verb: HttpVerb, path: &'static str) -> Self {
        Self { verb, path }
    }
}

pub const SERVER_TS_ROUTE_REGISTRY: &[HttpRoute] = &[
    HttpRoute::new(HttpVerb::Post, "/mcp_initialize"),
    HttpRoute::new(HttpVerb::Get, "/mcp_catalog"),
    HttpRoute::new(HttpVerb::Post, "/mcp_schema"),
    HttpRoute::new(HttpVerb::Post, "/mcp"),
    HttpRoute::new(HttpVerb::Post, "/mcp_stream"),
    HttpRoute::new(HttpVerb::Post, "/ctx_sample"),
    HttpRoute::new(HttpVerb::Get, "/list_handlers"),
    HttpRoute::new(HttpVerb::Post, "/generate_api_key"),
    HttpRoute::new(HttpVerb::Get, "/list_api_keys"),
    HttpRoute::new(HttpVerb::Post, "/revoke_api_key"),
    HttpRoute::new(HttpVerb::Post, "/long_running"),
    HttpRoute::new(HttpVerb::Post, "/mrt_audit"),
    HttpRoute::new(HttpVerb::Post, "/mrt_brain_get"),
    HttpRoute::new(HttpVerb::Post, "/mrt_general_ci"),
    HttpRoute::new(HttpVerb::Post, "/mrt_general_ci_compile"),
    HttpRoute::new(HttpVerb::Post, "/mrt_general_ci_fast"),
    HttpRoute::new(HttpVerb::Post, "/mrt_wave_dry_run"),
    HttpRoute::new(HttpVerb::Post, "/mrt_wave_apply"),
    HttpRoute::new(HttpVerb::Post, "/mrt_lsp_diagnostics"),
    HttpRoute::new(HttpVerb::Post, "/mrt_compile"),
    HttpRoute::new(HttpVerb::Post, "/mrt_rspu_validate"),
    HttpRoute::new(HttpVerb::Post, "/mrt_rspu_proofs"),
    HttpRoute::new(HttpVerb::Post, "/mrt_lra_init"),
    HttpRoute::new(HttpVerb::Post, "/mrt_lra_validate"),
    HttpRoute::new(HttpVerb::Post, "/mrt_lra_serve"),
    HttpRoute::new(HttpVerb::Post, "/mrt_lra_check"),
    HttpRoute::new(HttpVerb::Post, "/mrt_lra_sign"),
    HttpRoute::new(HttpVerb::Post, "/mrt_lra_verify"),
    HttpRoute::new(HttpVerb::Post, "/read_text_file"),
    HttpRoute::new(HttpVerb::Post, "/write_file"),
    HttpRoute::new(HttpVerb::Post, "/list_directory"),
    HttpRoute::new(HttpVerb::Post, "/directory_tree"),
    HttpRoute::new(HttpVerb::Post, "/search_files"),
    HttpRoute::new(HttpVerb::Post, "/run_cargo"),
    HttpRoute::new(HttpVerb::Get, "/health"),
    HttpRoute::new(HttpVerb::Post, "/read_netlist"),
    HttpRoute::new(HttpVerb::Post, "/run_simulator"),
    HttpRoute::new(HttpVerb::Post, "/estimate_resources"),
    HttpRoute::new(HttpVerb::Post, "/parity_check"),
];
