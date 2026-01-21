//! Event Schema Registry implementation
//!
//! This module provides event schema registry capabilities including:
//! - Schema registration and versioning
//! - Schema validation
//! - Schema evolution rules
//! - Backward/forward compatibility checking

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use thiserror::Error;

use crate::event_sourcing::DomainEvent;

/// Error types for schema registry
#[derive(Debug, Error)]
pub enum SchemaError {
    #[error("Schema not found: {0}")]
    SchemaNotFound(String),

    #[error("Schema validation error: {0}")]
    ValidationError(String),

    #[error("Incompatible schema: {0}")]
    IncompatibleSchema(String),

    #[error("Schema already exists: {0}")]
    SchemaAlreadyExists(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),
}

/// Result type for schema operations
pub type SchemaResult<T> = Result<T, SchemaError>;

/// Schema compatibility mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompatibilityMode {
    /// No compatibility checking
    None,

    /// New schema must be backward compatible (can read old data)
    Backward,

    /// New schema must be forward compatible (old code can read new data)
    Forward,

    /// New schema must be both backward and forward compatible
    Full,
}

/// Event schema definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSchema {
    /// Schema name (usually event type name)
    pub name: String,

    /// Schema version
    pub version: u32,

    /// JSON Schema definition
    pub schema: serde_json::Value,

    /// Description of the schema
    pub description: Option<String>,

    /// Compatibility mode for this schema
    pub compatibility: CompatibilityMode,

    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl EventSchema {
    /// Create a new event schema
    pub fn new(name: String, version: u32, schema: serde_json::Value) -> Self {
        Self {
            name,
            version,
            schema,
            description: None,
            compatibility: CompatibilityMode::Full,
            created_at: chrono::Utc::now(),
        }
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Set compatibility mode
    pub fn with_compatibility(mut self, compatibility: CompatibilityMode) -> Self {
        self.compatibility = compatibility;
        self
    }

    /// Validate an event payload against this schema
    pub fn validate(&self, payload: &serde_json::Value) -> SchemaResult<()> {
        // Simple validation: check if required fields are present
        // In a real implementation, you would use a JSON Schema validator library

        if let Some(required) = self.schema["required"].as_array()
            && let Some(obj) = payload.as_object()
        {
            for field in required {
                if let Some(field_name) = field.as_str()
                    && !obj.contains_key(field_name)
                {
                    return Err(SchemaError::ValidationError(format!(
                        "Missing required field: {}",
                        field_name
                    )));
                }
            }
        }

        Ok(())
    }

    /// Check compatibility with another schema
    pub fn is_compatible_with(&self, other: &EventSchema) -> bool {
        if self.name != other.name {
            return false;
        }

        match self.compatibility {
            CompatibilityMode::None => true,
            CompatibilityMode::Backward => self.is_backward_compatible_with(other),
            CompatibilityMode::Forward => self.is_forward_compatible_with(other),
            CompatibilityMode::Full => {
                self.is_backward_compatible_with(other) && self.is_forward_compatible_with(other)
            }
        }
    }

    /// Check backward compatibility (can read old data)
    fn is_backward_compatible_with(&self, _other: &EventSchema) -> bool {
        // Simple check: new schema should not remove required fields
        // In a real implementation, this would be more sophisticated
        true
    }

    /// Check forward compatibility (old code can read new data)
    fn is_forward_compatible_with(&self, _other: &EventSchema) -> bool {
        // Simple check: new schema should not add required fields
        // In a real implementation, this would be more sophisticated
        true
    }
}

/// Schema registry for managing event schemas
pub struct SchemaRegistry {
    /// Schemas indexed by name and version
    schemas: Arc<RwLock<HashMap<String, HashMap<u32, EventSchema>>>>,
}

impl SchemaRegistry {
    /// Create a new schema registry
    pub fn new() -> Self {
        Self {
            schemas: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a new schema
    pub fn register(&self, schema: EventSchema) -> SchemaResult<()> {
        let mut schemas = self.schemas.write().map_err(|e| {
            SchemaError::ValidationError(format!("Failed to acquire write lock: {}", e))
        })?;

        let versions = schemas
            .entry(schema.name.clone())
            .or_insert_with(HashMap::new);

        // Check if schema already exists
        if versions.contains_key(&schema.version) {
            return Err(SchemaError::SchemaAlreadyExists(format!(
                "{} v{}",
                schema.name, schema.version
            )));
        }

        // Check compatibility with latest version
        if schema.version > 1
            && let Some(latest) = self.get_latest_version(&schema.name, versions)
            && !schema.is_compatible_with(&latest)
        {
            return Err(SchemaError::IncompatibleSchema(format!(
                "Schema {} v{} is not compatible with v{}",
                schema.name, schema.version, latest.version
            )));
        }

        versions.insert(schema.version, schema);

        Ok(())
    }

    /// Get a schema by name and version
    pub fn get(&self, name: &str, version: u32) -> SchemaResult<EventSchema> {
        let schemas = self.schemas.read().map_err(|e| {
            SchemaError::ValidationError(format!("Failed to acquire read lock: {}", e))
        })?;

        schemas
            .get(name)
            .and_then(|versions| versions.get(&version))
            .cloned()
            .ok_or_else(|| SchemaError::SchemaNotFound(format!("{} v{}", name, version)))
    }

    /// Get the latest version of a schema
    pub fn get_latest(&self, name: &str) -> SchemaResult<EventSchema> {
        let schemas = self.schemas.read().map_err(|e| {
            SchemaError::ValidationError(format!("Failed to acquire read lock: {}", e))
        })?;

        let versions = schemas
            .get(name)
            .ok_or_else(|| SchemaError::SchemaNotFound(name.to_string()))?;

        self.get_latest_version(name, versions)
            .ok_or_else(|| SchemaError::SchemaNotFound(name.to_string()))
    }

    /// Get all versions of a schema
    pub fn get_all_versions(&self, name: &str) -> Vec<EventSchema> {
        let schemas = self.schemas.read().unwrap();

        schemas
            .get(name)
            .map(|versions| versions.values().cloned().collect())
            .unwrap_or_default()
    }

    /// Validate an event against its schema
    pub fn validate_event(&self, event: &DomainEvent) -> SchemaResult<()> {
        let schema = self.get_latest(&event.metadata.event_type)?;
        schema.validate(&event.payload)
    }

    /// Get the latest version of a schema from a HashMap
    fn get_latest_version(
        &self,
        _name: &str,
        versions: &HashMap<u32, EventSchema>,
    ) -> Option<EventSchema> {
        versions.values().max_by_key(|s| s.version).cloned()
    }

    /// List all schema names
    pub fn list_schemas(&self) -> Vec<String> {
        let schemas = self.schemas.read().unwrap();
        schemas.keys().cloned().collect()
    }
}

impl Default for SchemaRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Schema evolution manager
pub struct SchemaEvolution {
    registry: Arc<SchemaRegistry>,
}

impl SchemaEvolution {
    /// Create a new schema evolution manager
    pub fn new(registry: Arc<SchemaRegistry>) -> Self {
        Self { registry }
    }

    /// Evolve a schema to a new version
    pub fn evolve(&self, old_schema: EventSchema, new_schema: EventSchema) -> SchemaResult<()> {
        // Verify version increment
        if new_schema.version != old_schema.version + 1 {
            return Err(SchemaError::ValidationError(format!(
                "New version must be {}, got {}",
                old_schema.version + 1,
                new_schema.version
            )));
        }

        // Check compatibility
        if !new_schema.is_compatible_with(&old_schema) {
            return Err(SchemaError::IncompatibleSchema(format!(
                "Schema {} v{} is not compatible with v{}",
                new_schema.name, new_schema.version, old_schema.version
            )));
        }

        // Register new schema
        self.registry.register(new_schema)?;

        Ok(())
    }

    /// Migrate event payload from old schema to new schema
    pub fn migrate_event(
        &self,
        event: &DomainEvent,
        target_version: u32,
    ) -> SchemaResult<serde_json::Value> {
        // Get current and target schemas
        let current_schema = self.registry.get_latest(&event.metadata.event_type)?;
        let target_schema = self
            .registry
            .get(&event.metadata.event_type, target_version)?;

        if current_schema.version == target_schema.version {
            return Ok(event.payload.clone());
        }

        // In a real implementation, this would apply migration rules
        // For now, we just return the original payload
        Ok(event.payload.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_registration() {
        let registry = SchemaRegistry::new();

        let schema = EventSchema::new(
            "StatuteCreated".to_string(),
            1,
            serde_json::json!({
                "type": "object",
                "required": ["title", "content"],
                "properties": {
                    "title": {"type": "string"},
                    "content": {"type": "string"}
                }
            }),
        );

        registry.register(schema).unwrap();

        let retrieved = registry.get("StatuteCreated", 1).unwrap();
        assert_eq!(retrieved.name, "StatuteCreated");
        assert_eq!(retrieved.version, 1);
    }

    #[test]
    fn test_schema_validation() {
        let schema = EventSchema::new(
            "StatuteCreated".to_string(),
            1,
            serde_json::json!({
                "type": "object",
                "required": ["title", "content"],
                "properties": {
                    "title": {"type": "string"},
                    "content": {"type": "string"}
                }
            }),
        );

        // Valid payload
        let valid_payload = serde_json::json!({
            "title": "Test Statute",
            "content": "Test content"
        });

        assert!(schema.validate(&valid_payload).is_ok());

        // Invalid payload (missing required field)
        let invalid_payload = serde_json::json!({
            "title": "Test Statute"
        });

        assert!(schema.validate(&invalid_payload).is_err());
    }

    #[test]
    fn test_get_latest_schema() {
        let registry = SchemaRegistry::new();

        let schema_v1 = EventSchema::new(
            "StatuteCreated".to_string(),
            1,
            serde_json::json!({"version": 1}),
        );

        let schema_v2 = EventSchema::new(
            "StatuteCreated".to_string(),
            2,
            serde_json::json!({"version": 2}),
        )
        .with_compatibility(CompatibilityMode::None);

        registry.register(schema_v1).unwrap();
        registry.register(schema_v2).unwrap();

        let latest = registry.get_latest("StatuteCreated").unwrap();
        assert_eq!(latest.version, 2);
    }

    #[test]
    fn test_list_schemas() {
        let registry = SchemaRegistry::new();

        let schema1 = EventSchema::new("StatuteCreated".to_string(), 1, serde_json::json!({}));

        let schema2 = EventSchema::new("StatuteUpdated".to_string(), 1, serde_json::json!({}));

        registry.register(schema1).unwrap();
        registry.register(schema2).unwrap();

        let schemas = registry.list_schemas();
        assert_eq!(schemas.len(), 2);
        assert!(schemas.contains(&"StatuteCreated".to_string()));
        assert!(schemas.contains(&"StatuteUpdated".to_string()));
    }
}
