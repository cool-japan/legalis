use super::*;

/// Format of government database export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GovernmentDataFormat {
    /// JSON format (common in modern APIs)
    Json,
    /// XML format (common in older systems)
    Xml,
    /// CSV format (simple tabular data)
    Csv,
    /// Custom delimiter-separated values
    Dsv { delimiter: char },
    /// Akoma Ntoso (legislative XML standard)
    AkomaNtoso,
    /// LegalDocML
    LegalDocML,
}

/// Import source configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportSource {
    /// Source name
    pub name: String,
    /// Source URL or file path
    pub location: String,
    /// Data format
    pub format: GovernmentDataFormat,
    /// Authentication credentials (if needed)
    pub credentials: Option<String>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl ImportSource {
    /// Creates a new import source.
    pub fn new(
        name: impl Into<String>,
        location: impl Into<String>,
        format: GovernmentDataFormat,
    ) -> Self {
        Self {
            name: name.into(),
            location: location.into(),
            format,
            credentials: None,
            metadata: HashMap::new(),
        }
    }

    /// Sets authentication credentials.
    pub fn with_credentials(mut self, credentials: impl Into<String>) -> Self {
        self.credentials = Some(credentials.into());
        self
    }

    /// Adds metadata.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// Result of a bulk import operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkImportResult {
    /// Source name
    pub source: String,
    /// Number of statutes imported successfully
    pub imported: usize,
    /// Number of statutes skipped (duplicates, etc.)
    pub skipped: usize,
    /// Number of statutes that failed to import
    pub failed: usize,
    /// Errors encountered during import
    pub errors: Vec<String>,
    /// Import timestamp
    pub timestamp: DateTime<Utc>,
    /// Import duration in milliseconds
    pub duration_ms: u64,
}

impl BulkImportResult {
    /// Creates a new bulk import result.
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            imported: 0,
            skipped: 0,
            failed: 0,
            errors: Vec::new(),
            timestamp: Utc::now(),
            duration_ms: 0,
        }
    }

    /// Returns total number of statutes processed.
    pub fn total_processed(&self) -> usize {
        self.imported + self.skipped + self.failed
    }

    /// Returns success rate (0.0-1.0).
    pub fn success_rate(&self) -> f64 {
        let total = self.total_processed();
        if total == 0 {
            1.0
        } else {
            self.imported as f64 / total as f64
        }
    }

    /// Returns whether the import was fully successful.
    pub fn is_success(&self) -> bool {
        self.failed == 0
    }
}

/// Import strategy for handling duplicates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImportStrategy {
    /// Skip duplicate statutes
    Skip,
    /// Update existing statutes
    Update,
    /// Create new version of existing statutes
    NewVersion,
    /// Fail on duplicates
    FailOnDuplicate,
}

/// Bulk importer for government databases.
#[derive(Debug)]
pub struct BulkImporter {
    /// Import strategy
    strategy: ImportStrategy,
    /// Batch size for processing
    batch_size: usize,
    /// Validate before import
    validate: bool,
    /// Auto-enrich imported statutes
    auto_enrich: bool,
}

impl BulkImporter {
    /// Creates a new bulk importer with default settings.
    pub fn new() -> Self {
        Self {
            strategy: ImportStrategy::Skip,
            batch_size: 100,
            validate: true,
            auto_enrich: false,
        }
    }

    /// Sets the import strategy.
    pub fn with_strategy(mut self, strategy: ImportStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    /// Sets the batch size.
    pub fn with_batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size;
        self
    }

    /// Enables or disables validation.
    pub fn with_validation(mut self, validate: bool) -> Self {
        self.validate = validate;
        self
    }

    /// Enables or disables auto-enrichment.
    pub fn with_auto_enrich(mut self, auto_enrich: bool) -> Self {
        self.auto_enrich = auto_enrich;
        self
    }

    /// Imports statutes from a source.
    pub fn import(
        &self,
        registry: &mut StatuteRegistry,
        source: &ImportSource,
        statutes: Vec<StatuteEntry>,
    ) -> BulkImportResult {
        let start = std::time::Instant::now();
        let mut result = BulkImportResult::new(&source.name);

        for entry in statutes {
            let statute_id = entry.statute.id.clone();
            match self.import_single(registry, entry) {
                Ok(true) => result.imported += 1,
                Ok(false) => result.skipped += 1,
                Err(e) => {
                    result.failed += 1;
                    result.errors.push(format!("{}: {}", statute_id, e));
                }
            }
        }

        result.duration_ms = start.elapsed().as_millis() as u64;
        result
    }

    fn import_single(
        &self,
        registry: &mut StatuteRegistry,
        entry: StatuteEntry,
    ) -> RegistryResult<bool> {
        // Validate if enabled
        if self.validate {
            let validator = Validator::with_defaults();
            if let Err(errors) = validator.validate(&entry) {
                return Err(RegistryError::InvalidOperation(format!(
                    "Validation failed: {:?}",
                    errors
                )));
            }
        }

        // Check if statute already exists
        let statute_id = entry.statute.id.clone();
        let exists = registry.contains(&statute_id);

        if exists {
            match self.strategy {
                ImportStrategy::Skip => return Ok(false),
                ImportStrategy::Update => {
                    registry.update(&statute_id, entry.statute)?;
                    return Ok(true);
                }
                ImportStrategy::NewVersion => {
                    registry.update(&statute_id, entry.statute)?;
                    return Ok(true);
                }
                ImportStrategy::FailOnDuplicate => {
                    return Err(RegistryError::DuplicateId(statute_id));
                }
            }
        }

        // Register new statute
        registry.register(entry)?;
        Ok(true)
    }
}

impl Default for BulkImporter {
    fn default() -> Self {
        Self::new()
    }
}
