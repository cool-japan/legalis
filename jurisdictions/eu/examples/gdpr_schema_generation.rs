//! JSON Schema Generation for GDPR Types
//!
//! This example demonstrates how to generate JSON schemas for GDPR types,
//! which can be used for API documentation, validation, and code generation.
//!
//! Run with: cargo run --example gdpr_schema_generation --features schema,serde

#[cfg(all(feature = "schema", feature = "serde"))]
use legalis_eu::gdpr::article6::DataProcessing;
#[cfg(all(feature = "schema", feature = "serde"))]
use legalis_eu::gdpr::types::{LawfulBasis, PersonalDataCategory, SpecialCategory};
#[cfg(all(feature = "schema", feature = "serde"))]
use schemars::schema_for;

#[cfg(all(feature = "schema", feature = "serde"))]
fn main() {
    println!("=== JSON Schema Generation for GDPR Types ===\n");

    // Generate schema for LawfulBasis
    println!("1. Schema for LawfulBasis (Article 6):");
    println!("{}", "=".repeat(60));
    let lawful_basis_schema = schema_for!(LawfulBasis);
    let schema_json = serde_json::to_string_pretty(&lawful_basis_schema).unwrap();
    println!("{}\n", schema_json);

    // Generate schema for SpecialCategory
    println!("2. Schema for SpecialCategory (Article 9):");
    println!("{}", "=".repeat(60));
    let special_category_schema = schema_for!(SpecialCategory);
    let schema_json = serde_json::to_string_pretty(&special_category_schema).unwrap();
    println!("{}\n", schema_json);

    // Generate schema for PersonalDataCategory
    println!("3. Schema for PersonalDataCategory:");
    println!("{}", "=".repeat(60));
    let data_category_schema = schema_for!(PersonalDataCategory);
    let schema_json = serde_json::to_string_pretty(&data_category_schema).unwrap();
    println!("{}\n", schema_json);

    // Generate schema for DataProcessing
    println!("4. Schema for DataProcessing:");
    println!("{}", "=".repeat(60));
    let data_processing_schema = schema_for!(DataProcessing);
    let schema_json = serde_json::to_string_pretty(&data_processing_schema).unwrap();
    println!("{}\n", schema_json);

    println!("\n=== Use Cases ===\n");
    println!("These schemas can be used for:");
    println!("  • API documentation generation (OpenAPI/Swagger)");
    println!("  • Client code generation (TypeScript, Python, etc.)");
    println!("  • Request/response validation in web frameworks");
    println!("  • Form generation for user interfaces");
    println!("  • Configuration file validation");
    println!("\nExample: OpenAPI Integration");
    println!("The generated schemas can be embedded in OpenAPI specifications:");
    println!(
        r#"
paths:
  /gdpr/processing:
    post:
      requestBody:
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/DataProcessing'
components:
  schemas:
    DataProcessing:
      [Generated schema from schema_for!(DataProcessing)]
"#
    );
}

#[cfg(not(all(feature = "schema", feature = "serde")))]
fn main() {
    eprintln!("This example requires both 'schema' and 'serde' features.");
    eprintln!("Run with: cargo run --example gdpr_schema_generation --features schema,serde");
    std::process::exit(1);
}
