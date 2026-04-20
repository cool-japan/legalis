//! Streaming support for large result sets.
//!
//! This module provides Stream implementations for efficiently
//! iterating over large collections of statutes.

use super::*;
use async_stream::stream;
use futures::Stream;

/// Creates a stream of all statutes.
pub fn stream_all(
    registry: std::sync::Arc<tokio::sync::RwLock<StatuteRegistry>>,
    chunk_size: usize,
) -> impl Stream<Item = Vec<StatuteEntry>> {
    stream! {
        let registry = registry.read().await;
        let statutes: Vec<StatuteEntry> = registry.list().into_iter().cloned().collect();
        drop(registry);

        for chunk in statutes.chunks(chunk_size) {
            yield chunk.to_vec();
        }
    }
}

/// Creates a stream of statutes matching a query.
pub fn stream_search(
    registry: std::sync::Arc<tokio::sync::RwLock<StatuteRegistry>>,
    query: SearchQuery,
    chunk_size: usize,
) -> impl Stream<Item = Vec<StatuteEntry>> {
    stream! {
        let registry = registry.read().await;
        let results: Vec<StatuteEntry> = registry.search(&query).iter().map(|&e| e.clone()).collect();
        drop(registry);

        for chunk in results.chunks(chunk_size) {
            yield chunk.to_vec();
        }
    }
}

/// Creates a stream of statute summaries.
pub fn stream_summaries(
    registry: std::sync::Arc<tokio::sync::RwLock<StatuteRegistry>>,
    chunk_size: usize,
) -> impl Stream<Item = Vec<StatuteSummary>> {
    stream! {
        let registry = registry.read().await;
        let summaries: Vec<StatuteSummary> = registry
            .list_summaries()
            .into_iter()
            .collect();
        drop(registry);

        for chunk in summaries.chunks(chunk_size) {
            yield chunk.to_vec();
        }
    }
}
