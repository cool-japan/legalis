//! Transaction support for batch updates to legal data.
//!
//! This module provides transactional semantics for updating legal structures,
//! allowing multiple operations to be grouped together and committed or rolled
//! back as a unit.
//!
//! # Features
//!
//! - **Atomic Updates**: All changes succeed or all fail
//! - **Rollback**: Undo changes if validation fails
//! - **Audit Trail**: Track all changes made in a transaction
//! - **Validation**: Ensure consistency before committing
//!
//! # Examples
//!
//! ```
//! use legalis_core::{Statute, Effect, EffectType};
//! use legalis_core::transactions::{Transaction, TransactionBuilder};
//!
//! let mut txn = TransactionBuilder::new()
//!     .with_description("Update tax laws")
//!     .build();
//!
//! // Add operations to the transaction
//! let statute1 = Statute::new("tax-1", "Tax Law 1", Effect::new(EffectType::Grant, "Credit"));
//! let statute2 = Statute::new("tax-2", "Tax Law 2", Effect::new(EffectType::Obligation, "File"));
//!
//! txn.add_statute(statute1);
//! txn.add_statute(statute2);
//!
//! // Commit the transaction
//! let result = txn.commit();
//! assert!(result.is_ok());
//! ```

use crate::{Effect, Statute, ValidationError};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Represents a single operation in a transaction.
#[derive(Debug, Clone)]
pub enum Operation {
    /// Add a new statute
    AddStatute(Statute),
    /// Update an existing statute
    UpdateStatute { old: Box<Statute>, new: Statute },
    /// Remove a statute
    RemoveStatute(String),
    /// Modify statute effect
    ModifyEffect {
        statute_id: String,
        new_effect: Effect,
    },
}

impl Operation {
    /// Get a description of this operation.
    pub fn description(&self) -> String {
        match self {
            Operation::AddStatute(s) => format!("Add statute: {}", s.id),
            Operation::UpdateStatute { old, new } => {
                format!("Update statute: {} -> {}", old.id, new.id)
            }
            Operation::RemoveStatute(id) => format!("Remove statute: {}", id),
            Operation::ModifyEffect { statute_id, .. } => {
                format!("Modify effect of statute: {}", statute_id)
            }
        }
    }

    /// Get the statute ID affected by this operation.
    pub fn affected_statute_id(&self) -> String {
        match self {
            Operation::AddStatute(s) => s.id.clone(),
            Operation::UpdateStatute { new, .. } => new.id.clone(),
            Operation::RemoveStatute(id) => id.clone(),
            Operation::ModifyEffect { statute_id, .. } => statute_id.clone(),
        }
    }
}

/// Result of a transaction commit.
#[derive(Debug, Clone, PartialEq)]
pub enum TransactionResult {
    /// Transaction committed successfully
    Success {
        operations_count: usize,
        timestamp: DateTime<Utc>,
    },
    /// Transaction failed validation
    ValidationFailed { errors: Vec<ValidationError> },
    /// Transaction was rolled back
    RolledBack { reason: String },
}

/// A transaction for batch updates.
///
/// Groups multiple operations together and ensures they all succeed or all fail.
#[derive(Debug, Clone)]
pub struct Transaction {
    /// Unique transaction ID
    pub id: String,
    /// Human-readable description
    pub description: Option<String>,
    /// Operations in this transaction
    operations: Vec<Operation>,
    /// Transaction metadata
    metadata: HashMap<String, String>,
    /// Timestamp when transaction was created
    created_at: DateTime<Utc>,
    /// Whether transaction is committed
    committed: bool,
}

impl Transaction {
    /// Create a new transaction.
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            description: None,
            operations: Vec::new(),
            metadata: HashMap::new(),
            created_at: Utc::now(),
            committed: false,
        }
    }

    /// Add a statute to the transaction.
    pub fn add_statute(&mut self, statute: Statute) {
        self.operations.push(Operation::AddStatute(statute));
    }

    /// Update a statute in the transaction.
    pub fn update_statute(&mut self, old: Statute, new: Statute) {
        self.operations.push(Operation::UpdateStatute {
            old: Box::new(old),
            new,
        });
    }

    /// Remove a statute from the transaction.
    pub fn remove_statute(&mut self, statute_id: impl ToString) {
        self.operations
            .push(Operation::RemoveStatute(statute_id.to_string()));
    }

    /// Modify the effect of a statute.
    pub fn modify_effect(&mut self, statute_id: impl ToString, new_effect: Effect) {
        self.operations.push(Operation::ModifyEffect {
            statute_id: statute_id.to_string(),
            new_effect,
        });
    }

    /// Get all operations in this transaction.
    pub fn operations(&self) -> &[Operation] {
        &self.operations
    }

    /// Get the number of operations.
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Check if the transaction has been committed.
    pub fn is_committed(&self) -> bool {
        self.committed
    }

    /// Set metadata for this transaction.
    pub fn set_metadata(&mut self, key: impl ToString, value: impl ToString) {
        self.metadata.insert(key.to_string(), value.to_string());
    }

    /// Get metadata value.
    pub fn get_metadata(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(|s| s.as_str())
    }

    /// Validate all operations in the transaction.
    pub fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        for operation in &self.operations {
            match operation {
                Operation::AddStatute(statute) | Operation::UpdateStatute { new: statute, .. } => {
                    let validation_errors = statute.validate();
                    errors.extend(validation_errors);
                }
                _ => {}
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Commit the transaction.
    ///
    /// This validates all operations and marks the transaction as committed.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_core::transactions::Transaction;
    ///
    /// let mut txn = Transaction::new();
    /// txn.add_statute(Statute::new("test", "Test", Effect::grant("Test")));
    ///
    /// let result = txn.commit();
    /// assert!(result.is_ok());
    /// ```
    pub fn commit(&mut self) -> Result<TransactionResult, TransactionResult> {
        if self.committed {
            return Err(TransactionResult::RolledBack {
                reason: "Transaction already committed".to_string(),
            });
        }

        match self.validate() {
            Ok(()) => {
                self.committed = true;
                Ok(TransactionResult::Success {
                    operations_count: self.operations.len(),
                    timestamp: Utc::now(),
                })
            }
            Err(errors) => Err(TransactionResult::ValidationFailed { errors }),
        }
    }

    /// Rollback the transaction.
    ///
    /// Clears all operations and marks the transaction as not committed.
    pub fn rollback(&mut self) -> TransactionResult {
        self.operations.clear();
        self.committed = false;
        TransactionResult::RolledBack {
            reason: "Manually rolled back".to_string(),
        }
    }

    /// Get a summary of the transaction.
    pub fn summary(&self) -> String {
        let mut summary = format!("Transaction {}", self.id);
        if let Some(ref desc) = self.description {
            summary.push_str(&format!(": {}", desc));
        }
        summary.push_str(&format!("\nOperations: {}", self.operations.len()));
        summary.push_str(&format!(
            "\nCreated: {}",
            self.created_at.format("%Y-%m-%d %H:%M:%S")
        ));
        summary.push_str(&format!("\nCommitted: {}", self.committed));
        summary
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating transactions.
///
/// # Examples
///
/// ```
/// use legalis_core::transactions::TransactionBuilder;
///
/// let txn = TransactionBuilder::new()
///     .with_description("Update regulations")
///     .with_metadata("author", "Legal Team")
///     .build();
///
/// assert_eq!(txn.description, Some("Update regulations".to_string()));
/// ```
pub struct TransactionBuilder {
    description: Option<String>,
    metadata: HashMap<String, String>,
}

impl TransactionBuilder {
    /// Create a new transaction builder.
    pub fn new() -> Self {
        Self {
            description: None,
            metadata: HashMap::new(),
        }
    }

    /// Set the transaction description.
    pub fn with_description(mut self, description: impl ToString) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// Add metadata to the transaction.
    pub fn with_metadata(mut self, key: impl ToString, value: impl ToString) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }

    /// Build the transaction.
    pub fn build(self) -> Transaction {
        let mut txn = Transaction::new();
        txn.description = self.description;
        txn.metadata = self.metadata;
        txn
    }
}

impl Default for TransactionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Batch processor for executing multiple transactions.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_core::transactions::{BatchProcessor, Transaction};
///
/// let mut batch = BatchProcessor::new();
///
/// let mut txn1 = Transaction::new();
/// txn1.add_statute(Statute::new("s1", "Statute 1", Effect::grant("Grant")));
///
/// let mut txn2 = Transaction::new();
/// txn2.add_statute(Statute::new("s2", "Statute 2", Effect::grant("Grant")));
///
/// batch.add_transaction(txn1);
/// batch.add_transaction(txn2);
///
/// let results = batch.execute();
/// assert_eq!(results.len(), 2);
/// ```
pub struct BatchProcessor {
    transactions: Vec<Transaction>,
    stop_on_error: bool,
}

impl BatchProcessor {
    /// Create a new batch processor.
    pub fn new() -> Self {
        Self {
            transactions: Vec::new(),
            stop_on_error: true,
        }
    }

    /// Add a transaction to the batch.
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    /// Set whether to stop on first error.
    pub fn stop_on_error(mut self, stop: bool) -> Self {
        self.stop_on_error = stop;
        self
    }

    /// Execute all transactions in the batch.
    ///
    /// Returns a vector of results, one for each transaction.
    pub fn execute(&mut self) -> Vec<TransactionResult> {
        let mut results = Vec::new();

        for txn in &mut self.transactions {
            match txn.commit() {
                Ok(result) => results.push(result),
                Err(error) => {
                    results.push(error);
                    if self.stop_on_error {
                        break;
                    }
                }
            }
        }

        results
    }

    /// Get the number of transactions in the batch.
    pub fn transaction_count(&self) -> usize {
        self.transactions.len()
    }

    /// Get all transactions in the batch.
    pub fn transactions(&self) -> &[Transaction] {
        &self.transactions
    }
}

impl Default for BatchProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_basic() {
        let txn = Transaction::new();
        assert!(!txn.is_committed());
        assert_eq!(txn.operation_count(), 0);
    }

    #[test]
    fn test_transaction_add_statute() {
        let mut txn = Transaction::new();
        let statute = Statute::new("test", "Test", Effect::grant("Test"));
        txn.add_statute(statute);

        assert_eq!(txn.operation_count(), 1);
    }

    #[test]
    fn test_transaction_commit() {
        let mut txn = Transaction::new();
        let statute = Statute::new("test", "Test", Effect::grant("Test"));
        txn.add_statute(statute);

        let _result = txn.commit();
        assert!(txn.is_committed());
    }

    #[test]
    fn test_transaction_rollback() {
        let mut txn = Transaction::new();
        let statute = Statute::new("test", "Test", Effect::grant("Test"));
        txn.add_statute(statute);

        let _result = txn.rollback();
        assert_eq!(txn.operation_count(), 0);
        assert!(!txn.is_committed());
    }

    #[test]
    fn test_transaction_validation() {
        let mut txn = Transaction::new();
        // Invalid statute (empty ID)
        let statute = Statute::new("", "Test", Effect::grant("Test"));
        txn.add_statute(statute);

        let result = txn.commit();
        assert!(result.is_err());
    }

    #[test]
    fn test_transaction_builder() {
        let txn = TransactionBuilder::new()
            .with_description("Test transaction")
            .with_metadata("author", "Test")
            .build();

        assert_eq!(txn.description, Some("Test transaction".to_string()));
        assert_eq!(txn.get_metadata("author"), Some("Test"));
    }

    #[test]
    fn test_batch_processor() {
        let mut batch = BatchProcessor::new();

        let mut txn1 = Transaction::new();
        txn1.add_statute(Statute::new("s1", "Statute 1", Effect::grant("Grant")));

        let mut txn2 = Transaction::new();
        txn2.add_statute(Statute::new("s2", "Statute 2", Effect::grant("Grant")));

        batch.add_transaction(txn1);
        batch.add_transaction(txn2);

        assert_eq!(batch.transaction_count(), 2);

        let results = batch.execute();
        assert_eq!(results.len(), 2);
    }
}
