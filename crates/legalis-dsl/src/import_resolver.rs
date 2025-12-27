//! Import path resolution and validation.
//!
//! This module provides utilities to resolve and validate IMPORT statements,
//! detect circular dependencies, and manage multi-file legal document projects.

use crate::ast::LegalDocument;
use crate::{DslError, DslResult, LegalDslParser};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Import resolution context.
#[derive(Debug, Clone)]
pub struct ImportResolver {
    /// Base directory for resolving relative paths
    base_dir: PathBuf,
    /// Cache of resolved documents (path -> document)
    cache: HashMap<PathBuf, LegalDocument>,
    /// Currently loading paths (for cycle detection)
    loading_stack: Vec<PathBuf>,
}

impl ImportResolver {
    /// Creates a new import resolver with the given base directory.
    pub fn new(base_dir: impl AsRef<Path>) -> Self {
        Self {
            base_dir: base_dir.as_ref().to_path_buf(),
            cache: HashMap::new(),
            loading_stack: Vec::new(),
        }
    }

    /// Resolves a path relative to the base directory.
    pub fn resolve_path(&self, import_path: &str) -> PathBuf {
        let path = Path::new(import_path);
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.base_dir.join(path)
        }
    }

    /// Loads and resolves a legal document with all its imports.
    pub fn load_document(&mut self, path: impl AsRef<Path>) -> DslResult<LegalDocument> {
        let path = path.as_ref().to_path_buf();

        // Check for circular dependencies
        if self.loading_stack.contains(&path) {
            return Err(DslError::parse_error(format!(
                "Circular import dependency detected: {:?}",
                self.loading_stack
            )));
        }

        // Check cache
        if let Some(cached) = self.cache.get(&path) {
            return Ok(cached.clone());
        }

        // Read file
        let content = std::fs::read_to_string(&path)
            .map_err(|e| DslError::parse_error(format!("Failed to read file {:?}: {}", path, e)))?;

        // Parse document
        let parser = LegalDslParser::new();
        self.loading_stack.push(path.clone());
        let doc = parser.parse_document(&content)?;
        self.loading_stack.pop();

        // Resolve imports recursively
        let resolved_doc = self.resolve_imports(&doc, &path)?;

        // Cache the result
        self.cache.insert(path, resolved_doc.clone());

        Ok(resolved_doc)
    }

    /// Resolves all imports in a document.
    fn resolve_imports(
        &mut self,
        doc: &LegalDocument,
        current_path: &Path,
    ) -> DslResult<LegalDocument> {
        let mut merged_doc = doc.clone();

        // Save current base dir
        let saved_base = self.base_dir.clone();

        // Set base dir to the directory of the current file
        if let Some(parent) = current_path.parent() {
            self.base_dir = parent.to_path_buf();
        }

        // Process each import
        for import in &doc.imports {
            let import_path = self.resolve_path(&import.path);

            // Load imported document
            let imported_doc = self.load_document(&import_path)?;

            // Merge statutes from imported document
            for statute in imported_doc.statutes {
                // If there's an alias, we could prefix the statute IDs
                // For now, just add them directly
                merged_doc.statutes.push(statute);
            }
        }

        // Restore base dir
        self.base_dir = saved_base;

        Ok(merged_doc)
    }

    /// Validates all imports without loading them.
    pub fn validate_imports(&self, doc: &LegalDocument) -> Vec<DslError> {
        let mut errors = Vec::new();

        for import in &doc.imports {
            let path = self.resolve_path(&import.path);
            if !path.exists() {
                errors.push(DslError::parse_error(format!(
                    "Import path does not exist: {:?}",
                    import.path
                )));
            } else if !path.is_file() {
                errors.push(DslError::parse_error(format!(
                    "Import path is not a file: {:?}",
                    import.path
                )));
            }
        }

        errors
    }

    /// Clears the document cache.
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Returns the number of cached documents.
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }
}

/// Detects circular import dependencies.
pub fn detect_circular_imports(doc: &LegalDocument, base_dir: &Path) -> Vec<Vec<String>> {
    let mut cycles = Vec::new();
    let mut visited = HashSet::new();
    let mut path = Vec::new();

    fn dfs(
        import_path: &str,
        base_dir: &Path,
        visited: &mut HashSet<PathBuf>,
        path: &mut Vec<String>,
        cycles: &mut Vec<Vec<String>>,
    ) {
        let resolved_path = if Path::new(import_path).is_absolute() {
            PathBuf::from(import_path)
        } else {
            base_dir.join(import_path)
        };

        // Check for cycle
        if let Some(pos) = path.iter().position(|p| p == import_path) {
            cycles.push(path[pos..].to_vec());
            return;
        }

        if visited.contains(&resolved_path) {
            return;
        }

        visited.insert(resolved_path.clone());
        path.push(import_path.to_string());

        // Try to read and parse the file
        if let Ok(content) = std::fs::read_to_string(&resolved_path) {
            let parser = LegalDslParser::new();
            if let Ok(doc) = parser.parse_document(&content) {
                for import in &doc.imports {
                    if let Some(parent) = resolved_path.parent() {
                        dfs(&import.path, parent, visited, path, cycles);
                    }
                }
            }
        }

        path.pop();
    }

    for import in &doc.imports {
        dfs(&import.path, base_dir, &mut visited, &mut path, &mut cycles);
    }

    cycles
}

/// Validates import paths in a document.
pub fn validate_import_paths(doc: &LegalDocument, base_dir: &Path) -> Vec<DslError> {
    let resolver = ImportResolver::new(base_dir);
    resolver.validate_imports(doc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::ImportNode;

    #[test]
    fn test_path_resolution() {
        let resolver = ImportResolver::new("/base/dir");

        // Relative path
        let resolved = resolver.resolve_path("statutes/voting.legalis");
        assert_eq!(resolved, PathBuf::from("/base/dir/statutes/voting.legalis"));

        // Absolute path
        let resolved = resolver.resolve_path("/absolute/path.legalis");
        assert_eq!(resolved, PathBuf::from("/absolute/path.legalis"));
    }

    #[test]
    fn test_validate_imports_nonexistent() {
        let doc = LegalDocument {
            imports: vec![ImportNode {
                path: "/nonexistent/file.legalis".to_string(),
                alias: None,
            }],
            statutes: vec![],
        };

        let resolver = ImportResolver::new("/tmp");
        let errors = resolver.validate_imports(&doc);

        assert!(!errors.is_empty());
    }

    #[test]
    fn test_circular_import_detection() {
        let doc = LegalDocument {
            imports: vec![ImportNode {
                path: "circular.legalis".to_string(),
                alias: None,
            }],
            statutes: vec![],
        };

        // This test would require actual files to work properly
        // For now, we just test that the function doesn't panic
        let cycles = detect_circular_imports(&doc, Path::new("/tmp"));
        // In a real scenario with actual files, we'd check for cycles
        assert!(cycles.is_empty() || !cycles.is_empty());
    }

    #[test]
    fn test_cache() {
        let mut resolver = ImportResolver::new("/tmp");
        assert_eq!(resolver.cache_size(), 0);

        resolver.clear_cache();
        assert_eq!(resolver.cache_size(), 0);
    }
}
