//! S-expression type definitions.
//!
//! The `SExpr` enum is the core data type for the homoiconic representation.
//! Every MIRR AST node maps to a unique S-expression form.

#![forbid(unsafe_code)]

use std::fmt;

/// An S-expression value.
///
/// S-expressions are the universal data representation for MIRR's
/// code-as-data layer. Every variant is bounded: lists by `MAX_SEXPR_NODES`,
/// strings by `MAX_SEXPR_STRING_LEN`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SExpr {
    /// Atom: symbol (identifier or keyword).
    /// Used for head tags (`signal`, `guard`, `always`), operators, kinds.
    Symbol(String),

    /// Atom: unsigned integer literal.
    /// Used for widths, cycle counts, delay values.
    Integer(u64),

    /// Atom: boolean literal.
    /// True/false for guard conditions, literal values.
    Bool(bool),

    /// Atom: string literal.
    /// Used for signal names, module names, pattern names.
    Str(String),

    /// Compound: ordered list of sub-expressions.
    /// Tagged lists: `(signal "name" input (unsigned 16))`
    List(Vec<SExpr>),

    /// Quoted expression: `'(...)`
    /// Returns the expression unevaluated.
    Quote(Box<SExpr>),

    /// Quasiquoted expression: `` `(...) ``
    /// Template with unquote splices.
    Quasiquote(Box<SExpr>),

    /// Unquote splice inside quasiquote: `,expr`
    /// Evaluated and spliced into the enclosing quasiquote.
    Unquote(Box<SExpr>),
}

impl SExpr {
    // -- Constructors --

    /// Create a symbol atom.
    pub fn sym(s: &str) -> Self {
        Self::Symbol(s.to_string())
    }

    /// Create an integer atom.
    pub fn int(n: u64) -> Self {
        Self::Integer(n)
    }

    /// Create a boolean atom.
    pub fn bool_val(b: bool) -> Self {
        Self::Bool(b)
    }

    /// Create a string atom.
    pub fn str_val(s: &str) -> Self {
        Self::Str(s.to_string())
    }

    /// Create a list from a vector of sub-expressions.
    pub fn list(items: Vec<SExpr>) -> Self {
        Self::List(items)
    }

    // -- Predicates --

    /// True if this is a symbol.
    pub fn is_symbol(&self) -> bool {
        matches!(self, Self::Symbol(_))
    }

    /// True if this is an integer.
    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }

    /// True if this is a boolean.
    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(_))
    }

    /// True if this is a string literal.
    pub fn is_str(&self) -> bool {
        matches!(self, Self::Str(_))
    }

    /// True if this is a list.
    pub fn is_list(&self) -> bool {
        matches!(self, Self::List(_))
    }

    /// True if this is an atom (not a list, quote, quasiquote, or unquote).
    pub fn is_atom(&self) -> bool {
        matches!(self, Self::Symbol(_) | Self::Integer(_) | Self::Bool(_) | Self::Str(_))
    }

    // -- Accessors --

    /// Extract the symbol string, if this is a symbol.
    pub fn as_symbol(&self) -> Option<&str> {
        match self {
            Self::Symbol(s) => Some(s),
            _ => None,
        }
    }

    /// Extract the integer value, if this is an integer.
    pub fn as_integer(&self) -> Option<u64> {
        match self {
            Self::Integer(n) => Some(*n),
            _ => None,
        }
    }

    /// Extract the boolean value, if this is a boolean.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Extract the string value, if this is a string literal.
    pub fn as_str_val(&self) -> Option<&str> {
        match self {
            Self::Str(s) => Some(s),
            _ => None,
        }
    }

    /// Extract the list elements, if this is a list.
    pub fn as_list(&self) -> Option<&[SExpr]> {
        match self {
            Self::List(items) => Some(items),
            _ => None,
        }
    }

    /// Return the head symbol of a tagged list, if this is a non-empty list
    /// whose first element is a symbol.
    pub fn head_symbol(&self) -> Option<&str> {
        match self {
            Self::List(items) if !items.is_empty() => items[0].as_symbol(),
            _ => None,
        }
    }

    /// Count total nodes in this S-expression tree (bounded traversal).
    pub fn node_count(&self) -> usize {
        let mut count = 0usize;
        let mut stack: Vec<&SExpr> = vec![self];
        let limit = super::MAX_SEXPR_NODES + 1;
        while let Some(node) = stack.pop() {
            count += 1;
            if count >= limit {
                return count;
            }
            match node {
                Self::List(items) => {
                    for item in items.iter().rev() {
                        stack.push(item);
                    }
                }
                Self::Quote(inner) | Self::Quasiquote(inner) | Self::Unquote(inner) => {
                    stack.push(inner);
                }
                _ => {}
            }
        }
        count
    }
}

impl fmt::Display for SExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Symbol(s) => write!(f, "{s}"),
            Self::Integer(n) => write!(f, "{n}"),
            Self::Bool(true) => write!(f, "true"),
            Self::Bool(false) => write!(f, "false"),
            Self::Str(s) => write!(f, "\"{s}\""),
            Self::List(items) => {
                write!(f, "(")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{item}")?;
                }
                write!(f, ")")
            }
            Self::Quote(inner) => write!(f, "'{inner}"),
            Self::Quasiquote(inner) => write!(f, "`{inner}"),
            Self::Unquote(inner) => write!(f, ",{inner}"),
        }
    }
}
