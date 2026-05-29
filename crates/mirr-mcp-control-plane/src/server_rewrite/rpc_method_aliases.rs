#![forbid(unsafe_code)]

pub const LIST_TOOLS_ALIASES: &[&str] = &["ListTools", "listTools", "List_Tools"];
pub const LIST_RESOURCES_ALIASES: &[&str] = &["ListResources", "listResources"];
pub const INITIALIZE_ALIAS: &str = "initialize";
pub const CALL_TOOL_ALIASES: &[&str] = &["CallTool", "callTool"];
pub const TOOLS_CALL_METHOD: &str = "tools/call";

pub fn resolve_method_alias(method: &str, call_tool_name: Option<&str>) -> Option<String> {
    if LIST_TOOLS_ALIASES.contains(&method) {
        return Some("mcp_schema".to_owned());
    }
    if LIST_RESOURCES_ALIASES.contains(&method) {
        return Some("resources/templates/list".to_owned());
    }
    if method == INITIALIZE_ALIAS {
        return Some("mcp_initialize".to_owned());
    }
    if method == TOOLS_CALL_METHOD || CALL_TOOL_ALIASES.contains(&method) {
        return call_tool_name.map(ToOwned::to_owned);
    }

    None
}
