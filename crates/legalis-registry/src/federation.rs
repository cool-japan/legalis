use super::*;
use std::collections::BTreeSet;

// ========================================================================
// 1. Federated Registry Discovery
// ========================================================================

/// Registry metadata for federation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RegistryMetadata {
    /// Unique registry identifier
    pub registry_id: Uuid,
    /// Registry name
    pub name: String,
    /// Registry endpoint URL
    pub endpoint: String,
    /// Supported API version
    pub api_version: String,
    /// Jurisdictions covered by this registry
    pub jurisdictions: Vec<String>,
    /// Registry capabilities
    pub capabilities: Vec<RegistryCapability>,
    /// Last seen timestamp
    pub last_seen: DateTime<Utc>,
    /// Trust level (0-100)
    pub trust_level: u8,
}

impl RegistryMetadata {
    /// Creates new registry metadata.
    pub fn new(name: String, endpoint: String) -> Self {
        Self {
            registry_id: Uuid::new_v4(),
            name,
            endpoint,
            api_version: "1.0.0".to_string(),
            jurisdictions: Vec::new(),
            capabilities: Vec::new(),
            last_seen: Utc::now(),
            trust_level: 50,
        }
    }

    /// Updates the last seen timestamp.
    pub fn update_last_seen(&mut self) {
        self.last_seen = Utc::now();
    }

    /// Checks if the registry is active (seen within the last hour).
    pub fn is_active(&self) -> bool {
        let now = Utc::now();
        let elapsed = now.signed_duration_since(self.last_seen);
        elapsed.num_hours() < 1
    }
}

/// Registry capability flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegistryCapability {
    /// Supports full-text search
    FullTextSearch,
    /// Supports version control
    VersionControl,
    /// Supports real-time updates
    RealTimeUpdates,
    /// Supports event sourcing
    EventSourcing,
    /// Supports GraphQL queries
    GraphQL,
    /// Supports bulk operations
    BulkOperations,
}

/// Registry discovery service.
#[derive(Debug)]
pub struct RegistryDiscovery {
    /// Known registries
    registries: Arc<Mutex<HashMap<Uuid, RegistryMetadata>>>,
    /// Discovery interval in seconds
    #[allow(dead_code)]
    discovery_interval: u64,
}

impl RegistryDiscovery {
    /// Creates a new registry discovery service.
    pub fn new() -> Self {
        Self {
            registries: Arc::new(Mutex::new(HashMap::new())),
            discovery_interval: 300, // 5 minutes
        }
    }

    /// Registers a new registry.
    pub fn register(&self, metadata: RegistryMetadata) {
        let mut registries = self.registries.lock().expect("registries mutex poisoned");
        registries.insert(metadata.registry_id, metadata);
    }

    /// Unregisters a registry.
    pub fn unregister(&self, registry_id: Uuid) -> bool {
        let mut registries = self.registries.lock().expect("registries mutex poisoned");
        registries.remove(&registry_id).is_some()
    }

    /// Lists all registered registries.
    pub fn list_registries(&self) -> Vec<RegistryMetadata> {
        self.registries
            .lock()
            .expect("registries mutex poisoned")
            .values()
            .cloned()
            .collect()
    }

    /// Finds registries by jurisdiction.
    pub fn find_by_jurisdiction(&self, jurisdiction: &str) -> Vec<RegistryMetadata> {
        self.registries
            .lock()
            .expect("registries mutex poisoned")
            .values()
            .filter(|r| r.jurisdictions.contains(&jurisdiction.to_string()))
            .cloned()
            .collect()
    }

    /// Gets active registries only.
    pub fn get_active_registries(&self) -> Vec<RegistryMetadata> {
        self.registries
            .lock()
            .expect("registries mutex poisoned")
            .values()
            .filter(|r| r.is_active())
            .cloned()
            .collect()
    }

    /// Updates registry metadata.
    pub fn update_metadata(
        &self,
        registry_id: Uuid,
        metadata: RegistryMetadata,
    ) -> Result<(), String> {
        let mut registries = self.registries.lock().expect("registries mutex poisoned");
        if let std::collections::hash_map::Entry::Occupied(mut e) = registries.entry(registry_id) {
            e.insert(metadata);
            Ok(())
        } else {
            Err("Registry not found".to_string())
        }
    }
}

impl Default for RegistryDiscovery {
    fn default() -> Self {
        Self::new()
    }
}

// ========================================================================
// 2. Cross-Registry Statute Queries
// ========================================================================

/// Cross-registry query request.
#[derive(Debug, Clone)]
pub struct FederatedQuery {
    /// Query text
    pub query: String,
    /// Target jurisdictions
    pub jurisdictions: Vec<String>,
    /// Target registries (if empty, queries all)
    pub target_registries: Vec<Uuid>,
    /// Maximum results per registry
    pub max_results_per_registry: usize,
    /// Timeout in seconds
    pub timeout: u64,
}

impl FederatedQuery {
    /// Creates a new federated query.
    pub fn new(query: String) -> Self {
        Self {
            query,
            jurisdictions: Vec::new(),
            target_registries: Vec::new(),
            max_results_per_registry: 50,
            timeout: 30,
        }
    }

    /// Filters by jurisdictions.
    pub fn with_jurisdictions(mut self, jurisdictions: Vec<String>) -> Self {
        self.jurisdictions = jurisdictions;
        self
    }

    /// Targets specific registries.
    pub fn with_target_registries(mut self, registries: Vec<Uuid>) -> Self {
        self.target_registries = registries;
        self
    }
}

/// Federated query result from a single registry.
#[derive(Debug, Clone)]
pub struct RegistryQueryResult {
    /// Source registry ID
    pub registry_id: Uuid,
    /// Registry name
    pub registry_name: String,
    /// Matched statute IDs
    pub statute_ids: Vec<String>,
    /// Query execution time
    pub execution_time: std::time::Duration,
    /// Success flag
    pub success: bool,
    /// Error message if failed
    pub error: Option<String>,
}

/// Aggregated federated query results.
#[derive(Debug, Clone)]
pub struct FederatedQueryResult {
    /// Query text
    pub query: String,
    /// Results from each registry
    pub registry_results: Vec<RegistryQueryResult>,
    /// Total statutes found
    pub total_statutes: usize,
    /// Number of registries queried
    pub registries_queried: usize,
    /// Number of successful queries
    pub successful_queries: usize,
    /// Total execution time
    pub total_execution_time: std::time::Duration,
}

/// Cross-registry query engine.
#[derive(Debug)]
pub struct FederatedQueryEngine {
    discovery: Arc<RegistryDiscovery>,
}

impl FederatedQueryEngine {
    /// Creates a new federated query engine.
    pub fn new(discovery: Arc<RegistryDiscovery>) -> Self {
        Self { discovery }
    }

    /// Executes a federated query across multiple registries.
    pub fn execute(&self, query: FederatedQuery) -> FederatedQueryResult {
        let start = std::time::Instant::now();
        let registries = if query.target_registries.is_empty() {
            self.discovery.get_active_registries()
        } else {
            self.discovery
                .list_registries()
                .into_iter()
                .filter(|r| query.target_registries.contains(&r.registry_id))
                .collect()
        };

        let mut registry_results = Vec::new();
        let mut total_statutes = 0;

        for registry in &registries {
            let result = self.query_single_registry(registry, &query);
            total_statutes += result.statute_ids.len();
            registry_results.push(result);
        }

        let successful_queries = registry_results.iter().filter(|r| r.success).count();

        FederatedQueryResult {
            query: query.query.clone(),
            registry_results,
            total_statutes,
            registries_queried: registries.len(),
            successful_queries,
            total_execution_time: start.elapsed(),
        }
    }

    fn query_single_registry(
        &self,
        registry: &RegistryMetadata,
        _query: &FederatedQuery,
    ) -> RegistryQueryResult {
        let start = std::time::Instant::now();

        // Simulated query execution
        // In a real implementation, this would make HTTP calls to the remote registry
        RegistryQueryResult {
            registry_id: registry.registry_id,
            registry_name: registry.name.clone(),
            statute_ids: Vec::new(),
            execution_time: start.elapsed(),
            success: true,
            error: None,
        }
    }
}

// ========================================================================
// 3. Registry Peering Agreements
// ========================================================================

/// Peering agreement status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PeeringStatus {
    /// Agreement pending approval
    Pending,
    /// Agreement active
    Active,
    /// Agreement suspended
    Suspended,
    /// Agreement terminated
    Terminated,
}

/// Data sharing level in peering agreement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SharingLevel {
    /// Share public data only
    Public,
    /// Share metadata only
    Metadata,
    /// Share full statute data
    Full,
    /// Bidirectional full sharing
    Bidirectional,
}

/// Registry peering agreement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeeringAgreement {
    /// Agreement ID
    pub id: Uuid,
    /// Local registry ID
    pub local_registry: Uuid,
    /// Peer registry ID
    pub peer_registry: Uuid,
    /// Agreement status
    pub status: PeeringStatus,
    /// Data sharing level
    pub sharing_level: SharingLevel,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
    /// Expiration date
    pub expires_at: Option<DateTime<Utc>>,
    /// Terms and conditions
    pub terms: String,
}

impl PeeringAgreement {
    /// Creates a new peering agreement.
    pub fn new(local_registry: Uuid, peer_registry: Uuid, sharing_level: SharingLevel) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            local_registry,
            peer_registry,
            status: PeeringStatus::Pending,
            sharing_level,
            created_at: now,
            updated_at: now,
            expires_at: None,
            terms: String::new(),
        }
    }

    /// Activates the peering agreement.
    pub fn activate(&mut self) {
        self.status = PeeringStatus::Active;
        self.updated_at = Utc::now();
    }

    /// Suspends the peering agreement.
    pub fn suspend(&mut self) {
        self.status = PeeringStatus::Suspended;
        self.updated_at = Utc::now();
    }

    /// Terminates the peering agreement.
    pub fn terminate(&mut self) {
        self.status = PeeringStatus::Terminated;
        self.updated_at = Utc::now();
    }

    /// Checks if the agreement is active and not expired.
    pub fn is_valid(&self) -> bool {
        if self.status != PeeringStatus::Active {
            return false;
        }
        if let Some(expires_at) = self.expires_at {
            Utc::now() < expires_at
        } else {
            true
        }
    }
}

/// Peering agreement manager.
#[derive(Debug)]
pub struct PeeringManager {
    agreements: Arc<Mutex<HashMap<Uuid, PeeringAgreement>>>,
}

impl PeeringManager {
    /// Creates a new peering manager.
    pub fn new() -> Self {
        Self {
            agreements: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Creates a new peering agreement.
    pub fn create_agreement(&self, agreement: PeeringAgreement) -> Uuid {
        let id = agreement.id;
        let mut agreements = self.agreements.lock().expect("agreements mutex poisoned");
        agreements.insert(id, agreement);
        id
    }

    /// Gets a peering agreement by ID.
    pub fn get_agreement(&self, id: Uuid) -> Option<PeeringAgreement> {
        self.agreements
            .lock()
            .expect("agreements mutex poisoned")
            .get(&id)
            .cloned()
    }

    /// Lists all agreements for a registry.
    pub fn list_agreements(&self, registry_id: Uuid) -> Vec<PeeringAgreement> {
        self.agreements
            .lock()
            .expect("agreements mutex poisoned")
            .values()
            .filter(|a| a.local_registry == registry_id || a.peer_registry == registry_id)
            .cloned()
            .collect()
    }

    /// Gets active agreements for a registry.
    pub fn get_active_agreements(&self, registry_id: Uuid) -> Vec<PeeringAgreement> {
        self.list_agreements(registry_id)
            .into_iter()
            .filter(|a| a.is_valid())
            .collect()
    }

    /// Updates an agreement.
    pub fn update_agreement(&self, id: Uuid, agreement: PeeringAgreement) -> Result<(), String> {
        let mut agreements = self.agreements.lock().expect("agreements mutex poisoned");
        if let std::collections::hash_map::Entry::Occupied(mut e) = agreements.entry(id) {
            e.insert(agreement);
            Ok(())
        } else {
            Err("Agreement not found".to_string())
        }
    }

    /// Deletes an agreement.
    pub fn delete_agreement(&self, id: Uuid) -> bool {
        let mut agreements = self.agreements.lock().expect("agreements mutex poisoned");
        agreements.remove(&id).is_some()
    }
}

impl Default for PeeringManager {
    fn default() -> Self {
        Self::new()
    }
}

// ========================================================================
// 4. Federated Search Aggregation
// ========================================================================

/// Search result ranking strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RankingStrategy {
    /// Rank by relevance score
    Relevance,
    /// Rank by registry trust level
    TrustLevel,
    /// Rank by recency
    Recency,
    /// Combined ranking
    Combined,
}

/// Aggregated search result.
#[derive(Debug, Clone)]
pub struct AggregatedSearchResult {
    /// Statute ID
    pub statute_id: String,
    /// Source registry ID
    pub registry_id: Uuid,
    /// Registry name
    pub registry_name: String,
    /// Relevance score (0.0 - 1.0)
    pub relevance_score: f64,
    /// Registry trust level
    pub trust_level: u8,
    /// Combined score
    pub combined_score: f64,
}

/// Federated search aggregator.
#[derive(Debug)]
pub struct FederatedSearchAggregator {
    ranking_strategy: RankingStrategy,
}

impl FederatedSearchAggregator {
    /// Creates a new search aggregator.
    pub fn new(ranking_strategy: RankingStrategy) -> Self {
        Self { ranking_strategy }
    }

    /// Aggregates results from multiple registries.
    pub fn aggregate(
        &self,
        federated_result: &FederatedQueryResult,
    ) -> Vec<AggregatedSearchResult> {
        let mut results = Vec::new();

        for registry_result in &federated_result.registry_results {
            if !registry_result.success {
                continue;
            }

            for statute_id in &registry_result.statute_ids {
                let result = AggregatedSearchResult {
                    statute_id: statute_id.clone(),
                    registry_id: registry_result.registry_id,
                    registry_name: registry_result.registry_name.clone(),
                    relevance_score: 1.0, // Would be calculated based on query match
                    trust_level: 50,      // Would come from registry metadata
                    combined_score: 0.0,
                };
                results.push(result);
            }
        }

        self.rank_results(&mut results);
        results
    }

    fn rank_results(&self, results: &mut [AggregatedSearchResult]) {
        for result in results.iter_mut() {
            result.combined_score = match self.ranking_strategy {
                RankingStrategy::Relevance => result.relevance_score,
                RankingStrategy::TrustLevel => f64::from(result.trust_level) / 100.0,
                RankingStrategy::Recency => 0.5, // Placeholder
                RankingStrategy::Combined => {
                    (result.relevance_score * 0.5) + (f64::from(result.trust_level) / 100.0 * 0.5)
                }
            };
        }

        results.sort_by(|a, b| {
            b.combined_score
                .partial_cmp(&a.combined_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    /// Deduplicates results across registries.
    pub fn deduplicate(&self, results: Vec<AggregatedSearchResult>) -> Vec<AggregatedSearchResult> {
        let mut seen = BTreeSet::new();
        results
            .into_iter()
            .filter(|r| seen.insert(r.statute_id.clone()))
            .collect()
    }
}

// ========================================================================
// 5. Trust Frameworks for Federation
// ========================================================================

/// Trust level category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrustLevel {
    /// Untrusted (0-20)
    Untrusted,
    /// Low trust (21-40)
    Low,
    /// Medium trust (41-60)
    Medium,
    /// High trust (61-80)
    High,
    /// Verified (81-100)
    Verified,
}

impl TrustLevel {
    /// Converts a numeric score to a trust level.
    pub fn from_score(score: u8) -> Self {
        match score {
            0..=20 => TrustLevel::Untrusted,
            21..=40 => TrustLevel::Low,
            41..=60 => TrustLevel::Medium,
            61..=80 => TrustLevel::High,
            81..=100 => TrustLevel::Verified,
            _ => TrustLevel::Medium,
        }
    }

    /// Converts trust level to numeric score.
    pub fn to_score(&self) -> u8 {
        match self {
            TrustLevel::Untrusted => 10,
            TrustLevel::Low => 30,
            TrustLevel::Medium => 50,
            TrustLevel::High => 70,
            TrustLevel::Verified => 90,
        }
    }
}

/// Trust metric for a registry.
#[derive(Debug, Clone)]
pub struct TrustMetric {
    /// Registry ID
    pub registry_id: Uuid,
    /// Uptime percentage (0-100)
    pub uptime: f64,
    /// Response time average (ms)
    pub avg_response_time: u64,
    /// Successful queries percentage
    pub success_rate: f64,
    /// Data quality score (0-100)
    pub data_quality: u8,
    /// Community reputation score (0-100)
    pub reputation: u8,
    /// Calculated trust score
    pub trust_score: u8,
}

impl TrustMetric {
    /// Creates a new trust metric.
    pub fn new(registry_id: Uuid) -> Self {
        Self {
            registry_id,
            uptime: 100.0,
            avg_response_time: 100,
            success_rate: 100.0,
            data_quality: 50,
            reputation: 50,
            trust_score: 50,
        }
    }

    /// Calculates the trust score based on metrics.
    pub fn calculate_trust_score(&mut self) {
        let uptime_score = self.uptime;
        let response_score = if self.avg_response_time < 100 {
            100.0
        } else if self.avg_response_time < 500 {
            80.0
        } else if self.avg_response_time < 1000 {
            60.0
        } else {
            40.0
        };
        let success_score = self.success_rate;

        self.trust_score = ((uptime_score * 0.3)
            + (response_score * 0.2)
            + (success_score * 0.2)
            + (f64::from(self.data_quality) * 0.15)
            + (f64::from(self.reputation) * 0.15)) as u8;
    }

    /// Gets the trust level category.
    pub fn trust_level(&self) -> TrustLevel {
        TrustLevel::from_score(self.trust_score)
    }
}

/// Trust framework manager.
#[derive(Debug)]
pub struct TrustFramework {
    metrics: Arc<Mutex<HashMap<Uuid, TrustMetric>>>,
}

impl TrustFramework {
    /// Creates a new trust framework.
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Updates trust metrics for a registry.
    pub fn update_metrics(&self, metric: TrustMetric) {
        let mut metrics = self.metrics.lock().expect("metrics mutex poisoned");
        metrics.insert(metric.registry_id, metric);
    }

    /// Gets trust metrics for a registry.
    pub fn get_metrics(&self, registry_id: Uuid) -> Option<TrustMetric> {
        self.metrics
            .lock()
            .expect("metrics mutex poisoned")
            .get(&registry_id)
            .cloned()
    }

    /// Gets trust score for a registry.
    pub fn get_trust_score(&self, registry_id: Uuid) -> u8 {
        self.metrics
            .lock()
            .expect("metrics mutex poisoned")
            .get(&registry_id)
            .map(|m| m.trust_score)
            .unwrap_or(50)
    }

    /// Lists all registries by trust level.
    pub fn list_by_trust_level(&self, min_level: TrustLevel) -> Vec<Uuid> {
        self.metrics
            .lock()
            .expect("metrics mutex poisoned")
            .values()
            .filter(|m| m.trust_level() >= min_level)
            .map(|m| m.registry_id)
            .collect()
    }

    /// Recalculates all trust scores.
    pub fn recalculate_all(&self) {
        let mut metrics = self.metrics.lock().expect("metrics mutex poisoned");
        for metric in metrics.values_mut() {
            metric.calculate_trust_score();
        }
    }
}

impl Default for TrustFramework {
    fn default() -> Self {
        Self::new()
    }
}
