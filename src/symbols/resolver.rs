//! Cross-module symbol resolution and namespace management.
//!
//! This module provides high-level symbol resolution functionality that integrates
//! symbol tables with the import system. It enables proper symbol visibility across
//! module boundaries and provides unified resolution for both local and imported symbols.
//!
//! Key features:
//! - Cross-module symbol resolution using import context
//! - Symbol visibility and namespace management
//! - Integration between symbol tables and import loading
//!
//! WARNING: Cross-module resolution is central to MEGA-11 and MEGA-5.
//! Do not remove alias/symbol conflict checking semantics without updating
//! all subsystem interactions (validation/typechecking/emit).
//! - Type-preserving symbol lookups across module boundaries
//! - Proper error reporting for symbol resolution failures
//!
//! Error codes: E911-E920.

#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::path::PathBuf;

use crate::ast::program::{MirrProgram, Module};
use crate::ast::types::ExtendedType;
use crate::error::MirrError;
use crate::import::ImportContext;
use crate::span::Span;

use super::{ModuleSymbols, SymbolInfo, SymbolTable};

/// Symbol conflict information.
#[derive(Debug, Clone, PartialEq)]
pub struct SymbolConflict {
    /// Name of the conflicting symbol
    pub symbol_name: String,
    /// Modules that have conflicting symbols with this name
    pub conflicting_modules: Vec<PathBuf>,
}

/// Cross-module symbol resolver that integrates with the import system.
#[derive(Debug)]
pub struct CrossModuleResolver {
    /// Symbol table containing all loaded modules
    symbol_table: SymbolTable,
    /// Import context with resolved files and dependencies
    import_context: ImportContext,
    /// Cache for resolved qualified symbols (alias.symbol -> SymbolInfo)
    qualified_cache: HashMap<String, SymbolInfo>,
}

/// Symbol resolution result containing the resolved symbol and its metadata.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedSymbol {
    /// The resolved symbol information
    pub symbol: SymbolInfo,
    /// The module where this symbol was resolved from
    pub source_module: PathBuf,
    /// Whether this was a local or cross-module resolution
    pub is_local: bool,
    /// The qualified name used for resolution (if applicable)
    pub qualified_name: Option<String>,
}

impl ResolvedSymbol {
    /// Create a new resolved symbol from local resolution.
    pub fn local(symbol: SymbolInfo, source_module: PathBuf) -> Self {
        Self { symbol, source_module, is_local: true, qualified_name: None }
    }

    /// Create a new resolved symbol from cross-module resolution.
    pub fn cross_module(
        symbol: SymbolInfo,
        source_module: PathBuf,
        qualified_name: String,
    ) -> Self {
        Self { symbol, source_module, is_local: false, qualified_name: Some(qualified_name) }
    }

    /// Get the extended type of the resolved symbol.
    pub fn extended_type(&self) -> &ExtendedType {
        &self.symbol.ty
    }

    /// Get a display string for this resolved symbol.
    pub fn display_name(&self) -> String {
        if let Some(ref qualified) = self.qualified_name {
            qualified.clone()
        } else {
            self.symbol.name.clone()
        }
    }
}

impl CrossModuleResolver {
    /// Create a new cross-module resolver from symbol table and import context.
    pub fn new(symbol_table: SymbolTable, import_context: ImportContext) -> Self {
        Self { symbol_table, import_context, qualified_cache: HashMap::new() }
    }

    /// Build a cross-module resolver from a main program and its imports.
    ///
    /// This is the primary factory method for creating resolvers. It processes
    /// the main program, loads all imports, builds the symbol table, and creates
    /// a fully functional cross-module resolver.
    pub fn from_program(
        program: &MirrProgram,
        main_path: PathBuf,
        load_program: impl Fn(
            &PathBuf,
        )
            -> Result<(Module, Vec<crate::ast::program::ImportDecl>), MirrError>,
    ) -> Result<Self, MirrError> {
        // Build symbol table from the main module (imports come from MirrProgram)
        let symbol_table = SymbolTable::from_module(
            &program.module,
            &program.imports,
            main_path.clone(),
            load_program,
        )?;

        // For now, create an empty import context - this should be integrated
        // with the actual import resolution in a future update
        let import_context = ImportContext::new();

        Ok(Self::new(symbol_table, import_context))
    }

    /// Build a cross-module resolver with full import resolution.
    ///
    /// This method integrates with the import system to provide complete
    /// cross-module symbol resolution. It loads all imports, builds dependency
    /// graphs, and creates a unified symbol table across all modules.
    pub fn from_program_with_imports(
        program: &MirrProgram,
        main_path: PathBuf,
    ) -> Result<Self, MirrError> {
        use crate::import::resolve_imports;

        // First resolve all imports
        let base_dir = main_path.parent().ok_or_else(|| MirrError::SymbolError {
            message: "[E918] Cannot determine base directory for import resolution.".to_string(),
            span: None,
        })?;

        let import_context = resolve_imports(program, base_dir)?;

        // Build symbol table starting from main module
        let mut symbol_table = SymbolTable::new();

        // Add main module
        let main_module_symbols =
            Self::build_module_symbols(&program.module, &program.imports, main_path.clone())?;
        symbol_table.add_module(main_module_symbols)?;
        symbol_table.set_current_module(main_path);

        // Add all imported modules to the symbol table
        for (file_path, resolved_file) in &import_context.files {
            let module_symbols = Self::build_module_symbols(
                &resolved_file.program.module,
                &resolved_file.program.imports,
                file_path.clone(),
            )?;
            symbol_table.add_module(module_symbols)?;
        }

        Ok(Self::new(symbol_table, import_context))
    }

    /// Helper to build module symbols from a parsed module and its imports.
    ///
    /// Note: `imports` comes from `MirrProgram.imports`, not from `Module`.
    fn build_module_symbols(
        module: &Module,
        imports: &[crate::ast::program::ImportDecl],
        module_path: PathBuf,
    ) -> Result<ModuleSymbols, MirrError> {
        let mut module_symbols = ModuleSymbols::new(module.name.clone(), module_path.clone());

        // Add all signals
        for signal in &module.signals {
            let symbol_info = SymbolInfo::from_signal(signal, module_path.clone());
            module_symbols.add_symbol(symbol_info)?;
        }

        // Add imports (from MirrProgram.imports)
        for import in imports {
            module_symbols.add_import(import.clone())?;
        }

        Ok(module_symbols)
    }

    /// Set the current module for resolution context.
    pub fn set_current_module(&mut self, module_path: PathBuf) {
        self.symbol_table.set_current_module(module_path);
    }

    /// Resolve a symbol name, trying both local and qualified resolution.
    ///
    /// This is the main entry point for symbol resolution. It first attempts
    /// local resolution in the current module, then tries to parse the name
    /// as a qualified name (alias.symbol) and resolve it cross-module.
    pub fn resolve_symbol(
        &mut self,
        symbol_name: &str,
        span: Option<Span>,
    ) -> Result<ResolvedSymbol, MirrError> {
        // First try local resolution
        if let Ok(symbol) = self.symbol_table.resolve_local(symbol_name) {
            let current_module =
                self.symbol_table.current_module().ok_or_else(|| MirrError::SymbolError {
                    message: "[E911] No current module set for symbol resolution.".to_string(),
                    span,
                })?;

            return Ok(ResolvedSymbol::local(symbol.clone(), current_module.path.clone()));
        }

        // Try qualified resolution (alias.symbol)
        if let Some((alias, name)) = self.parse_qualified_name(symbol_name) {
            self.resolve_qualified_symbol(&alias, &name, span)
        } else {
            // Symbol not found in any scope
            let unknown = "<unknown>".to_string();
            let current = self.symbol_table.current_module().map(|m| &m.name).unwrap_or(&unknown);

            Err(MirrError::SymbolError {
                message: format!(
                    "[E912] Symbol '{}' not found in current module '{}' or any imported modules.",
                    symbol_name, current
                ),
                span,
            })
        }
    }

    /// Resolve a qualified symbol name (alias.symbol).
    pub fn resolve_qualified_symbol(
        &mut self,
        alias: &str,
        symbol_name: &str,
        span: Option<Span>,
    ) -> Result<ResolvedSymbol, MirrError> {
        let qualified_name = format!("{}.{}", alias, symbol_name);

        // Check cache first
        if let Some(cached) = self.qualified_cache.get(&qualified_name) {
            // Find the source module for the cached symbol
            let source_module = cached.module_path.clone();
            return Ok(ResolvedSymbol::cross_module(cached.clone(), source_module, qualified_name));
        }

        // Resolve using symbol table
        let outer_span = span;
        let symbol =
            self.symbol_table.resolve_qualified(alias, symbol_name).map_err(|mut err| {
                // Enhance error with span information
                if let MirrError::SymbolError { ref mut span, .. } = err {
                    *span = span.or(outer_span);
                }
                err
            })?;

        let resolved = ResolvedSymbol::cross_module(
            symbol.clone(),
            symbol.module_path.clone(),
            qualified_name.clone(),
        );

        // Cache the result
        self.qualified_cache.insert(qualified_name, symbol.clone());

        Ok(resolved)
    }

    /// Get all available symbols in the current module.
    pub fn list_local_symbols(&self) -> Result<Vec<&SymbolInfo>, MirrError> {
        let current = self.symbol_table.current_module().ok_or_else(|| MirrError::SymbolError {
            message: "[E913] No current module set for symbol listing.".to_string(),
            span: None,
        })?;

        Ok(current.symbols.values().collect())
    }

    /// Get all available imports in the current module.
    pub fn list_available_imports(&self) -> Result<Vec<String>, MirrError> {
        let current = self.symbol_table.current_module().ok_or_else(|| MirrError::SymbolError {
            message: "[E914] No current module set for import listing.".to_string(),
            span: None,
        })?;

        let import_scope = current.import_scope();
        Ok(import_scope.aliases.keys().cloned().collect())
    }

    /// Check if a symbol exists in the current module's scope.
    pub fn symbol_exists(&self, symbol_name: &str) -> bool {
        // Check local symbols
        if self.symbol_table.resolve_local(symbol_name).is_ok() {
            return true;
        }

        // Check qualified symbols
        if let Some((alias, name)) = self.parse_qualified_name(symbol_name) {
            self.symbol_table.resolve_qualified(&alias, &name).is_ok()
        } else {
            false
        }
    }

    /// Get the type of a symbol without full resolution.
    pub fn get_symbol_type(&self, symbol_name: &str) -> Result<ExtendedType, MirrError> {
        // Try local first
        if let Ok(symbol) = self.symbol_table.resolve_local(symbol_name) {
            return Ok(symbol.ty.clone());
        }

        // Try qualified
        if let Some((alias, name)) = self.parse_qualified_name(symbol_name) {
            let symbol = self.symbol_table.resolve_qualified(&alias, &name)?;
            Ok(symbol.ty.clone())
        } else {
            Err(MirrError::SymbolError {
                message: format!(
                    "[E915] Cannot determine type of unknown symbol '{}'.",
                    symbol_name
                ),
                span: None,
            })
        }
    }

    /// Add a module to the resolver's symbol table.
    pub fn add_module(&mut self, module_symbols: ModuleSymbols) -> Result<(), MirrError> {
        self.symbol_table.add_module(module_symbols)
    }

    /// Get the underlying symbol table (immutable).
    pub fn symbol_table(&self) -> &SymbolTable {
        &self.symbol_table
    }

    /// Get the underlying import context (immutable).
    pub fn import_context(&self) -> &ImportContext {
        &self.import_context
    }

    /// Check if a symbol is visible from the current module.
    ///
    /// Symbol visibility rules:
    /// 1. All symbols in the current module are visible
    /// 2. Symbols from imported modules are visible via qualified names
    /// 3. Private symbols (if implemented) are only visible within their module
    pub fn is_symbol_visible(&self, symbol: &SymbolInfo, from_module: &PathBuf) -> bool {
        // Same module - always visible
        if symbol.module_path == *from_module {
            return true;
        }

        // Different module - check if it's imported
        if let Some(current) = self.symbol_table.get_module(from_module) {
            let import_scope = current.import_scope();
            // Check if the symbol's module is in our import scope
            for alias_path in import_scope.aliases.values() {
                if *alias_path == symbol.module_path {
                    return true;
                }
            }
        }

        false
    }

    /// Get all symbols visible from the current module.
    pub fn get_visible_symbols(&self) -> Result<Vec<ResolvedSymbol>, MirrError> {
        let current = self.symbol_table.current_module().ok_or_else(|| MirrError::SymbolError {
            message: "[E919] No current module set for visibility check.".to_string(),
            span: None,
        })?;

        let mut visible_symbols = Vec::new();

        // Add local symbols
        for symbol in current.symbols.values() {
            visible_symbols.push(ResolvedSymbol::local(symbol.clone(), current.path.clone()));
        }

        // Add imported symbols
        let import_scope = current.import_scope();
        for (alias, module_path) in &import_scope.aliases {
            if let Some(imported_module) = self.symbol_table.get_module(module_path) {
                for symbol in imported_module.symbols.values() {
                    let qualified_name = format!("{}.{}", alias, symbol.name);
                    visible_symbols.push(ResolvedSymbol::cross_module(
                        symbol.clone(),
                        module_path.clone(),
                        qualified_name,
                    ));
                }
            }
        }

        Ok(visible_symbols)
    }

    /// Get symbols by namespace (module alias).
    pub fn get_symbols_in_namespace(&self, alias: &str) -> Result<Vec<ResolvedSymbol>, MirrError> {
        let current = self.symbol_table.current_module().ok_or_else(|| MirrError::SymbolError {
            message: "[E920] No current module set for namespace query.".to_string(),
            span: None,
        })?;

        let import_scope = current.import_scope();
        let module_path =
            import_scope.resolve_alias(alias).ok_or_else(|| MirrError::SymbolError {
                message: format!(
                    "[E907] Unknown import alias '{}' in module '{}'.",
                    alias, current.name
                ),
                span: None,
            })?;

        let target_module =
            self.symbol_table.get_module(module_path).ok_or_else(|| MirrError::SymbolError {
                message: format!(
                    "[E908] Imported module '{}' (alias '{}') not found in symbol table.",
                    module_path.display(),
                    alias
                ),
                span: None,
            })?;

        let mut namespace_symbols = Vec::new();
        for symbol in target_module.symbols.values() {
            let qualified_name = format!("{}.{}", alias, symbol.name);
            namespace_symbols.push(ResolvedSymbol::cross_module(
                symbol.clone(),
                module_path.clone(),
                qualified_name,
            ));
        }

        Ok(namespace_symbols)
    }

    /// Check for symbol name conflicts across visible modules.
    pub fn check_symbol_conflicts(&self) -> Result<Vec<SymbolConflict>, MirrError> {
        let visible_symbols = self.get_visible_symbols()?;
        let mut conflicts = Vec::new();

        // Group symbols by name
        let mut symbol_groups: HashMap<String, Vec<&ResolvedSymbol>> = HashMap::new();
        for symbol in &visible_symbols {
            symbol_groups.entry(symbol.symbol.name.clone()).or_default().push(symbol);
        }

        // Find conflicts (multiple symbols with same name from different sources)
        for (name, symbols) in symbol_groups {
            if symbols.len() > 1 {
                // Check if they're from different modules
                let mut sources = std::collections::HashSet::new();
                for symbol in &symbols {
                    sources.insert(&symbol.source_module);
                }

                if sources.len() > 1 {
                    let conflict_sources =
                        symbols.iter().map(|s| s.source_module.clone()).collect();

                    conflicts.push(SymbolConflict {
                        symbol_name: name,
                        conflicting_modules: conflict_sources,
                    });
                }
            }
        }

        Ok(conflicts)
    }

    /// Validate that all imported modules are properly loaded.
    pub fn validate_imports(&self) -> Result<(), MirrError> {
        let current = self.symbol_table.current_module().ok_or_else(|| MirrError::SymbolError {
            message: "[E916] No current module set for import validation.".to_string(),
            span: None,
        })?;

        for import in &current.imports {
            let import_path = PathBuf::from(&import.path);

            if self.symbol_table.get_module(&import_path).is_none() {
                return Err(MirrError::SymbolError {
                    message: format!(
                        "[E917] Imported module '{}' (alias '{}') is not loaded in symbol table.",
                        import.path, import.alias
                    ),
                    span: import.span,
                });
            }
        }

        Ok(())
    }

    /// Clear the resolution cache.
    pub fn clear_cache(&mut self) {
        self.qualified_cache.clear();
    }

    /// Get resolution statistics.
    pub fn get_stats(&self) -> ResolverStats {
        let module_count = self.symbol_table.modules.len();
        let mut total_symbols = 0;
        let mut total_imports = 0;

        for module in self.symbol_table.modules.values() {
            total_symbols += module.symbols.len();
            total_imports += module.imports.len();
        }

        ResolverStats {
            loaded_modules: module_count,
            total_symbols,
            total_imports,
            cached_qualified_symbols: self.qualified_cache.len(),
        }
    }

    /// Parse a qualified name into (alias, symbol) parts.
    fn parse_qualified_name(&self, name: &str) -> Option<(String, String)> {
        let parts: Vec<&str> = name.splitn(2, '.').collect();
        if parts.len() == 2 {
            Some((parts[0].to_string(), parts[1].to_string()))
        } else {
            None
        }
    }
}

/// Statistics about the resolver state.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ResolverStats {
    /// Number of modules loaded in the symbol table
    pub loaded_modules: usize,
    /// Total number of symbols across all modules
    pub total_symbols: usize,
    /// Total number of imports across all modules
    pub total_imports: usize,
    /// Number of cached qualified symbol resolutions
    pub cached_qualified_symbols: usize,
}

/// Convenience function to create a resolver from a MIRR program.
///
/// This is a high-level convenience function that handles the complete
/// symbol resolution setup for a MIRR program.
pub fn create_resolver_for_program(
    program: &MirrProgram,
    main_path: PathBuf,
    load_module: impl Fn(&PathBuf) -> Result<Module, MirrError>,
) -> Result<CrossModuleResolver, MirrError> {
    let wrapped_load = |path: &PathBuf| {
        let module = load_module(path)?;
        Ok((module, vec![]))
    };
    CrossModuleResolver::from_program(program, main_path, wrapped_load)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::program::{ImportDecl, SignalDecl};
    use crate::ast::types::{SignalKind, SignalType};

    fn create_test_module(name: &str, signals: Vec<(&str, SignalKind, SignalType)>) -> Module {
        let signal_decls = signals
            .into_iter()
            .map(|(name, kind, ty)| SignalDecl {
                name: name.to_string(),
                kind,
                ty: ExtendedType::from_core(ty),
                origin: None,
                span: None,
            })
            .collect();

        Module {
            name: name.to_string(),
            signals: signal_decls,
            guards: Vec::new(),
            reflexes: Vec::new(),
            properties: Vec::new(),
            pattern_calls: Vec::new(),
            pattern_origins: Vec::new(),
            span: None,
        }
    }

    #[test]
    fn test_resolver_local_symbol() {
        let module = create_test_module(
            "TestModule",
            vec![
                ("clk", SignalKind::Input, SignalType::Unsigned(1)),
                ("data", SignalKind::Output, SignalType::Unsigned(8)),
            ],
        );

        let main_path = PathBuf::from("test.mirr");
        let load_fn = |_: &PathBuf| -> Result<(Module, Vec<ImportDecl>), MirrError> {
            Ok((module.clone(), Vec::new()))
        };

        let program = MirrProgram {
            patterns: Vec::new(),
            imports: Vec::new(),
            struct_defs: Vec::new(),
            interface_defs: Vec::new(),
            module: module.clone(),
        };

        let mut resolver = CrossModuleResolver::from_program(&program, main_path, load_fn).unwrap();

        // Test local symbol resolution
        let resolved = resolver.resolve_symbol("clk", None).unwrap();
        assert!(resolved.is_local);
        assert_eq!(resolved.symbol.name, "clk");
        assert_eq!(resolved.symbol.kind, SignalKind::Input);
    }

    #[test]
    fn test_resolver_unknown_symbol() {
        let module = create_test_module(
            "TestModule",
            vec![("clk", SignalKind::Input, SignalType::Unsigned(1))],
        );

        let main_path = PathBuf::from("test.mirr");
        let load_fn = |_: &PathBuf| -> Result<(Module, Vec<ImportDecl>), MirrError> {
            Ok((module.clone(), Vec::new()))
        };

        let program = MirrProgram {
            patterns: Vec::new(),
            imports: Vec::new(),
            struct_defs: Vec::new(),
            interface_defs: Vec::new(),
            module: module.clone(),
        };

        let mut resolver = CrossModuleResolver::from_program(&program, main_path, load_fn).unwrap();

        // Test unknown symbol
        let result = resolver.resolve_symbol("unknown", None);
        assert!(result.is_err());
        if let Err(MirrError::SymbolError { message, .. }) = result {
            assert!(message.contains("E912"));
            assert!(message.contains("not found"));
        }
    }

    #[test]
    fn test_resolver_stats() {
        let module = create_test_module(
            "TestModule",
            vec![
                ("clk", SignalKind::Input, SignalType::Unsigned(1)),
                ("data", SignalKind::Output, SignalType::Unsigned(8)),
            ],
        );

        let main_path = PathBuf::from("test.mirr");
        let load_fn = |_: &PathBuf| -> Result<(Module, Vec<ImportDecl>), MirrError> {
            Ok((module.clone(), Vec::new()))
        };

        let program = MirrProgram {
            patterns: Vec::new(),
            imports: Vec::new(),
            struct_defs: Vec::new(),
            interface_defs: Vec::new(),
            module: module.clone(),
        };

        let resolver = CrossModuleResolver::from_program(&program, main_path, load_fn).unwrap();

        let stats = resolver.get_stats();
        assert_eq!(stats.loaded_modules, 1);
        assert_eq!(stats.total_symbols, 2);
        assert_eq!(stats.total_imports, 0);
        assert_eq!(stats.cached_qualified_symbols, 0);
    }

    #[test]
    fn test_parse_qualified_name() {
        let symbol_table = SymbolTable::new();
        let import_context = ImportContext::new();
        let resolver = CrossModuleResolver::new(symbol_table, import_context);

        // Test valid qualified name
        let result = resolver.parse_qualified_name("alias.symbol");
        assert_eq!(result, Some(("alias".to_string(), "symbol".to_string())));

        // Test unqualified name
        let result = resolver.parse_qualified_name("symbol");
        assert_eq!(result, None);

        // Test multiple dots (should only split on first dot)
        let result = resolver.parse_qualified_name("alias.symbol.field");
        assert_eq!(result, Some(("alias".to_string(), "symbol.field".to_string())));
    }

    #[test]
    fn test_symbol_exists() {
        let module = create_test_module(
            "TestModule",
            vec![("clk", SignalKind::Input, SignalType::Unsigned(1))],
        );

        let main_path = PathBuf::from("test.mirr");
        let load_fn = |_: &PathBuf| -> Result<(Module, Vec<ImportDecl>), MirrError> {
            Ok((module.clone(), Vec::new()))
        };

        let program = MirrProgram {
            patterns: Vec::new(),
            imports: Vec::new(),
            struct_defs: Vec::new(),
            interface_defs: Vec::new(),
            module: module.clone(),
        };

        let resolver = CrossModuleResolver::from_program(&program, main_path, load_fn).unwrap();

        assert!(resolver.symbol_exists("clk"));
        assert!(!resolver.symbol_exists("unknown"));
    }
}
