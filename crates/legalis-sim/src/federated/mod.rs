//! Federated Simulation (v0.3.2).
//!
//! This module lets several mutually-distrustful organizations collaborate on a
//! simulation / model without sharing their raw data. It combines four real
//! building blocks:
//!
//! - **Privacy-preserving distributed simulation** — every organization keeps an
//!   isolated [`DataPartition`]; the [`FederationHub`] and [`FederatedSimulation`]
//!   orchestrate training and aggregation so that no raw records ever leave the
//!   owning organization (see [`DataPartition::access`]).
//! - **Federated learning** — per-organization [`LocalModel`]s are combined with
//!   [`FederatedAveraging`] (FedAvg / DP-FedAvg). See [`learning`].
//! - **Secure multi-party computation** — the summation step of FedAvg runs
//!   through a [`SecureAggregator`] built on additive secret sharing, so the hub
//!   only ever observes the aggregate. See [`secure_mpc`].
//! - **Differential privacy** — Laplace / Gaussian mechanisms protect shared
//!   aggregates, with a [`PrivacyAccountant`] enforcing an `(ε, δ)` budget. See
//!   [`differential_privacy`].
//!
//! Cross-organization result sharing is mediated by a [`SharingPolicy`] (minimum
//! cohort size, mandatory DP, raw-access prohibition) and produces auditable
//! [`SharedResult`]s and [`PrivateMetricsSummary`]s that reuse the crate's
//! [`SimulationMetrics`].

pub mod differential_privacy;
pub mod learning;
pub mod secure_mpc;

pub use differential_privacy::*;
pub use learning::*;
pub use secure_mpc::*;

use crate::SimulationMetrics;
use crate::error::{SimResult, SimulationError};
use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Identifier for a participating organization.
pub type OrgId = String;

/// A single private training record (feature vector and label).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalRecord {
    /// Feature vector.
    pub features: Vec<f64>,
    /// Target label (typically `0.0` or `1.0`).
    pub label: f64,
}

/// An organization's isolated data partition.
///
/// Raw records are private to the owning organization: [`DataPartition::access`]
/// returns an error for any requester other than the owner, which is how
/// partition isolation is enforced at the API boundary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPartition {
    /// Organization that owns (and may access) this partition.
    owner: OrgId,
    /// The private records.
    records: Vec<LocalRecord>,
}

impl DataPartition {
    /// Creates an empty partition owned by `owner`.
    pub fn new(owner: OrgId) -> Self {
        Self {
            owner,
            records: Vec::new(),
        }
    }

    /// Returns the owning organization's identifier.
    pub fn owner(&self) -> &str {
        &self.owner
    }

    /// Appends a private record.
    pub fn add_record(&mut self, features: Vec<f64>, label: f64) {
        self.records.push(LocalRecord { features, label });
    }

    /// Returns the number of records.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Returns whether the partition is empty.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Accesses the raw records, enforcing partition isolation.
    ///
    /// Only the owning organization (`requester == owner`) may read raw records.
    pub fn access(&self, requester: &str) -> SimResult<&[LocalRecord]> {
        if requester != self.owner {
            return Err(SimulationError::InvalidParameter(format!(
                "partition isolation violation: '{}' may not access data owned by '{}'",
                requester, self.owner
            )));
        }
        Ok(&self.records)
    }

    /// Builds owner-only `(features, label)` training pairs.
    fn training_data(&self, requester: &str) -> SimResult<Vec<(Vec<f64>, f64)>> {
        Ok(self
            .access(requester)?
            .iter()
            .map(|r| (r.features.clone(), r.label))
            .collect())
    }
}

/// A participating organization: its isolated data, local model, and DP budget.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederatedOrganization {
    /// Unique organization identifier.
    pub id: OrgId,
    /// Human-readable name.
    pub name: String,
    /// Isolated private data partition.
    pub partition: DataPartition,
    /// Local model trained on the private partition.
    pub local_model: LocalModel,
    /// Per-organization privacy accountant.
    pub accountant: PrivacyAccountant,
}

impl FederatedOrganization {
    /// Creates a new organization with an empty partition and zeroed model.
    pub fn new(
        id: OrgId,
        name: String,
        num_features: usize,
        learning_rate: f64,
        budget: PrivacyBudget,
    ) -> SimResult<Self> {
        let partition = DataPartition::new(id.clone());
        let local_model = LocalModel::new(id.clone(), num_features, learning_rate)?;
        let accountant = PrivacyAccountant::new(budget.epsilon, budget.delta)?;
        Ok(Self {
            id,
            name,
            partition,
            local_model,
            accountant,
        })
    }

    /// Adds a private record, validating its dimensionality.
    pub fn add_record(&mut self, features: Vec<f64>, label: f64) -> SimResult<()> {
        if features.len() != self.local_model.num_features() {
            return Err(SimulationError::InvalidParameter(format!(
                "record has {} features, model expects {}",
                features.len(),
                self.local_model.num_features()
            )));
        }
        self.partition.add_record(features, label);
        Ok(())
    }

    /// Returns the number of local samples.
    pub fn sample_count(&self) -> usize {
        self.partition.len()
    }

    /// Trains the local model on the private partition and emits an update.
    ///
    /// Returns `(final_loss, update)`.
    fn local_update(&mut self, epochs: usize) -> SimResult<(f64, ModelUpdate)> {
        let data = self.partition.training_data(&self.id)?;
        if data.is_empty() {
            return Err(SimulationError::InvalidParameter(format!(
                "organization '{}' has no local data to train on",
                self.id
            )));
        }
        let loss = self.local_model.train(&data, epochs)?;
        Ok((loss, self.local_model.to_update(data.len())))
    }
}

/// Policy governing what may be shared across organizations.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SharingPolicy {
    /// Minimum number of underlying individuals an aggregate must cover (k-anonymity).
    pub min_cohort_size: usize,
    /// Whether shared aggregates must carry differential-privacy noise.
    pub require_dp: bool,
    /// Whether cross-organization raw-data access is permitted at all.
    pub allow_raw_access: bool,
}

impl Default for SharingPolicy {
    fn default() -> Self {
        Self {
            min_cohort_size: 10,
            require_dp: true,
            allow_raw_access: false,
        }
    }
}

impl SharingPolicy {
    /// Creates a sharing policy.
    pub fn new(min_cohort_size: usize, require_dp: bool, allow_raw_access: bool) -> Self {
        Self {
            min_cohort_size,
            require_dp,
            allow_raw_access,
        }
    }
}

/// An auditable, privacy-preserving aggregate that may be shared across organizations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedResult {
    /// Organizations that contributed to the aggregate.
    pub source_orgs: Vec<OrgId>,
    /// Named aggregate metrics (DP-noised when `dp_applied`).
    pub aggregate_metrics: HashMap<String, f64>,
    /// Number of underlying individuals covered by the aggregate.
    pub cohort_size: usize,
    /// Total `ε` spent producing this result.
    pub epsilon_spent: f64,
    /// Whether differential-privacy noise was applied.
    pub dp_applied: bool,
}

/// Report summarising one federated training round.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundReport {
    /// Completed round number.
    pub round: usize,
    /// Number of organizations that participated.
    pub participating_orgs: usize,
    /// Total local samples across all participants.
    pub total_samples: usize,
    /// Mean local loss across participants (post-training, pre-aggregation).
    pub average_local_loss: f64,
    /// Resulting global parameters.
    pub global_parameters: Vec<f64>,
    /// Whether differential-privacy noise was added to the global model.
    pub dp_noise_applied: bool,
    /// Whether secure aggregation was used for the summation step.
    pub secure_aggregation: bool,
}

/// A differentially-private cross-organization summary built from [`SimulationMetrics`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateMetricsSummary {
    /// Number of organizations that contributed.
    pub contributing_orgs: usize,
    /// True total number of law applications (cohort size for the policy check).
    pub cohort_size: usize,
    /// DP-noised total number of law applications.
    pub total_applications: f64,
    /// DP-noised deterministic-outcome count.
    pub deterministic_count: f64,
    /// DP-noised discretionary-outcome count.
    pub discretion_count: f64,
    /// DP-noised void-outcome count.
    pub void_count: f64,
    /// Total `ε` spent producing this summary.
    pub epsilon_spent: f64,
}

/// Central coordinator for a federation of organizations.
///
/// The hub registers organizations, runs federated rounds (local training,
/// secure aggregation, optional DP), enforces the [`SharingPolicy`] on every
/// cross-organization release, and charges a dedicated release privacy budget.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationHub {
    /// Registered organizations keyed by identifier.
    organizations: HashMap<OrgId, FederatedOrganization>,
    /// Stable registration order (for deterministic round iteration).
    order: Vec<OrgId>,
    /// Cross-organization sharing policy.
    policy: SharingPolicy,
    /// FedAvg coordinator (holds the global model and optional DP-FedAvg config).
    coordinator: FederatedAveraging,
    /// Number of features each organization's model uses.
    num_features: usize,
    /// Fixed-point scale used by the secure aggregator.
    secure_scale: f64,
    /// Privacy budget governing cross-organization releases.
    release_accountant: PrivacyAccountant,
}

impl FederationHub {
    /// Creates a federation hub.
    ///
    /// `release_budget` governs the hub's cross-organization releases (shared
    /// aggregates and private metric summaries).
    pub fn new(
        num_features: usize,
        policy: SharingPolicy,
        release_budget: PrivacyBudget,
    ) -> SimResult<Self> {
        if num_features == 0 {
            return Err(SimulationError::InvalidParameter(
                "num_features must be greater than zero".to_string(),
            ));
        }
        Ok(Self {
            organizations: HashMap::new(),
            order: Vec::new(),
            policy,
            coordinator: FederatedAveraging::new(num_features + 1)?,
            num_features,
            secure_scale: 65536.0,
            release_accountant: PrivacyAccountant::new(
                release_budget.epsilon,
                release_budget.delta,
            )?,
        })
    }

    /// Enables DP-FedAvg for federated rounds.
    pub fn enable_dp_fedavg(&mut self, config: DpFedConfig) {
        self.coordinator.set_dp(config);
    }

    /// Registers an organization. Errors on dimension mismatch or duplicate id.
    pub fn register(&mut self, org: FederatedOrganization) -> SimResult<()> {
        if org.local_model.num_features() != self.num_features {
            return Err(SimulationError::InvalidParameter(format!(
                "organization '{}' model has {} features, hub expects {}",
                org.id,
                org.local_model.num_features(),
                self.num_features
            )));
        }
        if self.organizations.contains_key(&org.id) {
            return Err(SimulationError::InvalidParameter(format!(
                "organization '{}' is already registered",
                org.id
            )));
        }
        self.order.push(org.id.clone());
        self.organizations.insert(org.id.clone(), org);
        Ok(())
    }

    /// Returns the number of registered organizations.
    pub fn num_organizations(&self) -> usize {
        self.organizations.len()
    }

    /// Returns a registered organization by identifier.
    pub fn organization(&self, id: &str) -> Option<&FederatedOrganization> {
        self.organizations.get(id)
    }

    /// Returns the sharing policy.
    pub fn policy(&self) -> &SharingPolicy {
        &self.policy
    }

    /// Returns the FedAvg coordinator.
    pub fn coordinator(&self) -> &FederatedAveraging {
        &self.coordinator
    }

    /// Returns the hub's release privacy accountant.
    pub fn release_accountant(&self) -> &PrivacyAccountant {
        &self.release_accountant
    }

    /// Attempts raw cross-organization data access, enforcing the sharing policy.
    ///
    /// Cross-organization raw access is denied unless the policy explicitly allows
    /// it; even then, partition isolation still applies.
    pub fn request_raw_access(&self, requester: &str, target: &str) -> SimResult<&[LocalRecord]> {
        if requester != target && !self.policy.allow_raw_access {
            return Err(SimulationError::InvalidParameter(format!(
                "sharing policy forbids cross-organization raw access from '{}' to '{}'",
                requester, target
            )));
        }
        let org = self.organizations.get(target).ok_or_else(|| {
            SimulationError::InvalidParameter(format!("unknown organization '{}'", target))
        })?;
        org.partition.access(requester)
    }

    /// Runs one privacy-preserving federated round.
    ///
    /// Steps: (1) each organization trains locally on its isolated partition;
    /// (2) the per-organization parameter vectors are combined through secure
    /// aggregation; (3) the global model is formed (sample-weighted FedAvg, or
    /// clipped unweighted mean plus Gaussian noise under DP-FedAvg); (4) the new
    /// global model is distributed back to every organization.
    pub fn run_round<R: RngExt>(
        &mut self,
        local_epochs: usize,
        rng: &mut R,
    ) -> SimResult<RoundReport> {
        let num_orgs = self.organizations.len();
        if num_orgs < 2 {
            return Err(SimulationError::InvalidParameter(
                "a federated round requires at least 2 organizations".to_string(),
            ));
        }

        // Stable order avoids borrowing `self.order` while mutating organizations.
        let order = self.order.clone();

        // 1. Local training on isolated partitions.
        let mut updates = Vec::with_capacity(num_orgs);
        let mut losses = Vec::with_capacity(num_orgs);
        for id in &order {
            let org = self.organizations.get_mut(id).ok_or_else(|| {
                SimulationError::InvalidParameter(format!("organization '{}' vanished", id))
            })?;
            let (loss, update) = org.local_update(local_epochs)?;
            losses.push(loss);
            updates.push(update);
        }

        let total_samples: usize = updates.iter().map(|u| u.num_samples).sum();
        if total_samples == 0 {
            return Err(SimulationError::InvalidParameter(
                "total sample count across organizations is zero".to_string(),
            ));
        }

        let dp_enabled = self.coordinator.dp_enabled();
        let dp_config = self.coordinator.dp_config();

        // 2. Build per-organization vectors, then securely sum them.
        let vectors: Vec<Vec<f64>> = updates
            .iter()
            .map(|update| {
                let mut params = update.parameters.clone();
                if let Some(config) = dp_config {
                    clip_l2_norm(&mut params, config.clip_norm);
                } else {
                    let weight = update.num_samples as f64;
                    for v in params.iter_mut() {
                        *v *= weight;
                    }
                }
                params
            })
            .collect();

        let aggregator = SecureAggregator::new(num_orgs, self.secure_scale)?;
        let mut global = aggregator.aggregate_vectors(&vectors, rng)?;

        // 3. Finalise the global model.
        let mut dp_noise_applied = false;
        if let Some(config) = dp_config {
            let n = num_orgs as f64;
            for v in global.iter_mut() {
                *v /= n;
            }
            let mechanism =
                GaussianMechanism::new(config.clip_norm / n, config.epsilon, config.delta)?;
            for v in global.iter_mut() {
                *v = mechanism.add_noise(*v, rng);
            }
            dp_noise_applied = true;
            // Charge each organization's own budget for participating in the round.
            let params = PrivacyParams::new(config.epsilon, config.delta)?;
            for id in &order {
                if let Some(org) = self.organizations.get_mut(id) {
                    org.accountant.spend(params)?;
                }
            }
        } else {
            let total = total_samples as f64;
            for v in global.iter_mut() {
                *v /= total;
            }
        }

        self.coordinator.set_global(global.clone())?;

        // 4. Distribute the new global model back to every organization.
        for id in &order {
            if let Some(org) = self.organizations.get_mut(id) {
                org.local_model.set_parameters(&global)?;
            }
        }

        let average_local_loss = losses.iter().sum::<f64>() / losses.len() as f64;
        let _ = dp_enabled; // dp state captured via dp_noise_applied

        Ok(RoundReport {
            round: self.coordinator.round(),
            participating_orgs: num_orgs,
            total_samples,
            average_local_loss,
            global_parameters: global,
            dp_noise_applied,
            secure_aggregation: true,
        })
    }

    /// Produces a shareable aggregate of one named metric across organizations.
    ///
    /// Enforces the minimum cohort size and (when required) applies Laplace noise,
    /// charging the hub's release budget. `per_org_values` maps each contributing
    /// organization to its local scalar value for `metric_name`.
    pub fn share_aggregate<R: RngExt>(
        &mut self,
        metric_name: &str,
        per_org_values: &[(OrgId, f64)],
        mechanism: &LaplaceMechanism,
        rng: &mut R,
    ) -> SimResult<SharedResult> {
        if per_org_values.is_empty() {
            return Err(SimulationError::InvalidParameter(
                "no contributing organizations".to_string(),
            ));
        }

        let mut cohort = 0usize;
        for (id, _) in per_org_values {
            let org = self.organizations.get(id).ok_or_else(|| {
                SimulationError::InvalidParameter(format!(
                    "organization '{}' is not registered",
                    id
                ))
            })?;
            cohort += org.sample_count();
        }
        if cohort < self.policy.min_cohort_size {
            return Err(SimulationError::InvalidParameter(format!(
                "cohort size {} is below the policy minimum of {}",
                cohort, self.policy.min_cohort_size
            )));
        }

        let raw_sum: f64 = per_org_values.iter().map(|(_, v)| v).sum();
        let mut aggregate_metrics = HashMap::new();
        let mut epsilon_spent = 0.0;
        let mut dp_applied = false;

        if self.policy.require_dp {
            let params = mechanism.privacy_params();
            self.release_accountant.spend(params)?;
            aggregate_metrics.insert(metric_name.to_string(), mechanism.add_noise(raw_sum, rng));
            epsilon_spent = params.epsilon;
            dp_applied = true;
        } else {
            aggregate_metrics.insert(metric_name.to_string(), raw_sum);
        }

        Ok(SharedResult {
            source_orgs: per_org_values.iter().map(|(id, _)| id.clone()).collect(),
            aggregate_metrics,
            cohort_size: cohort,
            epsilon_spent,
            dp_applied,
        })
    }

    /// Produces a differentially-private cross-organization summary of [`SimulationMetrics`].
    ///
    /// The four outcome counts are summed across organizations and (when the
    /// policy requires DP) independently perturbed with the supplied Laplace
    /// mechanism, charging the hub's release budget once per count.
    pub fn private_metrics_summary<R: RngExt>(
        &mut self,
        per_org: &[(OrgId, SimulationMetrics)],
        mechanism: &LaplaceMechanism,
        rng: &mut R,
    ) -> SimResult<PrivateMetricsSummary> {
        if per_org.is_empty() {
            return Err(SimulationError::InvalidParameter(
                "no contributing organizations".to_string(),
            ));
        }

        let total: usize = per_org.iter().map(|(_, m)| m.total_applications).sum();
        if total < self.policy.min_cohort_size {
            return Err(SimulationError::InvalidParameter(format!(
                "cohort size {} is below the policy minimum of {}",
                total, self.policy.min_cohort_size
            )));
        }

        let deterministic: usize = per_org.iter().map(|(_, m)| m.deterministic_count).sum();
        let discretion: usize = per_org.iter().map(|(_, m)| m.discretion_count).sum();
        let void: usize = per_org.iter().map(|(_, m)| m.void_count).sum();

        let (apps, det, disc, vd, epsilon_spent) = if self.policy.require_dp {
            let params = mechanism.privacy_params();
            // Four counting queries under sequential composition.
            for _ in 0..4 {
                self.release_accountant.spend(params)?;
            }
            (
                mechanism.add_noise(total as f64, rng),
                mechanism.add_noise(deterministic as f64, rng),
                mechanism.add_noise(discretion as f64, rng),
                mechanism.add_noise(void as f64, rng),
                params.epsilon * 4.0,
            )
        } else {
            (
                total as f64,
                deterministic as f64,
                discretion as f64,
                void as f64,
                0.0,
            )
        };

        Ok(PrivateMetricsSummary {
            contributing_orgs: per_org.len(),
            cohort_size: total,
            total_applications: apps,
            deterministic_count: det,
            discretion_count: disc,
            void_count: vd,
            epsilon_spent,
        })
    }
}

/// Top-level privacy-preserving federated simulation.
///
/// Wraps a [`FederationHub`] and drives multi-round federated training, retaining
/// a history of [`RoundReport`]s.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederatedSimulation {
    /// The underlying federation hub.
    hub: FederationHub,
    /// History of completed rounds.
    history: Vec<RoundReport>,
}

impl FederatedSimulation {
    /// Creates a federated simulation around an existing hub.
    pub fn new(hub: FederationHub) -> Self {
        Self {
            hub,
            history: Vec::new(),
        }
    }

    /// Returns a shared reference to the hub.
    pub fn hub(&self) -> &FederationHub {
        &self.hub
    }

    /// Returns a mutable reference to the hub.
    pub fn hub_mut(&mut self) -> &mut FederationHub {
        &mut self.hub
    }

    /// Returns the recorded round history.
    pub fn history(&self) -> &[RoundReport] {
        &self.history
    }

    /// Runs `rounds` federated training rounds, returning the per-round reports.
    pub fn train<R: RngExt>(
        &mut self,
        rounds: usize,
        local_epochs: usize,
        rng: &mut R,
    ) -> SimResult<Vec<RoundReport>> {
        if rounds == 0 {
            return Err(SimulationError::InvalidParameter(
                "rounds must be greater than zero".to_string(),
            ));
        }
        let mut reports = Vec::with_capacity(rounds);
        for _ in 0..rounds {
            let report = self.hub.run_round(local_epochs, rng)?;
            reports.push(report.clone());
            self.history.push(report);
        }
        Ok(reports)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{SeedableRng, rngs::StdRng};

    fn budget() -> PrivacyBudget {
        PrivacyBudget {
            epsilon: 100.0,
            delta: 1e-3,
        }
    }

    fn make_org(id: &str, base: f64) -> FederatedOrganization {
        let mut org =
            FederatedOrganization::new(id.to_string(), id.to_string(), 1, 0.2, budget()).unwrap();
        for i in 1..=15 {
            org.add_record(vec![base + i as f64], 1.0).unwrap();
            org.add_record(vec![base - i as f64], 0.0).unwrap();
        }
        org
    }

    #[test]
    fn test_partition_isolation() {
        let mut partition = DataPartition::new("org-a".to_string());
        partition.add_record(vec![1.0], 1.0);
        assert_eq!(partition.len(), 1);
        assert!(partition.access("org-a").is_ok());
        assert!(partition.access("org-b").is_err());
    }

    #[test]
    fn test_organization_record_dim_check() {
        let mut org =
            FederatedOrganization::new("a".to_string(), "A".to_string(), 2, 0.1, budget()).unwrap();
        assert!(org.add_record(vec![1.0, 2.0], 1.0).is_ok());
        assert!(org.add_record(vec![1.0], 1.0).is_err());
        assert_eq!(org.sample_count(), 1);
    }

    #[test]
    fn test_hub_registration_and_errors() {
        let mut hub = FederationHub::new(1, SharingPolicy::default(), budget()).unwrap();
        hub.register(make_org("a", 0.0)).unwrap();
        assert_eq!(hub.num_organizations(), 1);

        // Duplicate id rejected.
        assert!(hub.register(make_org("a", 0.0)).is_err());

        // Wrong feature dimension rejected.
        let wrong =
            FederatedOrganization::new("b".to_string(), "B".to_string(), 3, 0.1, budget()).unwrap();
        assert!(hub.register(wrong).is_err());
    }

    #[test]
    fn test_sharing_policy_default() {
        let policy = SharingPolicy::default();
        assert_eq!(policy.min_cohort_size, 10);
        assert!(policy.require_dp);
        assert!(!policy.allow_raw_access);
    }

    #[test]
    fn test_request_raw_access_denied_and_allowed() {
        let mut strict = FederationHub::new(1, SharingPolicy::default(), budget()).unwrap();
        strict.register(make_org("a", 0.0)).unwrap();
        strict.register(make_org("b", 0.0)).unwrap();
        // Owner may access its own data; cross-org is denied.
        assert!(strict.request_raw_access("a", "a").is_ok());
        assert!(strict.request_raw_access("a", "b").is_err());

        // Even with raw access allowed, partition isolation still blocks
        // impersonation of a different owner.
        let mut open = FederationHub::new(1, SharingPolicy::new(10, true, true), budget()).unwrap();
        open.register(make_org("a", 0.0)).unwrap();
        open.register(make_org("b", 0.0)).unwrap();
        assert!(open.request_raw_access("a", "b").is_err());
    }

    #[test]
    fn test_run_round_requires_two_orgs() {
        let mut rng = StdRng::seed_from_u64(1);
        let mut hub = FederationHub::new(1, SharingPolicy::default(), budget()).unwrap();
        hub.register(make_org("a", 0.0)).unwrap();
        assert!(hub.run_round(1, &mut rng).is_err());
    }

    #[test]
    fn test_run_round_trains_and_distributes() {
        let mut rng = StdRng::seed_from_u64(2);
        let mut hub = FederationHub::new(1, SharingPolicy::default(), budget()).unwrap();
        hub.register(make_org("a", 0.0)).unwrap();
        hub.register(make_org("b", 0.0)).unwrap();

        let report = hub.run_round(5, &mut rng).unwrap();
        assert_eq!(report.round, 1);
        assert_eq!(report.participating_orgs, 2);
        assert_eq!(report.total_samples, 60);
        assert!(report.secure_aggregation);
        assert!(!report.dp_noise_applied);

        // Global model distributed back: every org now shares the global params.
        let global = hub.coordinator().global_parameters().to_vec();
        for id in ["a", "b"] {
            let org = hub.organization(id).unwrap();
            assert_eq!(org.local_model.parameters(), global);
        }
    }

    #[test]
    fn test_run_round_with_dp() {
        let mut rng = StdRng::seed_from_u64(3);
        let mut hub = FederationHub::new(1, SharingPolicy::default(), budget()).unwrap();
        hub.enable_dp_fedavg(DpFedConfig::new(2.0, 0.5, 1e-5).unwrap());
        hub.register(make_org("a", 0.0)).unwrap();
        hub.register(make_org("b", 0.0)).unwrap();

        let report = hub.run_round(3, &mut rng).unwrap();
        assert!(report.dp_noise_applied);
        // Each organization's per-round privacy budget was charged.
        assert_eq!(hub.organization("a").unwrap().accountant.num_queries(), 1);
    }

    #[test]
    fn test_share_aggregate_enforces_cohort() {
        let mut rng = StdRng::seed_from_u64(4);
        // Require a cohort larger than the two orgs can supply (60 records).
        let policy = SharingPolicy::new(1000, true, false);
        let mut hub = FederationHub::new(1, policy, budget()).unwrap();
        hub.register(make_org("a", 0.0)).unwrap();
        hub.register(make_org("b", 0.0)).unwrap();

        let mech = LaplaceMechanism::new(1.0, 0.5).unwrap();
        let values = vec![("a".to_string(), 10.0), ("b".to_string(), 20.0)];
        assert!(
            hub.share_aggregate("cost", &values, &mech, &mut rng)
                .is_err()
        );
    }

    #[test]
    fn test_share_aggregate_dp_charges_budget() {
        let mut rng = StdRng::seed_from_u64(5);
        let mut hub = FederationHub::new(1, SharingPolicy::default(), budget()).unwrap();
        hub.register(make_org("a", 0.0)).unwrap();
        hub.register(make_org("b", 0.0)).unwrap();

        let mech = LaplaceMechanism::new(1.0, 0.5).unwrap();
        let values = vec![("a".to_string(), 10.0), ("b".to_string(), 20.0)];
        let result = hub
            .share_aggregate("cost", &values, &mech, &mut rng)
            .unwrap();
        assert!(result.dp_applied);
        assert_eq!(result.source_orgs.len(), 2);
        assert_eq!(result.cohort_size, 60);
        assert!((result.epsilon_spent - 0.5).abs() < 1e-9);
        assert!((hub.release_accountant().spent_epsilon() - 0.5).abs() < 1e-9);
        assert!(result.aggregate_metrics.contains_key("cost"));
    }

    #[test]
    fn test_share_aggregate_unknown_org() {
        let mut rng = StdRng::seed_from_u64(6);
        let mut hub = FederationHub::new(1, SharingPolicy::new(0, false, false), budget()).unwrap();
        hub.register(make_org("a", 0.0)).unwrap();
        hub.register(make_org("b", 0.0)).unwrap();
        let mech = LaplaceMechanism::new(1.0, 0.5).unwrap();
        let values = vec![("ghost".to_string(), 1.0)];
        assert!(hub.share_aggregate("x", &values, &mech, &mut rng).is_err());
    }

    #[test]
    fn test_private_metrics_summary() {
        let mut rng = StdRng::seed_from_u64(7);
        let mut hub = FederationHub::new(1, SharingPolicy::new(5, true, false), budget()).unwrap();
        hub.register(make_org("a", 0.0)).unwrap();
        hub.register(make_org("b", 0.0)).unwrap();

        let mut m1 = SimulationMetrics::new();
        m1.total_applications = 100;
        m1.deterministic_count = 70;
        m1.discretion_count = 20;
        m1.void_count = 10;
        let mut m2 = SimulationMetrics::new();
        m2.total_applications = 50;
        m2.deterministic_count = 30;
        m2.discretion_count = 15;
        m2.void_count = 5;

        let mech = LaplaceMechanism::new(2.0, 0.2).unwrap();
        let per_org = vec![("a".to_string(), m1), ("b".to_string(), m2)];
        let summary = hub
            .private_metrics_summary(&per_org, &mech, &mut rng)
            .unwrap();

        assert_eq!(summary.contributing_orgs, 2);
        assert_eq!(summary.cohort_size, 150);
        // Four counting queries charged.
        assert!((summary.epsilon_spent - 0.8).abs() < 1e-9);
        assert_eq!(hub.release_accountant().num_queries(), 4);
        // DP-noised totals stay reasonably close to the true sums.
        assert!((summary.total_applications - 150.0).abs() < 60.0);
    }

    #[test]
    fn test_private_metrics_summary_cohort_too_small() {
        let mut rng = StdRng::seed_from_u64(8);
        let mut hub =
            FederationHub::new(1, SharingPolicy::new(10_000, true, false), budget()).unwrap();
        hub.register(make_org("a", 0.0)).unwrap();
        hub.register(make_org("b", 0.0)).unwrap();

        let mut m = SimulationMetrics::new();
        m.total_applications = 5;
        let mech = LaplaceMechanism::new(1.0, 0.5).unwrap();
        let per_org = vec![("a".to_string(), m)];
        assert!(
            hub.private_metrics_summary(&per_org, &mech, &mut rng)
                .is_err()
        );
    }

    #[test]
    fn test_federated_simulation_multi_round() {
        let mut rng = StdRng::seed_from_u64(9);
        let mut hub = FederationHub::new(1, SharingPolicy::default(), budget()).unwrap();
        hub.register(make_org("a", 0.0)).unwrap();
        hub.register(make_org("b", 0.0)).unwrap();

        let mut sim = FederatedSimulation::new(hub);
        let reports = sim.train(4, 5, &mut rng).unwrap();
        assert_eq!(reports.len(), 4);
        assert_eq!(sim.history().len(), 4);
        assert_eq!(reports[3].round, 4);
        // Federated training should reduce the average local loss over rounds.
        assert!(reports[3].average_local_loss < reports[0].average_local_loss);

        assert!(sim.train(0, 1, &mut rng).is_err());
    }

    #[test]
    fn test_federated_learning_converges_to_consensus() {
        let mut rng = StdRng::seed_from_u64(10);
        let mut hub = FederationHub::new(1, SharingPolicy::default(), budget()).unwrap();
        hub.register(make_org("a", 0.0)).unwrap();
        hub.register(make_org("b", 0.0)).unwrap();

        let mut sim = FederatedSimulation::new(hub);
        sim.train(10, 5, &mut rng).unwrap();

        // The shared global model classifies the separable pattern correctly.
        let org = sim.hub().organization("a").unwrap();
        assert!(org.local_model.predict(&[20.0]).unwrap() > 0.5);
        assert!(org.local_model.predict(&[-20.0]).unwrap() < 0.5);
    }
}
