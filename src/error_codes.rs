// ---------------------------------------------------------------------------
//! Typed error code registry for the MIRR compiler.
//!
//! Every error emitted by the compiler MUST use a variant from this enum.
//! Using an inline `"[Ennn]"` string literal in production code is a
//! Zero-Debt violation — enforced by `build.rs`.
//!
//! ## Adding a new code
//! 1. Add a variant here with its numeric value (`#[repr(u16)]`).
//! 2. Add the entry to `error_registry.toml` (the canonical docs source).
//! 3. Use `ErrorCode::YourVariant` at the call site.
// ---------------------------------------------------------------------------

#![forbid(unsafe_code)]

/// Every MIRR diagnostic error code, statically typed.
///
/// The `#[repr(u16)]` discriminant is the numeric part of the code.
/// `E806` → `StructNameEmpty = 806`.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum ErrorCode {
    // ── E1xx — Parse / Lexical ──────────────────────────────────────────────
    ParseFallback = 100,
    MirrSourceEmpty = 101,
    ExpectedModuleEof = 102,
    ExpectedModuleFound = 103,
    MalformedModule = 104,
    ModuleNameEmpty = 105,
    ModuleNotClosed = 106,
    UnexpectedModuleLine = 107,
    MalformedSignal = 108,
    SignalMissingSemicolon = 109,
    SignalMissingColon = 110,
    SignalNameEmpty = 111,
    SignalKindMissing = 112,
    SignalTypeMissing = 113,
    SignalTooManyTokens = 114,
    SignalUnknownKind = 115,
    SignalInvalidUWidth = 116,
    SignalInvalidIWidth = 117,
    SignalUnknownType = 118,
    GuardUnexpectedEof = 119,
    GuardMalformed = 120,
    GuardNameEmpty = 121,
    GuardMissingWhen = 122,
    GuardExpectedWhen = 123,
    GuardMalformedWhen = 124,
    GuardConditionError = 125,
    GuardMissingFor = 126,
    GuardExpectedFor = 127,
    GuardMalformedFor = 128,
    GuardMissingCycleCount = 129,
    GuardInvalidCycleCount = 130,
    GuardNotClosed = 131,
    GuardExpectedClose = 132,
    AssignmentMissingEq = 133,
    AssignmentTargetEmpty = 134,
    AssignmentRhsEmpty = 135,
    AssignmentExprError = 136,
    ReflexUnexpectedEof = 137,
    ReflexMalformed = 138,
    ReflexNameEmpty = 139,
    ReflexMissingOn = 140,
    ReflexMaxGuardNames = 141,
    ReflexBodyTooLong = 142,
    ReflexNoGuardNames = 143,
    ReflexAssignmentError = 144,
    ReflexNotClosed = 145,
    ReflexEmptyBody = 146,
    PropertyUnexpectedEof = 147,
    PropertyMalformed = 148,
    PropertyNameEmpty = 149,
    PropertyMissingFormula = 150,
    PropertyNotClosed = 151,
    PropertyExpectedClose = 152,
    PropertyBadKeyword = 153,
    PropertyNeedsParens = 154,
    PropertyAntecedentError = 155,
    PropertyConsequentError = 156,
    PropertyConsequentBad = 157,
    ForLoopMalformed = 158,
    ForLoopRangeMalformed = 159,
    ForLoopRangeEmpty = 160,
    ForLoopBodyEmpty = 161,
    ForLoopTooManyIter = 162,
    ForLoopSignalsTooMany = 163,
    ForLoopSuffixBad = 164,
    ForLoopStepZero = 165,
    ForLoopBodyError = 166,
    // 167–169 reserved
    ImportMalformed = 170,
    ImportPathNotString = 171,
    ImportPathEmpty = 172,
    ImportAliasMissing = 173,
    ImportAliasEmpty = 174,
    ImportAliasMustFollow = 175,
    ImportResolveFailed = 176,
    ImportCircular = 177,
    ImportTooDeep = 178,
    ImportFileMissing = 179,
    LexHexLiteralInvalid = 180,
    LexUnexpectedChar = 181,
    // 182 reserved
    MacroExpandError = 183,
    MacroCallBadArgs = 184,
    MacroCallTooManyArgs = 185,
    MacroCallTooFewArgs = 186,
    // 187–189 reserved
    StructFieldMalformed = 190,
    StructFieldNameEmpty = 191,
    StructFieldTypeBad = 192,
    StructTooManyFields = 193,
    StructUnclosed = 194,
    StructDuplField = 195,
    StructNotFound = 196,
    StructTypeMismatch = 197,
    // 198–199 reserved

    // ── E2xx — Semantic Analysis ─────────────────────────────────────────────
    SemanticFallback = 200,
    DuplicateSignalName = 201,
    UndeclaredSignal = 202,
    SignalDirectionMismatch = 203,
    GuardRefUndeclared = 204,
    ReflexGuardUndeclared = 205,
    PropertySignalUndeclared = 206,
    AssignmentToInput = 207,
    AssignmentToUndeclared = 208,
    PropertyAlwaysNever = 209,
    CyclicDependency = 210,
    SignalWidthMismatch = 211,
    ReflexDuplicate = 212,
    GuardDuplicate = 213,
    PropertyDuplicate = 214,
    ModuleDuplicate = 215,
    ModuleNotFound = 216,
    // 217–225 reserved
    SemanticMaxErrors = 226,
    // 227–228 reserved
    SemanticUnreachable = 229,
    SemanticArrayIndex = 230,
    SemanticBudgetExhausted = 231,

    // ── E3xx — Temporal Compilation ─────────────────────────────────────────
    TemporalFallback = 300,
    TemporalGuardDepth = 301,
    TemporalCondUnsupported = 302,
    TemporalMissingSubguard = 303,
    TemporalIterBudget = 304,
    TemporalNoResult = 305,
    TemporalCondLowerFailed = 306,

    // ── E4xx — Pattern Expansion ─────────────────────────────────────────────
    PatternFallback = 400,
    // 401–416 reserved
    PatternDuplicate = 417,
    PatternDuplicateParam = 418,
    PatternTooManyParams = 419,
    PatternEmptyReflect = 420,
    PatternReflectTooLong = 421,
    // 422–425 reserved
    PatternParamKindMismatch = 426,
    PatternParamNotPattern = 427,
    PatternCyclic = 428,

    // ── E5xx — Width Inference ───────────────────────────────────────────────
    WidthFallback = 500,
    WidthConflict = 501,
    WidthUnsolvable = 502,
    WidthOverflow = 503,
    WidthNarrowing = 504,
    WidthMissingConstraint = 505,
    WidthBitfieldOverflow = 506,
    WidthArrayMismatch = 507,
    WidthStructField = 508,
    WidthFixedPoint = 509,
    WidthFifo = 510,
    WidthBundle = 511,

    // ── E6xx — Type Checking ─────────────────────────────────────────────────
    TypeFallback = 600,
    TypeMismatch = 601,
    TypeBoolExpected = 602,
    TypeUnsignedExpected = 603,
    TypeSignedExpected = 604,
    TypeArrayMismatch = 605,
    TypeStructMismatch = 606,
    TypeWidthExceeds = 607,
    TypeRefinementFailed = 608,
    TypeSessionViolation = 609,
    // E610–E625 — Extended Type Checking (MEGA-1)
    ExtTypeRefineBound = 610,
    ExtTypeRefineRange = 611,
    ExtTypeRefineWidth = 612,
    ExtTypeLinearUnused = 613,
    ExtTypeLinearDouble = 614,
    ExtTypeLinearEscape = 615,
    ExtTypeEffectPure = 616,
    ExtTypeEffectMix = 617,
    ExtTypeClockCross = 618,
    ExtTypeClockUndef = 619,
    ExtTypePhantomMismatch = 620,
    ExtTypePhantomUndef = 621,
    ExtTypeNatOverflow = 622,
    ExtTypeNatMismatch = 623,
    ExtTypeDepMismatch = 624,
    ExtTypeSessionProtocol = 625,

    // ── E7xx — R-SPU Emission ────────────────────────────────────────────────
    RspuFallback = 700,
    RspuRegisterAlloc = 701,
    RspuInstrEncoding = 702,
    RspuOpcodeUnknown = 703,
    RspuImmediateOverflow = 704,
    RspuLabelUndefined = 705,
    RspuLabelDuplicate = 706,
    RspuBranchRange = 707,
    RspuMemAlignment = 708,
    // 709–710 reserved
    RspuBytecodeVerify = 711,
    RspuParity = 712,
    RspuChecksumFail = 713,
    RspuOutputTooLarge = 714,
    // 715–719 reserved
    RspuFirmwareHeader = 720,
    RspuFirmwareSection = 721,

    // ── E8xx — S-expression / Struct / Import ────────────────────────────────
    SExprFallback = 800,
    SExprParseError = 801,
    SExprUnexpectedToken = 802,
    SExprUnclosedParen = 803,
    SExprInvalidAtom = 804,
    StructHeaderExpected = 805,
    StructNameEmpty = 806,
    StructOpenBrace = 807,
    StructMaxFields = 808,
    StructFieldSemicolon = 809,
    StructFieldColon = 810,
    StructFieldKindBad = 811,
    StructFieldTypeEmpty = 812,
    StructFieldTypeBadDecl = 813,
    SExprTooDeep = 814,
    SExprTooLong = 815,

    // ── E9xx — SAT / Equivalence ─────────────────────────────────────────────
    SatFallback = 900,
    // E901–E910 — Build Certification (MEGA-16)
    ReceiptGenerationFailed = 901,
    SourceHashMismatch = 902,
    SignatureVerificationFailed = 903,
    ToolchainHashMismatch = 904,
    MissingRequiredField = 905,
    BootstrapParityFailure = 906,
    ExplainTargetNotFound = 907,
    AstDiffOverflow = 908,
    OpcodeMismatch = 909,
    CertificateSchemaUnsupported = 910,
    SatVarLimit = 911,
    SatClauseLimit = 912,
    SatLiteralLimit = 913,
    SatAssumptionConflict = 914,
    SatDependencyLoop = 915,
    SatNoResult = 916,
    SatBadModel = 917,
    SatInternalError = 918,

    // ── E10xx — Symbolic Analysis ────────────────────────────────────────────
    SymbolicFallback = 1000,
    SymbolicWidthExceeds = 1001,
    // 1002 reserved
    SymbolicSignalLimit = 1003,

    // ── E11xx — Totality ─────────────────────────────────────────────────────
    TotalityFallback = 1100,

    // ── E12xx — Testing & Tooling (MEGA-17) ──────────────────────────────────
    RiscvEmissionFailed = 1201,
    ArmEmissionFailed = 1202,
    HlsSchedulingFailed = 1203,
    SymbolicAnalysisOverflow = 1204,
    FpgaTargetUnsupported = 1205,
    NextestConfigInvalid = 1206,
    TestFixtureMissing = 1207,
    ExampleCompilationFailed = 1208,
    ZeroDebtCloseoutFailed = 1209,
}

impl ErrorCode {
    /// Returns the canonical bracket form: `"[E806]"`.
    pub fn bracketed(self) -> String {
        format!("[E{}]", self as u16)
    }

    /// Returns the bare code string: `"E806"`.
    pub fn as_str(self) -> String {
        format!("E{}", self as u16)
    }

    /// Returns the numeric value.
    pub fn number(self) -> u16 {
        self as u16
    }

    /// Returns the error category as a human-readable label.
    pub fn category(self) -> ErrorCategory {
        match self as u16 {
            100..=199 => ErrorCategory::Parse,
            200..=299 => ErrorCategory::Semantic,
            300..=399 => ErrorCategory::Temporal,
            400..=499 => ErrorCategory::Pattern,
            500..=599 => ErrorCategory::Width,
            600..=699 => ErrorCategory::Type,
            700..=799 => ErrorCategory::Rspu,
            800..=899 => ErrorCategory::SExpr,
            900..=999 => ErrorCategory::Sat,
            1000..=1099 => ErrorCategory::Symbolic,
            1100..=1199 => ErrorCategory::Totality,
            1200..=1299 => ErrorCategory::Tooling,
            _ => ErrorCategory::Other,
        }
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "E{}", *self as u16)
    }
}

/// Broad category of an error code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCategory {
    Parse,
    Semantic,
    Temporal,
    Pattern,
    Width,
    Type,
    Rspu,
    SExpr,
    Sat,
    Symbolic,
    Totality,
    Tooling,
    Other,
}

/// Numeric shorthand for use during Phase 3 migration.
///
/// Returns the bracketed bracket form from a raw code number, e.g. `ec(806)` → `"[E806]"`.
/// Use this when migrating a raw `"[Ennn]"` literal that does not yet have a
/// named `ErrorCode` variant. Replace with a proper variant in Phase 4+.
///
/// `error_codes.rs` is excluded from the `build.rs` raw-string scanner,
/// so the `"[E"` pattern inside this function is safe.
#[inline]
pub fn ec(n: u16) -> String {
    format!("[E{}]", n)
}

// ── Convenience: build a MirrError from a typed code ──────────────────────

/// Build a `crate::error::MirrError` from a typed `ErrorCode` and message.
///
/// This is the **preferred** call site API. Use this instead of
/// `format!("[Ennn] ...")` string embedding.
///
/// ```rust,ignore
/// use crate::error_codes::{ErrorCode, mirrcode};
/// return Err(mirrcode(ErrorCode::GuardNameEmpty, "Guard name cannot be empty."));
/// ```
pub fn mirrcode(code: ErrorCode, message: impl std::fmt::Display) -> crate::error::MirrError {
    let full_msg = format!("{} {}", code.bracketed(), message);
    match code.category() {
        ErrorCategory::Parse => {
            crate::error::MirrError::ParseError { message: full_msg, span: None }
        }
        ErrorCategory::Semantic => {
            crate::error::MirrError::SemanticError { message: full_msg, span: None }
        }
        ErrorCategory::Temporal => {
            crate::error::MirrError::TemporalCompilationError { message: full_msg, span: None }
        }
        ErrorCategory::Pattern => {
            crate::error::MirrError::PatternError { message: full_msg, span: None }
        }
        ErrorCategory::Width => {
            crate::error::MirrError::WidthError { message: full_msg, span: None }
        }
        ErrorCategory::Type => crate::error::MirrError::TypeError { message: full_msg, span: None },
        ErrorCategory::Rspu => crate::error::MirrError::RspuError { message: full_msg, span: None },
        ErrorCategory::SExpr => {
            crate::error::MirrError::SExprError { message: full_msg, span: None }
        }
        ErrorCategory::Sat => crate::error::MirrError::SatError { message: full_msg, span: None },
        ErrorCategory::Symbolic => {
            crate::error::MirrError::SymbolicError { message: full_msg, span: None }
        }
        ErrorCategory::Totality => {
            crate::error::MirrError::TotalityError { message: full_msg, span: None }
        }
        ErrorCategory::Tooling => {
            crate::error::MirrError::ToolingError { message: full_msg, span: None }
        }
        ErrorCategory::Other => {
            crate::error::MirrError::SemanticError { message: full_msg, span: None }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bracketed_format() {
        assert_eq!(ErrorCode::StructNameEmpty.bracketed(), "[E806]");
        assert_eq!(ErrorCode::GuardNameEmpty.bracketed(), "[E121]");
        assert_eq!(ErrorCode::ParseFallback.bracketed(), "[E100]");
    }

    #[test]
    fn as_str_format() {
        assert_eq!(ErrorCode::DuplicateSignalName.as_str(), "E201");
    }

    #[test]
    fn number_matches_repr() {
        assert_eq!(ErrorCode::StructNameEmpty.number(), 806);
        assert_eq!(ErrorCode::OpcodeMismatch.number(), 909);
    }

    #[test]
    fn category_correct() {
        assert_eq!(ErrorCode::GuardNameEmpty.category(), ErrorCategory::Parse);
        assert_eq!(ErrorCode::DuplicateSignalName.category(), ErrorCategory::Semantic);
        assert_eq!(ErrorCode::StructNameEmpty.category(), ErrorCategory::SExpr);
        assert_eq!(ErrorCode::OpcodeMismatch.category(), ErrorCategory::Sat);
        assert_eq!(ErrorCode::SymbolicWidthExceeds.category(), ErrorCategory::Symbolic);
        assert_eq!(ErrorCode::RiscvEmissionFailed.category(), ErrorCategory::Tooling);
    }

    #[test]
    fn mirrcode_builds_correct_variant() {
        let err = mirrcode(ErrorCode::GuardNameEmpty, "Guard name cannot be empty.");
        assert!(err.message().contains("[E121]"));
        assert!(err.message().contains("Guard name cannot be empty."));
        assert!(matches!(err, crate::error::MirrError::ParseError { .. }));
    }

    #[test]
    fn mirrcode_sat_variant() {
        let err = mirrcode(ErrorCode::ReceiptGenerationFailed, "receipt failed");
        assert!(matches!(err, crate::error::MirrError::SatError { .. }));
    }

    #[test]
    fn mirrcode_tooling_variant() {
        let err = mirrcode(ErrorCode::RiscvEmissionFailed, "riscv failed");
        assert!(matches!(err, crate::error::MirrError::ToolingError { .. }));
    }
}
