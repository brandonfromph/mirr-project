//! Multi-file import resolution and compilation infrastructure.
//!
//! This module provides the complete import pipeline for MIRR programs:
//! 1. Path resolution - converting import paths to absolute file paths
//! 2. File loading - reading and parsing imported MIRR files
//! 3. Dependency graph construction - tracking import relationships
//! 4. Cycle detection - preventing circular imports
//! 5. Symbol resolution - making imported symbols available to modules
//!
//! The import system supports hierarchical imports with aliases and prevents
//! circular dependencies using dependency graph analysis.

#![forbid(unsafe_code)]

pub mod loader;
pub mod resolver;

pub use loader::{CircularDependencyError, DependencyGraph, ImportLoader, LoadResult};
pub use resolver::{ImportResolver, ResolveError, ResolvedFile};

use crate::ast::program::{InterfaceDef, MirrProgram, StructDef};
use crate::error::MirrError;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Maximum import depth to prevent infinite recursion
pub const MAX_IMPORT_DEPTH: usize = 32;

/// Maximum number of files that can be imported in a single compilation
pub const MAX_TOTAL_IMPORTS: usize = 256;

/// Import context containing all resolved files and their relationships.
#[derive(Debug, Clone)]
pub struct ImportContext {
    /// All resolved files indexed by their canonical path
    pub files: HashMap<PathBuf, ResolvedFile>,
    /// Dependency relationships between files
    pub dependencies: DependencyGraph,
    /// Import aliases mapped to their target file paths
    pub aliases: HashMap<String, PathBuf>,
    /// Symbol table mapping symbol names to their defining files
    pub symbols: HashMap<String, Vec<PathBuf>>,
    /// Global struct definitions collected from all imported files
    pub struct_defs: Vec<StructDef>,
    /// Global interface definitions collected from all imported files
    pub interface_defs: Vec<InterfaceDef>,
}

impl ImportContext {
    /// Create a new empty import context.
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            dependencies: DependencyGraph::new(),
            aliases: HashMap::new(),
            symbols: HashMap::new(),
            struct_defs: Vec::new(),
            interface_defs: Vec::new(),
        }
    }

    /// Get the resolved file for an import alias.
    pub fn resolve_alias(&self, alias: &str) -> Option<&ResolvedFile> {
        self.aliases.get(alias).and_then(|path| self.files.get(path))
    }

    /// Check if an alias is already used.
    pub fn has_alias(&self, alias: &str) -> bool {
        self.aliases.contains_key(alias)
    }

    /// Add a resolved file to the context.
    pub fn add_file(
        &mut self,
        path: PathBuf,
        file: ResolvedFile,
        alias: String,
    ) -> Result<(), ImportError> {
        // Check for symbol conflicts before adding the file
        self.check_symbol_conflicts(&file, &path)?;

        // Add symbols from this file
        self.register_symbols(&file, &path);

        self.aliases.insert(alias, path.clone());
        self.files.insert(path, file);
        Ok(())
    }

    /// Check for symbol conflicts when adding a new file.
    fn check_symbol_conflicts(
        &self,
        file: &ResolvedFile,
        file_path: &std::path::Path,
    ) -> Result<(), ImportError> {
        // Extract symbols from the file's module
        let module = &file.program.module;

        // Check module name conflict
        if let Some(existing_files) = self.symbols.get(&module.name) {
            if let Some(existing_file) = existing_files.first() {
                return Err(ImportError::SymbolConflict(
                    module.name.clone(),
                    existing_file.clone(),
                    file_path.to_path_buf(),
                ));
            }
        }

        // Check signal name conflicts
        for signal in &module.signals {
            if let Some(existing_files) = self.symbols.get(&signal.name) {
                if let Some(existing_file) = existing_files.first() {
                    return Err(ImportError::SymbolConflict(
                        signal.name.clone(),
                        existing_file.clone(),
                        file_path.to_path_buf(),
                    ));
                }
            }
        }

        // Check guard name conflicts
        for guard in &module.guards {
            if let Some(existing_files) = self.symbols.get(&guard.name) {
                if let Some(existing_file) = existing_files.first() {
                    return Err(ImportError::SymbolConflict(
                        guard.name.clone(),
                        existing_file.clone(),
                        file_path.to_path_buf(),
                    ));
                }
            }
        }

        // Check reflex name conflicts
        for reflex in &module.reflexes {
            if let Some(existing_files) = self.symbols.get(&reflex.name) {
                if let Some(existing_file) = existing_files.first() {
                    return Err(ImportError::SymbolConflict(
                        reflex.name.clone(),
                        existing_file.clone(),
                        file_path.to_path_buf(),
                    ));
                }
            }
        }

        // Check property name conflicts
        for property in &module.properties {
            if let Some(existing_files) = self.symbols.get(&property.name) {
                if let Some(existing_file) = existing_files.first() {
                    return Err(ImportError::SymbolConflict(
                        property.name.clone(),
                        existing_file.clone(),
                        file_path.to_path_buf(),
                    ));
                }
            }
        }

        Ok(())
    }

    /// Register symbols from a file in the symbol table.
    fn register_symbols(&mut self, file: &ResolvedFile, file_path: &std::path::Path) {
        let module = &file.program.module;

        // Collect struct and interface definitions
        self.struct_defs.extend(file.program.struct_defs.clone());
        self.interface_defs.extend(file.program.interface_defs.clone());

        // Register module name
        self.symbols.entry(module.name.clone()).or_default().push(file_path.to_path_buf());

        // Register signal names
        for signal in &module.signals {
            self.symbols.entry(signal.name.clone()).or_default().push(file_path.to_path_buf());
        }

        // Register guard names
        for guard in &module.guards {
            self.symbols.entry(guard.name.clone()).or_default().push(file_path.to_path_buf());
        }

        // Register reflex names
        for reflex in &module.reflexes {
            self.symbols.entry(reflex.name.clone()).or_default().push(file_path.to_path_buf());
        }

        // Register property names
        for property in &module.properties {
            self.symbols.entry(property.name.clone()).or_default().push(file_path.to_path_buf());
        }
    }

    /// Check if a symbol is defined across multiple imports.
    pub fn has_symbol_conflict(&self, symbol: &str) -> bool {
        self.symbols.get(symbol).is_some_and(|files| files.len() > 1)
    }

    /// Get all files that define a particular symbol.
    pub fn get_symbol_sources(&self, symbol: &str) -> Option<&Vec<PathBuf>> {
        self.symbols.get(symbol)
    }
}

impl Default for ImportContext {
    fn default() -> Self {
        Self::new()
    }
}

/// High-level function to resolve all imports in a MIRR program.
///
/// This is the main entry point for import resolution. It processes all
/// import declarations in the given program and returns a complete
/// import context with all resolved files and dependencies.
pub fn resolve_imports(
    program: &MirrProgram,
    base_path: &Path,
) -> Result<ImportContext, MirrError> {
    let resolver = ImportResolver::new(base_path);
    let mut loader = ImportLoader::new(resolver);

    loader
        .load_imports(&program.imports)
        .map_err(|e| MirrError::ImportError { message: e.to_string(), span: None })
}

/// Error codes for import resolution (E13xx series).
#[derive(Debug, Clone, PartialEq)]
pub enum ImportError {
    /// E1301: File not found
    FileNotFound(PathBuf),
    /// E1302: Circular dependency detected
    CircularDependency(Vec<PathBuf>),
    /// E1303: Import depth limit exceeded
    DepthLimitExceeded(usize),
    /// E1304: Total import limit exceeded
    TotalLimitExceeded(usize),
    /// E1305: Alias already in use
    AliasConflict(String),
    /// E1306: Invalid import path
    InvalidPath(String),
    /// E1307: Parse error in imported file
    ParseError(PathBuf, MirrError),
    /// E1308: Permission denied reading file
    PermissionDenied(PathBuf),
    /// E1309: Symbol conflict between imports
    SymbolConflict(String, PathBuf, PathBuf),
}

impl ImportError {
    /// Get the error code for this import error.
    pub fn code(&self) -> &'static str {
        match self {
            ImportError::FileNotFound(_) => "E1301",
            ImportError::CircularDependency(_) => "E1302",
            ImportError::DepthLimitExceeded(_) => "E1303",
            ImportError::TotalLimitExceeded(_) => "E1304",
            ImportError::AliasConflict(_) => "E1305",
            ImportError::InvalidPath(_) => "E1306",
            ImportError::ParseError(_, _) => "E1307",
            ImportError::PermissionDenied(_) => "E1308",
            ImportError::SymbolConflict(_, _, _) => "E1309",
        }
    }
}

impl std::fmt::Display for ImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportError::FileNotFound(path) => {
                write!(f, "[{}] Import file not found: {}", self.code(), path.display())?;
                write!(f, "\n  help: Check that the file exists and the path is correct")?;
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    if !stem.ends_with(".mirr") {
                        write!(f, "\n  help: Try adding the '.mirr' extension")?;
                    }
                }
                Ok(())
            }
            ImportError::CircularDependency(cycle) => {
                write!(f, "[{}] Circular dependency detected: ", self.code())?;
                for (i, path) in cycle.iter().enumerate() {
                    if i > 0 {
                        write!(f, " → ")?;
                    }
                    write!(f, "{}", path.display())?;
                }
                write!(f, "\n  help: Restructure imports to break the circular dependency")?;
                write!(f, "\n  help: Consider creating a shared module for common definitions")?;
                Ok(())
            }
            ImportError::DepthLimitExceeded(depth) => {
                write!(
                    f,
                    "[{}] Import depth limit exceeded: {depth} > {MAX_IMPORT_DEPTH}",
                    self.code()
                )?;
                write!(f, "\n  help: Reduce import nesting or check for circular dependencies")?;
                Ok(())
            }
            ImportError::TotalLimitExceeded(total) => {
                write!(
                    f,
                    "[{}] Total import limit exceeded: {total} > {MAX_TOTAL_IMPORTS}",
                    self.code()
                )?;
                write!(f, "\n  help: Consider consolidating modules or splitting the project")?;
                Ok(())
            }
            ImportError::AliasConflict(alias) => {
                write!(f, "[{}] Import alias '{alias}' is already in use", self.code())?;
                write!(f, "\n  help: Use a different alias name for this import")?;
                write!(f, "\n  help: Example: import \"file.mirr\" as {alias}_v2;")?;
                Ok(())
            }
            ImportError::InvalidPath(path) => {
                write!(f, "[{}] Invalid import path: {path}", self.code())?;
                if path.contains("..") {
                    write!(f, "\n  help: Path traversal (..) is not allowed for security")?;
                    write!(f, "\n  help: Use absolute paths or relative paths within the project")?;
                } else if path.is_empty() {
                    write!(f, "\n  help: Import path cannot be empty")?;
                } else {
                    write!(
                        f,
                        "\n  help: Check path format and ensure it follows MIRR conventions"
                    )?;
                }
                Ok(())
            }
            ImportError::ParseError(path, error) => {
                write!(f, "[{}] Parse error in {}: {error}", self.code(), path.display())?;
                write!(f, "\n  help: Fix the syntax errors in the imported file first")?;
                Ok(())
            }
            ImportError::PermissionDenied(path) => {
                write!(f, "[{}] Permission denied reading: {}", self.code(), path.display())?;
                write!(f, "\n  help: Check file permissions or run with appropriate privileges")?;
                Ok(())
            }
            ImportError::SymbolConflict(symbol, file1, file2) => {
                write!(f, "[{}] Symbol '{symbol}' conflicts between imports", self.code())?;
                write!(f, "\n  note: '{}' defines '{symbol}'", file1.display())?;
                write!(f, "\n  note: '{}' also defines '{symbol}'", file2.display())?;
                write!(
                    f,
                    "\n  help: Use different aliases to disambiguate: import \"{}\" as alias1;",
                    file1.display()
                )?;
                Ok(())
            }
        }
    }
}

impl std::error::Error for ImportError {}
