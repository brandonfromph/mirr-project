#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use super::axum_route_host::{AxumHostConfig, AxumMcpHostState};
use super::mrt_dispatch_quota_host_boundary::DEFAULT_QUOTA_HYDRATE_ROWS;
use super::transport_mode_resolution::{resolve_transport_config, TransportConfig, TransportMode};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransportStartupAction {
    StartStdio,
    StartStream { bind_addr: SocketAddr },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransportBootstrapDecision {
    pub transport: TransportConfig,
    pub action: TransportStartupAction,
}

pub trait TransportStartupRunner {
    fn start_stdio(&self, state: AxumMcpHostState) -> Result<(), String>;
    fn start_stream(
        &self,
        bind_addr: SocketAddr,
        state: AxumMcpHostState,
        config: AxumHostConfig,
    ) -> Result<(), String>;
}

pub fn decide_transport_startup(
    env: &BTreeMap<String, String>,
    fallback_port: u16,
) -> TransportBootstrapDecision {
    let transport = resolve_transport_config(env, fallback_port);
    let action = match transport.mode {
        TransportMode::Stdio => TransportStartupAction::StartStdio,
        TransportMode::Stream => TransportStartupAction::StartStream {
            bind_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), transport.stream_port),
        },
    };

    TransportBootstrapDecision { transport, action }
}

pub fn bootstrap_transport_with_runner<R: TransportStartupRunner>(
    env: &BTreeMap<String, String>,
    fallback_port: u16,
    state: AxumMcpHostState,
    host_config: AxumHostConfig,
    runner: &R,
) -> Result<TransportBootstrapDecision, String> {
    state
        .hydrate_token_quota_state_from_sink(DEFAULT_QUOTA_HYDRATE_ROWS)
        .map_err(|_| "quota_state_hydration_failed".to_owned())?;

    let decision = decide_transport_startup(env, fallback_port);
    match decision.action {
        TransportStartupAction::StartStdio => runner.start_stdio(state)?,
        TransportStartupAction::StartStream { bind_addr } => {
            runner.start_stream(bind_addr, state, host_config)?
        }
    }

    Ok(decision)
}
