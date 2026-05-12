#![forbid(unsafe_code)]

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use sha2::{Digest, Sha256};

use crate::ast::program::ImportDecl;
use crate::ast::MirrProgram;
use crate::error::{MirrError, PipelineErrors};
use crate::parser::parse_mirr;
use crate::pipeline::{run_pipeline, PipelineConfig, PipelineResult};

const MAX_IMPORT_DEPTH: usize = 32;
const MAX_TOTAL_IMPORTS: usize = 256;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkspaceConfig {
    pub root_dir: PathBuf,
}

impl WorkspaceConfig {
    pub fn new(root_dir: impl Into<PathBuf>) -> Self {
        Self { root_dir: root_dir.into() }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct FileState {
    source: String,
    source_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkspaceImportStats {
    pub files_loaded: usize,
    pub max_depth: usize,
    pub cycle_checks: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkspaceImportFile {
    pub path: PathBuf,
    pub source: String,
    pub source_hash: String,
    pub program: MirrProgram,
    pub alias: String,
    pub imports: Vec<ImportDecl>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkspaceDependencyGraph {
    dependencies: HashMap<PathBuf, HashSet<PathBuf>>,
}

impl WorkspaceDependencyGraph {
    pub fn new() -> Self {
        Self { dependencies: HashMap::new() }
    }

    pub fn add_dependency(&mut self, from: PathBuf, to: PathBuf) {
        self.dependencies.entry(from).or_default().insert(to);
    }

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

    pub fn dependency_count(&self) -> usize {
        self.dependencies.values().map(HashSet::len).sum()
    }

    pub fn topological_sort(&self) -> Result<Vec<PathBuf>, WorkspaceCycleError> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();
        let mut temp = HashSet::new();

        let mut nodes = self.all_files();
        if nodes.is_empty() {
            nodes.extend(self.dependencies.keys().cloned());
        }

        for node in nodes {
            if !visited.contains(&node) {
                self.dfs_topological_sort(&node, &mut visited, &mut temp, &mut result)?;
            }
        }

        Ok(result)
    }

    fn dfs_topological_sort(
        &self,
        node: &PathBuf,
        visited: &mut HashSet<PathBuf>,
        temp: &mut HashSet<PathBuf>,
        result: &mut Vec<PathBuf>,
    ) -> Result<(), WorkspaceCycleError> {
        if temp.contains(node) {
            return Err(WorkspaceCycleError { cycle: vec![node.clone()] });
        }
        if visited.contains(node) {
            return Ok(());
        }

        temp.insert(node.clone());
        if let Some(deps) = self.dependencies.get(node) {
            for dep in deps {
                self.dfs_topological_sort(dep, visited, temp, result)?;
            }
        }
        temp.remove(node);
        visited.insert(node.clone());
        result.push(node.clone());
        Ok(())
    }
}

impl Default for WorkspaceDependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkspaceImports {
    pub files: HashMap<PathBuf, WorkspaceImportFile>,
    pub dependency_graph: WorkspaceDependencyGraph,
    pub compilation_order: Vec<PathBuf>,
    pub stats: WorkspaceImportStats,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkspaceArtifactSummary {
    pub loaded_files: usize,
    pub dependency_nodes: usize,
    pub source_hash: String,
    pub workspace_hash: String,
}

#[derive(Clone)]
pub struct WorkspaceSnapshot {
    pub root_path: PathBuf,
    pub config: WorkspaceConfig,
    pub source_hash: String,
    pub workspace_hash: String,
    pub imports: Option<WorkspaceImports>,
    pub pipeline: Rc<PipelineResult>,
    pub imported_paths: Vec<PathBuf>,
    pub artifact_summary: WorkspaceArtifactSummary,
}

impl WorkspaceSnapshot {
    pub fn has_imports(&self) -> bool {
        self.imports.as_ref().is_some_and(|imports| !imports.files.is_empty())
    }

    pub fn imported_file_count(&self) -> usize {
        self.imports.as_ref().map_or(0, |imports| imports.files.len())
    }

    pub fn dependency_node_count(&self) -> usize {
        self.imports.as_ref().map_or(0, |imports| imports.dependency_graph.all_files().len())
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
                write!(f, "import error in {}: {message}", path.display())
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

pub struct Workspace {
    config: WorkspaceConfig,
    files: HashMap<PathBuf, FileState>,
    snapshots: HashMap<PathBuf, WorkspaceSnapshot>,
}

struct ImportLoadState<'a> {
    files: &'a mut HashMap<PathBuf, WorkspaceImportFile>,
    graph: &'a mut WorkspaceDependencyGraph,
    processing: &'a mut HashSet<PathBuf>,
    stats: &'a mut WorkspaceImportStats,
}

impl Workspace {
    pub fn new(root_dir: impl Into<PathBuf>) -> Self {
        Self {
            config: WorkspaceConfig::new(root_dir),
            files: HashMap::new(),
            snapshots: HashMap::new(),
        }
    }

    pub fn config(&self) -> &WorkspaceConfig {
        &self.config
    }

    pub fn open_file(&mut self, path: impl AsRef<Path>, source: impl Into<String>) -> PathBuf {
        self.update_file(path, source)
    }

    pub fn update_file(&mut self, path: impl AsRef<Path>, source: impl Into<String>) -> PathBuf {
        let path = canonical_or_self(path.as_ref());
        let source = source.into();
        let source_hash = hash_text(&source);
        self.files.insert(path.clone(), FileState { source, source_hash });
        self.invalidate_path(&path);
        path
    }

    pub fn close_file(&mut self, path: impl AsRef<Path>) -> bool {
        let path = canonical_or_self(path.as_ref());
        let removed = self.files.remove(&path).is_some();
        if removed {
            self.invalidate_path(&path);
        }
        removed
    }

    pub fn source_for(&self, path: &Path) -> Option<&str> {
        self.files.get(path).map(|state| state.source.as_str())
    }

    pub fn snapshot(&self, path: impl AsRef<Path>) -> Option<&WorkspaceSnapshot> {
        let path = canonical_or_self(path.as_ref());
        self.snapshots.get(&path)
    }

    pub fn compile_snapshot(
        &mut self,
        root_path: impl AsRef<Path>,
        config: &PipelineConfig,
    ) -> Result<WorkspaceSnapshot, WorkspaceError> {
        let root_path = canonical_or_self(root_path.as_ref());
        let root_source = self.load_source(&root_path)?;
        let root_source_hash = hash_text(&root_source);

        let parsed = parse_mirr(&root_source)
            .map_err(|error| WorkspaceError::Parse { path: root_path.clone(), error })?;

        let imports = if parsed.imports.is_empty() {
            None
        } else {
            Some(self.resolve_imports(&root_path, &parsed.imports)?)
        };

        let workspace_hash = self.workspace_hash(&root_path, &root_source_hash, &imports, config);

        if let Some(existing) = self.snapshots.get(&root_path) {
            if existing.workspace_hash == workspace_hash {
                return Ok(existing.clone());
            }
        }

        let pipeline =
            Rc::new(run_pipeline(&root_source, config).map_err(WorkspaceError::Pipeline)?);

        let mut imported_paths = imports
            .as_ref()
            .map(|load| load.files.keys().cloned().collect::<Vec<_>>())
            .unwrap_or_default();
        imported_paths.sort();

        let dependency_node_count =
            imports.as_ref().map_or(0, |load| load.dependency_graph.all_files().len());

        let artifact_summary = WorkspaceArtifactSummary {
            loaded_files: imports.as_ref().map_or(0, |load| load.files.len()),
            dependency_nodes: dependency_node_count,
            source_hash: root_source_hash.clone(),
            workspace_hash: workspace_hash.clone(),
        };

        let snapshot = WorkspaceSnapshot {
            root_path: root_path.clone(),
            config: self.config.clone(),
            source_hash: root_source_hash,
            workspace_hash,
            imports,
            pipeline,
            imported_paths,
            artifact_summary,
        };

        self.snapshots.insert(root_path, snapshot.clone());
        Ok(snapshot)
    }

    fn load_source(&mut self, path: &Path) -> Result<String, WorkspaceError> {
        if let Some(state) = self.files.get(path) {
            return Ok(state.source.clone());
        }

        let source = fs::read_to_string(path).map_err(|error| WorkspaceError::Io {
            path: path.to_path_buf(),
            message: error.to_string(),
        })?;
        let source_hash = hash_text(&source);
        self.files.insert(path.to_path_buf(), FileState { source: source.clone(), source_hash });
        Ok(source)
    }

    fn invalidate_path(&mut self, path: &Path) {
        self.snapshots.retain(|root, snapshot| {
            if root == path {
                return false;
            }
            !snapshot.imported_paths.iter().any(|imported| imported == path)
        });
    }

    fn resolve_imports(
        &mut self,
        root_path: &Path,
        imports: &[ImportDecl],
    ) -> Result<WorkspaceImports, WorkspaceError> {
        let mut files = HashMap::new();
        let mut graph = WorkspaceDependencyGraph::new();
        let mut processing = HashSet::new();
        let mut stats = WorkspaceImportStats { files_loaded: 0, max_depth: 0, cycle_checks: 0 };
        let compilation_order = {
            let mut load_state = ImportLoadState {
                files: &mut files,
                graph: &mut graph,
                processing: &mut processing,
                stats: &mut stats,
            };

            for import in imports {
                self.load_import_decl(root_path, import, 1, &mut load_state)?;
            }

            load_state.stats.files_loaded = load_state.files.len();
            load_state.stats.cycle_checks = load_state.stats.cycle_checks.saturating_add(1);

            load_state.graph.topological_sort().map_err(|cycle| WorkspaceError::Import {
                path: root_path.to_path_buf(),
                message: format!("circular dependency detected: {:?}", cycle.cycle),
            })?
        };

        Ok(WorkspaceImports { files, dependency_graph: graph, compilation_order, stats })
    }

    fn load_import_decl(
        &mut self,
        current_file: &Path,
        import: &ImportDecl,
        depth: usize,
        state: &mut ImportLoadState<'_>,
    ) -> Result<(), WorkspaceError> {
        if depth > MAX_IMPORT_DEPTH {
            return Err(WorkspaceError::Import {
                path: current_file.to_path_buf(),
                message: format!("import depth limit exceeded: {depth}"),
            });
        }

        if state.files.len() >= MAX_TOTAL_IMPORTS {
            return Err(WorkspaceError::Import {
                path: current_file.to_path_buf(),
                message: format!("total import limit exceeded: {}", state.files.len()),
            });
        }

        let file_path = self.resolve_import_path(current_file, &import.path)?;
        if state.files.contains_key(&file_path) {
            return Ok(());
        }
        if !state.processing.insert(file_path.clone()) {
            return Err(WorkspaceError::Import {
                path: file_path.clone(),
                message: "circular dependency detected".to_string(),
            });
        }

        let result = (|| -> Result<(), WorkspaceError> {
            let source = self.load_source(&file_path)?;
            let source_hash = hash_text(&source);
            let program = parse_mirr(&source)
                .map_err(|error| WorkspaceError::Parse { path: file_path.clone(), error })?;

            let resolved = WorkspaceImportFile {
                path: file_path.clone(),
                source: source.clone(),
                source_hash,
                program: program.clone(),
                alias: import.alias.clone(),
                imports: program.imports.clone(),
            };

            for child in &resolved.imports {
                let dep_path = self.resolve_import_path(&file_path, &child.path)?;
                state.graph.add_dependency(file_path.clone(), dep_path);
                self.load_import_decl(&file_path, child, depth + 1, state)?;
            }

            state.stats.max_depth = state.stats.max_depth.max(depth);
            state.files.insert(file_path.clone(), resolved);
            Ok(())
        })();

        state.processing.remove(&file_path);
        result
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
        if import_path.contains("..") {
            return Err(WorkspaceError::Import {
                path: current_file.to_path_buf(),
                message: format!("path traversal not allowed: {import_path}"),
            });
        }

        let current_dir = current_file.parent().unwrap_or(self.config.root_dir.as_path());
        let mut candidates = Vec::new();
        let raw = Path::new(import_path);

        if raw.is_absolute() {
            candidates.push(raw.to_path_buf());
        } else {
            candidates.push(current_dir.join(raw));
            candidates.push(self.config.root_dir.join(raw));
        }

        if !import_path.ends_with(".mirr") {
            let with_ext = format!("{import_path}.mirr");
            candidates.push(current_dir.join(&with_ext));
            candidates.push(self.config.root_dir.join(&with_ext));
        }

        for candidate in candidates {
            if candidate.exists() && candidate.is_file() {
                return Ok(candidate.canonicalize().unwrap_or(candidate));
            }
        }

        Err(WorkspaceError::Import {
            path: current_file.to_path_buf(),
            message: format!("import file not found: {import_path}"),
        })
    }

    fn workspace_hash(
        &self,
        root_path: &Path,
        root_source_hash: &str,
        imports: &Option<WorkspaceImports>,
        config: &PipelineConfig,
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(root_path.to_string_lossy().as_bytes());
        hasher.update([0]);
        hasher.update(root_source_hash.as_bytes());
        hasher.update([0]);
        hasher.update(config_fingerprint(config).as_bytes());
        hasher.update([0]);

        if let Some(imports) = imports {
            let mut entries: Vec<(String, String)> = imports
                .files
                .iter()
                .map(|(path, file)| (path.to_string_lossy().to_string(), file.source_hash.clone()))
                .collect();
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            for (path, hash) in entries {
                hasher.update(path.as_bytes());
                hasher.update([0]);
                hasher.update(hash.as_bytes());
                hasher.update([0]);
            }
        }

        hasher.update(env!("CARGO_PKG_VERSION").as_bytes());
        let digest = hasher.finalize();
        hex_lower(digest.as_ref())
    }
}

fn canonical_or_self(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

fn hash_text(text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    hex_lower(hasher.finalize().as_ref())
}

fn hex_lower(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(HEX[(byte >> 4) as usize] as char);
        encoded.push(HEX[(byte & 0x0f) as usize] as char);
    }
    encoded
}

fn config_fingerprint(config: &PipelineConfig) -> String {
    format!(
        "typecheck={};simplify={};sat_simplify={};width={};temporal={};rspu={};extended_typecheck={};simulate={};mape_k={};retiming={};totality={};symbolic={};emit_mape_k_rtl={};hls={};logic_optimize={};mape_k_ticks={:?};mape_k_partition={:?}",
        config.typecheck,
        config.simplify,
        config.sat_simplify,
        config.width,
        config.temporal,
        config.rspu,
        config.extended_typecheck,
        config.simulate,
        config.mape_k,
        config.retiming,
        config.totality,
        config.symbolic,
        config.emit_mape_k_rtl,
        config.hls,
        config.logic_optimize,
        config.mape_k_ticks,
        config.mape_k_partition
    )
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkspaceCycleError {
    cycle: Vec<PathBuf>,
}

impl WorkspaceCycleError {
    pub fn cycle(&self) -> &[PathBuf] {
        &self.cycle
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn write_file(dir: &Path, name: &str, content: &str) -> PathBuf {
        let path = dir.join(name);
        fs::write(&path, content).unwrap();
        path
    }

    fn root_config() -> PipelineConfig {
        PipelineConfig { temporal: false, rspu: false, mape_k: false, ..PipelineConfig::default() }
    }

    #[test]
    fn workspace_caches_unchanged_snapshot() {
        let tmp = TempDir::new().unwrap();
        let root = write_file(tmp.path(), "main.mirr", "module top {\n  signal x: in bool;\n}\n");
        let mut workspace = Workspace::new(tmp.path());

        let first = workspace.compile_snapshot(&root, &root_config()).unwrap();
        let second = workspace.compile_snapshot(&root, &root_config()).unwrap();

        assert_eq!(first.workspace_hash, second.workspace_hash);
        assert_eq!(workspace.snapshot(&root).unwrap().workspace_hash, first.workspace_hash);
    }

    #[test]
    fn workspace_tracks_imports_and_invalidates_on_dependency_change() {
        let tmp = TempDir::new().unwrap();
        let root = write_file(
            tmp.path(),
            "main.mirr",
            "import \"dep.mirr\" as dep;\nmodule top {\n  signal x: in bool;\n}\n",
        );
        let dep = write_file(tmp.path(), "dep.mirr", "module dep {\n  signal y: in bool;\n}\n");

        let mut workspace = Workspace::new(tmp.path());
        let first = workspace.compile_snapshot(&root, &root_config()).unwrap();
        assert!(first.imported_paths.iter().any(|path| path == &dep.canonicalize().unwrap()));
        assert!(first.has_imports());

        workspace.update_file(&dep, "module dep {\n  signal y: out bool;\n}\n");
        let second = workspace.compile_snapshot(&root, &root_config()).unwrap();

        assert_ne!(first.workspace_hash, second.workspace_hash);
        assert_eq!(second.imported_file_count(), 1);
    }
}
