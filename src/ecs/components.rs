#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use crate::ast::types::{SignalKind, ExtendedType, BinaryOp, UnaryOp, LiteralValue};

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

/// Component: Type (Width, Refinement, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeComponent(pub ExtendedType);

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
