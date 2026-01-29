//! Example demonstrating SDK generation from OpenAPI specification.
//!
//! This example shows how to:
//! - Parse OpenAPI specifications
//! - Configure SDK generation for different languages
//! - Generate TypeScript and Python SDKs
//! - Access generated files and package metadata

use legalis_api::sdk_generator::{
    AuthMethod, RetryConfig, SdkConfig, SdkLanguage, generate_sdk, parse_openapi_spec,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== SDK Generation Example ===\n");

    // Get OpenAPI spec from the API
    let spec_json = legalis_api::openapi::generate_spec();
    let spec = parse_openapi_spec(&spec_json)?;

    println!("Parsed OpenAPI spec:");
    println!("  Title: {}", spec.info.title);
    println!("  Version: {}", spec.info.version);
    println!("  Endpoints: {}\n", spec.paths.len());

    // Configure TypeScript SDK generation
    println!("Generating TypeScript SDK...");
    let ts_config = SdkConfig {
        language: SdkLanguage::TypeScript,
        package_name: "@legalis/sdk".to_string(),
        version: "0.1.0".to_string(),
        base_url: "http://localhost:3000".to_string(),
        auth_method: Some(AuthMethod::JWT {
            header_name: "Authorization".to_string(),
        }),
        retry_config: RetryConfig {
            max_retries: 3,
            initial_delay_ms: 100,
            max_delay_ms: 5000,
            backoff_multiplier: 2.0,
            retry_on_status_codes: vec![429, 500, 502, 503, 504],
        },
        timeout_seconds: 30,
        enable_streaming: true,
        generate_tests: true,
        generate_docs: true,
        output_dir: "./typescript-sdk".to_string(),
    };

    let ts_sdk = generate_sdk(&spec, &ts_config)?;
    println!("TypeScript SDK generated successfully!");
    println!("  Package: {}", ts_sdk.package_metadata.name);
    println!("  Version: {}", ts_sdk.package_metadata.version);
    println!("  Files generated: {}", ts_sdk.files.len());
    println!("  Files:");
    for file_path in ts_sdk.files.keys() {
        println!("    - {}", file_path);
    }
    println!();

    // Configure Python SDK generation
    println!("Generating Python SDK...");
    let py_config = SdkConfig {
        language: SdkLanguage::Python,
        package_name: "legalis-sdk".to_string(),
        version: "0.1.0".to_string(),
        base_url: "http://localhost:3000".to_string(),
        auth_method: Some(AuthMethod::OAuth2 {
            token_url: "https://auth.example.com/token".to_string(),
            scopes: vec!["read".to_string(), "write".to_string()],
        }),
        retry_config: RetryConfig::default(),
        timeout_seconds: 30,
        enable_streaming: true,
        generate_tests: true,
        generate_docs: true,
        output_dir: "./python-sdk".to_string(),
    };

    let py_sdk = generate_sdk(&spec, &py_config)?;
    println!("Python SDK generated successfully!");
    println!("  Package: {}", py_sdk.package_metadata.name);
    println!("  Version: {}", py_sdk.package_metadata.version);
    println!("  Files generated: {}", py_sdk.files.len());
    println!("  Files:");
    for file_path in py_sdk.files.keys() {
        println!("    - {}", file_path);
    }
    println!();

    // Show sample generated code
    println!("=== Sample TypeScript Client Code ===");
    if let Some(client_code) = ts_sdk.files.get("src/client.ts") {
        // Show first 20 lines
        let lines: Vec<&str> = client_code.lines().take(20).collect();
        for line in lines {
            println!("{}", line);
        }
        println!("...\n");
    }

    println!("=== Sample Python Client Code ===");
    if let Some(client_code) = py_sdk.files.get("legalis-sdk/client.py") {
        // Show first 20 lines
        let lines: Vec<&str> = client_code.lines().take(20).collect();
        for line in lines {
            println!("{}", line);
        }
        println!("...\n");
    }

    println!("=== SDK Generation Complete ===");
    println!("Generated SDKs can be found in:");
    println!("  TypeScript: {}", ts_config.output_dir);
    println!("  Python: {}", py_config.output_dir);
    println!("\nNext steps:");
    println!("  1. Review generated code");
    println!("  2. Run tests: npm test (TypeScript) or pytest (Python)");
    println!("  3. Build: npm run build (TypeScript) or python setup.py sdist (Python)");
    println!("  4. Publish: npm publish (TypeScript) or twine upload dist/* (Python)");

    Ok(())
}
