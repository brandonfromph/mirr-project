#![forbid(unsafe_code)]

use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransportMode {
    Stdio,
    Stream,
}

impl TransportMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stdio => "stdio",
            Self::Stream => "stream",
        }
    }
}

pub const TRANSPORT_STREAM_FEATURE_FLAG: &str = "MRT_TRANSPORT_STREAM_MODE";
pub const TRANSPORT_STREAM_PORT_KEY: &str = "MRT_TRANSPORT_STREAM_PORT";
pub const DEFAULT_STREAM_PORT: u16 = 3333;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransportConfig {
    pub mode: TransportMode,
    pub feature_flag: &'static str,
    pub stream_port: u16,
}

pub fn parse_stream_port(raw_value: Option<&str>, fallback: u16) -> u16 {
    let Some(raw) = raw_value else {
        return fallback;
    };

    if raw.is_empty() {
        return fallback;
    }

    let Ok(parsed) = raw.parse::<f64>() else {
        return fallback;
    };

    if !parsed.is_finite() {
        return fallback;
    }

    let bounded = parsed.trunc() as i64;
    if !(1..=65_535).contains(&bounded) {
        return fallback;
    }

    bounded as u16
}

pub fn is_stream_mode_enabled(raw_value: Option<&str>) -> bool {
    let Some(raw) = raw_value else {
        return false;
    };

    if raw.is_empty() {
        return false;
    }

    let normalized = raw.trim().to_ascii_lowercase();
    normalized == "1" || normalized == "true" || normalized == "stream"
}

pub fn resolve_transport_config(
    env: &BTreeMap<String, String>,
    fallback_port: u16,
) -> TransportConfig {
    let stream_enabled =
        is_stream_mode_enabled(env.get(TRANSPORT_STREAM_FEATURE_FLAG).map(String::as_str));
    let stream_port =
        parse_stream_port(env.get(TRANSPORT_STREAM_PORT_KEY).map(String::as_str), fallback_port);

    TransportConfig {
        mode: if stream_enabled { TransportMode::Stream } else { TransportMode::Stdio },
        feature_flag: TRANSPORT_STREAM_FEATURE_FLAG,
        stream_port,
    }
}
