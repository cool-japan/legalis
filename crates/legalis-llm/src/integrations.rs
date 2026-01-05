//! Integration layer for popular LLM frameworks.
//!
//! This module provides compatibility layers and adapters for:
//! - LangChain (Python/JS LLM framework)
//! - LlamaIndex (data framework for LLM applications)
//! - Haystack (NLP framework with LLM support)
//! - Semantic Kernel (Microsoft's AI SDK)
//! - Vercel AI SDK (React/Next.js streaming AI)

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// LangChain-compatible message format.
///
/// LangChain uses a message-based conversation model with roles
/// (system, human, ai, function).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LangChainMessage {
    /// Message role (system, human, ai, function)
    #[serde(rename = "type")]
    pub role: String,
    /// Message content
    pub content: String,
    /// Additional data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_kwargs: Option<HashMap<String, serde_json::Value>>,
}

impl LangChainMessage {
    /// Creates a system message.
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".to_string(),
            content: content.into(),
            additional_kwargs: None,
        }
    }

    /// Creates a human message.
    pub fn human(content: impl Into<String>) -> Self {
        Self {
            role: "human".to_string(),
            content: content.into(),
            additional_kwargs: None,
        }
    }

    /// Creates an AI message.
    pub fn ai(content: impl Into<String>) -> Self {
        Self {
            role: "ai".to_string(),
            content: content.into(),
            additional_kwargs: None,
        }
    }

    /// Creates a function message.
    pub fn function(content: impl Into<String>, name: String) -> Self {
        let mut kwargs = HashMap::new();
        kwargs.insert("name".to_string(), serde_json::Value::String(name));

        Self {
            role: "function".to_string(),
            content: content.into(),
            additional_kwargs: Some(kwargs),
        }
    }

    /// Adds additional metadata.
    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.additional_kwargs
            .get_or_insert_with(HashMap::new)
            .insert(key, value);
        self
    }
}

/// LangChain-compatible prompt template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LangChainPromptTemplate {
    /// Template string with {variable} placeholders
    pub template: String,
    /// Input variables
    pub input_variables: Vec<String>,
    /// Template format (f-string, jinja2, etc.)
    #[serde(default = "default_template_format")]
    pub template_format: String,
}

fn default_template_format() -> String {
    "f-string".to_string()
}

impl LangChainPromptTemplate {
    /// Creates a new prompt template.
    pub fn new(template: impl Into<String>, variables: Vec<String>) -> Self {
        Self {
            template: template.into(),
            input_variables: variables,
            template_format: "f-string".to_string(),
        }
    }

    /// Formats the template with the given values.
    pub fn format(&self, values: &HashMap<String, String>) -> Result<String> {
        let mut result = self.template.clone();

        for var in &self.input_variables {
            let value = values
                .get(var)
                .ok_or_else(|| anyhow!("Missing variable: {}", var))?;
            result = result.replace(&format!("{{{}}}", var), value);
        }

        Ok(result)
    }

    /// Exports to LangChain JSON format.
    pub fn to_langchain_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(Into::into)
    }

    /// Imports from LangChain JSON format.
    pub fn from_langchain_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(Into::into)
    }
}

/// LlamaIndex-compatible document format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlamaIndexDocument {
    /// Document ID
    pub doc_id: String,
    /// Document text
    pub text: String,
    /// Embedding vector (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding: Option<Vec<f32>>,
    /// Metadata
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
    /// Relationships to other documents
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub relationships: HashMap<String, String>,
}

impl LlamaIndexDocument {
    /// Creates a new document.
    pub fn new(doc_id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            doc_id: doc_id.into(),
            text: text.into(),
            embedding: None,
            metadata: HashMap::new(),
            relationships: HashMap::new(),
        }
    }

    /// Adds an embedding.
    pub fn with_embedding(mut self, embedding: Vec<f32>) -> Self {
        self.embedding = Some(embedding);
        self
    }

    /// Adds metadata.
    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Adds a relationship to another document.
    pub fn with_relationship(mut self, rel_type: String, target_id: String) -> Self {
        self.relationships.insert(rel_type, target_id);
        self
    }

    /// Exports to LlamaIndex JSON format.
    pub fn to_llamaindex_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(Into::into)
    }

    /// Imports from LlamaIndex JSON format.
    pub fn from_llamaindex_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(Into::into)
    }
}

/// Haystack-compatible document format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HaystackDocument {
    /// Document content
    pub content: String,
    /// Content type (text, table, image, etc.)
    pub content_type: String,
    /// Document ID
    pub id: String,
    /// Metadata
    #[serde(default)]
    pub meta: HashMap<String, serde_json::Value>,
    /// Score (for search results)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
    /// Embedding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding: Option<Vec<f32>>,
}

impl HaystackDocument {
    /// Creates a new text document.
    pub fn new(id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            content_type: "text".to_string(),
            id: id.into(),
            meta: HashMap::new(),
            score: None,
            embedding: None,
        }
    }

    /// Sets the content type.
    pub fn with_content_type(mut self, content_type: String) -> Self {
        self.content_type = content_type;
        self
    }

    /// Adds metadata.
    pub fn with_meta(mut self, key: String, value: serde_json::Value) -> Self {
        self.meta.insert(key, value);
        self
    }

    /// Sets the score.
    pub fn with_score(mut self, score: f32) -> Self {
        self.score = Some(score);
        self
    }

    /// Sets the embedding.
    pub fn with_embedding(mut self, embedding: Vec<f32>) -> Self {
        self.embedding = Some(embedding);
        self
    }

    /// Exports to Haystack JSON format.
    pub fn to_haystack_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(Into::into)
    }

    /// Imports from Haystack JSON format.
    pub fn from_haystack_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(Into::into)
    }
}

/// Semantic Kernel-compatible function definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticKernelFunction {
    /// Function name
    pub name: String,
    /// Function description
    pub description: String,
    /// Input parameters
    pub parameters: Vec<SemanticKernelParameter>,
    /// Output description
    pub output: String,
}

impl SemanticKernelFunction {
    /// Creates a new function definition.
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        output: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            parameters: Vec::new(),
            output: output.into(),
        }
    }

    /// Adds a parameter.
    pub fn with_parameter(mut self, param: SemanticKernelParameter) -> Self {
        self.parameters.push(param);
        self
    }

    /// Exports to Semantic Kernel JSON format.
    pub fn to_sk_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(Into::into)
    }

    /// Imports from Semantic Kernel JSON format.
    pub fn from_sk_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(Into::into)
    }
}

/// Semantic Kernel parameter definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticKernelParameter {
    /// Parameter name
    pub name: String,
    /// Parameter description
    pub description: String,
    /// Default value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// Whether required
    #[serde(default)]
    pub required: bool,
}

impl SemanticKernelParameter {
    /// Creates a new parameter.
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            default_value: None,
            required: true,
        }
    }

    /// Sets the default value.
    pub fn with_default(mut self, default: impl Into<String>) -> Self {
        self.default_value = Some(default.into());
        self.required = false;
        self
    }

    /// Sets whether the parameter is required.
    pub fn with_required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }
}

/// Vercel AI SDK-compatible streaming format.
///
/// The Vercel AI SDK expects streaming responses in a specific format
/// for use with React/Next.js applications.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VercelAIStreamChunk {
    /// Chunk type (text, function_call, error, done)
    #[serde(rename = "type")]
    pub chunk_type: String,
    /// Text content (for text chunks)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Function call data (for function_call chunks)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<VercelFunctionCall>,
    /// Error message (for error chunks)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl VercelAIStreamChunk {
    /// Creates a text chunk.
    pub fn text(content: impl Into<String>) -> Self {
        Self {
            chunk_type: "text".to_string(),
            text: Some(content.into()),
            function_call: None,
            error: None,
        }
    }

    /// Creates a function call chunk.
    pub fn function_call(name: String, arguments: serde_json::Value) -> Self {
        Self {
            chunk_type: "function_call".to_string(),
            text: None,
            function_call: Some(VercelFunctionCall { name, arguments }),
            error: None,
        }
    }

    /// Creates an error chunk.
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            chunk_type: "error".to_string(),
            text: None,
            function_call: None,
            error: Some(message.into()),
        }
    }

    /// Creates a done chunk.
    pub fn done() -> Self {
        Self {
            chunk_type: "done".to_string(),
            text: None,
            function_call: None,
            error: None,
        }
    }

    /// Converts to Vercel AI SDK streaming format (newline-delimited JSON).
    pub fn to_stream_format(&self) -> Result<String> {
        let json = serde_json::to_string(self)?;
        Ok(format!("{}\n", json))
    }
}

/// Vercel AI function call data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VercelFunctionCall {
    /// Function name
    pub name: String,
    /// Function arguments (JSON)
    pub arguments: serde_json::Value,
}

/// Integration utility functions.
pub mod integration_utils {
    use super::*;
    use futures::stream::StreamExt;

    /// Converts our conversation messages to LangChain format.
    #[allow(dead_code)]
    pub fn to_langchain_messages(messages: &[(String, String)]) -> Vec<LangChainMessage> {
        messages
            .iter()
            .enumerate()
            .map(|(i, (role, content))| {
                if role == "system" {
                    LangChainMessage::system(content)
                } else if i % 2 == 0 {
                    LangChainMessage::human(content)
                } else {
                    LangChainMessage::ai(content)
                }
            })
            .collect()
    }

    /// Converts a stream to Vercel AI SDK format.
    #[allow(dead_code)]
    pub async fn stream_to_vercel_format(mut stream: crate::TextStream) -> Result<Vec<String>> {
        let mut chunks = Vec::new();

        while let Some(result) = stream.next().await {
            match result {
                Ok(chunk) => {
                    let vercel_chunk = VercelAIStreamChunk::text(&chunk.content);
                    chunks.push(vercel_chunk.to_stream_format()?);

                    if chunk.is_final {
                        chunks.push(VercelAIStreamChunk::done().to_stream_format()?);
                    }
                }
                Err(e) => {
                    let error_chunk = VercelAIStreamChunk::error(e.to_string());
                    chunks.push(error_chunk.to_stream_format()?);
                    break;
                }
            }
        }

        Ok(chunks)
    }

    /// Converts our document format to LlamaIndex format.
    #[allow(dead_code)]
    pub fn to_llamaindex_document(
        id: String,
        text: String,
        metadata: HashMap<String, serde_json::Value>,
    ) -> LlamaIndexDocument {
        let mut doc = LlamaIndexDocument::new(id, text);
        for (key, value) in metadata {
            doc = doc.with_metadata(key, value);
        }
        doc
    }

    /// Converts our document format to Haystack format.
    #[allow(dead_code)]
    pub fn to_haystack_document(
        id: String,
        content: String,
        metadata: HashMap<String, serde_json::Value>,
    ) -> HaystackDocument {
        let mut doc = HaystackDocument::new(id, content);
        for (key, value) in metadata {
            doc = doc.with_meta(key, value);
        }
        doc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_langchain_message() {
        let msg = LangChainMessage::system("You are a helpful assistant");
        assert_eq!(msg.role, "system");
        assert_eq!(msg.content, "You are a helpful assistant");

        let msg = LangChainMessage::human("Hello");
        assert_eq!(msg.role, "human");

        let msg = LangChainMessage::ai("Hi there!");
        assert_eq!(msg.role, "ai");

        let msg = LangChainMessage::function("result", "calculate".to_string());
        assert_eq!(msg.role, "function");
    }

    #[test]
    fn test_langchain_prompt_template() {
        let template = LangChainPromptTemplate::new(
            "Hello {name}, you are {age} years old.",
            vec!["name".to_string(), "age".to_string()],
        );

        let mut values = HashMap::new();
        values.insert("name".to_string(), "Alice".to_string());
        values.insert("age".to_string(), "30".to_string());

        let result = template.format(&values).unwrap();
        assert_eq!(result, "Hello Alice, you are 30 years old.");
    }

    #[test]
    fn test_langchain_template_missing_variable() {
        let template = LangChainPromptTemplate::new("Hello {name}", vec!["name".to_string()]);

        let values = HashMap::new();
        let result = template.format(&values);
        assert!(result.is_err());
    }

    #[test]
    fn test_llamaindex_document() {
        let doc = LlamaIndexDocument::new("doc1", "This is a test document")
            .with_metadata("author".to_string(), serde_json::json!("Alice"))
            .with_relationship("parent".to_string(), "doc0".to_string())
            .with_embedding(vec![0.1, 0.2, 0.3]);

        assert_eq!(doc.doc_id, "doc1");
        assert_eq!(doc.text, "This is a test document");
        assert!(doc.embedding.is_some());
        assert_eq!(
            doc.metadata.get("author").unwrap(),
            &serde_json::json!("Alice")
        );
        assert_eq!(doc.relationships.get("parent").unwrap(), "doc0");
    }

    #[test]
    fn test_llamaindex_json_roundtrip() {
        let doc = LlamaIndexDocument::new("doc1", "Test");
        let json = doc.to_llamaindex_json().unwrap();
        let doc2 = LlamaIndexDocument::from_llamaindex_json(&json).unwrap();
        assert_eq!(doc.doc_id, doc2.doc_id);
        assert_eq!(doc.text, doc2.text);
    }

    #[test]
    fn test_haystack_document() {
        let doc = HaystackDocument::new("doc1", "Test content")
            .with_content_type("text".to_string())
            .with_meta("source".to_string(), serde_json::json!("test.txt"))
            .with_score(0.95)
            .with_embedding(vec![0.1, 0.2]);

        assert_eq!(doc.id, "doc1");
        assert_eq!(doc.content, "Test content");
        assert_eq!(doc.score, Some(0.95));
        assert!(doc.embedding.is_some());
    }

    #[test]
    fn test_haystack_json_roundtrip() {
        let doc = HaystackDocument::new("doc1", "Test");
        let json = doc.to_haystack_json().unwrap();
        let doc2 = HaystackDocument::from_haystack_json(&json).unwrap();
        assert_eq!(doc.id, doc2.id);
        assert_eq!(doc.content, doc2.content);
    }

    #[test]
    fn test_semantic_kernel_function() {
        let func = SemanticKernelFunction::new(
            "calculate_sum",
            "Calculates the sum of two numbers",
            "The sum as a number",
        )
        .with_parameter(SemanticKernelParameter::new("a", "First number"))
        .with_parameter(SemanticKernelParameter::new("b", "Second number").with_default("0"));

        assert_eq!(func.name, "calculate_sum");
        assert_eq!(func.parameters.len(), 2);
        assert_eq!(func.parameters[1].default_value, Some("0".to_string()));
    }

    #[test]
    fn test_semantic_kernel_json_roundtrip() {
        let func = SemanticKernelFunction::new("test", "Test function", "Result");
        let json = func.to_sk_json().unwrap();
        let func2 = SemanticKernelFunction::from_sk_json(&json).unwrap();
        assert_eq!(func.name, func2.name);
    }

    #[test]
    fn test_vercel_stream_chunk() {
        let chunk = VercelAIStreamChunk::text("Hello");
        assert_eq!(chunk.chunk_type, "text");
        assert_eq!(chunk.text, Some("Hello".to_string()));

        let chunk = VercelAIStreamChunk::error("Error occurred");
        assert_eq!(chunk.chunk_type, "error");
        assert!(chunk.error.is_some());

        let chunk = VercelAIStreamChunk::done();
        assert_eq!(chunk.chunk_type, "done");
    }

    #[test]
    fn test_vercel_stream_format() {
        let chunk = VercelAIStreamChunk::text("Hello");
        let formatted = chunk.to_stream_format().unwrap();
        assert!(formatted.ends_with('\n'));
        assert!(formatted.contains("\"type\":\"text\""));
    }

    #[test]
    fn test_utils_to_langchain_messages() {
        let messages = vec![
            ("system".to_string(), "You are helpful".to_string()),
            ("user".to_string(), "Hello".to_string()),
            ("assistant".to_string(), "Hi there!".to_string()),
        ];

        let lc_messages = integration_utils::to_langchain_messages(&messages);
        assert_eq!(lc_messages.len(), 3);
        assert_eq!(lc_messages[0].role, "system");
    }

    #[test]
    fn test_utils_document_conversion() {
        let mut metadata = HashMap::new();
        metadata.insert("key".to_string(), serde_json::json!("value"));

        let llamaindex_doc = integration_utils::to_llamaindex_document(
            "doc1".to_string(),
            "Test".to_string(),
            metadata.clone(),
        );
        assert_eq!(llamaindex_doc.doc_id, "doc1");

        let haystack_doc = integration_utils::to_haystack_document(
            "doc1".to_string(),
            "Test".to_string(),
            metadata,
        );
        assert_eq!(haystack_doc.id, "doc1");
    }
}
