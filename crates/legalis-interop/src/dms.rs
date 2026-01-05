//! Document Management System (DMS) integration for legalis-interop.
//!
//! This module provides integration capabilities with various document management systems.

use crate::{ConversionReport, InteropError, InteropResult, LegalConverter, LegalFormat};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Document metadata for DMS integration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    /// Document ID in the DMS
    pub id: String,
    /// Document title
    pub title: String,
    /// Document format
    pub format: LegalFormat,
    /// Document version
    pub version: String,
    /// Author
    pub author: Option<String>,
    /// Creation timestamp (Unix timestamp in milliseconds)
    pub created_at: Option<u64>,
    /// Last modified timestamp (Unix timestamp in milliseconds)
    pub modified_at: Option<u64>,
    /// Document tags
    pub tags: Vec<String>,
    /// Custom metadata fields
    pub custom_fields: HashMap<String, String>,
}

impl DocumentMetadata {
    /// Creates new document metadata.
    pub fn new(id: String, title: String, format: LegalFormat) -> Self {
        Self {
            id,
            title,
            format,
            version: "1.0".to_string(),
            author: None,
            created_at: None,
            modified_at: None,
            tags: Vec::new(),
            custom_fields: HashMap::new(),
        }
    }

    /// Sets the version.
    pub fn with_version(mut self, version: String) -> Self {
        self.version = version;
        self
    }

    /// Sets the author.
    pub fn with_author(mut self, author: String) -> Self {
        self.author = Some(author);
        self
    }

    /// Adds a tag.
    pub fn with_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }

    /// Adds a custom field.
    pub fn with_custom_field(mut self, key: String, value: String) -> Self {
        self.custom_fields.insert(key, value);
        self
    }
}

/// Document in the DMS.
#[derive(Debug, Clone)]
pub struct Document {
    /// Document metadata
    pub metadata: DocumentMetadata,
    /// Document content
    pub content: String,
}

impl Document {
    /// Creates a new document.
    pub fn new(metadata: DocumentMetadata, content: String) -> Self {
        Self { metadata, content }
    }

    /// Returns the document size in bytes.
    pub fn size(&self) -> usize {
        self.content.len()
    }
}

/// Trait for DMS providers.
pub trait DmsProvider: Send + Sync {
    /// Returns the provider name.
    fn name(&self) -> &str;

    /// Retrieves a document by ID.
    fn get_document(&self, id: &str) -> InteropResult<Document>;

    /// Stores a document.
    fn put_document(&mut self, document: Document) -> InteropResult<String>;

    /// Lists documents matching a query.
    fn list_documents(&self, query: &DmsQuery) -> InteropResult<Vec<DocumentMetadata>>;

    /// Deletes a document by ID.
    fn delete_document(&mut self, id: &str) -> InteropResult<()>;

    /// Updates document metadata.
    fn update_metadata(&mut self, id: &str, metadata: DocumentMetadata) -> InteropResult<()>;
}

/// Query for listing documents.
#[derive(Debug, Clone, Default)]
pub struct DmsQuery {
    /// Filter by format
    pub format: Option<LegalFormat>,
    /// Filter by tags
    pub tags: Vec<String>,
    /// Filter by author
    pub author: Option<String>,
    /// Maximum number of results
    pub limit: Option<usize>,
    /// Offset for pagination
    pub offset: Option<usize>,
}

impl DmsQuery {
    /// Creates a new empty query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filters by format.
    pub fn with_format(mut self, format: LegalFormat) -> Self {
        self.format = Some(format);
        self
    }

    /// Filters by tag.
    pub fn with_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }

    /// Filters by author.
    pub fn with_author(mut self, author: String) -> Self {
        self.author = Some(author);
        self
    }

    /// Sets the limit.
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the offset.
    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }
}

/// File-based DMS provider (for local storage).
#[derive(Debug, Clone)]
pub struct FileDmsProvider {
    /// Root directory for document storage
    root_dir: PathBuf,
    /// In-memory metadata store
    metadata_store: HashMap<String, DocumentMetadata>,
}

impl FileDmsProvider {
    /// Creates a new file-based DMS provider.
    pub fn new(root_dir: PathBuf) -> InteropResult<Self> {
        // Ensure root directory exists
        std::fs::create_dir_all(&root_dir)?;

        Ok(Self {
            root_dir,
            metadata_store: HashMap::new(),
        })
    }

    /// Returns the file path for a document ID.
    fn document_path(&self, id: &str) -> PathBuf {
        self.root_dir.join(format!("{}.txt", id))
    }

    /// Returns the metadata path for a document ID.
    fn metadata_path(&self, id: &str) -> PathBuf {
        self.root_dir.join(format!("{}.meta.json", id))
    }

    /// Loads metadata from disk.
    fn load_metadata(&self, id: &str) -> InteropResult<DocumentMetadata> {
        let path = self.metadata_path(id);
        let json = std::fs::read_to_string(path)?;
        serde_json::from_str(&json).map_err(|e| InteropError::SerializationError(e.to_string()))
    }

    /// Saves metadata to disk.
    fn save_metadata(&self, metadata: &DocumentMetadata) -> InteropResult<()> {
        let path = self.metadata_path(&metadata.id);
        let json = serde_json::to_string_pretty(metadata)
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;
        std::fs::write(path, json)?;
        Ok(())
    }
}

impl DmsProvider for FileDmsProvider {
    fn name(&self) -> &str {
        "file"
    }

    fn get_document(&self, id: &str) -> InteropResult<Document> {
        let content_path = self.document_path(id);
        let content = std::fs::read_to_string(content_path)?;
        let metadata = self.load_metadata(id)?;

        Ok(Document::new(metadata, content))
    }

    fn put_document(&mut self, document: Document) -> InteropResult<String> {
        let id = document.metadata.id.clone();

        // Write content
        let content_path = self.document_path(&id);
        std::fs::write(content_path, &document.content)?;

        // Write metadata
        self.save_metadata(&document.metadata)?;

        // Update in-memory store
        self.metadata_store.insert(id.clone(), document.metadata);

        Ok(id)
    }

    fn list_documents(&self, query: &DmsQuery) -> InteropResult<Vec<DocumentMetadata>> {
        let mut results = Vec::new();

        // Read all metadata files
        for entry in std::fs::read_dir(&self.root_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json")
                && path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .is_some_and(|s| s.ends_with(".meta.json"))
            {
                if let Ok(json) = std::fs::read_to_string(&path) {
                    if let Ok(metadata) = serde_json::from_str::<DocumentMetadata>(&json) {
                        // Apply filters
                        if let Some(format) = query.format {
                            if metadata.format != format {
                                continue;
                            }
                        }

                        if !query.tags.is_empty() {
                            let has_all_tags =
                                query.tags.iter().all(|tag| metadata.tags.contains(tag));
                            if !has_all_tags {
                                continue;
                            }
                        }

                        if let Some(ref author) = query.author {
                            if metadata.author.as_ref() != Some(author) {
                                continue;
                            }
                        }

                        results.push(metadata);
                    }
                }
            }
        }

        // Apply pagination
        if let Some(offset) = query.offset {
            results = results.into_iter().skip(offset).collect();
        }

        if let Some(limit) = query.limit {
            results.truncate(limit);
        }

        Ok(results)
    }

    fn delete_document(&mut self, id: &str) -> InteropResult<()> {
        let content_path = self.document_path(id);
        let metadata_path = self.metadata_path(id);

        std::fs::remove_file(content_path)?;
        std::fs::remove_file(metadata_path)?;

        self.metadata_store.remove(id);

        Ok(())
    }

    fn update_metadata(&mut self, id: &str, metadata: DocumentMetadata) -> InteropResult<()> {
        // Verify document exists
        let content_path = self.document_path(id);
        if !content_path.exists() {
            return Err(InteropError::ConversionError(format!(
                "Document {} not found",
                id
            )));
        }

        self.save_metadata(&metadata)?;
        self.metadata_store.insert(id.to_string(), metadata);

        Ok(())
    }
}

/// DMS integration for legal format conversion.
pub struct DmsIntegration {
    provider: Box<dyn DmsProvider>,
    converter: LegalConverter,
}

impl DmsIntegration {
    /// Creates a new DMS integration.
    pub fn new(provider: Box<dyn DmsProvider>) -> Self {
        Self {
            provider,
            converter: LegalConverter::new(),
        }
    }

    /// Converts a document from one format to another within the DMS.
    pub fn convert_document(
        &mut self,
        source_id: &str,
        target_format: LegalFormat,
    ) -> InteropResult<(String, ConversionReport)> {
        // Retrieve source document
        let source_doc = self.provider.get_document(source_id)?;

        // Convert
        let (converted_content, report) = self.converter.convert(
            &source_doc.content,
            source_doc.metadata.format,
            target_format,
        )?;

        // Create new document with converted content
        let target_id = format!("{}_converted", source_id);
        let target_metadata = DocumentMetadata::new(
            target_id.clone(),
            format!("{} (converted)", source_doc.metadata.title),
            target_format,
        )
        .with_author(
            source_doc
                .metadata
                .author
                .unwrap_or_else(|| "system".to_string()),
        )
        .with_tag("converted".to_string())
        .with_custom_field("source_document".to_string(), source_id.to_string());

        let target_doc = Document::new(target_metadata, converted_content);

        // Store converted document
        let stored_id = self.provider.put_document(target_doc)?;

        Ok((stored_id, report))
    }

    /// Batch converts multiple documents.
    pub fn batch_convert(
        &mut self,
        document_ids: &[String],
        target_format: LegalFormat,
    ) -> Vec<InteropResult<(String, ConversionReport)>> {
        document_ids
            .iter()
            .map(|id| self.convert_document(id, target_format))
            .collect()
    }

    /// Exports a document to a file.
    pub fn export_to_file(&self, document_id: &str, output_path: &Path) -> InteropResult<()> {
        let document = self.provider.get_document(document_id)?;
        std::fs::write(output_path, document.content)?;
        Ok(())
    }

    /// Imports a document from a file.
    pub fn import_from_file(
        &mut self,
        input_path: &Path,
        format: LegalFormat,
        title: String,
    ) -> InteropResult<String> {
        let content = std::fs::read_to_string(input_path)?;
        // Generate a simple ID based on title and content length
        let id = format!("{}_{:x}", title.replace(' ', "_"), content.len());

        let metadata = DocumentMetadata::new(id.clone(), title, format);
        let document = Document::new(metadata, content);

        self.provider.put_document(document)
    }

    /// Returns the DMS provider name.
    pub fn provider_name(&self) -> &str {
        self.provider.name()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_document_metadata() {
        let metadata = DocumentMetadata::new(
            "doc-1".to_string(),
            "Test Document".to_string(),
            LegalFormat::Catala,
        )
        .with_version("1.1".to_string())
        .with_author("Alice".to_string())
        .with_tag("legal".to_string())
        .with_custom_field("jurisdiction".to_string(), "US".to_string());

        assert_eq!(metadata.id, "doc-1");
        assert_eq!(metadata.title, "Test Document");
        assert_eq!(metadata.format, LegalFormat::Catala);
        assert_eq!(metadata.version, "1.1");
        assert_eq!(metadata.author, Some("Alice".to_string()));
        assert!(metadata.tags.contains(&"legal".to_string()));
        assert_eq!(
            metadata.custom_fields.get("jurisdiction"),
            Some(&"US".to_string())
        );
    }

    #[test]
    fn test_document_size() {
        let metadata =
            DocumentMetadata::new("doc-1".to_string(), "Test".to_string(), LegalFormat::Catala);
        let doc = Document::new(metadata, "Hello, World!".to_string());

        assert_eq!(doc.size(), 13);
    }

    #[test]
    fn test_dms_query() {
        let query = DmsQuery::new()
            .with_format(LegalFormat::Catala)
            .with_tag("legal".to_string())
            .with_author("Alice".to_string())
            .with_limit(10)
            .with_offset(5);

        assert_eq!(query.format, Some(LegalFormat::Catala));
        assert_eq!(query.tags, vec!["legal".to_string()]);
        assert_eq!(query.author, Some("Alice".to_string()));
        assert_eq!(query.limit, Some(10));
        assert_eq!(query.offset, Some(5));
    }

    #[test]
    fn test_file_dms_provider() {
        let temp_dir = TempDir::new().unwrap();
        let mut provider = FileDmsProvider::new(temp_dir.path().to_owned()).unwrap();

        let metadata = DocumentMetadata::new(
            "test-1".to_string(),
            "Test Document".to_string(),
            LegalFormat::Catala,
        );
        let doc = Document::new(metadata, "declaration scope Test:".to_string());

        // Put document
        let id = provider.put_document(doc).unwrap();
        assert_eq!(id, "test-1");

        // Get document
        let retrieved = provider.get_document(&id).unwrap();
        assert_eq!(retrieved.metadata.id, "test-1");
        assert_eq!(retrieved.content, "declaration scope Test:");
    }

    #[test]
    fn test_file_dms_list_documents() {
        let temp_dir = TempDir::new().unwrap();
        let mut provider = FileDmsProvider::new(temp_dir.path().to_owned()).unwrap();

        // Add multiple documents
        for i in 0..5 {
            let metadata = DocumentMetadata::new(
                format!("doc-{}", i),
                format!("Document {}", i),
                LegalFormat::Catala,
            )
            .with_tag("test".to_string());

            let doc = Document::new(metadata, format!("Content {}", i));
            provider.put_document(doc).unwrap();
        }

        // List all documents
        let query = DmsQuery::new();
        let results = provider.list_documents(&query).unwrap();
        assert_eq!(results.len(), 5);

        // List with limit
        let query = DmsQuery::new().with_limit(3);
        let results = provider.list_documents(&query).unwrap();
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_file_dms_delete() {
        let temp_dir = TempDir::new().unwrap();
        let mut provider = FileDmsProvider::new(temp_dir.path().to_owned()).unwrap();

        let metadata = DocumentMetadata::new(
            "test-1".to_string(),
            "Test".to_string(),
            LegalFormat::Catala,
        );
        let doc = Document::new(metadata, "Content".to_string());

        provider.put_document(doc).unwrap();
        assert!(provider.get_document("test-1").is_ok());

        provider.delete_document("test-1").unwrap();
        assert!(provider.get_document("test-1").is_err());
    }

    #[test]
    fn test_dms_integration_convert() {
        let temp_dir = TempDir::new().unwrap();
        let provider = Box::new(FileDmsProvider::new(temp_dir.path().to_owned()).unwrap());
        let mut integration = DmsIntegration::new(provider);

        // Add a source document
        let metadata = DocumentMetadata::new(
            "source-1".to_string(),
            "Source Document".to_string(),
            LegalFormat::Catala,
        );
        let doc = Document::new(
            metadata,
            "declaration scope Test:\n  context input content integer".to_string(),
        );

        integration.provider.put_document(doc).unwrap();

        // Convert to L4
        let (target_id, report) = integration
            .convert_document("source-1", LegalFormat::L4)
            .unwrap();

        assert_eq!(target_id, "source-1_converted");
        assert!(report.statutes_converted >= 1);

        // Verify converted document exists
        let converted = integration.provider.get_document(&target_id).unwrap();
        assert_eq!(converted.metadata.format, LegalFormat::L4);
        assert!(converted.content.contains("RULE"));
    }

    #[test]
    fn test_dms_integration_provider_name() {
        let temp_dir = TempDir::new().unwrap();
        let provider = Box::new(FileDmsProvider::new(temp_dir.path().to_owned()).unwrap());
        let integration = DmsIntegration::new(provider);

        assert_eq!(integration.provider_name(), "file");
    }
}
