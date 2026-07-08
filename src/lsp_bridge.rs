#![forbid(unsafe_code)]

pub mod compiler {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum CompilerLspRoute {
        Hover,
        Completion,
        Definition,
        DocumentSymbols,
    }
}

pub mod types {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct DocumentId(String);

    impl DocumentId {
        pub fn new(value: impl Into<String>) -> Self {
            Self(value.into())
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct DocumentRevision(u32);

    impl DocumentRevision {
        pub fn new(value: u32) -> Self {
            Self(value)
        }

        pub fn value(self) -> u32 {
            self.0
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Position {
        line: u32,
        column: u32,
    }

    impl Position {
        pub fn new(line: u32, column: u32) -> Self {
            Self { line, column }
        }

        pub fn line(self) -> u32 {
            self.line
        }

        pub fn column(self) -> u32 {
            self.column
        }
    }
}

pub mod handshake {
    use crate::error::MirrError;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct ProtocolVersion(u16);

    impl ProtocolVersion {
        pub fn new(value: u16) -> Self {
            Self(value)
        }

        pub fn value(self) -> u16 {
            self.0
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct HandshakeSessionId(u64);

    impl HandshakeSessionId {
        pub fn from_nonce(nonce: u64) -> Self {
            const MIX_A: u64 = 0x9E37_79B9_7F4A_7C15;
            const MIX_B: u64 = 0xD1B5_4A32_D192_ED03;
            let mixed = nonce.wrapping_mul(MIX_A).rotate_left(17) ^ MIX_B;
            Self(mixed)
        }

        pub fn value(self) -> u64 {
            self.0
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum TextEncoding {
        Utf8,
        Utf16,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Capability {
        Hover,
        Completion,
        Definition,
        DocumentSymbols,
        DiagnosticsPublish,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    pub struct CapabilityAgreement {
        capabilities: Vec<Capability>,
    }

    impl CapabilityAgreement {
        pub fn contains(&self, capability: Capability) -> bool {
            self.capabilities.contains(&capability)
        }

        pub fn intersection(&self, other: &Self) -> Self {
            let mut intersection = Vec::with_capacity(self.capabilities.len());
            for capability in &self.capabilities {
                if other.capabilities.contains(capability) {
                    intersection.push(*capability);
                }
            }
            Self { capabilities: intersection }
        }

        pub fn as_slice(&self) -> &[Capability] {
            &self.capabilities
        }
    }
    impl std::iter::FromIterator<Capability> for CapabilityAgreement {
        fn from_iter<I: IntoIterator<Item = Capability>>(iter: I) -> Self {
            let mut deduplicated = Vec::new();
            for capability in iter {
                if !deduplicated.contains(&capability) {
                    deduplicated.push(capability);
                }
            }
            Self { capabilities: deduplicated }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ClientHandshakeHello {
        client_id: String,
        protocol_version: ProtocolVersion,
        session_id: HandshakeSessionId,
    }

    impl ClientHandshakeHello {
        pub fn new(
            client_id: impl Into<String>,
            protocol_version: ProtocolVersion,
            session_id: HandshakeSessionId,
        ) -> Self {
            Self { client_id: client_id.into(), protocol_version, session_id }
        }

        pub fn client_id(&self) -> &str {
            &self.client_id
        }

        pub fn protocol_version(&self) -> ProtocolVersion {
            self.protocol_version
        }

        pub fn session_id(&self) -> HandshakeSessionId {
            self.session_id
        }

        fn set_protocol_version(&mut self, protocol_version: ProtocolVersion) {
            self.protocol_version = protocol_version;
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct HandshakeRequest {
        client_hello: ClientHandshakeHello,
        capabilities: CapabilityAgreement,
        text_encoding: TextEncoding,
    }

    impl HandshakeRequest {
        pub fn new(
            client_hello: ClientHandshakeHello,
            capabilities: CapabilityAgreement,
            text_encoding: TextEncoding,
        ) -> Self {
            Self { client_hello, capabilities, text_encoding }
        }

        pub fn client_hello(&self) -> &ClientHandshakeHello {
            &self.client_hello
        }

        pub fn capabilities(&self) -> &CapabilityAgreement {
            &self.capabilities
        }

        pub fn text_encoding(&self) -> TextEncoding {
            self.text_encoding
        }

        pub fn set_protocol_version(&mut self, protocol_version: ProtocolVersion) {
            self.client_hello.set_protocol_version(protocol_version);
        }

        pub fn set_capabilities(&mut self, capabilities: CapabilityAgreement) {
            self.capabilities = capabilities;
        }

        pub fn set_text_encoding(&mut self, text_encoding: TextEncoding) {
            self.text_encoding = text_encoding;
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ServerHandshakeHello {
        supported_protocol: ProtocolVersion,
        offered_capabilities: CapabilityAgreement,
        supported_encodings: Vec<TextEncoding>,
    }

    impl ServerHandshakeHello {
        pub fn new(
            supported_protocol: ProtocolVersion,
            offered_capabilities: CapabilityAgreement,
            supported_encodings: Vec<TextEncoding>,
        ) -> Self {
            let mut deduplicated_encodings = Vec::with_capacity(supported_encodings.len());
            for encoding in supported_encodings {
                if !deduplicated_encodings.contains(&encoding) {
                    deduplicated_encodings.push(encoding);
                }
            }

            Self {
                supported_protocol,
                offered_capabilities,
                supported_encodings: deduplicated_encodings,
            }
        }

        pub fn supported_protocol(&self) -> ProtocolVersion {
            self.supported_protocol
        }

        pub fn offered_capabilities(&self) -> &CapabilityAgreement {
            &self.offered_capabilities
        }

        fn supports_encoding(&self, encoding: TextEncoding) -> bool {
            self.supported_encodings.contains(&encoding)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CapabilityRejectionReason {
        ProtocolVersionUnsupported {},
        MissingRequiredCapability {},
        UnsupportedTextEncoding {},
    }

    impl CapabilityRejectionReason {
        #[allow(non_upper_case_globals)]
        pub const ProtocolVersionUnsupported: Self = Self::ProtocolVersionUnsupported {};

        #[allow(non_upper_case_globals)]
        pub const MissingRequiredCapability: Self = Self::MissingRequiredCapability {};

        #[allow(non_upper_case_globals)]
        pub const UnsupportedTextEncoding: Self = Self::UnsupportedTextEncoding {};
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct AcceptedHandshake {
        negotiated_capabilities: CapabilityAgreement,
        protocol_version: ProtocolVersion,
        text_encoding: TextEncoding,
    }

    impl AcceptedHandshake {
        fn new(
            negotiated_capabilities: CapabilityAgreement,
            protocol_version: ProtocolVersion,
            text_encoding: TextEncoding,
        ) -> Self {
            Self { negotiated_capabilities, protocol_version, text_encoding }
        }

        pub fn negotiated_capabilities(&self) -> CapabilityAgreement {
            self.negotiated_capabilities.clone()
        }

        pub fn protocol_version(&self) -> ProtocolVersion {
            self.protocol_version
        }

        pub fn text_encoding(&self) -> TextEncoding {
            self.text_encoding
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum HandshakeDecision {
        Accepted(AcceptedHandshake),
        Rejected(CapabilityRejectionReason),
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct RejectedHandshake {
        client_id: String,
        reason: CapabilityRejectionReason,
    }

    impl RejectedHandshake {
        fn new(client_id: String, reason: CapabilityRejectionReason) -> Self {
            Self { client_id, reason }
        }

        pub fn client_id(&self) -> &str {
            &self.client_id
        }

        pub fn reason_code(&self) -> CapabilityRejectionReason {
            self.reason
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct HandshakeNegotiation {
        session_id: HandshakeSessionId,
        client_id: String,
        decision: HandshakeDecision,
    }

    impl HandshakeNegotiation {
        fn accepted(
            session_id: HandshakeSessionId,
            client_id: String,
            accepted: AcceptedHandshake,
        ) -> Self {
            Self { session_id, client_id, decision: HandshakeDecision::Accepted(accepted) }
        }

        fn rejected(
            session_id: HandshakeSessionId,
            client_id: String,
            reason: CapabilityRejectionReason,
        ) -> Self {
            Self { session_id, client_id, decision: HandshakeDecision::Rejected(reason) }
        }

        pub fn session_id(&self) -> HandshakeSessionId {
            self.session_id
        }

        pub fn decision(&self) -> &HandshakeDecision {
            &self.decision
        }

        pub fn transport_ready(&self) -> bool {
            matches!(&self.decision, HandshakeDecision::Accepted(_))
        }

        pub fn expect_accepted(&self) -> Result<AcceptedHandshake, MirrError> {
            match &self.decision {
                HandshakeDecision::Accepted(accepted) => Ok(accepted.clone()),
                HandshakeDecision::Rejected(reason) => {
                    Err(MirrError::InternalError(format!("expected accepted handshake, got rejection: {reason:?}")))
                }
            }
        }

        pub fn expect_rejected(&self) -> Result<RejectedHandshake, MirrError> {
            match &self.decision {
                HandshakeDecision::Rejected(reason) => {
                    Ok(RejectedHandshake::new(self.client_id.clone(), *reason))
                }
                HandshakeDecision::Accepted(_) => {
                    Err(MirrError::InternalError("expected rejected handshake, got accepted decision".to_string()))
                }
            }
        }
    }

    const REQUIRED_CAPABILITIES: [Capability; 4] = [
        Capability::Hover,
        Capability::Completion,
        Capability::Definition,
        Capability::DiagnosticsPublish,
    ];

    fn missing_required_capability(capabilities: &CapabilityAgreement) -> Option<Capability> {
        REQUIRED_CAPABILITIES.into_iter().find(|&capability| !capabilities.contains(capability))
    }

    pub fn negotiate_handshake(
        request: HandshakeRequest,
        server: ServerHandshakeHello,
    ) -> HandshakeNegotiation {
        let session_id = request.client_hello().session_id();
        let client_id = request.client_hello().client_id().to_string();

        if request.client_hello().protocol_version() != server.supported_protocol() {
            return HandshakeNegotiation::rejected(
                session_id,
                client_id,
                CapabilityRejectionReason::ProtocolVersionUnsupported {},
            );
        }

        if !server.supports_encoding(request.text_encoding()) {
            return HandshakeNegotiation::rejected(
                session_id,
                client_id,
                CapabilityRejectionReason::UnsupportedTextEncoding {},
            );
        }

        if missing_required_capability(request.capabilities()).is_some() {
            return HandshakeNegotiation::rejected(
                session_id,
                client_id,
                CapabilityRejectionReason::MissingRequiredCapability {},
            );
        }

        let negotiated_capabilities =
            request.capabilities().intersection(server.offered_capabilities());

        HandshakeNegotiation::accepted(
            session_id,
            client_id,
            AcceptedHandshake::new(
                negotiated_capabilities,
                request.client_hello().protocol_version(),
                request.text_encoding(),
            ),
        )
    }
}

pub mod routing {
    use crate::error::MirrError;
    use super::compiler::CompilerLspRoute;
    use super::types::{DocumentId, DocumentRevision, Position};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct UiRequestId(u64);

    impl UiRequestId {
        pub fn new(value: u64) -> Self {
            Self(value)
        }

        pub fn value(self) -> u64 {
            self.0
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum UiRequestKind {
        Hover { position: Position },
        Completion { position: Position },
        Definition { position: Position },
        DocumentSymbols,
        Custom(u32),
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct UiRequest {
        id: UiRequestId,
        document_id: DocumentId,
        document_revision: DocumentRevision,
        kind: UiRequestKind,
    }

    impl UiRequest {
        pub fn new(
            id: UiRequestId,
            document_id: DocumentId,
            document_revision: DocumentRevision,
            kind: UiRequestKind,
        ) -> Self {
            Self { id, document_id, document_revision, kind }
        }

        pub fn id(&self) -> UiRequestId {
            self.id
        }

        pub fn document_id(&self) -> &DocumentId {
            &self.document_id
        }

        pub fn document_revision(&self) -> DocumentRevision {
            self.document_revision
        }

        pub fn kind(&self) -> &UiRequestKind {
            &self.kind
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct RoutedCompilerRequest {
        request_id: UiRequestId,
        document_id: DocumentId,
        document_revision: DocumentRevision,
        compiler_route: CompilerLspRoute,
    }

    impl RoutedCompilerRequest {
        fn new(
            request_id: UiRequestId,
            document_id: DocumentId,
            document_revision: DocumentRevision,
            compiler_route: CompilerLspRoute,
        ) -> Self {
            Self { request_id, document_id, document_revision, compiler_route }
        }

        pub fn request_id(&self) -> UiRequestId {
            self.request_id
        }

        pub fn document_id(&self) -> &DocumentId {
            &self.document_id
        }

        pub fn document_revision(&self) -> DocumentRevision {
            self.document_revision
        }

        pub fn compiler_route(&self) -> CompilerLspRoute {
            self.compiler_route
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum UiRouteRejection {
        UnsupportedRequestKind { request_id: UiRequestId, kind: UiRequestKind },
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum RoutedUiRequest {
        Routed(RoutedCompilerRequest),
        Rejected(UiRouteRejection),
    }

    impl RoutedUiRequest {
        pub fn expect_routed(self) -> Result<RoutedCompilerRequest, MirrError> {
            match self {
                RoutedUiRequest::Routed(routed) => Ok(routed),
                RoutedUiRequest::Rejected(rejection) => {
                    Err(MirrError::InternalError(format!("expected routed request, got rejection: {rejection:?}")))
                }
            }
        }
    }

    pub fn route_ui_request(request: UiRequest) -> RoutedUiRequest {
        let UiRequest { id, document_id, document_revision, kind } = request;

        match kind {
            UiRequestKind::Hover { .. } => RoutedUiRequest::Routed(RoutedCompilerRequest::new(
                id,
                document_id,
                document_revision,
                CompilerLspRoute::Hover,
            )),
            UiRequestKind::Completion { .. } => {
                RoutedUiRequest::Routed(RoutedCompilerRequest::new(
                    id,
                    document_id,
                    document_revision,
                    CompilerLspRoute::Completion,
                ))
            }
            UiRequestKind::Definition { .. } => {
                RoutedUiRequest::Routed(RoutedCompilerRequest::new(
                    id,
                    document_id,
                    document_revision,
                    CompilerLspRoute::Definition,
                ))
            }
            UiRequestKind::DocumentSymbols => RoutedUiRequest::Routed(RoutedCompilerRequest::new(
                id,
                document_id,
                document_revision,
                CompilerLspRoute::DocumentSymbols,
            )),
            custom @ UiRequestKind::Custom(_) => {
                RoutedUiRequest::Rejected(UiRouteRejection::UnsupportedRequestKind {
                    request_id: id,
                    kind: custom,
                })
            }
        }
    }
}

pub mod diagnostics {
    use super::types::{DocumentId, DocumentRevision, Position};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum DiagnosticStreamContractVersion {
        V1,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct DiagnosticCode(u16);

    impl DiagnosticCode {
        pub fn new(value: u16) -> Self {
            Self(value)
        }

        pub fn value(self) -> u16 {
            self.0
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum DiagnosticSeverity {
        Error,
        Warning,
        Information,
        Hint,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct DiagnosticRange {
        start: Position,
        end: Position,
    }

    impl DiagnosticRange {
        pub fn single_line(line: u32, start_column: u32, end_column: u32) -> Self {
            Self { start: Position::new(line, start_column), end: Position::new(line, end_column) }
        }

        pub fn start(&self) -> Position {
            self.start
        }

        pub fn end(&self) -> Position {
            self.end
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct DiagnosticItem {
        code: DiagnosticCode,
        severity: DiagnosticSeverity,
        range: DiagnosticRange,
    }

    impl DiagnosticItem {
        pub fn new(
            code: DiagnosticCode,
            severity: DiagnosticSeverity,
            range: DiagnosticRange,
        ) -> Self {
            Self { code, severity, range }
        }

        pub fn code(&self) -> DiagnosticCode {
            self.code
        }

        pub fn severity(&self) -> DiagnosticSeverity {
            self.severity
        }

        pub fn range(&self) -> &DiagnosticRange {
            &self.range
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct DiagnosticPublication {
        contract_version: DiagnosticStreamContractVersion,
        document_id: DocumentId,
        document_revision: DocumentRevision,
        items: Vec<DiagnosticItem>,
        is_clear_signal: bool,
    }

    impl DiagnosticPublication {
        pub fn contract_version(&self) -> DiagnosticStreamContractVersion {
            self.contract_version
        }

        pub fn document_id(&self) -> &DocumentId {
            &self.document_id
        }

        pub fn document_revision(&self) -> DocumentRevision {
            self.document_revision
        }

        pub fn items(&self) -> &[DiagnosticItem] {
            &self.items
        }

        pub fn is_clear_signal(&self) -> bool {
            self.is_clear_signal
        }
    }

    pub fn publish_diagnostics(
        document_id: DocumentId,
        document_revision: DocumentRevision,
        items: Vec<DiagnosticItem>,
    ) -> DiagnosticPublication {
        DiagnosticPublication {
            contract_version: DiagnosticStreamContractVersion::V1,
            document_id,
            document_revision,
            items,
            is_clear_signal: false,
        }
    }

    pub fn clear_diagnostics_publication(
        document_id: DocumentId,
        document_revision: DocumentRevision,
    ) -> DiagnosticPublication {
        DiagnosticPublication {
            contract_version: DiagnosticStreamContractVersion::V1,
            document_id,
            document_revision,
            items: Vec::new(),
            is_clear_signal: true,
        }
    }
}
