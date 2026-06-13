#![forbid(unsafe_code)]

use crate::ast::types::{BinaryOp, ExtendedType, LiteralValue, UnaryOp};
use serde::{Deserialize, Serialize};

/// The Entity ID: The fundamental atom of the ECS compiler.
/// Fixed-size u32 (NASA P10 Rule #1 & #2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct EntityId(pub u32);

/// Component: Signal Name
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NameComponent(pub String);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EntityKind {
    SIGNAL(crate::ast::types::SignalKind),
    GUARD,
    REFLEX,
    PROPERTY,
    MODULE,
    PATTERN,
    ASSIGNMENT,
}

impl EntityKind {
    pub fn describe(&self) -> &'static str {
        match self {
            EntityKind::SIGNAL(_) => "signal",
            EntityKind::GUARD => "guard",
            EntityKind::REFLEX => "reflex",
            EntityKind::PROPERTY => "property",
            EntityKind::MODULE => "module",
            EntityKind::PATTERN => "pattern",
            EntityKind::ASSIGNMENT => "assignment",
        }
    }
}

/// Component: Signal Kind (in/out/internal)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct KindComponent(pub EntityKind);

impl KindComponent {
    pub const PATTERN: Self = KindComponent(EntityKind::PATTERN);
    pub const MODULE: Self = KindComponent(EntityKind::MODULE);
    pub const SIGNAL: Self =
        KindComponent(EntityKind::SIGNAL(crate::ast::types::SignalKind::Internal));
    pub const GUARD: Self = KindComponent(EntityKind::GUARD);
    pub const REFLEX: Self = KindComponent(EntityKind::REFLEX);
    pub const ASSIGNMENT: Self = KindComponent(EntityKind::ASSIGNMENT);
    pub const PROPERTY: Self = KindComponent(EntityKind::PROPERTY);
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

/// Component: Namespace Scope from imports (e.g. `isa` from `import "isa_map.mirr" as isa`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleScopeComponent(pub String);

/// Component: Pattern Definition for pattern expansion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternDefComponent(pub crate::ast::pattern::PatternDef);

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingSignalRef(pub String);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ArrayIndexComponent {
    pub array: EntityId,
    pub index: EntityId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldAccessComponent {
    pub object: EntityId,
    pub field: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrayLiteralComponent(pub Vec<EntityId>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructLiteralComponent {
    pub name: String,
    pub fields: Vec<(String, EntityId)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnfoldIndexComponent(pub String);

/// Component: Multiplexer (SmaRTLy Optimization Target)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MuxComponent {
    pub select: EntityId,
    pub true_val: EntityId,
    pub false_val: EntityId,
}

/// Component: Source code span for diagnostic tracing.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SpanComponent(pub crate::span::Span);

/// Component: High-dimensional vector embedding (e.g. 1536d)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorComponent(pub Vec<f32>);

/// Component: Reflex (Logic that reacts to guards)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflexComponent {
    pub guards: Vec<EntityId>,
    pub assignments: Vec<EntityId>,
}

/// Component: Assignment within a reflex
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AssignmentComponent {
    pub target: EntityId,
    pub value: EntityId,
}

/// Component: Property assertion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyComponent {
    pub formula: crate::ast::property::PropertyFormula,
    pub formula_exprs: Vec<EntityId>,
}

/// Component: The raw source text of a code chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkTextComponent(pub String);

/// Component: Source file path relative to workspace root
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourcePathComponent(pub String);

/// Component: Line range in the source file
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LineRangeComponent(pub (usize, usize));

// --- Instruction & Dispatch Components (Phase 1 RS-16) ---

/// Component: Numeric opcode for an instruction (e.g. ADD=0).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct OpcodeComponent(pub u16);

/// Component: Mapping of opcodes to implementation Reflex Entities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructionTableComponent {
    /// Map of opcode to entity ID of the reflex/pattern that implements it.
    pub opcodes: std::collections::HashMap<u16, EntityId>,
}

// --- Width Inference Components (Phase 4a ECS) ---

/// Defines the minimum mathematical width of an expression based on operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WidthConstraintComponent {
    /// Node must be exactly `width` bits (literal or declared signal).
    Fixed(u32),
    /// Node width = max(left, right) + 1  (for Add).
    MaxPlusOne { left: EntityId, right: EntityId },
    /// Node width = max(left, right)  (for Sub, And, Or, Xor).
    MaxOf { left: EntityId, right: EntityId },
    /// Node width = left + right  (for Mul).
    SumOf { left: EntityId, right: EntityId },
    /// Node width = left_width + shift_const  (for Shl with constant shift).
    LeftPlusConst { left: EntityId, shift_amount: u32 },
    /// Node width = left_width + 63  (for Shl with variable shift — worst case).
    LeftPlusMaxShift { left: EntityId },
    /// Node width = max(1, left_width - shift_const)  (for Shr with constant shift).
    LeftMinusConst { left: EntityId, shift_amount: u32 },
    /// Node width = left_width  (for Shr with variable shift, Unary Not).
    SameAs { source: EntityId },
    /// Node width = source_width + 1  (for unsigned-to-signed negate).
    SameAsPlusOne { source: EntityId },
    /// Node width = sw.min(narrow_width) (for BitwiseAnd with literal).
    Narrowed { source: EntityId, narrow_width: u32 },
    /// Node width = 1  (for comparison operators and boolean literals).
    Boolean,
    /// Node width = sum of all element widths (for array/struct literals).
    SumAll { elements: Vec<EntityId> },
}

/// The hardware implementation strategy chosen during temporal synthesis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemporalStrategy {
    /// Shift-register pipeline for short delays (N ≤ 16 cycles).
    ShiftRegister,
    /// Saturating counter + comparator for long delays (N > 16 cycles).
    Counter { counter_width: u32 },
    /// Runtime-expression delay with static upper bound.
    DynamicCounter { max_delay: u64, counter_width: u32 },
    /// Multi-component complex guard.
    Complex,
}

/// Component: Compiled temporal hardware primitive attached to a Guard entity.
///
/// Set by `temporal_synthesis_system()` in `src/ecs/systems.rs` after lowering.
/// Provides full EntityId-level traceability from MIRR guard declaration to RTL.
/// Required for DO-178C artifact traceability (declared in Proposal 110 Wave 4).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalNodeComponent {
    /// The synthesis strategy that was applied to this guard.
    pub strategy: TemporalStrategy,
    /// Names of all generated hardware signals (SR stages, counter, comparator).
    pub generated_signals: Vec<String>,
    /// The final output signal name that represents the active guard condition.
    pub output_signal: String,
    /// The cycle count this guard was compiled with.
    pub delay_cycles: u64,
}
