//! Multi-file compilation coordinator with dependency graph management.
//!
//! The loader orchestrates the complete import process:
//! 1. Recursively resolves all import dependencies
//! 2. Constructs a dependency graph between files
//! 3. Detects circular dependencies
//! 4. Provides a topological ordering for compilation
//! 5. Manages alias conflicts and symbol resolution
//!
//! WARNING: Core import mapper - keep semantics stable; avoid partial state updates.
//! DO NOT edit unless encompassed by an integration regression test.

#![forbid(unsafe_code)]

use super::resolver::ImportResolver;
use super::{ImportContext, ImportError};
use crate::ast::program::ImportDecl;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

/// Result of the import loading process.
#[derive(Debug, Clone)]
pub struct LoadResult {
    /// Complete import context with all resolved files
    pub context: ImportContext,
    /// Topological ordering of files for compilation
    pub compilation_order: Vec<PathBuf>,
    /// Statistics about the import process
    pub stats: LoadStats,
}

/// Statistics about the import loading process.
#[derive(Debug, Clone, Default)]
pub struct LoadStats {
    /// Total number of files loaded
    pub files_loaded: usize,
    /// Maximum import depth reached
    pub max_depth: usize,
    /// Number of alias conflicts detected and resolved
    pub alias_conflicts: usize,
    /// Number of circular dependency checks performed
    pub cycle_checks: usize,
}

/// Dependency graph for tracking import relationships.
#[derive(Debug, Clone)]
pub struct DependencyGraph {
    /// Adjacency list: file -> set of files it imports
    dependencies: HashMap<PathBuf, HashSet<PathBuf>>,
    /// Reverse dependencies: file -> set of files that import it
    dependents: HashMap<PathBuf, HashSet<PathBuf>>,
}

impl DependencyGraph {
    /// Create a new empty dependency graph.
    pub fn new() -> Self {
        Self { dependencies: HashMap::new(), dependents: HashMap::new() }
    }

    /// Add a dependency edge: `from` imports `to`.
    pub fn add_dependency(&mut self, from: PathBuf, to: PathBuf) {
        self.dependencies.entry(from.clone()).or_default().insert(to.clone());

        self.dependents.entry(to).or_default().insert(from);
    }

    /// Get all files that the given file directly imports.
    pub fn get_dependencies(&self, file: &PathBuf) -> Option<&HashSet<PathBuf>> {
        self.dependencies.get(file)
    }

    /// Get all files that directly import the given file.
    pub fn get_dependents(&self, file: &PathBuf) -> Option<&HashSet<PathBuf>> {
        self.dependents.get(file)
    }

    /// Check for circular dependencies starting from the given file.
    pub fn find_cycle(&self, start: &PathBuf) -> Option<Vec<PathBuf>> {
        let mut visited = HashSet::new();
        let mut path = Vec::new();

        if self.dfs_cycle_check(start, &mut visited, &mut path) {
            Some(path)
        } else {
            None
        }
    }

    /// Depth-first search for cycle detection.
    fn dfs_cycle_check(
        &self,
        current: &PathBuf,
        visited: &mut HashSet<PathBuf>,
        path: &mut Vec<PathBuf>,
    ) -> bool {
        if path.contains(current) {
            // Found a cycle - reconstruct the cycle path
            if let Some(cycle_start) = path.iter().position(|p| p == current) {
                path.drain(0..cycle_start);
                path.push(current.clone());
                return true;
            }
            return false; // Should be unreachable if path contains current
        }

        if visited.contains(current) {
            return false;
        }

        visited.insert(current.clone());
        path.push(current.clone());

        if let Some(deps) = self.dependencies.get(current) {
            for dep in deps {
                if self.dfs_cycle_check(dep, visited, path) {
                    return true;
                }
            }
        }

        path.pop();
        false
    }

    /// Get a topological ordering of all files in the graph.
    pub fn topological_sort(&self) -> Result<Vec<PathBuf>, CircularDependencyError> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();
        let mut temp_visited = HashSet::new();

        // Get all nodes in the graph
        let mut all_nodes: HashSet<PathBuf> = HashSet::new();
        for (from, deps) in &self.dependencies {
            all_nodes.insert(from.clone());
            for dep in deps {
                all_nodes.insert(dep.clone());
            }
        }

        for node in &all_nodes {
            if !visited.contains(node) {
                self.dfs_topological_sort(node, &mut visited, &mut temp_visited, &mut result)?
            }
        }

        // result is already in correct compilation order (dependencies first)
        Ok(result)
    }

    /// Depth-first search for topological sorting.
    fn dfs_topological_sort(
        &self,
        node: &PathBuf,
        visited: &mut HashSet<PathBuf>,
        temp_visited: &mut HashSet<PathBuf>,
        result: &mut Vec<PathBuf>,
    ) -> Result<(), CircularDependencyError> {
        if temp_visited.contains(node) {
            // Found a cycle
            let cycle = self.find_cycle(node).unwrap_or_else(|| vec![node.clone()]);
            return Err(CircularDependencyError::new(cycle));
        }

        if visited.contains(node) {
            return Ok(());
        }

        temp_visited.insert(node.clone());

        if let Some(deps) = self.dependencies.get(node) {
            for dep in deps {
                self.dfs_topological_sort(dep, visited, temp_visited, result)?;
            }
        }

        temp_visited.remove(node);
        visited.insert(node.clone());
        result.push(node.clone());

        Ok(())
    }

    /// Check if the graph has any cycles.
    pub fn has_cycles(&self) -> bool {
        self.topological_sort().is_err()
    }

    /// Get all files in the graph.
    pub fn all_files(&self) -> HashSet<PathBuf> {
        let mut files = HashSet::new();
        for (from, deps) in &self.dependencies {
            files.insert(from.clone());
            for dep in deps {
                files.insert(dep.clone());
            }
        }
        files
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Error indicating a circular dependency was found.
#[derive(Debug, Clone, PartialEq)]
pub struct CircularDependencyError {
    /// The cycle path showing the circular dependency
    pub cycle: Vec<PathBuf>,
}

impl CircularDependencyError {
    /// Create a new circular dependency error.
    pub fn new(cycle: Vec<PathBuf>) -> Self {
        Self { cycle }
    }
}

impl std::fmt::Display for CircularDependencyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circular dependency detected: ")?;
        for (i, path) in self.cycle.iter().enumerate() {
            if i > 0 {
                write!(f, " → ")?;
            }
            write!(f, "{}", path.display())?;
        }
        Ok(())
    }
}

impl std::error::Error for CircularDependencyError {}

/// Multi-file import loader that coordinates the entire import process.
#[derive(Debug)]
pub struct ImportLoader {
    /// File resolver for path resolution and loading
    resolver: ImportResolver,
    /// Dependency graph tracking relationships
    dependencies: DependencyGraph,
    /// Set of files currently being processed (for cycle detection)
    processing: HashSet<PathBuf>,
    /// Statistics about the loading process
    stats: LoadStats,
}

impl ImportLoader {
    /// Create a new import loader with the given resolver.
    pub fn new(resolver: ImportResolver) -> Self {
        Self {
            resolver,
            dependencies: DependencyGraph::new(),
            processing: HashSet::new(),
            stats: LoadStats::default(),
        }
    }

    /// Load all imports starting from a list of import declarations.
    pub fn load_imports(&mut self, imports: &[ImportDecl]) -> Result<ImportContext, ImportError> {
        let mut context = ImportContext::new();

        // Process each top-level import
        for import in imports {
            self.load_import_recursive(import, &mut context)?;
        }

        // Check for circular dependencies
        self.stats.cycle_checks += 1;
        if let Some(cycle) = self.find_any_cycle(&context) {
            return Err(ImportError::CircularDependency(cycle));
        }

        // Get compilation order
        let _compilation_order = self
            .dependencies
            .topological_sort()
            .map_err(|e| ImportError::CircularDependency(e.cycle))?;

        self.stats.files_loaded = context.files.len();

        Ok(context)
    }

    /// Load a single import and all its transitive dependencies.
    fn load_import_recursive(
        &mut self,
        import: &ImportDecl,
        context: &mut ImportContext,
    ) -> Result<(), ImportError> {
        // Check for alias conflicts
        if context.has_alias(&import.alias) {
            self.stats.alias_conflicts += 1;
            return Err(ImportError::AliasConflict(import.alias.clone()));
        }

        // Resolve the path first to check for cycles/duplicates
        let file_path = self.resolver.resolve_path(import).map_err(ImportError::from)?;

        // Check if we're already processing this file (cycle detection)
        if self.processing.contains(&file_path) {
            let cycle = vec![file_path];
            return Err(ImportError::CircularDependency(cycle));
        }

        // Check if already loaded
        if context.files.contains_key(&file_path) {
            return Ok(());
        }

        // Now load and parse the file
        let resolved_file =
            self.resolver.load_file(&file_path, import.alias.clone()).map_err(ImportError::from)?;

        // Mark as processing
        self.processing.insert(file_path.clone());

        // Update depth tracking
        self.stats.max_depth = self.stats.max_depth.max(self.resolver.current_depth());

        // Process transitive imports
        let transitive_imports = resolved_file.imports.clone();
        for transitive_import in &transitive_imports {
            // Add dependency edge
            let dep_path =
                self.resolver.resolve_path(transitive_import).map_err(ImportError::from)?;
            self.dependencies.add_dependency(file_path.clone(), dep_path.clone());

            // Recursively load the dependency
            self.load_import_recursive(transitive_import, context)?;
        }

        // Add the resolved file to the context
        context.add_file(file_path.clone(), resolved_file, import.alias.clone())?;

        // Mark as finished processing
        self.processing.remove(&file_path);

        Ok(())
    }

    /// Find any cycle in the loaded files.
    fn find_any_cycle(&self, context: &ImportContext) -> Option<Vec<PathBuf>> {
        for file_path in context.files.keys() {
            if let Some(cycle) = self.dependencies.find_cycle(file_path) {
                return Some(cycle);
            }
        }
        None
    }

    /// Get the current load statistics.
    pub fn stats(&self) -> &LoadStats {
        &self.stats
    }

    /// Get a complete load result.
    pub fn result(self, context: ImportContext) -> Result<LoadResult, ImportError> {
        let compilation_order = self
            .dependencies
            .topological_sort()
            .map_err(|e| ImportError::CircularDependency(e.cycle))?;

        Ok(LoadResult { context, compilation_order, stats: self.stats })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_file(dir: &std::path::Path, name: &str, content: &str) -> PathBuf {
        let path = dir.join(name);
        fs::write(&path, content).unwrap();
        path
    }

    #[test]
    fn test_dependency_graph_simple() {
        let mut graph = DependencyGraph::new();
        let a = PathBuf::from("a.mirr");
        let b = PathBuf::from("b.mirr");

        graph.add_dependency(a.clone(), b.clone());

        assert!(graph.get_dependencies(&a).unwrap().contains(&b));
        assert!(graph.get_dependents(&b).unwrap().contains(&a));
    }

    #[test]
    fn test_dependency_graph_cycle_detection() {
        let mut graph = DependencyGraph::new();
        let a = PathBuf::from("a.mirr");
        let b = PathBuf::from("b.mirr");
        let c = PathBuf::from("c.mirr");

        // Create cycle: a -> b -> c -> a
        graph.add_dependency(a.clone(), b.clone());
        graph.add_dependency(b.clone(), c.clone());
        graph.add_dependency(c.clone(), a.clone());

        let cycle = graph.find_cycle(&a).unwrap();
        assert!(cycle.contains(&a));
        assert!(cycle.contains(&b));
        assert!(cycle.contains(&c));
    }

    #[test]
    fn test_dependency_graph_topological_sort() {
        let mut graph = DependencyGraph::new();
        let a = PathBuf::from("a.mirr");
        let b = PathBuf::from("b.mirr");
        let c = PathBuf::from("c.mirr");

        // Create DAG: a -> b, b -> c
        graph.add_dependency(a.clone(), b.clone());
        graph.add_dependency(b.clone(), c.clone());

        let order = graph.topological_sort().unwrap();
        let a_pos = order.iter().position(|p| p == &a).unwrap();
        let b_pos = order.iter().position(|p| p == &b).unwrap();
        let c_pos = order.iter().position(|p| p == &c).unwrap();

        // c should come before b, which should come before a
        assert!(c_pos < b_pos);
        assert!(b_pos < a_pos);
    }

    #[test]
    fn test_dependency_graph_cycle_in_topological_sort() {
        let mut graph = DependencyGraph::new();
        let a = PathBuf::from("a.mirr");
        let b = PathBuf::from("b.mirr");

        // Create cycle: a -> b -> a
        graph.add_dependency(a.clone(), b.clone());
        graph.add_dependency(b.clone(), a.clone());

        let result = graph.topological_sort();
        assert!(result.is_err());
    }

    #[test]
    fn test_import_loader_simple() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path();

        // Create test files
        create_test_file(base_path, "a.mirr", "module A { }");
        create_test_file(base_path, "b.mirr", r#"import "a.mirr" as a; module B { }"#);

        let resolver = ImportResolver::new(base_path);
        let mut loader = ImportLoader::new(resolver);

        let imports =
            vec![ImportDecl { path: "b.mirr".to_string(), alias: "b".to_string(), span: None }];

        let context = loader.load_imports(&imports).unwrap();

        // Should have loaded both files
        assert_eq!(context.files.len(), 2);
        assert!(context.has_alias("b"));

        let stats = loader.stats();
        assert_eq!(stats.files_loaded, 2);
        assert_eq!(stats.alias_conflicts, 0);
    }

    #[test]
    fn test_import_loader_alias_conflict() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path();

        create_test_file(base_path, "a.mirr", "module A { }");
        create_test_file(base_path, "b.mirr", "module B { }");

        let resolver = ImportResolver::new(base_path);
        let mut loader = ImportLoader::new(resolver);

        let imports = vec![
            ImportDecl { path: "a.mirr".to_string(), alias: "test".to_string(), span: None },
            ImportDecl {
                path: "b.mirr".to_string(),
                alias: "test".to_string(), // Duplicate alias
                span: None,
            },
        ];

        let result = loader.load_imports(&imports);
        assert!(matches!(result, Err(ImportError::AliasConflict(_))));
    }

    #[test]
    fn test_import_loader_circular_dependency() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path();

        // Create circular dependency: a -> b -> a
        create_test_file(base_path, "a.mirr", r#"import "b.mirr" as b; module A { }"#);
        create_test_file(base_path, "b.mirr", r#"import "a.mirr" as a; module B { }"#);

        let resolver = ImportResolver::new(base_path);
        let mut loader = ImportLoader::new(resolver);

        let imports =
            vec![ImportDecl { path: "a.mirr".to_string(), alias: "a".to_string(), span: None }];

        let result = loader.load_imports(&imports);
        assert!(matches!(result, Err(ImportError::CircularDependency(_))));
    }
}
