#![forbid(unsafe_code)]

pub mod canonical_discovery_method_metadata;
pub mod mrt_dispatch_tool_alias;
pub mod mrt_dispatch_tool_name;
pub mod mrt_dispatch_tool_role_allowlist;

pub use canonical_discovery_method_metadata::{
    discovery_method_by_name, DiscoveryMethodMetadata, DiscoveryParameter,
    CANONICAL_DISCOVERY_METHOD_METADATA,
};
pub use mrt_dispatch_tool_name::MrtDispatchTool;
