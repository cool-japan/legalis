//! Global Registry Network - Geo-distributed statute registry with jurisdiction-aware routing.
//!
//! This module provides infrastructure for a global network of statute registries with:
//! - Geo-distributed registry mesh with multi-region support
//! - Jurisdiction-aware routing for compliance
//! - Cross-border data sovereignty enforcement
//! - Global statute namespace management
//! - Latency-optimized replication strategies

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Geographic region for a registry node.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeographicRegion {
    /// North America (US, Canada, Mexico)
    NorthAmerica,
    /// European Union member states
    EuropeanUnion,
    /// Asia-Pacific region
    AsiaPacific,
    /// Latin America and Caribbean
    LatinAmerica,
    /// Middle East and North Africa
    MiddleEast,
    /// Sub-Saharan Africa
    Africa,
    /// Custom region with identifier
    Custom(String),
}

impl GeographicRegion {
    /// Get the default latency (in ms) between this region and another.
    pub fn default_latency_to(&self, other: &GeographicRegion) -> u64 {
        if self == other {
            return 5; // Same region
        }

        match (self, other) {
            (GeographicRegion::NorthAmerica, GeographicRegion::EuropeanUnion) => 80,
            (GeographicRegion::NorthAmerica, GeographicRegion::AsiaPacific) => 150,
            (GeographicRegion::EuropeanUnion, GeographicRegion::AsiaPacific) => 200,
            (GeographicRegion::EuropeanUnion, GeographicRegion::Africa) => 100,
            (GeographicRegion::NorthAmerica, GeographicRegion::LatinAmerica) => 60,
            _ => 250, // Default cross-continental latency
        }
    }
}

/// A node in the global registry mesh.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryNode {
    /// Unique node identifier
    pub node_id: Uuid,
    /// Geographic region of this node
    pub region: GeographicRegion,
    /// Endpoint URL for this node
    pub endpoint: String,
    /// Jurisdictions served by this node
    pub jurisdictions: HashSet<String>,
    /// Node status
    pub status: NodeStatus,
    /// When this node was created
    pub created_at: DateTime<Utc>,
    /// Last heartbeat timestamp
    pub last_heartbeat: DateTime<Utc>,
    /// Node capacity (requests per second)
    pub capacity: u32,
    /// Current load (0.0 - 1.0)
    pub load: f64,
}

impl RegistryNode {
    /// Creates a new registry node.
    pub fn new(region: GeographicRegion, endpoint: String) -> Self {
        let now = Utc::now();
        Self {
            node_id: Uuid::new_v4(),
            region,
            endpoint,
            jurisdictions: HashSet::new(),
            status: NodeStatus::Active,
            created_at: now,
            last_heartbeat: now,
            capacity: 1000,
            load: 0.0,
        }
    }

    /// Add a jurisdiction to this node.
    pub fn add_jurisdiction(&mut self, jurisdiction: String) {
        self.jurisdictions.insert(jurisdiction);
    }

    /// Check if this node serves a specific jurisdiction.
    pub fn serves_jurisdiction(&self, jurisdiction: &str) -> bool {
        self.jurisdictions.contains(jurisdiction)
    }

    /// Update the node's heartbeat.
    pub fn heartbeat(&mut self) {
        self.last_heartbeat = Utc::now();
    }

    /// Check if the node is healthy (heartbeat within last 60 seconds).
    pub fn is_healthy(&self) -> bool {
        let now = Utc::now();
        (now - self.last_heartbeat).num_seconds() < 60
    }

    /// Update the node's load.
    pub fn set_load(&mut self, load: f64) {
        self.load = load.clamp(0.0, 1.0);
    }

    /// Check if the node has capacity for more requests.
    pub fn has_capacity(&self) -> bool {
        self.load < 0.9 && self.status == NodeStatus::Active
    }
}

/// Status of a registry node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeStatus {
    /// Node is active and accepting requests
    Active,
    /// Node is in maintenance mode
    Maintenance,
    /// Node is draining connections
    Draining,
    /// Node is offline
    Offline,
}

/// Geo-distributed registry mesh manager.
#[derive(Debug, Clone)]
pub struct RegistryMesh {
    nodes: Arc<Mutex<HashMap<Uuid, RegistryNode>>>,
    /// Latency matrix between nodes
    latency_matrix: Arc<Mutex<HashMap<(Uuid, Uuid), u64>>>,
}

impl RegistryMesh {
    /// Creates a new empty registry mesh.
    pub fn new() -> Self {
        Self {
            nodes: Arc::new(Mutex::new(HashMap::new())),
            latency_matrix: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Register a new node in the mesh.
    pub fn register_node(&self, node: RegistryNode) -> Uuid {
        let node_id = node.node_id;
        self.nodes.lock().unwrap().insert(node_id, node);
        node_id
    }

    /// Get a node by ID.
    pub fn get_node(&self, node_id: Uuid) -> Option<RegistryNode> {
        self.nodes.lock().unwrap().get(&node_id).cloned()
    }

    /// List all active nodes.
    pub fn active_nodes(&self) -> Vec<RegistryNode> {
        self.nodes
            .lock()
            .unwrap()
            .values()
            .filter(|n| n.status == NodeStatus::Active && n.is_healthy())
            .cloned()
            .collect()
    }

    /// Find nodes serving a specific jurisdiction.
    pub fn nodes_for_jurisdiction(&self, jurisdiction: &str) -> Vec<RegistryNode> {
        self.active_nodes()
            .into_iter()
            .filter(|n| n.serves_jurisdiction(jurisdiction))
            .collect()
    }

    /// Find the closest node to a given region.
    pub fn closest_node(&self, region: &GeographicRegion) -> Option<RegistryNode> {
        self.active_nodes()
            .into_iter()
            .min_by_key(|n| n.region.default_latency_to(region))
    }

    /// Set latency between two nodes.
    pub fn set_latency(&self, from: Uuid, to: Uuid, latency_ms: u64) {
        self.latency_matrix
            .lock()
            .unwrap()
            .insert((from, to), latency_ms);
        self.latency_matrix
            .lock()
            .unwrap()
            .insert((to, from), latency_ms);
    }

    /// Get latency between two nodes.
    pub fn get_latency(&self, from: Uuid, to: Uuid) -> Option<u64> {
        self.latency_matrix
            .lock()
            .unwrap()
            .get(&(from, to))
            .copied()
    }

    /// Remove a node from the mesh.
    pub fn remove_node(&self, node_id: Uuid) -> bool {
        self.nodes.lock().unwrap().remove(&node_id).is_some()
    }

    /// Count total nodes in mesh.
    pub fn node_count(&self) -> usize {
        self.nodes.lock().unwrap().len()
    }
}

impl Default for RegistryMesh {
    fn default() -> Self {
        Self::new()
    }
}

/// Jurisdiction-aware routing configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JurisdictionRouter {
    /// Routing rules by jurisdiction
    routing_rules: HashMap<String, RoutingRule>,
}

impl JurisdictionRouter {
    /// Creates a new jurisdiction router.
    pub fn new() -> Self {
        Self {
            routing_rules: HashMap::new(),
        }
    }

    /// Add a routing rule for a jurisdiction.
    pub fn add_rule(&mut self, jurisdiction: String, rule: RoutingRule) {
        self.routing_rules.insert(jurisdiction, rule);
    }

    /// Get routing rule for a jurisdiction.
    pub fn get_rule(&self, jurisdiction: &str) -> Option<&RoutingRule> {
        self.routing_rules.get(jurisdiction)
    }

    /// Route a request based on jurisdiction.
    pub fn route(&self, jurisdiction: &str, mesh: &RegistryMesh) -> Option<Uuid> {
        let rule = self.get_rule(jurisdiction)?;

        let candidates = mesh.nodes_for_jurisdiction(jurisdiction);
        if candidates.is_empty() {
            return None;
        }

        match &rule.strategy {
            RoutingStrategy::Closest(region) => candidates
                .into_iter()
                .min_by_key(|n| n.region.default_latency_to(region))
                .map(|n| n.node_id),
            RoutingStrategy::LeastLoaded => candidates
                .into_iter()
                .filter(|n| n.has_capacity())
                .min_by(|a, b| a.load.partial_cmp(&b.load).unwrap())
                .map(|n| n.node_id),
            RoutingStrategy::Primary(primary_id) => {
                if candidates.iter().any(|n| n.node_id == *primary_id) {
                    Some(*primary_id)
                } else {
                    // Fallback to first available
                    candidates.first().map(|n| n.node_id)
                }
            }
        }
    }

    /// Count routing rules.
    pub fn rule_count(&self) -> usize {
        self.routing_rules.len()
    }
}

impl Default for JurisdictionRouter {
    fn default() -> Self {
        Self::new()
    }
}

/// Routing rule for a jurisdiction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingRule {
    /// Routing strategy to use
    pub strategy: RoutingStrategy,
    /// Whether data must stay in specific regions
    pub sovereignty_required: bool,
    /// Allowed regions for data storage
    pub allowed_regions: Vec<GeographicRegion>,
}

impl RoutingRule {
    /// Creates a new routing rule with closest strategy.
    pub fn closest(region: GeographicRegion) -> Self {
        Self {
            strategy: RoutingStrategy::Closest(region),
            sovereignty_required: false,
            allowed_regions: vec![],
        }
    }

    /// Creates a new routing rule with least loaded strategy.
    pub fn least_loaded() -> Self {
        Self {
            strategy: RoutingStrategy::LeastLoaded,
            sovereignty_required: false,
            allowed_regions: vec![],
        }
    }

    /// Creates a new routing rule with primary node strategy.
    pub fn primary(node_id: Uuid) -> Self {
        Self {
            strategy: RoutingStrategy::Primary(node_id),
            sovereignty_required: false,
            allowed_regions: vec![],
        }
    }

    /// Set sovereignty requirement.
    pub fn with_sovereignty(mut self, regions: Vec<GeographicRegion>) -> Self {
        self.sovereignty_required = true;
        self.allowed_regions = regions;
        self
    }

    /// Check if a region is allowed for this rule.
    pub fn allows_region(&self, region: &GeographicRegion) -> bool {
        if !self.sovereignty_required {
            return true;
        }
        self.allowed_regions.contains(region)
    }
}

/// Routing strategy for requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoutingStrategy {
    /// Route to closest node in specified region
    Closest(GeographicRegion),
    /// Route to least loaded node
    LeastLoaded,
    /// Route to specific primary node
    Primary(Uuid),
}

/// Data sovereignty compliance manager.
#[derive(Debug, Clone)]
pub struct SovereigntyManager {
    /// Compliance policies by jurisdiction
    policies: Arc<Mutex<HashMap<String, SovereigntyPolicy>>>,
}

impl SovereigntyManager {
    /// Creates a new sovereignty manager.
    pub fn new() -> Self {
        Self {
            policies: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Add a sovereignty policy for a jurisdiction.
    pub fn add_policy(&self, jurisdiction: String, policy: SovereigntyPolicy) {
        self.policies.lock().unwrap().insert(jurisdiction, policy);
    }

    /// Get policy for a jurisdiction.
    pub fn get_policy(&self, jurisdiction: &str) -> Option<SovereigntyPolicy> {
        self.policies.lock().unwrap().get(jurisdiction).cloned()
    }

    /// Validate if a node can store data for a jurisdiction.
    pub fn validate_storage(&self, jurisdiction: &str, node: &RegistryNode) -> bool {
        let policy = match self.get_policy(jurisdiction) {
            Some(p) => p,
            None => return true, // No policy = allow all
        };

        policy.allowed_regions.contains(&node.region)
    }

    /// Validate if data can be replicated between regions.
    pub fn validate_replication(
        &self,
        jurisdiction: &str,
        from_region: &GeographicRegion,
        to_region: &GeographicRegion,
    ) -> bool {
        let policy = match self.get_policy(jurisdiction) {
            Some(p) => p,
            None => return true,
        };

        if !policy.allow_cross_border_replication {
            return from_region == to_region;
        }

        policy.allowed_regions.contains(to_region)
    }

    /// List all jurisdictions with policies.
    pub fn jurisdictions_with_policies(&self) -> Vec<String> {
        self.policies.lock().unwrap().keys().cloned().collect()
    }

    /// Count policies.
    pub fn policy_count(&self) -> usize {
        self.policies.lock().unwrap().len()
    }
}

impl Default for SovereigntyManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Data sovereignty policy for a jurisdiction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SovereigntyPolicy {
    /// Regions where data can be stored
    pub allowed_regions: Vec<GeographicRegion>,
    /// Whether cross-border replication is allowed
    pub allow_cross_border_replication: bool,
    /// Encryption required for cross-border transfer
    pub require_encryption: bool,
    /// Data retention period in days
    pub retention_days: Option<u32>,
}

impl SovereigntyPolicy {
    /// Creates a strict policy (single region, no cross-border).
    pub fn strict(region: GeographicRegion) -> Self {
        Self {
            allowed_regions: vec![region],
            allow_cross_border_replication: false,
            require_encryption: true,
            retention_days: None,
        }
    }

    /// Creates a relaxed policy (multiple regions, cross-border allowed).
    pub fn relaxed(regions: Vec<GeographicRegion>) -> Self {
        Self {
            allowed_regions: regions,
            allow_cross_border_replication: true,
            require_encryption: false,
            retention_days: None,
        }
    }

    /// Set retention period.
    pub fn with_retention(mut self, days: u32) -> Self {
        self.retention_days = Some(days);
        self
    }

    /// Require encryption for cross-border transfers.
    pub fn require_encryption(mut self) -> Self {
        self.require_encryption = true;
        self
    }
}

/// Global statute namespace manager.
#[derive(Debug, Clone)]
pub struct GlobalNamespace {
    /// Namespace to jurisdiction mapping
    namespaces: Arc<Mutex<HashMap<String, NamespaceInfo>>>,
}

impl GlobalNamespace {
    /// Creates a new global namespace manager.
    pub fn new() -> Self {
        Self {
            namespaces: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Register a namespace for a jurisdiction.
    pub fn register_namespace(&self, namespace: String, info: NamespaceInfo) -> bool {
        self.namespaces
            .lock()
            .unwrap()
            .insert(namespace, info)
            .is_none()
    }

    /// Get namespace info.
    pub fn get_namespace(&self, namespace: &str) -> Option<NamespaceInfo> {
        self.namespaces.lock().unwrap().get(namespace).cloned()
    }

    /// Parse a fully qualified statute ID.
    pub fn parse_fqn(&self, fqn: &str) -> Option<(String, String)> {
        let parts: Vec<&str> = fqn.split("::").collect();
        if parts.len() == 2 {
            Some((parts[0].to_string(), parts[1].to_string()))
        } else {
            None
        }
    }

    /// Create a fully qualified statute ID.
    pub fn create_fqn(&self, namespace: &str, statute_id: &str) -> String {
        format!("{}::{}", namespace, statute_id)
    }

    /// List all registered namespaces.
    pub fn list_namespaces(&self) -> Vec<String> {
        self.namespaces.lock().unwrap().keys().cloned().collect()
    }

    /// Count namespaces.
    pub fn namespace_count(&self) -> usize {
        self.namespaces.lock().unwrap().len()
    }

    /// Remove a namespace.
    pub fn remove_namespace(&self, namespace: &str) -> bool {
        self.namespaces.lock().unwrap().remove(namespace).is_some()
    }
}

impl Default for GlobalNamespace {
    fn default() -> Self {
        Self::new()
    }
}

/// Information about a registered namespace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamespaceInfo {
    /// Jurisdiction this namespace represents
    pub jurisdiction: String,
    /// Human-readable description
    pub description: String,
    /// Authority managing this namespace
    pub authority: String,
    /// When this namespace was registered
    pub registered_at: DateTime<Utc>,
}

impl NamespaceInfo {
    /// Creates a new namespace info.
    pub fn new(jurisdiction: String, description: String, authority: String) -> Self {
        Self {
            jurisdiction,
            description,
            authority,
            registered_at: Utc::now(),
        }
    }
}

/// Latency-optimized replication strategy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationStrategy {
    /// Replication mode
    pub mode: ReplicationMode,
    /// Target replication latency in milliseconds
    pub target_latency_ms: u64,
    /// Maximum staleness allowed in seconds
    pub max_staleness_seconds: u64,
}

impl ReplicationStrategy {
    /// Creates a synchronous replication strategy.
    pub fn synchronous() -> Self {
        Self {
            mode: ReplicationMode::Synchronous,
            target_latency_ms: 100,
            max_staleness_seconds: 0,
        }
    }

    /// Creates an asynchronous replication strategy.
    pub fn asynchronous(max_staleness_seconds: u64) -> Self {
        Self {
            mode: ReplicationMode::Asynchronous,
            target_latency_ms: 1000,
            max_staleness_seconds,
        }
    }

    /// Creates a quorum-based replication strategy.
    pub fn quorum(quorum_size: usize) -> Self {
        Self {
            mode: ReplicationMode::Quorum { quorum_size },
            target_latency_ms: 200,
            max_staleness_seconds: 1,
        }
    }

    /// Set target latency.
    pub fn with_target_latency(mut self, latency_ms: u64) -> Self {
        self.target_latency_ms = latency_ms;
        self
    }
}

/// Replication mode for distributed updates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReplicationMode {
    /// All nodes must acknowledge before completion
    Synchronous,
    /// Updates propagate asynchronously
    Asynchronous,
    /// Quorum of nodes must acknowledge
    Quorum { quorum_size: usize },
}

/// Latency-aware replication manager.
#[derive(Debug, Clone)]
pub struct ReplicationManager {
    /// Replication strategy
    strategy: Arc<Mutex<ReplicationStrategy>>,
    /// Replication log
    log: Arc<Mutex<Vec<ReplicationLogEntry>>>,
}

impl ReplicationManager {
    /// Creates a new replication manager.
    pub fn new(strategy: ReplicationStrategy) -> Self {
        Self {
            strategy: Arc::new(Mutex::new(strategy)),
            log: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Get current replication strategy.
    pub fn strategy(&self) -> ReplicationStrategy {
        self.strategy.lock().unwrap().clone()
    }

    /// Update replication strategy.
    pub fn set_strategy(&self, strategy: ReplicationStrategy) {
        *self.strategy.lock().unwrap() = strategy;
    }

    /// Record a replication operation.
    pub fn record_replication(
        &self,
        source_node: Uuid,
        target_nodes: Vec<Uuid>,
        latency_ms: u64,
        success: bool,
    ) {
        let entry = ReplicationLogEntry {
            timestamp: Utc::now(),
            source_node,
            target_nodes,
            latency_ms,
            success,
        };
        self.log.lock().unwrap().push(entry);
    }

    /// Get replication statistics.
    pub fn statistics(&self) -> ReplicationStatistics {
        let log = self.log.lock().unwrap();
        let total = log.len();
        let successful = log.iter().filter(|e| e.success).count();
        let avg_latency = if total > 0 {
            log.iter().map(|e| e.latency_ms).sum::<u64>() / total as u64
        } else {
            0
        };

        ReplicationStatistics {
            total_replications: total,
            successful_replications: successful,
            failed_replications: total - successful,
            average_latency_ms: avg_latency,
        }
    }

    /// Get replication log entries.
    pub fn get_log(&self) -> Vec<ReplicationLogEntry> {
        self.log.lock().unwrap().clone()
    }

    /// Clear replication log.
    pub fn clear_log(&self) {
        self.log.lock().unwrap().clear();
    }
}

/// Log entry for a replication operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationLogEntry {
    /// When this replication occurred
    pub timestamp: DateTime<Utc>,
    /// Source node
    pub source_node: Uuid,
    /// Target nodes
    pub target_nodes: Vec<Uuid>,
    /// Latency in milliseconds
    pub latency_ms: u64,
    /// Whether replication succeeded
    pub success: bool,
}

/// Statistics about replication operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationStatistics {
    /// Total number of replication operations
    pub total_replications: usize,
    /// Successful replications
    pub successful_replications: usize,
    /// Failed replications
    pub failed_replications: usize,
    /// Average latency in milliseconds
    pub average_latency_ms: u64,
}

impl ReplicationStatistics {
    /// Calculate success rate (0.0 - 1.0).
    pub fn success_rate(&self) -> f64 {
        if self.total_replications == 0 {
            return 0.0;
        }
        self.successful_replications as f64 / self.total_replications as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_node_creation() {
        let mut node = RegistryNode::new(
            GeographicRegion::NorthAmerica,
            "https://na.registry.example.com".to_string(),
        );

        assert_eq!(node.region, GeographicRegion::NorthAmerica);
        assert_eq!(node.status, NodeStatus::Active);
        assert!(node.is_healthy());

        node.add_jurisdiction("US".to_string());
        assert!(node.serves_jurisdiction("US"));
        assert!(!node.serves_jurisdiction("UK"));
    }

    #[test]
    fn test_registry_mesh_operations() {
        let mesh = RegistryMesh::new();

        let node1 = RegistryNode::new(
            GeographicRegion::NorthAmerica,
            "https://na.example.com".to_string(),
        );
        let node1_id = node1.node_id;

        mesh.register_node(node1);
        assert_eq!(mesh.node_count(), 1);

        let retrieved = mesh.get_node(node1_id);
        assert!(retrieved.is_some());

        mesh.remove_node(node1_id);
        assert_eq!(mesh.node_count(), 0);
    }

    #[test]
    fn test_jurisdiction_routing() {
        let mesh = RegistryMesh::new();
        let mut router = JurisdictionRouter::new();

        let mut node = RegistryNode::new(
            GeographicRegion::EuropeanUnion,
            "https://eu.example.com".to_string(),
        );
        node.add_jurisdiction("UK".to_string());
        let node_id = node.node_id;

        mesh.register_node(node);

        let rule = RoutingRule::closest(GeographicRegion::EuropeanUnion);
        router.add_rule("UK".to_string(), rule);

        let routed = router.route("UK", &mesh);
        assert_eq!(routed, Some(node_id));
    }

    #[test]
    fn test_sovereignty_policy_strict() {
        let policy = SovereigntyPolicy::strict(GeographicRegion::EuropeanUnion);

        assert_eq!(policy.allowed_regions.len(), 1);
        assert!(!policy.allow_cross_border_replication);
        assert!(policy.require_encryption);
    }

    #[test]
    fn test_sovereignty_manager() {
        let manager = SovereigntyManager::new();
        let policy = SovereigntyPolicy::strict(GeographicRegion::EuropeanUnion);

        manager.add_policy("GDPR".to_string(), policy);
        assert_eq!(manager.policy_count(), 1);

        let node = RegistryNode::new(
            GeographicRegion::EuropeanUnion,
            "https://eu.example.com".to_string(),
        );

        assert!(manager.validate_storage("GDPR", &node));

        let us_node = RegistryNode::new(
            GeographicRegion::NorthAmerica,
            "https://us.example.com".to_string(),
        );

        assert!(!manager.validate_storage("GDPR", &us_node));
    }

    #[test]
    fn test_global_namespace() {
        let namespace = GlobalNamespace::new();

        let info = NamespaceInfo::new(
            "US".to_string(),
            "United States Federal Law".to_string(),
            "US Congress".to_string(),
        );

        assert!(namespace.register_namespace("us-federal".to_string(), info));
        assert_eq!(namespace.namespace_count(), 1);

        let fqn = namespace.create_fqn("us-federal", "title-18-sec-1001");
        assert_eq!(fqn, "us-federal::title-18-sec-1001");

        let parsed = namespace.parse_fqn(&fqn);
        assert_eq!(
            parsed,
            Some(("us-federal".to_string(), "title-18-sec-1001".to_string()))
        );
    }

    #[test]
    fn test_replication_strategy() {
        let sync = ReplicationStrategy::synchronous();
        assert!(matches!(sync.mode, ReplicationMode::Synchronous));
        assert_eq!(sync.max_staleness_seconds, 0);

        let async_strat = ReplicationStrategy::asynchronous(60);
        assert!(matches!(async_strat.mode, ReplicationMode::Asynchronous));
        assert_eq!(async_strat.max_staleness_seconds, 60);

        let quorum = ReplicationStrategy::quorum(3);
        assert!(matches!(
            quorum.mode,
            ReplicationMode::Quorum { quorum_size: 3 }
        ));
    }

    #[test]
    fn test_replication_manager() {
        let manager = ReplicationManager::new(ReplicationStrategy::synchronous());

        let source = Uuid::new_v4();
        let targets = vec![Uuid::new_v4(), Uuid::new_v4()];

        manager.record_replication(source, targets.clone(), 50, true);
        manager.record_replication(source, targets, 75, true);

        let stats = manager.statistics();
        assert_eq!(stats.total_replications, 2);
        assert_eq!(stats.successful_replications, 2);
        assert_eq!(stats.success_rate(), 1.0);
    }

    #[test]
    fn test_geographic_region_latency() {
        let na = GeographicRegion::NorthAmerica;
        let eu = GeographicRegion::EuropeanUnion;
        let ap = GeographicRegion::AsiaPacific;

        assert_eq!(na.default_latency_to(&na), 5);
        assert_eq!(na.default_latency_to(&eu), 80);
        assert_eq!(eu.default_latency_to(&ap), 200);
    }

    #[test]
    fn test_routing_rule_sovereignty() {
        let rule = RoutingRule::closest(GeographicRegion::EuropeanUnion)
            .with_sovereignty(vec![GeographicRegion::EuropeanUnion]);

        assert!(rule.sovereignty_required);
        assert!(rule.allows_region(&GeographicRegion::EuropeanUnion));
        assert!(!rule.allows_region(&GeographicRegion::NorthAmerica));
    }

    #[test]
    fn test_node_capacity_management() {
        let mut node = RegistryNode::new(
            GeographicRegion::AsiaPacific,
            "https://ap.example.com".to_string(),
        );

        assert!(node.has_capacity());

        node.set_load(0.95);
        assert!(!node.has_capacity());

        node.set_load(0.5);
        assert!(node.has_capacity());

        node.status = NodeStatus::Maintenance;
        assert!(!node.has_capacity());
    }

    #[test]
    fn test_sovereignty_cross_border_validation() {
        let manager = SovereigntyManager::new();
        let policy = SovereigntyPolicy::relaxed(vec![
            GeographicRegion::EuropeanUnion,
            GeographicRegion::NorthAmerica,
        ]);

        manager.add_policy("TRANSATLANTIC".to_string(), policy);

        assert!(manager.validate_replication(
            "TRANSATLANTIC",
            &GeographicRegion::EuropeanUnion,
            &GeographicRegion::NorthAmerica
        ));

        assert!(!manager.validate_replication(
            "TRANSATLANTIC",
            &GeographicRegion::EuropeanUnion,
            &GeographicRegion::AsiaPacific
        ));
    }

    #[test]
    fn test_mesh_closest_node() {
        let mesh = RegistryMesh::new();

        let na_node = RegistryNode::new(
            GeographicRegion::NorthAmerica,
            "https://na.example.com".to_string(),
        );
        let eu_node = RegistryNode::new(
            GeographicRegion::EuropeanUnion,
            "https://eu.example.com".to_string(),
        );

        mesh.register_node(na_node);
        mesh.register_node(eu_node);

        let closest = mesh.closest_node(&GeographicRegion::NorthAmerica);
        assert!(closest.is_some());
        assert_eq!(closest.unwrap().region, GeographicRegion::NorthAmerica);
    }

    #[test]
    fn test_replication_statistics() {
        let manager = ReplicationManager::new(ReplicationStrategy::asynchronous(60));

        let source = Uuid::new_v4();
        manager.record_replication(source, vec![Uuid::new_v4()], 100, true);
        manager.record_replication(source, vec![Uuid::new_v4()], 200, true);
        manager.record_replication(source, vec![Uuid::new_v4()], 150, false);

        let stats = manager.statistics();
        assert_eq!(stats.total_replications, 3);
        assert_eq!(stats.successful_replications, 2);
        assert_eq!(stats.failed_replications, 1);
        assert_eq!(stats.average_latency_ms, 150);
        assert!((stats.success_rate() - 0.666).abs() < 0.01);
    }

    #[test]
    fn test_least_loaded_routing() {
        let mesh = RegistryMesh::new();
        let mut router = JurisdictionRouter::new();

        let mut node1 = RegistryNode::new(
            GeographicRegion::NorthAmerica,
            "https://na1.example.com".to_string(),
        );
        node1.set_load(0.8);
        node1.add_jurisdiction("US".to_string());
        let _node1_id = node1.node_id;

        let mut node2 = RegistryNode::new(
            GeographicRegion::NorthAmerica,
            "https://na2.example.com".to_string(),
        );
        node2.set_load(0.3);
        node2.add_jurisdiction("US".to_string());
        let node2_id = node2.node_id;

        mesh.register_node(node1);
        mesh.register_node(node2);

        let rule = RoutingRule::least_loaded();
        router.add_rule("US".to_string(), rule);

        let routed = router.route("US", &mesh);
        assert_eq!(routed, Some(node2_id)); // Should route to less loaded node
    }

    #[test]
    fn test_namespace_removal() {
        let namespace = GlobalNamespace::new();
        let info = NamespaceInfo::new(
            "TEST".to_string(),
            "Test Jurisdiction".to_string(),
            "Test Authority".to_string(),
        );

        namespace.register_namespace("test".to_string(), info);
        assert_eq!(namespace.namespace_count(), 1);

        assert!(namespace.remove_namespace("test"));
        assert_eq!(namespace.namespace_count(), 0);
        assert!(!namespace.remove_namespace("test"));
    }

    #[test]
    fn test_node_heartbeat() {
        let mut node = RegistryNode::new(
            GeographicRegion::Africa,
            "https://af.example.com".to_string(),
        );

        let first_heartbeat = node.last_heartbeat;
        std::thread::sleep(std::time::Duration::from_millis(10));
        node.heartbeat();

        assert!(node.last_heartbeat > first_heartbeat);
        assert!(node.is_healthy());
    }

    #[test]
    fn test_replication_log_management() {
        let manager = ReplicationManager::new(ReplicationStrategy::quorum(2));

        manager.record_replication(Uuid::new_v4(), vec![Uuid::new_v4()], 50, true);
        assert_eq!(manager.get_log().len(), 1);

        manager.clear_log();
        assert_eq!(manager.get_log().len(), 0);
    }

    #[test]
    fn test_mesh_latency_matrix() {
        let mesh = RegistryMesh::new();

        let node1_id = Uuid::new_v4();
        let node2_id = Uuid::new_v4();

        mesh.set_latency(node1_id, node2_id, 150);

        assert_eq!(mesh.get_latency(node1_id, node2_id), Some(150));
        assert_eq!(mesh.get_latency(node2_id, node1_id), Some(150)); // Symmetric
    }
}
