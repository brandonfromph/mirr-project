//! Import path resolution and file loading infrastructure.
//!
//! The resolver handles converting import paths to absolute file paths and
//! loading the content of imported files. It supports:
//! - Relative and absolute import paths
//! - Standard library imports (stdlib/)
//! - Project-local imports
//! - Proper error handling for file system operations

#![forbid(unsafe_code)]

use super::{ImportError, MAX_IMPORT_DEPTH, MAX_TOTAL_IMPORTS};
use crate::ast::program::{ImportDecl, MirrProgram};
use crate::error::MirrError;
use crate::parser::parse_mirr;
use std::collections::HashSet;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

/// A resolved import file containing its parsed content and metadata.
#[derive(Debug, Clone)]
pub struct ResolvedFile {
    /// Canonical absolute path to the file
    pub path: PathBuf,
    /// Raw source content of the file
    pub content: String,
    /// Parsed MIRR program
    pub program: MirrProgram,
    /// Import alias for this file
    pub alias: String,
    /// Import declarations from this file (transitive imports)
    pub imports: Vec<ImportDecl>,
}

impl ResolvedFile {
    /// Create a new resolved file.
    pub fn new(path: PathBuf, content: String, program: MirrProgram, alias: String) -> Self {
        let imports = program.imports.clone();
        Self { path, content, program, alias, imports }
    }

    /// Get the module name from this file.
    pub fn module_name(&self) -> &str {
        &self.program.module.name
    }
}

/// Error type for path resolution operations.
#[derive(Debug, Clone, PartialEq)]
pub enum ResolveError {
    /// File not found at the resolved path (E1301)
    FileNotFound(PathBuf),
    /// I/O error reading the file (E1308 for permission, E1301 for others)
    IoError(PathBuf, String),
    /// Parse error in the imported file (E1307)
    ParseError(PathBuf, MirrError),
    /// Invalid import path format (E1306)
    InvalidPath(String),
    /// Maximum import depth exceeded (E1303)
    DepthExceeded(usize),
    /// Maximum total imports exceeded (E1304)
    TotalLimitExceeded(usize),
}

impl From<ResolveError> for ImportError {
    fn from(error: ResolveError) -> Self {
        match error {
            ResolveError::FileNotFound(path) => ImportError::FileNotFound(path),
            ResolveError::IoError(path, msg) => {
                if msg.contains("permission denied") {
                    ImportError::PermissionDenied(path)
                } else {
                    ImportError::FileNotFound(path)
                }
            }
            ResolveError::ParseError(path, error) => ImportError::ParseError(path, error),
            ResolveError::InvalidPath(path) => ImportError::InvalidPath(path),
            ResolveError::DepthExceeded(depth) => ImportError::DepthLimitExceeded(depth),
            ResolveError::TotalLimitExceeded(total) => ImportError::TotalLimitExceeded(total),
        }
    }
}

impl std::fmt::Display for ResolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolveError::FileNotFound(path) => {
                write!(f, "File not found: {}", path.display())
            }
            ResolveError::IoError(path, msg) => {
                write!(f, "I/O error reading {}: {msg}", path.display())
            }
            ResolveError::ParseError(path, error) => {
                write!(f, "Parse error in {}: {error}", path.display())
            }
            ResolveError::InvalidPath(path) => {
                write!(f, "Invalid import path: {path}")
            }
            ResolveError::DepthExceeded(depth) => {
                write!(f, "Import depth limit exceeded: {depth} > {MAX_IMPORT_DEPTH}")
            }
            ResolveError::TotalLimitExceeded(total) => {
                write!(f, "Total import limit exceeded: {total} > {MAX_TOTAL_IMPORTS}")
            }
        }
    }
}

impl std::error::Error for ResolveError {}

/// Import path resolver that handles file system operations.
#[derive(Debug, Clone)]
pub struct ImportResolver {
    /// Base directory for resolving relative imports
    base_path: PathBuf,
    /// Standard library search paths (e.g., stdlib/, lib/)
    stdlib_paths: Vec<PathBuf>,
    /// Cache of resolved paths to avoid duplicate resolution
    path_cache: HashSet<PathBuf>,
    /// Current import depth for recursion protection
    current_depth: usize,
    /// Total number of files imported
    total_imports: usize,
}

impl ImportResolver {
    /// Create a new import resolver with the given base path.
    pub fn new(base_path: &Path) -> Self {
        let mut stdlib_paths = Vec::new();

        // Standard library search paths
        stdlib_paths.push(base_path.join("stdlib"));
        stdlib_paths.push(base_path.join("lib"));

        // System-wide MIRR installation (if exists)
        if let Ok(mirr_home) = std::env::var("MIRR_HOME") {
            stdlib_paths.push(PathBuf::from(mirr_home).join("stdlib"));
        }

        Self {
            base_path: base_path.to_path_buf(),
            stdlib_paths,
            path_cache: HashSet::new(),
            current_depth: 0,
            total_imports: 0,
        }
    }

    /// Resolve an import declaration to an absolute file path.
    pub fn resolve_path(&self, import: &ImportDecl) -> Result<PathBuf, ResolveError> {
        let import_path = &import.path;

        // Validate path format
        if import_path.is_empty() {
            return Err(ResolveError::InvalidPath(import_path.clone()));
        }

        if import_path.contains("..") {
            return Err(ResolveError::InvalidPath(format!(
                "Path traversal not allowed: {import_path}"
            )));
        }

        // Try different resolution strategies
        let candidates = self.generate_path_candidates(import_path);

        for candidate in candidates {
            if candidate.exists() && candidate.is_file() {
                return Ok(candidate);
            }
        }

        Err(ResolveError::FileNotFound(PathBuf::from(import_path)))
    }

    /// Generate candidate paths for an import in order of precedence.
    fn generate_path_candidates(&self, import_path: &str) -> Vec<PathBuf> {
        let mut candidates = Vec::new();
        let path = Path::new(import_path);

        // 1. Absolute path (if provided)
        if path.is_absolute() {
            candidates.push(path.to_path_buf());
            return candidates;
        }

        // 2. Relative to base path (current file's directory)
        candidates.push(self.base_path.join(path));

        // 3. Add .mirr extension if not present
        if !import_path.ends_with(".mirr") {
            let with_ext = format!("{import_path}.mirr");
            candidates.push(self.base_path.join(&with_ext));

            // Also try in stdlib paths
            for stdlib_path in &self.stdlib_paths {
                candidates.push(stdlib_path.join(&with_ext));
            }
        }

        // 4. Standard library paths
        for stdlib_path in &self.stdlib_paths {
            candidates.push(stdlib_path.join(path));
        }

        candidates
    }

    /// Load and parse a file at the given path.
    pub fn load_file(&mut self, path: &Path, alias: String) -> Result<ResolvedFile, ResolveError> {
        // Check import limits
        if self.current_depth >= MAX_IMPORT_DEPTH {
            return Err(ResolveError::DepthExceeded(self.current_depth));
        }

        if self.total_imports >= MAX_TOTAL_IMPORTS {
            return Err(ResolveError::TotalLimitExceeded(self.total_imports));
        }

        // Canonicalize path to handle symlinks and relative components
        let canonical_path = path
            .canonicalize()
            .map_err(|e| ResolveError::IoError(path.to_path_buf(), e.to_string()))?;

        // Check if already loaded
        if self.path_cache.contains(&canonical_path) {
            return Err(ResolveError::InvalidPath(format!(
                "File already imported: {}",
                canonical_path.display()
            )));
        }

        // Read file content
        let content = std::fs::read_to_string(&canonical_path).map_err(|e| match e.kind() {
            ErrorKind::NotFound => ResolveError::FileNotFound(canonical_path.clone()),
            ErrorKind::PermissionDenied => {
                ResolveError::IoError(canonical_path.clone(), "permission denied".to_string())
            }
            _ => ResolveError::IoError(canonical_path.clone(), e.to_string()),
        })?;

        // Parse the content
        let program = parse_mirr(&content)
            .map_err(|e| ResolveError::ParseError(canonical_path.clone(), e))?;

        // Update tracking
        self.path_cache.insert(canonical_path.clone());
        self.total_imports += 1;

        Ok(ResolvedFile::new(canonical_path, content, program, alias))
    }

    /// Resolve an import declaration to a loaded file.
    pub fn resolve_import(&mut self, import: &ImportDecl) -> Result<ResolvedFile, ResolveError> {
        // First resolve the path
        let path = self.resolve_path(import)?;

        // Then load the file
        self.current_depth += 1;
        let result = self.load_file(&path, import.alias.clone());
        self.current_depth -= 1;

        result
    }

    /// Get the current import depth.
    pub fn current_depth(&self) -> usize {
        self.current_depth
    }

    /// Get the total number of imports processed.
    pub fn total_imports(&self) -> usize {
        self.total_imports
    }

    /// Reset the resolver state (for testing).
    #[cfg(test)]
    pub fn reset(&mut self) {
        self.path_cache.clear();
        self.current_depth = 0;
        self.total_imports = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_file(dir: &Path, name: &str, content: &str) -> PathBuf {
        let path = dir.join(name);
        fs::write(&path, content).unwrap();
        path
    }

    #[test]
    fn test_resolve_path_relative() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path();

        // Create a test file
        create_test_file(base_path, "test.mirr", "module TestModule { }");

        let resolver = ImportResolver::new(base_path);
        let import =
            ImportDecl { path: "test.mirr".to_string(), alias: "test".to_string(), span: None };

        let resolved = resolver.resolve_path(&import).unwrap();
        assert_eq!(resolved, base_path.join("test.mirr"));
    }

    #[test]
    fn test_resolve_path_with_extension() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path();

        // Create a test file
        create_test_file(base_path, "test.mirr", "module TestModule { }");

        let resolver = ImportResolver::new(base_path);
        let import = ImportDecl {
            path: "test".to_string(), // No extension
            alias: "test".to_string(),
            span: None,
        };

        let resolved = resolver.resolve_path(&import).unwrap();
        assert_eq!(resolved, base_path.join("test.mirr"));
    }

    #[test]
    fn test_resolve_path_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path();

        let resolver = ImportResolver::new(base_path);
        let import = ImportDecl {
            path: "nonexistent.mirr".to_string(),
            alias: "test".to_string(),
            span: None,
        };

        let result = resolver.resolve_path(&import);
        assert!(matches!(result, Err(ResolveError::FileNotFound(_))));
    }

    #[test]
    fn test_invalid_path_traversal() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path();

        let resolver = ImportResolver::new(base_path);
        let import = ImportDecl {
            path: "../../../etc/passwd".to_string(),
            alias: "test".to_string(),
            span: None,
        };

        let result = resolver.resolve_path(&import);
        assert!(matches!(result, Err(ResolveError::InvalidPath(_))));
    }

    #[test]
    fn test_load_file() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path();

        let content = "module TestModule { signal x: in u8; }";
        let path = create_test_file(base_path, "test.mirr", content);

        let mut resolver = ImportResolver::new(base_path);
        let resolved = resolver.load_file(&path, "test".to_string()).unwrap();

        assert_eq!(resolved.alias, "test");
        assert_eq!(resolved.content, content);
        assert_eq!(resolved.module_name(), "TestModule");
    }

    #[test]
    fn test_import_depth_limit() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path();

        let path = create_test_file(base_path, "test.mirr", "module TestModule { }");

        let mut resolver = ImportResolver::new(base_path);
        // Manually set depth to limit
        resolver.current_depth = MAX_IMPORT_DEPTH;

        let result = resolver.load_file(&path, "test".to_string());
        assert!(matches!(result, Err(ResolveError::DepthExceeded(_))));
    }
}
