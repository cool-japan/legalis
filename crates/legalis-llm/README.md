# legalis-llm

LLM integration layer for Legalis-RS.

## Overview

This crate provides an abstraction layer for LLM (Large Language Model) providers, enabling pluggable AI models for natural language processing of legal documents. It supports OpenAI, Anthropic, and mock providers for testing.

## Architecture

The `LLMProvider` trait allows swapping between different LLM providers without changing application logic:

```rust
#[async_trait]
pub trait LLMProvider: Send + Sync {
    async fn generate_text(&self, prompt: &str) -> Result<String>;
    async fn generate_structured<T: DeserializeOwned + Send>(&self, prompt: &str) -> Result<T>;
    fn provider_name(&self) -> &str;
    fn model_name(&self) -> &str;
}
```

## Providers

### OpenAI

```rust
use legalis_llm::{OpenAiClient, LLMConfig};

let client = OpenAiClient::new("your-api-key", "gpt-4")
    .with_config(LLMConfig::new()
        .with_temperature(0.3)
        .with_max_tokens(2048));

let response = client.generate_text("Parse this legal text...").await?;
```

### Anthropic (Claude)

```rust
use legalis_llm::AnthropicClient;

let client = AnthropicClient::new("your-api-key", "claude-3-opus");
let response = client.generate_text("Analyze this statute...").await?;
```

### OpenAI-Compatible APIs

```rust
let client = OpenAiClient::new("your-api-key", "model-name")
    .with_base_url("https://your-compatible-api.com/v1");
```

### Mock Provider (for Testing)

```rust
use legalis_llm::MockProvider;

let provider = MockProvider::new()
    .with_response("parse", r#"{"id": "test", "title": "Test"}"#);

let result: Statute = provider.generate_structured("parse this").await?;
```

## Law Compiler

The `LawCompiler` transforms natural language legal text into structured `Statute` objects:

```rust
use legalis_llm::{LawCompiler, OpenAiClient};

let client = OpenAiClient::new("api-key", "gpt-4");
let compiler = LawCompiler::new(client);

// Compile natural language to structured statute
let statute = compiler.compile(
    "Any person who has reached the age of 18 years shall have the right to vote."
).await?;

// Analyze a statute for issues
let report = compiler.analyze(&statute).await?;
println!("Issues found: {:?}", report.issues);

// Generate human-readable explanation
let explanation = compiler.explain(&statute).await?;
```

## Analysis Report

```rust
pub struct AnalysisReport {
    pub issues: Vec<String>,           // Identified problems
    pub ambiguities: Vec<String>,      // Ambiguous terms
    pub recommendations: Vec<String>,  // Improvement suggestions
    pub discretion_points: Vec<String>, // Areas requiring human judgment
}
```

## Configuration

```rust
let config = LLMConfig::new()
    .with_max_tokens(4096)
    .with_temperature(0.7)
    .with_system_prompt("You are a legal analysis assistant.");
```

## JSON Extraction

The crate includes intelligent JSON extraction that handles:
- JSON in markdown code blocks (```json...```)
- Raw JSON embedded in text
- Nested JSON objects

## License

MIT OR Apache-2.0
