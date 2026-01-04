//! Cross-region audit aggregation for distributed systems
//!
//! This module provides functionality for aggregating audit records across
//! multiple geographic regions while maintaining consistency and minimizing latency.

use super::{DistributedError, NodeId};
use crate::AuditRecord;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Geographic region identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RegionId(pub String);

impl RegionId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

/// Information about a geographic region
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionInfo {
    pub id: RegionId,
    pub name: String,
    pub nodes: Vec<NodeId>,
    pub is_primary: bool,
    pub latency_ms: u64,
}

impl RegionInfo {
    pub fn new(id: RegionId, name: String) -> Self {
        Self {
            id,
            name,
            nodes: Vec::new(),
            is_primary: false,
            latency_ms: 0,
        }
    }

    pub fn add_node(&mut self, node_id: NodeId) {
        if !self.nodes.contains(&node_id) {
            self.nodes.push(node_id);
        }
    }

    pub fn remove_node(&mut self, node_id: &NodeId) {
        self.nodes.retain(|n| n != node_id);
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

/// Aggregation strategy for cross-region data
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AggregationStrategy {
    /// Nearest region serves reads
    NearestRead,
    /// All regions serve reads (eventual consistency)
    AnyRead,
    /// Primary region serves reads
    PrimaryRead,
    /// Quorum-based reads
    QuorumRead,
}

/// Configuration for cross-region aggregation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationConfig {
    pub strategy: AggregationStrategy,
    pub replication_factor: usize,
    pub sync_interval_secs: u64,
    pub enable_compression: bool,
    pub enable_caching: bool,
}

impl Default for AggregationConfig {
    fn default() -> Self {
        Self {
            strategy: AggregationStrategy::QuorumRead,
            replication_factor: 3,
            sync_interval_secs: 300,
            enable_compression: true,
            enable_caching: true,
        }
    }
}

/// Aggregated statistics for a region
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionStats {
    pub region_id: RegionId,
    pub record_count: usize,
    pub last_update: DateTime<Utc>,
    pub storage_size_bytes: u64,
    pub node_count: usize,
}

impl RegionStats {
    pub fn new(region_id: RegionId) -> Self {
        Self {
            region_id,
            record_count: 0,
            last_update: Utc::now(),
            storage_size_bytes: 0,
            node_count: 0,
        }
    }
}

/// Aggregated audit records from multiple regions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedRecords {
    pub records: Vec<AuditRecord>,
    pub source_regions: Vec<RegionId>,
    pub aggregated_at: DateTime<Utc>,
    pub total_count: usize,
}

impl AggregatedRecords {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            source_regions: Vec::new(),
            aggregated_at: Utc::now(),
            total_count: 0,
        }
    }

    pub fn add_records(&mut self, records: Vec<AuditRecord>, region: RegionId) {
        self.total_count += records.len();
        self.records.extend(records);
        if !self.source_regions.contains(&region) {
            self.source_regions.push(region);
        }
        self.aggregated_at = Utc::now();
    }

    pub fn merge(&mut self, other: AggregatedRecords) {
        self.records.extend(other.records);
        for region in other.source_regions {
            if !self.source_regions.contains(&region) {
                self.source_regions.push(region);
            }
        }
        self.total_count += other.total_count;
        self.aggregated_at = Utc::now();
    }
}

impl Default for AggregatedRecords {
    fn default() -> Self {
        Self::new()
    }
}

/// Manager for cross-region audit aggregation
pub struct RegionAggregator {
    local_region: RegionId,
    regions: HashMap<RegionId, RegionInfo>,
    config: AggregationConfig,
    stats: HashMap<RegionId, RegionStats>,
}

impl RegionAggregator {
    pub fn new(local_region: RegionId, config: AggregationConfig) -> Self {
        let mut regions = HashMap::new();
        let mut region_info = RegionInfo::new(local_region.clone(), local_region.0.clone());
        region_info.is_primary = true;
        regions.insert(local_region.clone(), region_info);

        Self {
            local_region,
            regions,
            config,
            stats: HashMap::new(),
        }
    }

    /// Register a new region
    pub fn register_region(&mut self, region_info: RegionInfo) {
        self.regions.insert(region_info.id.clone(), region_info);
    }

    /// Get information about a region
    pub fn get_region(&self, region_id: &RegionId) -> Option<&RegionInfo> {
        self.regions.get(region_id)
    }

    /// Get all registered regions
    pub fn get_all_regions(&self) -> Vec<&RegionInfo> {
        self.regions.values().collect()
    }

    /// Add a node to a region
    pub fn add_node_to_region(
        &mut self,
        region_id: &RegionId,
        node_id: NodeId,
    ) -> Result<(), DistributedError> {
        let region = self
            .regions
            .get_mut(region_id)
            .ok_or_else(|| DistributedError::InvalidState("Region not found".to_string()))?;

        region.add_node(node_id);
        Ok(())
    }

    /// Remove a node from a region
    pub fn remove_node_from_region(
        &mut self,
        region_id: &RegionId,
        node_id: &NodeId,
    ) -> Result<(), DistributedError> {
        let region = self
            .regions
            .get_mut(region_id)
            .ok_or_else(|| DistributedError::InvalidState("Region not found".to_string()))?;

        region.remove_node(node_id);
        Ok(())
    }

    /// Update statistics for a region
    pub fn update_region_stats(&mut self, stats: RegionStats) {
        self.stats.insert(stats.region_id.clone(), stats);
    }

    /// Get statistics for a region
    pub fn get_region_stats(&self, region_id: &RegionId) -> Option<&RegionStats> {
        self.stats.get(region_id)
    }

    /// Get global statistics across all regions
    pub fn get_global_stats(&self) -> GlobalStats {
        let total_records: usize = self.stats.values().map(|s| s.record_count).sum();
        let total_storage: u64 = self.stats.values().map(|s| s.storage_size_bytes).sum();
        let total_nodes: usize = self.stats.values().map(|s| s.node_count).sum();

        GlobalStats {
            total_regions: self.regions.len(),
            total_records,
            total_storage_bytes: total_storage,
            total_nodes,
            last_aggregated: Utc::now(),
        }
    }

    /// Aggregate records from all regions
    pub fn aggregate_all_records(&self) -> AggregatedRecords {
        let mut aggregated = AggregatedRecords::new();

        for (region_id, stats) in &self.stats {
            aggregated.source_regions.push(region_id.clone());
            aggregated.total_count += stats.record_count;
        }

        aggregated.aggregated_at = Utc::now();
        aggregated
    }

    /// Get the primary region
    pub fn get_primary_region(&self) -> Option<&RegionInfo> {
        self.regions.values().find(|r| r.is_primary)
    }

    /// Set a region as primary
    pub fn set_primary_region(&mut self, region_id: &RegionId) -> Result<(), DistributedError> {
        // Clear existing primary
        for region in self.regions.values_mut() {
            region.is_primary = false;
        }

        // Set new primary
        let region = self
            .regions
            .get_mut(region_id)
            .ok_or_else(|| DistributedError::InvalidState("Region not found".to_string()))?;

        region.is_primary = true;
        Ok(())
    }

    /// Get regions sorted by latency
    pub fn get_regions_by_latency(&self) -> Vec<&RegionInfo> {
        let mut regions: Vec<&RegionInfo> = self.regions.values().collect();
        regions.sort_by_key(|r| r.latency_ms);
        regions
    }

    /// Find the nearest region based on latency
    pub fn get_nearest_region(&self) -> Option<&RegionInfo> {
        self.regions.values().min_by_key(|r| r.latency_ms)
    }

    /// Calculate replication targets for a record
    pub fn get_replication_targets(&self, exclude_region: Option<&RegionId>) -> Vec<RegionId> {
        let mut targets: Vec<RegionId> = self
            .regions
            .keys()
            .filter(|id| {
                if let Some(excluded) = exclude_region {
                    *id != excluded
                } else {
                    true
                }
            })
            .cloned()
            .collect();

        targets.truncate(self.config.replication_factor);
        targets
    }

    /// Get the local region ID
    pub fn local_region(&self) -> &RegionId {
        &self.local_region
    }

    /// Check if a region is registered
    pub fn has_region(&self, region_id: &RegionId) -> bool {
        self.regions.contains_key(region_id)
    }
}

/// Global statistics across all regions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalStats {
    pub total_regions: usize,
    pub total_records: usize,
    pub total_storage_bytes: u64,
    pub total_nodes: usize,
    pub last_aggregated: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_region_id() {
        let region1 = RegionId::new("us-east-1");
        let region2 = RegionId::new("eu-west-1");

        assert_eq!(region1.0, "us-east-1");
        assert_eq!(region2.0, "eu-west-1");
        assert_ne!(region1, region2);
    }

    #[test]
    fn test_region_info() {
        let region_id = RegionId::new("us-east-1");
        let mut region = RegionInfo::new(region_id.clone(), "US East 1".to_string());

        assert_eq!(region.id, region_id);
        assert_eq!(region.name, "US East 1");
        assert_eq!(region.node_count(), 0);
        assert!(!region.is_primary);

        let node1 = NodeId::new("node-1");
        region.add_node(node1.clone());
        assert_eq!(region.node_count(), 1);

        region.remove_node(&node1);
        assert_eq!(region.node_count(), 0);
    }

    #[test]
    fn test_aggregation_config_default() {
        let config = AggregationConfig::default();
        assert_eq!(config.strategy, AggregationStrategy::QuorumRead);
        assert_eq!(config.replication_factor, 3);
        assert_eq!(config.sync_interval_secs, 300);
        assert!(config.enable_compression);
        assert!(config.enable_caching);
    }

    #[test]
    fn test_region_stats() {
        let region_id = RegionId::new("us-east-1");
        let mut stats = RegionStats::new(region_id.clone());

        assert_eq!(stats.region_id, region_id);
        assert_eq!(stats.record_count, 0);
        assert_eq!(stats.storage_size_bytes, 0);
        assert_eq!(stats.node_count, 0);

        stats.record_count = 100;
        stats.storage_size_bytes = 10240;
        stats.node_count = 3;

        assert_eq!(stats.record_count, 100);
        assert_eq!(stats.storage_size_bytes, 10240);
        assert_eq!(stats.node_count, 3);
    }

    #[test]
    fn test_aggregated_records() {
        let mut aggregated = AggregatedRecords::new();

        assert_eq!(aggregated.records.len(), 0);
        assert_eq!(aggregated.source_regions.len(), 0);
        assert_eq!(aggregated.total_count, 0);

        let region1 = RegionId::new("us-east-1");
        aggregated.add_records(Vec::new(), region1.clone());

        assert_eq!(aggregated.source_regions.len(), 1);
        assert!(aggregated.source_regions.contains(&region1));
    }

    #[test]
    fn test_aggregated_records_merge() {
        let mut agg1 = AggregatedRecords::new();
        let mut agg2 = AggregatedRecords::new();

        let region1 = RegionId::new("us-east-1");
        let region2 = RegionId::new("eu-west-1");

        agg1.add_records(Vec::new(), region1.clone());
        agg2.add_records(Vec::new(), region2.clone());

        agg1.merge(agg2);

        assert_eq!(agg1.source_regions.len(), 2);
        assert!(agg1.source_regions.contains(&region1));
        assert!(agg1.source_regions.contains(&region2));
    }

    #[test]
    fn test_region_aggregator() {
        let local_region = RegionId::new("us-east-1");
        let config = AggregationConfig::default();
        let aggregator = RegionAggregator::new(local_region.clone(), config);

        assert_eq!(aggregator.local_region(), &local_region);
        assert_eq!(aggregator.get_all_regions().len(), 1);
    }

    #[test]
    fn test_register_region() {
        let local_region = RegionId::new("us-east-1");
        let config = AggregationConfig::default();
        let mut aggregator = RegionAggregator::new(local_region, config);

        let region2 = RegionInfo::new(RegionId::new("eu-west-1"), "EU West 1".to_string());
        aggregator.register_region(region2.clone());

        assert_eq!(aggregator.get_all_regions().len(), 2);
        assert!(aggregator.has_region(&region2.id));
    }

    #[test]
    fn test_add_remove_node_to_region() {
        let local_region = RegionId::new("us-east-1");
        let config = AggregationConfig::default();
        let mut aggregator = RegionAggregator::new(local_region.clone(), config);

        let node1 = NodeId::new("node-1");
        aggregator
            .add_node_to_region(&local_region, node1.clone())
            .unwrap();

        let region = aggregator.get_region(&local_region).unwrap();
        assert_eq!(region.node_count(), 1);

        aggregator
            .remove_node_from_region(&local_region, &node1)
            .unwrap();
        let region = aggregator.get_region(&local_region).unwrap();
        assert_eq!(region.node_count(), 0);
    }

    #[test]
    fn test_region_stats_update() {
        let local_region = RegionId::new("us-east-1");
        let config = AggregationConfig::default();
        let mut aggregator = RegionAggregator::new(local_region.clone(), config);

        let mut stats = RegionStats::new(local_region.clone());
        stats.record_count = 100;
        stats.storage_size_bytes = 10240;
        stats.node_count = 3;

        aggregator.update_region_stats(stats);

        let retrieved = aggregator.get_region_stats(&local_region).unwrap();
        assert_eq!(retrieved.record_count, 100);
        assert_eq!(retrieved.storage_size_bytes, 10240);
        assert_eq!(retrieved.node_count, 3);
    }

    #[test]
    fn test_global_stats() {
        let local_region = RegionId::new("us-east-1");
        let config = AggregationConfig::default();
        let mut aggregator = RegionAggregator::new(local_region.clone(), config);

        let region2 = RegionInfo::new(RegionId::new("eu-west-1"), "EU West 1".to_string());
        aggregator.register_region(region2.clone());

        let mut stats1 = RegionStats::new(local_region);
        stats1.record_count = 100;
        stats1.storage_size_bytes = 10240;
        stats1.node_count = 3;

        let mut stats2 = RegionStats::new(region2.id.clone());
        stats2.record_count = 200;
        stats2.storage_size_bytes = 20480;
        stats2.node_count = 5;

        aggregator.update_region_stats(stats1);
        aggregator.update_region_stats(stats2);

        let global = aggregator.get_global_stats();
        assert_eq!(global.total_regions, 2);
        assert_eq!(global.total_records, 300);
        assert_eq!(global.total_storage_bytes, 30720);
        assert_eq!(global.total_nodes, 8);
    }

    #[test]
    fn test_primary_region() {
        let local_region = RegionId::new("us-east-1");
        let config = AggregationConfig::default();
        let mut aggregator = RegionAggregator::new(local_region.clone(), config);

        let primary = aggregator.get_primary_region().unwrap();
        assert_eq!(primary.id, local_region);
        assert!(primary.is_primary);

        let region2 = RegionInfo::new(RegionId::new("eu-west-1"), "EU West 1".to_string());
        let region2_id = region2.id.clone();
        aggregator.register_region(region2);

        aggregator.set_primary_region(&region2_id).unwrap();

        let new_primary = aggregator.get_primary_region().unwrap();
        assert_eq!(new_primary.id, region2_id);
        assert!(new_primary.is_primary);
    }

    #[test]
    fn test_replication_targets() {
        let local_region = RegionId::new("us-east-1");
        let config = AggregationConfig {
            replication_factor: 2,
            ..Default::default()
        };
        let mut aggregator = RegionAggregator::new(local_region.clone(), config);

        let region2 = RegionInfo::new(RegionId::new("eu-west-1"), "EU West 1".to_string());
        let region3 = RegionInfo::new(RegionId::new("ap-south-1"), "AP South 1".to_string());

        aggregator.register_region(region2);
        aggregator.register_region(region3);

        let targets = aggregator.get_replication_targets(Some(&local_region));
        assert_eq!(targets.len(), 2);
        assert!(!targets.contains(&local_region));
    }

    #[test]
    fn test_regions_by_latency() {
        let local_region = RegionId::new("us-east-1");
        let config = AggregationConfig::default();
        let mut aggregator = RegionAggregator::new(local_region, config);

        let mut region2 = RegionInfo::new(RegionId::new("eu-west-1"), "EU West 1".to_string());
        region2.latency_ms = 100;

        let mut region3 = RegionInfo::new(RegionId::new("ap-south-1"), "AP South 1".to_string());
        region3.latency_ms = 200;

        aggregator.register_region(region2);
        aggregator.register_region(region3);

        let sorted = aggregator.get_regions_by_latency();
        assert_eq!(sorted.len(), 3);
        assert_eq!(sorted[0].latency_ms, 0); // local region
        assert_eq!(sorted[1].latency_ms, 100);
        assert_eq!(sorted[2].latency_ms, 200);
    }

    #[test]
    fn test_nearest_region() {
        let local_region = RegionId::new("us-east-1");
        let config = AggregationConfig::default();
        let mut aggregator = RegionAggregator::new(local_region, config);

        let mut region2 = RegionInfo::new(RegionId::new("eu-west-1"), "EU West 1".to_string());
        region2.latency_ms = 100;

        aggregator.register_region(region2);

        let nearest = aggregator.get_nearest_region().unwrap();
        assert_eq!(nearest.latency_ms, 0);
    }
}
