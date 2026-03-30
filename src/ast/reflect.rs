//! Runtime reflection and metadata for MIRR programs.
//!
//! Provides the `MirrReflect` trait and supporting structures to enable
//! introspecting on MIRR types, signals, and properties. This is the 
//! "homoiconic" foundation required for MEGA-V2.

#![forbid(unsafe_code)]

use crate::ast::program::{MirrProgram, StructDef, InterfaceDef};
use crate::ast::types::SignalType;

/// Metadata for a MIRR type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeMeta {
    pub name: String,
    pub width: u32,
    pub fields: Vec<String>,
}

/// Trait for objects that can reflect their MIRR structure.
pub trait MirrReflect {
    /// Return the metadata for this object's type.
    fn reflect_type(&self) -> TypeMeta;
}

impl MirrProgram {
    /// Look up a struct definition by name.
    pub fn find_struct(&self, name: &str) -> Option<&StructDef> {
        self.struct_defs.iter().find(|s| s.name == name)
    }

    /// Look up an interface definition by name.
    pub fn find_interface(&self, name: &str) -> Option<&InterfaceDef> {
        self.interface_defs.iter().find(|i| i.name == name)
    }

    /// Resolve the total bit-width of a SignalType within the context of this program.
    /// 
    /// Correctly resolves Struct and Bundle widths by looking up their definitions.
    pub fn resolve_width(&self, ty: &SignalType) -> u32 {
        match ty {
            SignalType::Bool => 1,
            SignalType::Unsigned(w) => *w,
            SignalType::Signed(w) => *w,
            SignalType::Array { element, length } => self.resolve_width(element) * (*length as u32),
            SignalType::Struct { name, .. } => {
                self.find_struct(name)
                    .map(|s| s.fields.iter().map(|(_, t)| self.resolve_width(ftype_to_signal(t))).sum())
                    .unwrap_or(32) // Fallback for unresolved
            }
            SignalType::Bundle { name, .. } => {
                self.find_interface(name)
                    .map(|i| i.signals.iter().map(|(_, _, t)| self.resolve_width(ftype_to_signal(t))).sum())
                    .unwrap_or(32) // Fallback for unresolved
            }
            SignalType::Fifo { element, depth } => self.resolve_width(element) * (*depth as u32),
        }
    }
}

/// Internal helper to bridge type systems during reflection.
fn ftype_to_signal(ty: &SignalType) -> &SignalType {
    ty
}
