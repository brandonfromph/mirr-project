#![forbid(unsafe_code)]

use crate::ast::types::{BinaryOp, ExtendedType, LiteralValue, SignalKind, UnaryOp};
use serde::{Deserialize, Serialize};

/// The Entity ID: The fundamental atom of the ECS compiler.
/// Fixed-size u32 (NASA P10 Rule #1 & #2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct EntityId(pub u32);

/// Component: Signal Name
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NameComponent(pub String);

/// Component: Signal Kind (in/out/internal)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct KindComponent(pub SignalKind);

impl KindComponent {
    pub const PATTERN: Self = KindComponent(SignalKind::Internal);
    pub const MODULE: Self = KindComponent(SignalKind::Internal);
    pub const SIGNAL: Self = KindComponent(SignalKind::Internal);
}

/// Component: Type (Width, Refinement, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeComponent(pub ExtendedType);

impl TypeComponent {
    pub fn pattern(_p: crate::ast::pattern::PatternDef) -> Self {
        TypeComponent(ExtendedType::new(crate::ast::types::SignalType::Bool, Default::default()))
    }
    pub fn signal(t: ExtendedType) -> Self {
        TypeComponent(t)
    }
}

/// Component: Parent Module ID
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ModuleComponent(pub EntityId);

/// Component: Temporal Cycle Count (for Guards)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CyclesComponent(pub u64);

/// Component: Reference to a Guard condition expression
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ConditionComponent(pub EntityId);

// --- Expression Components (Flat Representation) ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiteralComponent(pub LiteralValue);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct UnaryComponent {
    pub op: UnaryOp,
    pub operand: EntityId,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BinaryComponent {
    pub op: BinaryOp,
    pub left: EntityId,
    pub right: EntityId,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PrevComponent {
    pub signal: EntityId,
    pub delay: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SignalRefComponent(pub EntityId);

// --- Knowledge Base Components (Grounding) ---

/// Component: High-dimensional vector embedding (e.g. 1536d)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorComponent(pub Vec<f32>);

/// Component: The raw source text of a code chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkTextComponent(pub String);

/// Component: Source file path relative to workspace root
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourcePathComponent(pub String);

/// Component: Line range in the source file
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LineRangeComponent(pub (usize, usize));
