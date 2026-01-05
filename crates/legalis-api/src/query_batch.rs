//! Automatic query batching for GraphQL.
//!
//! This module provides automatic batching of multiple GraphQL queries into a single
//! HTTP request, reducing network overhead and improving performance.

use async_graphql::{Request, Response, Schema};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// A batch of GraphQL queries to be executed together.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryBatch {
    /// The queries to execute
    pub queries: Vec<BatchedQuery>,
    /// Whether to execute queries in parallel (default: true)
    #[serde(default = "default_parallel")]
    pub parallel: bool,
    /// Maximum execution time for the entire batch (ms)
    #[serde(default)]
    pub timeout_ms: Option<u64>,
}

fn default_parallel() -> bool {
    true
}

/// A single query in a batch.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchedQuery {
    /// The GraphQL query string
    pub query: String,
    /// Optional operation name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    /// Optional variables
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<serde_json::Value>,
    /// Optional ID for tracking this query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Response to a batch query request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResponse {
    /// The responses, in the same order as the queries
    pub responses: Vec<QueryResponse>,
    /// Execution metrics
    pub metrics: BatchMetrics,
}

/// Response for a single query in a batch.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResponse {
    /// The data returned by the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    /// Any errors that occurred
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub errors: Vec<serde_json::Value>,
    /// Optional ID from the batched query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Metrics about batch execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchMetrics {
    /// Total number of queries
    pub total_queries: usize,
    /// Number of successful queries
    pub successful: usize,
    /// Number of failed queries
    pub failed: usize,
    /// Total execution time (ms)
    pub total_duration_ms: u64,
    /// Average execution time per query (ms)
    pub avg_duration_ms: u64,
    /// Whether execution was parallel
    pub parallel: bool,
}

/// Configuration for the query batcher.
#[derive(Debug, Clone)]
pub struct BatchConfig {
    /// Maximum number of queries allowed in a single batch
    pub max_batch_size: usize,
    /// Default timeout for batch execution (ms)
    pub default_timeout_ms: u64,
    /// Whether to allow parallel execution by default
    pub allow_parallel: bool,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            max_batch_size: 10,
            default_timeout_ms: 30000, // 30 seconds
            allow_parallel: true,
        }
    }
}

/// Query batcher for executing multiple GraphQL queries.
pub struct QueryBatcher {
    config: BatchConfig,
}

impl QueryBatcher {
    /// Create a new query batcher with default configuration.
    pub fn new() -> Self {
        Self {
            config: BatchConfig::default(),
        }
    }

    /// Create a new query batcher with custom configuration.
    pub fn with_config(config: BatchConfig) -> Self {
        Self { config }
    }

    /// Execute a batch of queries.
    pub async fn execute_batch<Q, M, S>(
        &self,
        schema: &Schema<Q, M, S>,
        batch: QueryBatch,
    ) -> Result<BatchResponse, String>
    where
        Q: async_graphql::ObjectType + 'static,
        M: async_graphql::ObjectType + 'static,
        S: async_graphql::SubscriptionType + 'static,
    {
        // Validate batch size
        if batch.queries.len() > self.config.max_batch_size {
            return Err(format!(
                "Batch size {} exceeds maximum {}",
                batch.queries.len(),
                self.config.max_batch_size
            ));
        }

        if batch.queries.is_empty() {
            return Err("Batch cannot be empty".to_string());
        }

        let start = Instant::now();
        let parallel = batch.parallel && self.config.allow_parallel;
        let timeout =
            Duration::from_millis(batch.timeout_ms.unwrap_or(self.config.default_timeout_ms));

        // Execute queries
        let responses = if parallel {
            self.execute_parallel(schema, batch.queries, timeout)
                .await?
        } else {
            self.execute_sequential(schema, batch.queries, timeout)
                .await?
        };

        let total_duration = start.elapsed();
        let successful = responses.iter().filter(|r| r.errors.is_empty()).count();
        let failed = responses.len() - successful;

        let metrics = BatchMetrics {
            total_queries: responses.len(),
            successful,
            failed,
            total_duration_ms: total_duration.as_millis() as u64,
            avg_duration_ms: (total_duration.as_millis() as u64) / (responses.len() as u64),
            parallel,
        };

        Ok(BatchResponse { responses, metrics })
    }

    /// Execute queries in parallel.
    async fn execute_parallel<Q, M, S>(
        &self,
        schema: &Schema<Q, M, S>,
        queries: Vec<BatchedQuery>,
        timeout: Duration,
    ) -> Result<Vec<QueryResponse>, String>
    where
        Q: async_graphql::ObjectType + 'static,
        M: async_graphql::ObjectType + 'static,
        S: async_graphql::SubscriptionType + 'static,
    {
        let futures: Vec<_> = queries
            .into_iter()
            .map(|q| {
                let schema = schema.clone();
                async move { execute_single_query(&schema, q).await }
            })
            .collect();

        // Execute all queries in parallel with timeout
        match tokio::time::timeout(timeout, futures::future::join_all(futures)).await {
            Ok(results) => Ok(results),
            Err(_) => Err("Batch execution timed out".to_string()),
        }
    }

    /// Execute queries sequentially.
    async fn execute_sequential<Q, M, S>(
        &self,
        schema: &Schema<Q, M, S>,
        queries: Vec<BatchedQuery>,
        timeout: Duration,
    ) -> Result<Vec<QueryResponse>, String>
    where
        Q: async_graphql::ObjectType + 'static,
        M: async_graphql::ObjectType + 'static,
        S: async_graphql::SubscriptionType + 'static,
    {
        let start = Instant::now();
        let mut responses = Vec::new();

        for query in queries {
            // Check timeout before each query
            if start.elapsed() > timeout {
                return Err("Batch execution timed out".to_string());
            }

            let response = execute_single_query(schema, query).await;
            responses.push(response);
        }

        Ok(responses)
    }
}

impl Default for QueryBatcher {
    fn default() -> Self {
        Self::new()
    }
}

/// Execute a single query and convert to QueryResponse.
async fn execute_single_query<Q, M, S>(
    schema: &Schema<Q, M, S>,
    batched_query: BatchedQuery,
) -> QueryResponse
where
    Q: async_graphql::ObjectType + 'static,
    M: async_graphql::ObjectType + 'static,
    S: async_graphql::SubscriptionType + 'static,
{
    let mut request = Request::new(batched_query.query);

    if let Some(operation_name) = batched_query.operation_name {
        request = request.operation_name(operation_name);
    }

    if let Some(variables) = batched_query.variables {
        if let Ok(vars) = serde_json::from_value(variables) {
            request = request.variables(vars);
        }
    }

    let response: Response = schema.execute(request).await;

    QueryResponse {
        data: response.data.into_json().ok(),
        errors: response
            .errors
            .into_iter()
            .map(|e| {
                serde_json::json!({
                    "message": e.message,
                    "locations": e.locations,
                    "path": e.path,
                })
            })
            .collect(),
        id: batched_query.id,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_config_default() {
        let config = BatchConfig::default();
        assert_eq!(config.max_batch_size, 10);
        assert_eq!(config.default_timeout_ms, 30000);
        assert!(config.allow_parallel);
    }

    #[test]
    fn test_query_batcher_creation() {
        let batcher = QueryBatcher::new();
        assert_eq!(batcher.config.max_batch_size, 10);
    }

    #[test]
    fn test_batched_query_serialization() {
        let query = BatchedQuery {
            query: "{ test }".to_string(),
            operation_name: Some("TestOp".to_string()),
            variables: Some(serde_json::json!({"key": "value"})),
            id: Some("query-1".to_string()),
        };

        let json = serde_json::to_string(&query).unwrap();
        assert!(json.contains("test"));
        assert!(json.contains("TestOp"));
    }

    #[test]
    fn test_default_parallel() {
        assert!(default_parallel());
    }

    #[tokio::test]
    async fn test_execute_batch_empty() {
        use crate::graphql::GraphQLState;
        use crate::graphql::create_schema;

        let state = GraphQLState::new();
        let schema = create_schema(state);
        let batcher = QueryBatcher::new();

        let batch = QueryBatch {
            queries: vec![],
            parallel: true,
            timeout_ms: None,
        };

        let result = batcher.execute_batch(&schema, batch).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty"));
    }

    #[tokio::test]
    async fn test_execute_batch_exceeds_max() {
        use crate::graphql::GraphQLState;
        use crate::graphql::create_schema;

        let state = GraphQLState::new();
        let schema = create_schema(state);
        let batcher = QueryBatcher::new();

        let queries = vec![
            BatchedQuery {
                query: "{ statuteCount }".to_string(),
                operation_name: None,
                variables: None,
                id: None,
            };
            15 // More than max_batch_size (10)
        ];

        let batch = QueryBatch {
            queries,
            parallel: true,
            timeout_ms: None,
        };

        let result = batcher.execute_batch(&schema, batch).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds maximum"));
    }

    #[tokio::test]
    async fn test_execute_batch_simple() {
        use crate::graphql::GraphQLState;
        use crate::graphql::create_schema;

        let state = GraphQLState::new();
        let schema = create_schema(state);
        let batcher = QueryBatcher::new();

        let batch = QueryBatch {
            queries: vec![
                BatchedQuery {
                    query: "{ statuteCount }".to_string(),
                    operation_name: None,
                    variables: None,
                    id: Some("q1".to_string()),
                },
                BatchedQuery {
                    query: "{ statuteCount }".to_string(),
                    operation_name: None,
                    variables: None,
                    id: Some("q2".to_string()),
                },
            ],
            parallel: true,
            timeout_ms: Some(5000),
        };

        let result = batcher.execute_batch(&schema, batch).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.responses.len(), 2);
        assert_eq!(response.metrics.total_queries, 2);
        assert_eq!(response.metrics.successful, 2);
        assert_eq!(response.metrics.failed, 0);
        assert!(response.metrics.parallel);
    }

    #[tokio::test]
    async fn test_execute_batch_sequential() {
        use crate::graphql::GraphQLState;
        use crate::graphql::create_schema;

        let state = GraphQLState::new();
        let schema = create_schema(state);
        let batcher = QueryBatcher::new();

        let batch = QueryBatch {
            queries: vec![
                BatchedQuery {
                    query: "{ statuteCount }".to_string(),
                    operation_name: None,
                    variables: None,
                    id: Some("q1".to_string()),
                },
                BatchedQuery {
                    query: "{ statuteCount }".to_string(),
                    operation_name: None,
                    variables: None,
                    id: Some("q2".to_string()),
                },
            ],
            parallel: false,
            timeout_ms: Some(5000),
        };

        let result = batcher.execute_batch(&schema, batch).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.responses.len(), 2);
        assert!(!response.metrics.parallel);
    }

    #[tokio::test]
    async fn test_query_response_with_id() {
        use crate::graphql::GraphQLState;
        use crate::graphql::create_schema;

        let state = GraphQLState::new();
        let schema = create_schema(state);
        let batcher = QueryBatcher::new();

        let batch = QueryBatch {
            queries: vec![BatchedQuery {
                query: "{ statuteCount }".to_string(),
                operation_name: None,
                variables: None,
                id: Some("test-id-123".to_string()),
            }],
            parallel: true,
            timeout_ms: Some(5000),
        };

        let result = batcher.execute_batch(&schema, batch).await.unwrap();
        assert_eq!(result.responses[0].id, Some("test-id-123".to_string()));
    }
}
