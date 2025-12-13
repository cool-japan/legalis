# legalis-registry

Statute registry and collection management for Legalis-RS.

## Overview

`legalis-registry` provides an in-memory storage system for managing collections of legal statutes with versioning, tagging, and jurisdiction-based organization.

## Features

- **In-Memory Storage**: Fast statute storage with full CRUD operations
- **Versioning**: Track multiple versions of the same statute
- **Tag-Based Organization**: Organize statutes with flexible tagging
- **Jurisdiction Indexing**: Quick lookup by jurisdiction
- **Search**: Find statutes by ID, tags, or jurisdiction

## Usage

```rust
use legalis_registry::{StatuteRegistry, RegistryEntry};
use legalis_core::{Statute, Effect, EffectType};

// Create a registry
let mut registry = StatuteRegistry::new();

// Add a statute
let statute = Statute::new(
    "adult-rights",
    "Adult Rights",
    Effect::new(EffectType::Grant, "Full legal capacity"),
);

registry.add(statute.clone())?;

// Query by jurisdiction
let us_statutes = registry.find_by_jurisdiction("US")?;

// Query by tag
let civil_statutes = registry.find_by_tag("civil-rights")?;

// Get specific version
let statute_v1 = registry.get_version("adult-rights", 1)?;
```

## API

### StatuteRegistry

| Method | Description |
|--------|-------------|
| `new()` | Create empty registry |
| `add(statute)` | Add a statute |
| `get(id)` | Get latest version |
| `get_version(id, version)` | Get specific version |
| `find_by_jurisdiction(jurisdiction)` | Find by jurisdiction |
| `find_by_tag(tag)` | Find by tag |
| `list_all()` | List all statutes |
| `remove(id)` | Remove a statute |

## License

MIT OR Apache-2.0
