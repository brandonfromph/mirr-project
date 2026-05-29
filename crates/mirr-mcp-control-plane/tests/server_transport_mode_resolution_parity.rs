use std::collections::BTreeMap;

use mirr_mcp_control_plane::server_rewrite::transport_mode_resolution::{
    is_stream_mode_enabled, parse_stream_port, resolve_transport_config, TransportMode,
    DEFAULT_STREAM_PORT, TRANSPORT_STREAM_FEATURE_FLAG, TRANSPORT_STREAM_PORT_KEY,
};

#[test]
fn parse_stream_port_matches_ts_number_and_bounds_rules() {
    assert_eq!(parse_stream_port(None, 3333), 3333);
    assert_eq!(parse_stream_port(Some(""), 3333), 3333);
    assert_eq!(parse_stream_port(Some("abc"), 3333), 3333);
    assert_eq!(parse_stream_port(Some("0"), 3333), 3333);
    assert_eq!(parse_stream_port(Some("70000"), 3333), 3333);
    assert_eq!(parse_stream_port(Some("1234.99"), 3333), 1234);
    assert_eq!(parse_stream_port(Some("65535"), 3333), 65535);
    assert_eq!(parse_stream_port(Some("NaN"), 3333), 3333);
}

#[test]
fn stream_mode_enablement_is_explicit_and_fail_closed() {
    assert!(!is_stream_mode_enabled(None));
    assert!(!is_stream_mode_enabled(Some("")));
    assert!(!is_stream_mode_enabled(Some("yes")));
    assert!(is_stream_mode_enabled(Some("1")));
    assert!(is_stream_mode_enabled(Some("true")));
    assert!(is_stream_mode_enabled(Some("stream")));
    assert!(is_stream_mode_enabled(Some(" Stream ")));
}

#[test]
fn resolve_transport_config_uses_feature_flag_and_port_from_env() {
    let mut env = BTreeMap::<String, String>::new();
    env.insert(TRANSPORT_STREAM_FEATURE_FLAG.to_owned(), "true".to_owned());
    env.insert(TRANSPORT_STREAM_PORT_KEY.to_owned(), "4400".to_owned());

    let config = resolve_transport_config(&env, DEFAULT_STREAM_PORT);
    assert_eq!(config.mode, TransportMode::Stream);
    assert_eq!(config.feature_flag, TRANSPORT_STREAM_FEATURE_FLAG);
    assert_eq!(config.stream_port, 4400);
}

#[test]
fn resolve_transport_config_defaults_to_stdio_with_fallback_port() {
    let env = BTreeMap::<String, String>::new();

    let config = resolve_transport_config(&env, 3333);
    assert_eq!(config.mode, TransportMode::Stdio);
    assert_eq!(config.stream_port, 3333);
}

#[test]
fn resolve_transport_config_fail_closed_for_invalid_inputs() {
    let mut env = BTreeMap::<String, String>::new();
    env.insert(TRANSPORT_STREAM_FEATURE_FLAG.to_owned(), "banana".to_owned());
    env.insert(TRANSPORT_STREAM_PORT_KEY.to_owned(), "99999".to_owned());

    let config = resolve_transport_config(&env, 4100);
    assert_eq!(config.mode, TransportMode::Stdio);
    assert_eq!(config.stream_port, 4100);
}
