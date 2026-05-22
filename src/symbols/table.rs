//! Multi-module symbol table for cross-module symbol resolution.
//!
//! Provides hierarchical symbol tables that can track symbols across multiple
//! modules and support qualified name resolution (e.g., `tmr.VoterSignal`).
//!
//! Design:
//! - `SymbolTable`: Root table managing multiple modules
//! - `ModuleSymbols`: Per-module symbol storage
//! - `SymbolInfo`: Symbol metadata (type, kind, span)
//! - `ImportScope`: Import alias management
//!
//! Bounded by NASA Power-of-10:
//! - MAX_MODULES: Maximum imported modules per compilation unit
//! - MAX_SYMBOLS_PER_MODULE: Maximum symbols per module
//! - MAX_IMPORT_ALIASES: Maximum import aliases per module
//!
//! Error codes: E901-E910.

#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::path::PathBuf;

use crate::ast::program::{ImportDecl, Module, SignalDecl};
use crate::ast::types::{ExtendedType, SignalKind, SignalType};
use crate::error::MirrError;
use crate::span::Span;

/// Maximum number of imported modules per compilation unit (NASA P10).
pub const MAX_MODULES: usize = 32;

/// Maximum symbols per module (NASA P10).
pub const MAX_SYMBOLS_PER_MODULE: usize = 256;

/// Maximum import aliases per module (NASA P10).
pub const MAX_IMPORT_ALIASES: usize = 16;

/// Information about a symbol in a module.
#[derive(Debug, Clone, PartialEq)]
pub struct SymbolInfo {
    /// Symbol name (unqualified).
    pub name: String,
    /// Signal kind (input, output, internal).
    pub kind: SignalKind,
    /// Extended type with annotations.
    pub ty: ExtendedType,
    /// Source span for diagnostics.
    pub span: Option<Span>,
    /// Module path where this symbol is defined.
    pub module_path: PathBuf,
}

/// Per-module symbol storage.
#[derive(Debug, Clone)]
pub struct ModuleSymbols {
    /// Module name.
    pub name: String,
    /// Module file path.
    pub path: PathBuf,
    /// Map from symbol name to symbol info.
    pub symbols: HashMap<String, SymbolInfo>,
    /// Import declarations in this module.
    pub imports: Vec<ImportDecl>,
}

/// Import alias scope for resolving qualified names.
#[derive(Debug, Clone)]
pub struct ImportScope {
    /// Map from alias to module path.
    pub aliases: HashMap<String, PathBuf>,
}

/// Multi-module symbol table for cross-module resolution.
#[derive(Debug, Clone)]
pub struct SymbolTable {
    /// Map from module path to module symbols.
    pub modules: HashMap<PathBuf, ModuleSymbols>,
    /// Current module being processed (for error context).
    pub current_module: Option<PathBuf>,
}

impl SymbolInfo {
    /// Create a new symbol info from a signal declaration.
    pub fn from_signal(signal: &SignalDecl, module_path: PathBuf) -> Self {
        Self {
            name: signal.name.clone(),
            kind: signal.kind,
            ty: signal.ty.clone(),
            span: signal.span,
            module_path,
        }
    }

    /// Get the core signal type for compatibility with existing code.
    pub fn signal_type(&self) -> SignalType {
        self.ty.signal_type()
    }
}

impl ModuleSymbols {
    /// Create a new module symbols container.
    pub fn new(name: String, path: PathBuf) -> Self {
        Self {
            name,
            path,
            symbols: HashMap::with_capacity(MAX_SYMBOLS_PER_MODULE),
            imports: Vec::with_capacity(MAX_IMPORT_ALIASES),
        }
    }

    /// Add a symbol to this module.
    ///
    /// Returns an error if the symbol already exists or if we exceed bounds.
    pub fn add_symbol(&mut self, symbol: SymbolInfo) -> Result<(), MirrError> {
        if self.symbols.len() >= MAX_SYMBOLS_PER_MODULE {
            return Err(MirrError::SymbolError {
                message: format!("{} Module '{}' exceeds maximum symbol count ({}).", crate::error_codes::ec(901),
                    self.name, MAX_SYMBOLS_PER_MODULE
                ),
                span: symbol.span,
            });
        }

        if self.symbols.contains_key(&symbol.name) {
            return Err(MirrError::SymbolError {
                message: format!("{} Symbol '{}' is already defined in module '{}'.", crate::error_codes::ec(902),
                    symbol.name, self.name
                ),
                span: symbol.span,
            });
        }

        self.symbols.insert(symbol.name.clone(), symbol);
        Ok(())
    }

    /// Look up a symbol by name in this module.
    pub fn get_symbol(&self, name: &str) -> Option<&SymbolInfo> {
        self.symbols.get(name)
    }

    /// Add an import declaration to this module.
    pub fn add_import(&mut self, import: ImportDecl) -> Result<(), MirrError> {
        if self.imports.len() >= MAX_IMPORT_ALIASES {
            return Err(MirrError::SymbolError {
                message: format!("{} Module '{}' exceeds maximum import count ({}).", crate::error_codes::ec(903),
                    self.name, MAX_IMPORT_ALIASES
                ),
                span: import.span,
            });
        }

        // Check for duplicate alias.
        for existing in &self.imports {
            if existing.alias == import.alias {
                return Err(MirrError::SymbolError {
                    message: format!("{} Import alias '{}' is already defined in module '{}'.", crate::error_codes::ec(904),
                        import.alias, self.name
                    ),
                    span: import.span,
                });
            }
        }

        self.imports.push(import);
        Ok(())
    }

    /// Build import scope for this module.
    pub fn import_scope(&self) -> ImportScope {
        let mut aliases = HashMap::with_capacity(self.imports.len());

        for import in &self.imports {
            let path = PathBuf::from(&import.path);
            aliases.insert(import.alias.clone(), path);
        }

        ImportScope { aliases }
    }
}

impl ImportScope {
    /// Resolve an alias to a module path.
    pub fn resolve_alias(&self, alias: &str) -> Option<&PathBuf> {
        self.aliases.get(alias)
    }

    /// Check if an alias exists.
    pub fn has_alias(&self, alias: &str) -> bool {
        self.aliases.contains_key(alias)
    }
}

impl SymbolTable {
    /// Create a new empty symbol table.
    pub fn new() -> Self {
        Self { modules: HashMap::with_capacity(MAX_MODULES), current_module: None }
    }

    /// Add a module to the symbol table.
    ///
    /// Returns an error if we exceed module limits.
    pub fn add_module(&mut self, module_symbols: ModuleSymbols) -> Result<(), MirrError> {
        if self.modules.len() >= MAX_MODULES {
            return Err(MirrError::SymbolError {
                message: format!("{} Symbol table exceeds maximum module count ({}).", crate::error_codes::ec(905),
                    MAX_MODULES
                ),
                span: None,
            });
        }

        let path = module_symbols.path.clone();
        self.modules.insert(path, module_symbols);
        Ok(())
    }

    /// Set the current module being processed.
    pub fn set_current_module(&mut self, path: PathBuf) {
        self.current_module = Some(path);
    }

    /// Get the current module.
    pub fn current_module(&self) -> Option<&ModuleSymbols> {
        self.current_module.as_ref().and_then(|path| self.modules.get(path))
    }

    /// Get a module by path.
    pub fn get_module(&self, path: &PathBuf) -> Option<&ModuleSymbols> {
        self.modules.get(path)
    }

    /// Get a mutable module by path.
    pub fn get_module_mut(&mut self, path: &PathBuf) -> Option<&mut ModuleSymbols> {
        self.modules.get_mut(path)
    }

    /// Resolve a qualified name (alias.symbol) to symbol info.
    ///
    /// Returns the resolved symbol if found, or an appropriate error.
    pub fn resolve_qualified(
        &self,
        alias: &str,
        symbol_name: &str,
    ) -> Result<&SymbolInfo, MirrError> {
        // Get the current module's import scope.
        let current = self.current_module().ok_or_else(|| MirrError::SymbolError {
            message: format!("{} No current module set for symbol resolution.", crate::error_codes::ec(906)),
            span: None,
        })?;

        let import_scope = current.import_scope();

        // Resolve the alias to a module path.
        let target_path =
            import_scope.resolve_alias(alias).ok_or_else(|| MirrError::SymbolError {
                message: format!("{} Unknown import alias '{}' in module '{}'.", crate::error_codes::ec(907),
                    alias, current.name
                ),
                span: None,
            })?;

        // Get the target module.
        let target_module = self.get_module(target_path).ok_or_else(|| MirrError::SymbolError {
            message: format!("{} Imported module '{}' (alias '{}') not found in symbol table.", crate::error_codes::ec(908),
                target_path.display(),
                alias
            ),
            span: None,
        })?;

        // Look up the symbol in the target module.
        target_module.get_symbol(symbol_name).ok_or_else(|| MirrError::SymbolError {
            message: format!("{} Symbol '{}' not found in imported module '{}' (alias '{}').", crate::error_codes::ec(909),
                symbol_name, target_module.name, alias
            ),
            span: None,
        })
    }

    /// Resolve an unqualified symbol name in the current module.
    pub fn resolve_local(&self, symbol_name: &str) -> Result<&SymbolInfo, MirrError> {
        let current = self.current_module().ok_or_else(|| MirrError::SymbolError {
            message: format!("{} No current module set for symbol resolution.", crate::error_codes::ec(906)),
            span: None,
        })?;

        current.get_symbol(symbol_name).ok_or_else(|| MirrError::SymbolError {
            message: format!("{} Symbol '{}' not found in current module '{}'.", crate::error_codes::ec(910),
                symbol_name, current.name
            ),
            span: None,
        })
    }

    /// Build a symbol table from a parsed module and its imports.
    ///
    /// This is the main entry point for constructing symbol tables from
    /// parsed ASTs. It processes the module and recursively loads imports.
    ///
    /// Note: `imports` comes from `MirrProgram.imports`, not from `Module`.
    pub fn from_module(
        module: &Module,
        imports: &[ImportDecl],
        module_path: PathBuf,
        load_program: impl Fn(&PathBuf) -> Result<(Module, Vec<ImportDecl>), MirrError>,
    ) -> Result<Self, MirrError> {
        let mut table = Self::new();
        let mut processed = HashMap::with_capacity(MAX_MODULES);

        // Build the symbol table recursively.
        Self::build_from_module(
            module,
            imports,
            module_path.clone(),
            &mut table,
            &mut processed,
            &load_program,
        )?;

        // Set the main module as current.
        table.set_current_module(module_path);

        Ok(table)
    }

    /// Recursively build symbol table from a module and its imports.
    ///
    /// Note: `imports` comes from `MirrProgram.imports`, not from `Module`.
    fn build_from_module(
        module: &Module,
        imports: &[ImportDecl],
        module_path: PathBuf,
        table: &mut SymbolTable,
        processed: &mut HashMap<PathBuf, ()>,
        load_program: &impl Fn(&PathBuf) -> Result<(Module, Vec<ImportDecl>), MirrError>,
    ) -> Result<(), MirrError> {
        // Skip if already processed (cycle breaking).
        if processed.contains_key(&module_path) {
            return Ok(());
        }
        processed.insert(module_path.clone(), ());

        // Create module symbols container.
        let mut module_symbols = ModuleSymbols::new(module.name.clone(), module_path.clone());

        // Add all signals from this module.
        for signal in &module.signals {
            let symbol_info = SymbolInfo::from_signal(signal, module_path.clone());
            module_symbols.add_symbol(symbol_info)?;
        }

        // Add import declarations (from MirrProgram.imports).
        for (import_count, import) in imports.iter().enumerate() {
            if import_count >= MAX_IMPORT_ALIASES {
                break;
            }
            module_symbols.add_import(import.clone())?;
        }

        // Add this module to the table.
        table.add_module(module_symbols)?;

        // Recursively process imports.
        for import in imports {
            let import_path = PathBuf::from(&import.path);

            if !processed.contains_key(&import_path) {
                // Load the imported program (module + imports).
                let (imported_module, imported_imports) = load_program(&import_path)?;

                // Recursively process it.
                Self::build_from_module(
                    &imported_module,
                    &imported_imports,
                    import_path,
                    table,
                    processed,
                    load_program,
                )?;
            }
        }

        Ok(())
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}
