//! Statute Version Control Example
//!
//! This example demonstrates how to use `legalis-registry` for managing
//! statutes with full version control, snapshots, and backup capabilities.
//!
//! ## Features
//!
//! - **Version Control**: Track changes across statute revisions
//! - **Snapshots**: Point-in-time captures for rollback
//! - **Backups**: Full and incremental backup support
//! - **Search**: Tag-based, jurisdiction-based, and full-text search
//! - **Pagination**: Efficient handling of large statute collections

use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};
use legalis_registry::{Pagination, SearchQuery, StatuteEntry, StatuteRegistry, StatuteStatus};

fn create_initial_statute() -> Statute {
    Statute::new(
        "adult-age-act",
        "Adult Age Definition Act (Version 1)",
        Effect::new(EffectType::Grant, "Legal adult status at age 20"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 20,
    })
    .with_jurisdiction("JP")
}

fn create_amended_statute_v2() -> Statute {
    Statute::new(
        "adult-age-act",
        "Adult Age Definition Act (Version 2 - 2022 Amendment)",
        Effect::new(EffectType::Grant, "Legal adult status at age 18"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    })
    .with_jurisdiction("JP")
}

fn create_amended_statute_v3() -> Statute {
    Statute::new(
        "adult-age-act",
        "Adult Age Definition Act (Version 3 - Clarification)",
        Effect::new(
            EffectType::Grant,
            "Legal adult status at age 18; applies to all nationals",
        ),
    )
    .with_precondition(Condition::And(
        Box::new(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        }),
        Box::new(Condition::HasAttribute {
            key: "national".to_string(),
        }),
    ))
    .with_jurisdiction("JP")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "=".repeat(80));
    println!("   STATUTE VERSION CONTROL - Legalis-Registry Demo");
    println!("   法令バージョン管理システム");
    println!("{}", "=".repeat(80));
    println!();

    // =========================================================================
    // Step 1: Create Registry and Register Initial Statute
    // =========================================================================
    println!("Step 1: Create Registry and Register Initial Statute\n");

    let mut registry = StatuteRegistry::new();
    println!("   Created new StatuteRegistry");

    let initial_statute = create_initial_statute();
    let entry = StatuteEntry::new(initial_statute, "JP")
        .with_tag("civil-law")
        .with_tag("age-definition")
        .with_status(StatuteStatus::Active);

    let registry_id = registry.register(entry)?;
    println!(
        "   Registered: adult-age-act (Registry ID: {})",
        registry_id
    );
    println!("   Tags: civil-law, age-definition");
    println!("   Status: Active");
    println!();

    // =========================================================================
    // Step 2: Update Statute (Create New Versions)
    // =========================================================================
    println!("{}", "=".repeat(80));
    println!("Step 2: Update Statute (Version History)\n");

    // Version 2
    let v2_statute = create_amended_statute_v2();
    let v2 = registry.update("adult-age-act", v2_statute)?;
    println!("   Updated to Version {}: Adult age lowered to 18", v2);

    // Version 3
    let v3_statute = create_amended_statute_v3();
    let v3 = registry.update("adult-age-act", v3_statute)?;
    println!(
        "   Updated to Version {}: Added nationality requirement",
        v3
    );

    // List all versions
    let versions = registry.list_versions("adult-age-act");
    println!("\n   Version History: {:?}", versions);
    println!();

    // =========================================================================
    // Step 3: Retrieve Specific Versions
    // =========================================================================
    println!("{}", "=".repeat(80));
    println!("Step 3: Retrieve Specific Versions\n");

    for version in &versions {
        if let Ok(entry) = registry.get_version("adult-age-act", *version) {
            println!("   Version {}: {}", version, entry.statute.title);
            println!("      Effect: {}", entry.statute.effect.description);
        }
    }
    println!();

    // =========================================================================
    // Step 4: Add More Statutes for Search Demo
    // =========================================================================
    println!("{}", "=".repeat(80));
    println!("Step 4: Add Additional Statutes\n");

    let statutes_to_add = vec![
        (
            Statute::new(
                "voting-rights",
                "Voting Rights Act",
                Effect::new(EffectType::Grant, "Right to vote"),
            )
            .with_jurisdiction("JP"),
            vec!["civil-law", "electoral"],
        ),
        (
            Statute::new(
                "consumer-protection",
                "Consumer Protection Act",
                Effect::new(EffectType::Obligation, "Protect consumer rights"),
            )
            .with_jurisdiction("JP"),
            vec!["commercial-law", "consumer"],
        ),
        (
            Statute::new(
                "gdpr-compliance",
                "GDPR Compliance Regulation",
                Effect::new(EffectType::Obligation, "Data protection requirements"),
            )
            .with_jurisdiction("EU"),
            vec!["data-protection", "privacy"],
        ),
        (
            Statute::new(
                "environmental-act",
                "Environmental Protection Act",
                Effect::new(EffectType::Prohibition, "Prohibit harmful emissions"),
            )
            .with_jurisdiction("US"),
            vec!["environmental", "regulatory"],
        ),
    ];

    for (statute, tags) in statutes_to_add {
        let mut entry = StatuteEntry::new(
            statute.clone(),
            statute.jurisdiction.as_deref().unwrap_or("INTL"),
        )
        .with_status(StatuteStatus::Active);
        for tag in &tags {
            entry = entry.with_tag(*tag);
        }
        registry.register(entry)?;
        println!("   Registered: {} ({})", statute.title, statute.id);
    }
    println!("\n   Total statutes: {}", registry.count());
    println!();

    // =========================================================================
    // Step 5: Search by Tag and Jurisdiction
    // =========================================================================
    println!("{}", "=".repeat(80));
    println!("Step 5: Search Capabilities\n");

    // Search by tag
    println!("   Search by tag 'civil-law':");
    let civil_results = registry.query_by_tag("civil-law");
    for entry in civil_results {
        println!("      - {} ({})", entry.statute.title, entry.statute.id);
    }
    println!();

    // Search by jurisdiction
    println!("   Search by jurisdiction 'JP':");
    let jp_results = registry.query_by_jurisdiction("JP");
    for entry in jp_results {
        println!("      - {} ({})", entry.statute.title, entry.statute.id);
    }
    println!();

    // Advanced search with SearchQuery
    println!("   Advanced search (JP + Active):");
    let query = SearchQuery::new()
        .with_jurisdiction("JP")
        .with_status(StatuteStatus::Active);
    let search_results = registry.search(&query);
    for entry in search_results {
        println!("      - {} [{:?}]", entry.statute.title, entry.status);
    }
    println!();

    // =========================================================================
    // Step 6: Pagination
    // =========================================================================
    println!("{}", "=".repeat(80));
    println!("Step 6: Pagination Demo\n");

    let page1 = registry.list_paged(Pagination::new(0, 2));
    println!(
        "   Page 1 (offset=0, limit=2): {} of {} total",
        page1.items.len(),
        page1.total
    );
    for entry in &page1.items {
        println!("      - {}", entry.statute.title);
    }

    let page2 = registry.list_paged(Pagination::new(2, 2));
    println!(
        "\n   Page 2 (offset=2, limit=2): {} items",
        page2.items.len()
    );
    for entry in &page2.items {
        println!("      - {}", entry.statute.title);
    }
    println!();

    // =========================================================================
    // Step 7: Snapshots
    // =========================================================================
    println!("{}", "=".repeat(80));
    println!("Step 7: Snapshot Management\n");

    let snapshot = registry.create_snapshot(Some("Pre-amendment state".to_string()));
    println!("   Created snapshot: {}", snapshot.snapshot_id);
    println!("   Description: {:?}", snapshot.description);
    println!("   Statutes captured: {}", snapshot.backup.statutes.len());
    println!();

    // =========================================================================
    // Step 8: Backup Export/Import
    // =========================================================================
    println!("{}", "=".repeat(80));
    println!("Step 8: Backup System\n");

    let backup = registry.create_backup(Some("Full backup before release".to_string()));
    println!("   Created backup:");
    println!("      - Format version: {}", backup.metadata.format_version);
    println!("      - Statute count: {}", backup.metadata.statute_count);
    println!("      - Event count: {}", backup.metadata.event_count);

    // Export to JSON
    let json_backup = registry.export_backup(Some("JSON export".to_string()))?;
    println!("\n   Exported to JSON: {} bytes", json_backup.len());

    // Create new registry and import
    let mut new_registry = StatuteRegistry::new();
    new_registry.import_backup(&json_backup)?;
    println!(
        "   Imported into new registry: {} statutes",
        new_registry.count()
    );
    println!();

    // =========================================================================
    // Summary
    // =========================================================================
    println!("{}", "=".repeat(80));
    println!("   VERSION CONTROL SUMMARY");
    println!("{}", "=".repeat(80));
    println!();
    println!("   Core Capabilities:");
    println!("   | Feature            | Description                              |");
    println!("   |--------------------|------------------------------------------|");
    println!("   | Version History    | Track all changes to each statute        |");
    println!("   | ETag Concurrency   | Optimistic locking for updates           |");
    println!("   | Snapshots          | Point-in-time captures for rollback      |");
    println!("   | Full Backup        | Complete registry serialization          |");
    println!("   | Incremental Backup | Delta-based sync between registries      |");
    println!("   | Tag-based Search   | Organize statutes by topic               |");
    println!("   | Jurisdiction Query | Filter by legal jurisdiction             |");
    println!("   | Pagination         | Efficient large collection handling      |");
    println!();
    println!("   Use Cases:");
    println!("   - Legislative tracking and amendment history");
    println!("   - Multi-jurisdiction legal databases");
    println!("   - Regulatory compliance versioning");
    println!("   - Legal research repositories");
    println!();

    Ok(())
}
