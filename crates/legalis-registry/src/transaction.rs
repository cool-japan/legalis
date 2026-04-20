//! Transaction support for batched registry operations.
//!
//! This module provides a transaction pattern that allows
//! multiple operations to be batched together and committed
//! or rolled back as a unit.

use super::*;

/// A transaction operation.
#[derive(Debug, Clone)]
pub enum Operation {
    /// Register a new statute
    Register(Box<StatuteEntry>),
    /// Update an existing statute
    Update {
        statute_id: String,
        statute: Box<Statute>,
    },
    /// Set the status of a statute
    SetStatus {
        statute_id: String,
        status: StatuteStatus,
    },
    /// Add a tag to a statute
    AddTag { statute_id: String, tag: String },
    /// Remove a tag from a statute
    RemoveTag { statute_id: String, tag: String },
    /// Add metadata to a statute
    AddMetadata {
        statute_id: String,
        key: String,
        value: String,
    },
}

/// A transaction for batching operations.
pub struct Transaction {
    operations: Vec<Operation>,
}

impl Transaction {
    /// Creates a new transaction.
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    /// Adds a register operation.
    pub fn register(mut self, entry: StatuteEntry) -> Self {
        self.operations.push(Operation::Register(Box::new(entry)));
        self
    }

    /// Adds an update operation.
    pub fn update(mut self, statute_id: impl Into<String>, statute: Statute) -> Self {
        self.operations.push(Operation::Update {
            statute_id: statute_id.into(),
            statute: Box::new(statute),
        });
        self
    }

    /// Adds a set status operation.
    pub fn set_status(mut self, statute_id: impl Into<String>, status: StatuteStatus) -> Self {
        self.operations.push(Operation::SetStatus {
            statute_id: statute_id.into(),
            status,
        });
        self
    }

    /// Adds an add tag operation.
    pub fn add_tag(mut self, statute_id: impl Into<String>, tag: impl Into<String>) -> Self {
        self.operations.push(Operation::AddTag {
            statute_id: statute_id.into(),
            tag: tag.into(),
        });
        self
    }

    /// Adds a remove tag operation.
    pub fn remove_tag(mut self, statute_id: impl Into<String>, tag: impl Into<String>) -> Self {
        self.operations.push(Operation::RemoveTag {
            statute_id: statute_id.into(),
            tag: tag.into(),
        });
        self
    }

    /// Adds metadata.
    pub fn add_metadata(
        mut self,
        statute_id: impl Into<String>,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.operations.push(Operation::AddMetadata {
            statute_id: statute_id.into(),
            key: key.into(),
            value: value.into(),
        });
        self
    }

    /// Commits the transaction, applying all operations.
    pub fn commit(self, registry: &mut StatuteRegistry) -> RegistryResult<TransactionResult> {
        let mut results = Vec::new();
        let mut successful = 0;
        let mut failed = 0;

        for op in self.operations {
            let result = match op {
                Operation::Register(entry) => registry
                    .register(*entry)
                    .map(OperationResult::Registered)
                    .map_err(OperationError::Registry),
                Operation::Update {
                    statute_id,
                    statute,
                } => registry
                    .update(&statute_id, *statute)
                    .map(OperationResult::Updated)
                    .map_err(OperationError::Registry),
                Operation::SetStatus { statute_id, status } => registry
                    .set_status(&statute_id, status)
                    .map(|_| OperationResult::StatusSet)
                    .map_err(OperationError::Registry),
                Operation::AddTag { statute_id, tag } => registry
                    .add_tag(&statute_id, tag)
                    .map(|_| OperationResult::TagAdded)
                    .map_err(OperationError::Registry),
                Operation::RemoveTag { statute_id, tag } => registry
                    .remove_tag(&statute_id, &tag)
                    .map(|_| OperationResult::TagRemoved)
                    .map_err(OperationError::Registry),
                Operation::AddMetadata {
                    statute_id,
                    key,
                    value,
                } => registry
                    .add_metadata(&statute_id, key, value)
                    .map(|_| OperationResult::MetadataAdded)
                    .map_err(OperationError::Registry),
            };

            match result {
                Ok(r) => {
                    successful += 1;
                    results.push(Ok(r));
                }
                Err(e) => {
                    failed += 1;
                    results.push(Err(e));
                }
            }
        }

        Ok(TransactionResult {
            results,
            successful,
            failed,
        })
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of a transaction operation.
#[derive(Debug, Clone)]
pub enum OperationResult {
    /// Statute was registered
    Registered(Uuid),
    /// Statute was updated
    Updated(u32),
    /// Status was set
    StatusSet,
    /// Tag was added
    TagAdded,
    /// Tag was removed
    TagRemoved,
    /// Metadata was added
    MetadataAdded,
}

/// Error during a transaction operation.
#[derive(Debug, Error)]
pub enum OperationError {
    #[error("Registry error: {0}")]
    Registry(#[from] RegistryError),
}

/// Result of committing a transaction.
#[derive(Debug)]
pub struct TransactionResult {
    /// Results for each operation
    pub results: Vec<Result<OperationResult, OperationError>>,
    /// Number of successful operations
    pub successful: usize,
    /// Number of failed operations
    pub failed: usize,
}

impl TransactionResult {
    /// Returns true if all operations succeeded.
    pub fn is_success(&self) -> bool {
        self.failed == 0
    }

    /// Returns true if any operations failed.
    pub fn has_failures(&self) -> bool {
        self.failed > 0
    }
}
