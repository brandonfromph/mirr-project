#![forbid(unsafe_code)]

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use sha2::{Digest, Sha256};

use crate::ast::MirrProgram;
use crate::error::{MirrError, PipelineErrors};
use crate::parser::parse_mirr;
use crate::pipeline::{run_pipeline_on_program, PipelineConfig, PipelineResult};

/// Workspace manages multi-file MIRR projects, import resolution, and caching.
#[derive(Debug, Clone)]
pub struct Workspace {
    pub config: WorkspaceConfig,
    /// Cached source files by their canonical path.
    pub files: HashMap<PathBuf, String>,
    /// Incremental compilation snapshots.
    pub snapshots: HashMap<PathBuf, WorkspaceSnapshot>,
}

/// A frozen view of a compiled project at a point in time.
#[derive(Debug, Clone)]
pub struct WorkspaceSnapshot {
    pub root_path: PathBuf,
    pub pipeline: Rc<PipelineResult>,
    pub workspace_hash: String,
    pub metadata: WorkspaceArtifactSummary,
}

#[derive(Debug, Clone, Default)]
pub struct WorkspaceArtifactSummary {
    pub loaded_files: usize,
    pub dependency_nodes: usize,
    pub source_hash: String,
    pub workspace_hash: String,
}

#[derive(Debug, Clone)]
pub struct WorkspaceConfig {
    pub root_dir: PathBuf,
}

impl WorkspaceConfig {
    pub fn new(root_dir: impl Into<PathBuf>) -> Self {
        let root_dir = root_dir.into();
        Self { root_dir: fs::canonicalize(&root_dir).unwrap_or(root_dir) }
    }
}

/// Internal state during import resolution.
struct LoadState {
    files: HashMap<PathBuf, MirrProgram>,
    graph: WorkspaceDependencyGraph,
}

/// Tracks import relationships to detect cycles and sort for compilation.
#[derive(Debug, Default)]
pub struct WorkspaceDependencyGraph {
    /// Mapping from file to its set of direct imports.
    pub dependencies: HashMap<PathBuf, HashSet<PathBuf>>,
}

impl WorkspaceDependencyGraph {
    pub fn new() -> Self {
        Self { dependencies: HashMap::new() }
    }

    pub fn add_dependency(&mut self, file: PathBuf, dependency: PathBuf) {
        self.dependencies.entry(file).or_default().insert(dependency);
    }

    pub fn all_files(&self) -> HashSet<PathBuf> {
        let mut files = HashSet::new();
        for (k, v) in &self.dependencies {
            files.insert(k.clone());
            for dep in v {
                files.insert(dep.clone());
            }
        }
        files
    }

    pub fn dependency_count(&self) -> usize {
        self.dependencies.values().map(HashSet::len).sum()
    }

    pub fn topological_sort(&self) -> Result<Vec<PathBuf>, PathBuf> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();
        let mut temp = HashSet::new();

        let all_files = self.all_files();
        for file in all_files {
            self.visit(&file, &mut visited, &mut temp, &mut result)?;
        }

        Ok(result)
    }

    fn visit(
        &self,
        file: &Path,
        visited: &mut HashSet<PathBuf>,
        temp: &mut HashSet<PathBuf>,
        result: &mut Vec<PathBuf>,
    ) -> Result<(), PathBuf> {
        if temp.contains(file) {
            return Err(file.to_path_buf()); // Cycle detected
        }
        if !visited.contains(file) {
            temp.insert(file.to_path_buf());
            if let Some(deps) = self.dependencies.get(file) {
                for dep in deps {
                    self.visit(dep, visited, temp, result)?;
                }
            }
            temp.remove(file);
            visited.insert(file.to_path_buf());
            result.push(file.to_path_buf());
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum WorkspaceError {
    MissingSource(PathBuf),
    Io { path: PathBuf, message: String },
    Parse { path: PathBuf, error: MirrError },
    Import { path: PathBuf, message: String },
    Pipeline(PipelineErrors),
}

impl std::fmt::Display for WorkspaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingSource(path) => write!(f, "missing source for {}", path.display()),
            Self::Io { path, message } => write!(f, "I/O error for {}: {message}", path.display()),
            Self::Parse { path, error } => write!(f, "parse error in {}: {error}", path.display()),
            Self::Import { path, message } => {
                write!(f, "import resolution error in {}: {message}", path.display())
            }
            Self::Pipeline(errors) => write!(f, "{errors}"),
        }
    }
}

impl std::error::Error for WorkspaceError {}

impl From<PipelineErrors> for WorkspaceError {
    fn from(value: PipelineErrors) -> Self {
        Self::Pipeline(value)
    }
}

impl Workspace {
    pub fn new(root_dir: impl Into<PathBuf>) -> Self {
        Self {
            config: WorkspaceConfig::new(root_dir),
            files: HashMap::new(),
            snapshots: HashMap::new(),
        }
    }

    pub fn get_snapshot(&self, path: impl AsRef<Path>) -> Option<&WorkspaceSnapshot> {
        let path = canonical_or_self(path.as_ref());
        self.snapshots.get(&path)
    }

    /// Primary entry point for multi-file compilation.
    ///
    /// Performs recursive resolution, program merging (linking), and pipeline execution.
    pub fn compile_snapshot(
        &mut self,
        root_path: impl AsRef<Path>,
        config: &PipelineConfig,
    ) -> Result<WorkspaceSnapshot, WorkspaceError> {
        let root_path = canonical_or_self(root_path.as_ref());
        let root_source = self.load_source(&root_path)?;
        let root_source_hash = hash_text(&root_source);

        // 1. Recursive Resolution and Parsing
        let mut load_state =
            LoadState { files: HashMap::new(), graph: WorkspaceDependencyGraph::new() };
        self.resolve_imports_recursive(&root_path, &root_source, &mut load_state)?;

        // 2. Compute Workspace Hash (for caching)
        let workspace_hash =
            self.compute_workspace_hash(&root_path, &root_source_hash, &load_state, config);
        if let Some(existing) = self.snapshots.get(&root_path) {
            if existing.workspace_hash == workspace_hash {
                return Ok(existing.clone());
            }
        }

        // 3. Linking (Program Merging)
        let expanded = crate::compiler::macro_proc::expand_macros(&root_source);
        let mut merged_program = parse_mirr(&expanded)
            .map_err(|error| WorkspaceError::Parse { path: root_path.clone(), error })?;

        // Merge patterns from all imported files
        // (This turns Workspace into a real Linker!)
        for (path, program) in &load_state.files {
            if path == &root_path {
                continue;
            }

            // Find the alias used to import this file
            let mut alias = None;
            for (parent, deps) in &load_state.graph.dependencies {
                if deps.contains(path) {
                    // Look up alias in the parent's parse result
                    if let Some(prog) = load_state.files.get(parent) {
                        if let Some(imp) = prog.imports.iter().find(|_i| {
                            // This assumes resolution path matches; simple approximation
                            true
                        }) {
                            alias = Some(imp.alias.clone());
                        }
                    }
                }
            }

            let prefix = alias.map(|a| format!("{}::", a)).unwrap_or_default();
            for pat in &program.patterns {
                let mut aliased_pat = pat.clone();
                aliased_pat.name = format!("{}{}", prefix, pat.name);
                merged_program.patterns.push(aliased_pat);
            }
        }

        // 4. Pipeline Execution
        let pipeline = Rc::new(
            run_pipeline_on_program(merged_program, config).map_err(WorkspaceError::Pipeline)?,
        );

        let mut imported_paths: Vec<PathBuf> = load_state.files.keys().cloned().collect();
        imported_paths.sort();

        let artifact_summary = WorkspaceArtifactSummary {
            loaded_files: load_state.files.len(),
            dependency_nodes: load_state.graph.all_files().len(),
            source_hash: root_source_hash.clone(),
            workspace_hash: workspace_hash.clone(),
        };

        let snapshot = WorkspaceSnapshot {
            root_path: root_path.clone(),
            pipeline,
            workspace_hash,
            metadata: artifact_summary,
        };

        self.snapshots.insert(root_path, snapshot.clone());
        Ok(snapshot)
    }

    /// Recursively load and parse all dependencies.
    fn resolve_imports_recursive(
        &mut self,
        current_path: &Path,
        source: &str,
        state: &mut LoadState,
    ) -> Result<(), WorkspaceError> {
        let current_path = canonical_or_self(current_path);
        if state.files.contains_key(&current_path) {
            return Ok(());
        }

        let expanded = crate::compiler::macro_proc::expand_macros(source);
        let parsed = parse_mirr(&expanded)
            .map_err(|error| WorkspaceError::Parse { path: current_path.to_path_buf(), error })?;

        state.files.insert(current_path.to_path_buf(), parsed.clone());

        for import in &parsed.imports {
            let dep_path = self.resolve_import_path(&current_path, &import.path)?;
            state.graph.add_dependency(current_path.to_path_buf(), dep_path.clone());

            let dep_source = self.load_source(&dep_path)?;
            self.resolve_imports_recursive(&dep_path, &dep_source, state)?;
        }

        Ok(())
    }

    pub fn load_source(&mut self, path: &Path) -> Result<String, WorkspaceError> {
        let path = canonical_or_self(path);
        if let Some(s) = self.files.get(&path) {
            return Ok(s.clone());
        }

        let source = fs::read_to_string(&path).map_err(|error| WorkspaceError::Io {
            path: path.to_path_buf(),
            message: error.to_string(),
        })?;

        self.files.insert(path.to_path_buf(), source.clone());
        Ok(source)
    }

    fn resolve_import_path(
        &self,
        current_file: &Path,
        import_path: &str,
    ) -> Result<PathBuf, WorkspaceError> {
        if import_path.is_empty() {
            return Err(WorkspaceError::Import {
                path: current_file.to_path_buf(),
                message: "import path cannot be empty".to_string(),
            });
        }

        let current_dir = current_file.parent().unwrap_or_else(|| Path::new("."));
        let resolved = current_dir.join(import_path);

        // Security check: ensure path is within workspace root or subfolders.
        // Prevent path traversal like "../../etc/passwd".
        let canonical = canonical_or_self(&resolved);
        if !canonical.starts_with(&self.config.root_dir) {
            return Err(WorkspaceError::Import {
                path: current_file.to_path_buf(),
                message: format!(
                    "security violation: import path '{}' is outside workspace root",
                    import_path
                ),
            });
        }

        if !canonical.exists() {
            return Err(WorkspaceError::Import {
                path: current_file.to_path_buf(),
                message: format!("imported file does not exist: {}", resolved.display()),
            });
        }

        Ok(canonical)
    }

    fn compute_workspace_hash(
        &self,
        _root_path: &Path,
        root_source_hash: &str,
        state: &LoadState,
        config: &PipelineConfig,
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(root_source_hash.as_bytes());

        let mut sorted_files: Vec<_> = state.files.keys().collect();
        sorted_files.sort();

        for file in sorted_files {
            let content = self.files.get(file).cloned().unwrap_or_default();
            hasher.update(hash_text(&content).as_bytes());
        }

        hasher.update(serde_json::to_vec(config).unwrap());
        hex_encode(hasher.finalize().as_slice())
    }

    pub fn update_file(&mut self, path: impl Into<PathBuf>, content: String) {
        let path = path.into();
        self.files.insert(path.clone(), content);
        // Invalidate snapshots that might depend on this file
        // For simplicity in Phase 1, we just clear everything.
        self.snapshots.clear();
    }
}

impl WorkspaceSnapshot {
    pub fn imported_file_count(&self) -> usize {
        self.metadata.loaded_files
    }
}

fn canonical_or_self(path: &Path) -> PathBuf {
    fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

fn hash_text(text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    hex_encode(hasher.finalize().as_slice())
}

fn hex_encode(bytes: &[u8]) -> String {
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        encoded.push_str(&format!("{b:02x}"));
    }
    encoded
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn root_config() -> PipelineConfig {
        PipelineConfig { temporal: false, rspu: false, mape_k: false, ..PipelineConfig::default() }
    }

    #[test]
    fn test_workspace_load_and_compile() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("main.mirr");
        fs::write(&root, "module test { signal x: in bool; }").unwrap();

        let mut workspace = Workspace::new(tmp.path());
        let snapshot = workspace.compile_snapshot(&root, &root_config()).unwrap();

        assert_eq!(snapshot.metadata.loaded_files, 1);
        assert_eq!(snapshot.pipeline.program.module.name, "test");
    }
}
