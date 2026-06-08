#![forbid(unsafe_code)]

use mirrc::lsp_bridge::compiler::CompilerLspRoute;
use mirrc::lsp_bridge::diagnostics::{
    clear_diagnostics_publication, publish_diagnostics, DiagnosticCode, DiagnosticItem,
    DiagnosticRange, DiagnosticSeverity, DiagnosticStreamContractVersion,
};
use mirrc::lsp_bridge::handshake::{
    negotiate_handshake, Capability, CapabilityAgreement, CapabilityRejectionReason,
    ClientHandshakeHello, HandshakeDecision, HandshakeRequest, HandshakeSessionId, ProtocolVersion,
    ServerHandshakeHello, TextEncoding,
};
use mirrc::lsp_bridge::routing::{
    route_ui_request, RoutedUiRequest, UiRequest, UiRequestId, UiRequestKind, UiRouteRejection,
};
use mirrc::lsp_bridge::types::{DocumentId, DocumentRevision, Position};

fn server_hello() -> ServerHandshakeHello {
    ServerHandshakeHello::new(
        ProtocolVersion::new(3),
        CapabilityAgreement::from_iter(vec![
            Capability::Hover,
            Capability::Completion,
            Capability::Definition,
            Capability::DocumentSymbols,
            Capability::DiagnosticsPublish,
        ]),
        vec![TextEncoding::Utf8],
    )
}

fn client_request() -> HandshakeRequest {
    HandshakeRequest::new(
        ClientHandshakeHello::new(
            "vscode-stable",
            ProtocolVersion::new(3),
            HandshakeSessionId::from_nonce(9001),
        ),
        CapabilityAgreement::from_iter(vec![
            Capability::Hover,
            Capability::Completion,
            Capability::Definition,
            Capability::DiagnosticsPublish,
        ]),
        TextEncoding::Utf8,
    )
}

fn document_id() -> DocumentId {
    DocumentId::new("doc://wave3/contract.mirr")
}

fn ui_request(kind: UiRequestKind) -> UiRequest {
    UiRequest::new(UiRequestId::new(7), document_id(), DocumentRevision::new(12), kind)
}

fn diag_item(code: u16, start_line: u32) -> DiagnosticItem {
    DiagnosticItem::new(
        DiagnosticCode::new(code),
        DiagnosticSeverity::Error,
        DiagnosticRange::single_line(start_line, 1, 5),
    )
}

#[test]
fn handshake_accepts_when_client_and_server_share_protocol_version() {
    let response = negotiate_handshake(client_request(), server_hello());
    assert!(matches!(response.decision(), HandshakeDecision::Accepted(_)));
}

#[test]
fn handshake_rejects_when_protocol_version_is_out_of_range() {
    let mut request = client_request();
    request.set_protocol_version(ProtocolVersion::new(99));
    let response = negotiate_handshake(request, server_hello());

    assert!(matches!(
        response.decision(),
        HandshakeDecision::Rejected(CapabilityRejectionReason::ProtocolVersionUnsupported { .. })
    ));
}

#[test]
fn handshake_rejects_when_required_capability_missing_from_client_offer() {
    let mut request = client_request();
    request.set_capabilities(CapabilityAgreement::from_iter(vec![Capability::Completion]));
    let response = negotiate_handshake(request, server_hello());

    assert!(matches!(
        response.decision(),
        HandshakeDecision::Rejected(CapabilityRejectionReason::MissingRequiredCapability { .. })
    ));
}

#[test]
fn handshake_rejects_when_text_encoding_is_not_supported() {
    let mut request = client_request();
    request.set_text_encoding(TextEncoding::Utf16);
    let response = negotiate_handshake(request, server_hello());

    assert!(matches!(
        response.decision(),
        HandshakeDecision::Rejected(CapabilityRejectionReason::UnsupportedTextEncoding { .. })
    ));
}

#[test]
fn handshake_returns_stable_session_id_for_same_nonce() {
    let request = client_request();
    let left = negotiate_handshake(request.clone(), server_hello());
    let right = negotiate_handshake(request, server_hello());

    assert_eq!(left.session_id(), right.session_id());
}

#[test]
fn handshake_accepts_and_returns_negotiated_capability_intersection() {
    let response = negotiate_handshake(client_request(), server_hello());
    let accepted = response.expect_accepted();

    assert_eq!(
        accepted.negotiated_capabilities(),
        CapabilityAgreement::from_iter(vec![
            Capability::Hover,
            Capability::Completion,
            Capability::Definition,
            Capability::DiagnosticsPublish,
        ])
    );
}

#[test]
fn handshake_rejection_exposes_structured_reason_code() {
    let mut request = client_request();
    request.set_capabilities(CapabilityAgreement::from_iter(vec![Capability::Hover]));
    let response = negotiate_handshake(request, server_hello());
    let rejected = response.expect_rejected();

    assert_eq!(rejected.reason_code(), CapabilityRejectionReason::MissingRequiredCapability);
}

#[test]
fn handshake_decision_marks_transport_open_only_on_accept() {
    let accepted = negotiate_handshake(client_request(), server_hello());

    let mut rejected_request = client_request();
    rejected_request.set_protocol_version(ProtocolVersion::new(0));
    let rejected = negotiate_handshake(rejected_request, server_hello());

    assert!(accepted.transport_ready());
    assert!(!rejected.transport_ready());
}

#[test]
fn handshake_negotiation_is_pure_for_identical_inputs() {
    let request = client_request();
    let hello = server_hello();

    assert_eq!(
        negotiate_handshake(request.clone(), hello.clone()),
        negotiate_handshake(request, hello)
    );
}

#[test]
fn handshake_rejection_preserves_client_identity_for_ui_reporting() {
    let mut request = client_request();
    request.set_protocol_version(ProtocolVersion::new(255));
    let response = negotiate_handshake(request.clone(), server_hello());
    let rejected = response.expect_rejected();

    assert_eq!(rejected.client_id(), request.client_hello().client_id());
}

#[test]
fn routing_hover_request_targets_compiler_hover_route() {
    let routed =
        route_ui_request(ui_request(UiRequestKind::Hover { position: Position::new(3, 8) }))
            .expect_routed();

    assert_eq!(routed.compiler_route(), CompilerLspRoute::Hover);
}

#[test]
fn routing_completion_request_targets_compiler_completion_route() {
    let routed =
        route_ui_request(ui_request(UiRequestKind::Completion { position: Position::new(4, 2) }))
            .expect_routed();

    assert_eq!(routed.compiler_route(), CompilerLspRoute::Completion);
}

#[test]
fn routing_definition_request_targets_compiler_definition_route() {
    let routed =
        route_ui_request(ui_request(UiRequestKind::Definition { position: Position::new(6, 1) }))
            .expect_routed();

    assert_eq!(routed.compiler_route(), CompilerLspRoute::Definition);
}

#[test]
fn routing_document_symbols_request_targets_symbol_index_route() {
    let routed = route_ui_request(ui_request(UiRequestKind::DocumentSymbols)).expect_routed();

    assert_eq!(routed.compiler_route(), CompilerLspRoute::DocumentSymbols);
}

#[test]
fn routing_rejects_unknown_request_kind_without_compiler_route() {
    let routed = route_ui_request(ui_request(UiRequestKind::Custom(77)));

    assert!(matches!(
        routed,
        RoutedUiRequest::Rejected(UiRouteRejection::UnsupportedRequestKind { .. })
    ));
}

#[test]
fn routing_preserves_request_id_document_id_and_revision() {
    let request = ui_request(UiRequestKind::Hover { position: Position::new(8, 13) });
    let routed = route_ui_request(request.clone()).expect_routed();

    assert_eq!(routed.request_id(), request.id());
    assert_eq!(routed.document_id(), request.document_id());
    assert_eq!(routed.document_revision(), request.document_revision());
}

#[test]
fn routing_emits_internal_route_type_for_each_supported_ui_kind() {
    let supported = vec![
        UiRequestKind::Hover { position: Position::new(0, 0) },
        UiRequestKind::Completion { position: Position::new(0, 0) },
        UiRequestKind::Definition { position: Position::new(0, 0) },
        UiRequestKind::DocumentSymbols,
    ];

    for kind in supported {
        let routed = route_ui_request(ui_request(kind)).expect_routed();
        assert!(matches!(
            routed.compiler_route(),
            CompilerLspRoute::Hover
                | CompilerLspRoute::Completion
                | CompilerLspRoute::Definition
                | CompilerLspRoute::DocumentSymbols
        ));
    }
}

#[test]
fn diagnostics_publication_uses_stable_contract_version() {
    let envelope =
        publish_diagnostics(document_id(), DocumentRevision::new(12), vec![diag_item(201, 4)]);

    assert_eq!(envelope.contract_version(), DiagnosticStreamContractVersion::V1);
}

#[test]
fn diagnostics_publication_preserves_input_order_for_equal_severity_items() {
    let first = diag_item(301, 7);
    let second = diag_item(302, 7);
    let envelope = publish_diagnostics(
        document_id(),
        DocumentRevision::new(12),
        vec![first.clone(), second.clone()],
    );
    let published = envelope.items();

    assert_eq!(published[0], first);
    assert_eq!(published[1], second);
}

#[test]
fn diagnostics_clear_publication_emits_same_document_and_empty_items() {
    let clear = clear_diagnostics_publication(document_id(), DocumentRevision::new(13));

    assert_eq!(clear.contract_version(), DiagnosticStreamContractVersion::V1);
    assert!(clear.items().is_empty());
    assert!(clear.is_clear_signal());
}
