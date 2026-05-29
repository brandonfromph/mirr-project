use std::collections::BTreeSet;

use mirr_mcp_control_plane::server_rewrite::rpc_method_normalization::normalize_rpc_method_name;

#[test]
fn method_normalization_matches_alias_and_fallback_contract() {
    let known_methods: BTreeSet<String> =
        ["mcp_schema", "resources/templates/list", "mcp_initialize", "list_handlers", "mrt_audit"]
            .iter()
            .map(|value| (*value).to_owned())
            .collect();

    assert_eq!(normalize_rpc_method_name(Some("mrt_audit"), None, &known_methods), "mrt_audit");
    assert_eq!(
        normalize_rpc_method_name(Some("listHandlers"), None, &known_methods),
        "list_handlers"
    );
    assert_eq!(
        normalize_rpc_method_name(Some("list-handlers"), None, &known_methods),
        "list_handlers"
    );
    assert_eq!(normalize_rpc_method_name(Some("ListTools"), None, &known_methods), "mcp_schema");
    assert_eq!(
        normalize_rpc_method_name(Some("ListResources"), None, &known_methods),
        "resources/templates/list"
    );
    assert_eq!(
        normalize_rpc_method_name(Some("initialize"), None, &known_methods),
        "mcp_initialize"
    );
    assert_eq!(
        normalize_rpc_method_name(Some("CallTool"), Some("mrt_audit"), &known_methods),
        "mrt_audit"
    );
    assert_eq!(
        normalize_rpc_method_name(Some("tools/call"), Some("mrt_audit"), &known_methods),
        "mrt_audit"
    );
    assert_eq!(
        normalize_rpc_method_name(Some("unknown_method"), None, &known_methods),
        "unknown_method"
    );
}
