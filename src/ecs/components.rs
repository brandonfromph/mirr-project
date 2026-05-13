#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use crate::ast::types::SignalKind;
use crate::ast::types::ExtendedType;

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
