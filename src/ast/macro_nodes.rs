//! AST nodes for structural macros (Phase 3).
//!
//! These nodes represent unexpanded macro constructs like `for` loops, `match` blocks,
//! and `if/else` statements. They are parsed into an intermediate AST and then lowered
//! into the core MIRR AST (`Module`, `SignalDecl`, `Guard`, `Reflex`) by `ast_expand.rs`.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

use super::expr::Expr;
use super::program::{Assignment, Guard, SignalDecl};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModuleMacroStmt {
    Signal(SignalDecl),
    Guard(Guard),
    Reflex(UnexpandedReflex),
    Property(super::property::PropertyDecl),
    PatternCall(super::pattern::PatternCall),
    ForLoop { var: String, start: i32, end: i32, body: Vec<ModuleMacroStmt> },
    LetBinding { name: String, ty: String, value: Expr },
    ClockDomain(super::program::ClockDomainDecl),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnexpandedReflex {
    pub name: String,
    pub guard_names: Vec<String>,
    pub statements: Vec<ReflexMacroStmt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<crate::span::Span>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReflexMacroStmt {
    Assignment(Assignment),
    LetBinding {
        name: String,
        ty: String,
        value: Expr,
        span: Option<crate::span::Span>,
    },
    OnBlock {
        guard_names: Vec<String>,
        body: Vec<ReflexMacroStmt>,
    },
    ForLoop {
        var: String,
        start: i32,
        end: i32,
        body: Vec<ReflexMacroStmt>,
    },
    IfElse {
        condition: Expr,
        true_branch: Vec<ReflexMacroStmt>,
        false_branch: Vec<ReflexMacroStmt>,
    },
    Match {
        expr: Expr,
        arms: Vec<MatchArm>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchArm {
    pub pattern: String,
    pub body: Vec<ReflexMacroStmt>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnexpandedModule {
    pub name: String,
    pub clock_domains: Vec<super::program::ClockDomainDecl>,
    pub statements: Vec<ModuleMacroStmt>,
    pub properties: Vec<super::property::PropertyDecl>,
    pub pattern_calls: Vec<super::pattern::PatternCall>,
}
