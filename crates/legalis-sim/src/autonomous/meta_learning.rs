//! Meta-learning for simulation optimisation.
//!
//! Across many optimisation campaigns, good starting parameters for a *new*
//! problem can be predicted from what worked on *similar* problems. This
//! submodule captures that with two transferable mechanisms:
//!
//! - [`MetaLearningStore`] — an archive of [`RunRecord`]s (problem context +
//!   tuned parameters + achieved score) supporting a similarity-weighted
//!   [`MetaLearningStore::warm_start`] that blends the parameters of the nearest,
//!   best-performing past runs.
//! - [`PerformanceModel`] — a transferable ridge-regression model predicting the
//!   achieved score from `[context, parameters]`, fitted across all past runs via
//!   the normal equations (Cholesky solve). It powers
//!   [`MetaLearner::recommend`], which meta-optimises starting parameters for a
//!   new context without running a single simulation.
//!
//! [`MetaLearner`] ties the two together behind one interface.

use super::{
    ParameterSpace, cholesky_decompose, cholesky_solve, euclidean_distance, is_improvement,
    worst_objective_value,
};
use crate::{Objective, SimResult, SimulationError};
use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const EPSILON: f64 = 1e-9;

/// A feature vector describing a simulation/optimisation problem instance.
///
/// Typical features: population size, statute count, problem difficulty, target
/// metric magnitude — anything that characterises the problem and is comparable
/// across runs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunContext {
    /// The ordered context features.
    pub features: Vec<f64>,
}

impl RunContext {
    /// Creates a context from a feature vector.
    pub fn new(features: Vec<f64>) -> Self {
        Self { features }
    }

    /// Returns the number of context features.
    pub fn dimension(&self) -> usize {
        self.features.len()
    }
}

/// A record of one completed optimisation campaign.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunRecord {
    /// The problem context.
    pub context: RunContext,
    /// The parameters that achieved `score`.
    pub parameters: HashMap<String, f64>,
    /// The achieved objective value.
    pub score: f64,
}

impl RunRecord {
    /// Creates a run record.
    pub fn new(context: RunContext, parameters: HashMap<String, f64>, score: f64) -> Self {
        Self {
            context,
            parameters,
            score,
        }
    }
}

/// An archive of past runs supporting similarity-weighted warm starts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaLearningStore {
    records: Vec<RunRecord>,
    objective: Objective,
}

impl MetaLearningStore {
    /// Creates an empty store for the given objective sense.
    pub fn new(objective: Objective) -> Self {
        Self {
            records: Vec::new(),
            objective,
        }
    }

    /// Adds a run record.
    pub fn record(&mut self, run: RunRecord) {
        self.records.push(run);
    }

    /// Returns the archived records.
    pub fn records(&self) -> &[RunRecord] {
        &self.records
    }

    /// Returns the number of archived runs.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Returns whether the archive is empty.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Returns the objective sense.
    pub fn objective(&self) -> Objective {
        self.objective
    }

    /// Computes per-feature `(min, max)` ranges across all records.
    fn context_ranges(&self, dim: usize) -> Vec<(f64, f64)> {
        let mut ranges = vec![(f64::INFINITY, f64::NEG_INFINITY); dim];
        for record in &self.records {
            for (i, &value) in record.context.features.iter().enumerate().take(dim) {
                ranges[i].0 = ranges[i].0.min(value);
                ranges[i].1 = ranges[i].1.max(value);
            }
        }
        ranges
    }

    fn normalize_context(features: &[f64], ranges: &[(f64, f64)]) -> Vec<f64> {
        features
            .iter()
            .zip(ranges.iter())
            .map(|(&v, &(lo, hi))| {
                let width = hi - lo;
                if width.abs() < EPSILON {
                    0.0
                } else {
                    (v - lo) / width
                }
            })
            .collect()
    }

    /// Recommends starting parameters for `context` by blending the `k` nearest,
    /// best-performing past runs.
    ///
    /// Neighbours are weighted by inverse normalised context distance times an
    /// objective-oriented quality weight (a softmax over neighbour scores), so
    /// nearer and better runs contribute more. The blended parameters are clamped
    /// to `space`.
    pub fn warm_start(
        &self,
        context: &RunContext,
        k: usize,
        space: &ParameterSpace,
    ) -> SimResult<HashMap<String, f64>> {
        if self.records.is_empty() {
            return Err(SimulationError::ExecutionError(
                "no historical runs to warm-start from".to_string(),
            ));
        }
        if k == 0 {
            return Err(SimulationError::InvalidParameter(
                "k must be greater than zero".to_string(),
            ));
        }
        let dim = self.records[0].context.dimension();
        if context.dimension() != dim {
            return Err(SimulationError::InvalidParameter(format!(
                "context has {} features, store expects {}",
                context.dimension(),
                dim
            )));
        }
        if self.records.iter().any(|r| r.context.dimension() != dim) {
            return Err(SimulationError::InvalidParameter(
                "all stored contexts must share the same dimensionality".to_string(),
            ));
        }

        let ranges = self.context_ranges(dim);
        let query = Self::normalize_context(&context.features, &ranges);

        // Rank records by normalised context distance.
        let mut neighbours: Vec<(usize, f64)> = self
            .records
            .iter()
            .enumerate()
            .map(|(idx, record)| {
                let normalized = Self::normalize_context(&record.context.features, &ranges);
                (idx, euclidean_distance(&query, &normalized))
            })
            .collect();
        neighbours.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        neighbours.truncate(k.min(self.records.len()));

        // Objective-oriented quality weighting (numerically stable softmax).
        let oriented = |score: f64| match self.objective {
            Objective::Maximize => score,
            Objective::Minimize => -score,
        };
        let max_oriented = neighbours
            .iter()
            .map(|&(idx, _)| oriented(self.records[idx].score))
            .fold(f64::NEG_INFINITY, f64::max);

        let mut weighted = HashMap::new();
        let mut total_weight = 0.0;
        for &(idx, distance) in &neighbours {
            let record = &self.records[idx];
            let quality = (oriented(record.score) - max_oriented).exp();
            let weight = quality / (distance + EPSILON);
            total_weight += weight;
            for name in space.names() {
                let bounds = space
                    .bounds_for(name)
                    .ok_or_else(|| SimulationError::InvalidParameter("unknown dimension".into()))?;
                let value = record.parameters.get(name).copied().unwrap_or(bounds.lower);
                *weighted.entry(name.clone()).or_insert(0.0) += weight * value;
            }
        }

        if total_weight < EPSILON {
            return Ok(space.center());
        }
        let blended: HashMap<String, f64> = weighted
            .into_iter()
            .map(|(name, sum)| (name, sum / total_weight))
            .collect();
        Ok(space.clamp_named(&blended))
    }
}

/// A transferable ridge-regression performance model.
///
/// Predicts the achieved score from the concatenated feature vector
/// `[1, context…, parameters…]` (parameters in sorted-name order), fitted across
/// all stored runs by solving the regularised normal equations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceModel {
    weights: Vec<f64>,
    context_dim: usize,
    param_names: Vec<String>,
    ridge: f64,
    fitted: bool,
}

impl PerformanceModel {
    /// Creates an unfitted model with the given ridge (L2) regularisation.
    pub fn new(ridge: f64) -> SimResult<Self> {
        if ridge < 0.0 || !ridge.is_finite() {
            return Err(SimulationError::InvalidParameter(
                "ridge regularisation must be non-negative and finite".to_string(),
            ));
        }
        Ok(Self {
            weights: Vec::new(),
            context_dim: 0,
            param_names: Vec::new(),
            ridge,
            fitted: false,
        })
    }

    /// Returns whether the model has been fitted.
    pub fn is_fitted(&self) -> bool {
        self.fitted
    }

    /// Returns the sorted parameter names the model was fitted on.
    pub fn param_names(&self) -> &[String] {
        &self.param_names
    }

    fn build_features(
        &self,
        context: &RunContext,
        parameters: &HashMap<String, f64>,
    ) -> SimResult<Vec<f64>> {
        if context.dimension() != self.context_dim {
            return Err(SimulationError::InvalidParameter(format!(
                "context has {} features, model expects {}",
                context.dimension(),
                self.context_dim
            )));
        }
        let mut features = Vec::with_capacity(1 + self.context_dim + self.param_names.len());
        features.push(1.0); // intercept
        features.extend_from_slice(&context.features);
        for name in &self.param_names {
            let value = parameters.get(name).ok_or_else(|| {
                SimulationError::InvalidParameter(format!("missing parameter '{name}'"))
            })?;
            features.push(*value);
        }
        Ok(features)
    }

    /// Fits the model across all runs in `store`.
    pub fn fit(&mut self, store: &MetaLearningStore) -> SimResult<()> {
        if store.is_empty() {
            return Err(SimulationError::InvalidParameter(
                "cannot fit a performance model on an empty store".to_string(),
            ));
        }
        let records = store.records();
        self.context_dim = records[0].context.dimension();
        let mut param_names: Vec<String> = records[0].parameters.keys().cloned().collect();
        param_names.sort();
        self.param_names = param_names;

        let p = 1 + self.context_dim + self.param_names.len();

        // Accumulate the normal-equation matrices AᵀA and Aᵀy.
        let mut ata = vec![vec![0.0; p]; p];
        let mut aty = vec![0.0; p];
        for record in records {
            let phi = self.build_features(&record.context, &record.parameters)?;
            for i in 0..p {
                aty[i] += phi[i] * record.score;
                for j in 0..p {
                    ata[i][j] += phi[i] * phi[j];
                }
            }
        }
        // Ridge regularisation on the diagonal (guarantees positive definiteness).
        let ridge = self.ridge.max(1e-8);
        for (i, row) in ata.iter_mut().enumerate() {
            row[i] += ridge;
        }

        let l = cholesky_decompose(&ata)?;
        self.weights = cholesky_solve(&l, &aty);
        self.fitted = true;
        Ok(())
    }

    /// Predicts the score for a `(context, parameters)` pair.
    pub fn predict(
        &self,
        context: &RunContext,
        parameters: &HashMap<String, f64>,
    ) -> SimResult<f64> {
        if !self.fitted {
            return Err(SimulationError::ExecutionError(
                "performance model has not been fitted".to_string(),
            ));
        }
        let phi = self.build_features(context, parameters)?;
        Ok(super::dot(&phi, &self.weights))
    }
}

/// Unified meta-learner over the store and the transferable performance model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaLearner {
    store: MetaLearningStore,
    model: Option<PerformanceModel>,
    ridge: f64,
}

impl MetaLearner {
    /// Creates a meta-learner with the given objective and ridge regularisation.
    pub fn new(objective: Objective, ridge: f64) -> SimResult<Self> {
        // Validate ridge eagerly so construction fails fast.
        PerformanceModel::new(ridge)?;
        Ok(Self {
            store: MetaLearningStore::new(objective),
            model: None,
            ridge,
        })
    }

    /// Records a completed run (invalidating any previously fitted model).
    pub fn record(&mut self, run: RunRecord) {
        self.store.record(run);
        self.model = None;
    }

    /// Returns the number of recorded runs.
    pub fn num_runs(&self) -> usize {
        self.store.len()
    }

    /// Returns the underlying store.
    pub fn store(&self) -> &MetaLearningStore {
        &self.store
    }

    /// Returns the fitted performance model, if any.
    pub fn model(&self) -> Option<&PerformanceModel> {
        self.model.as_ref()
    }

    /// Recommends starting parameters by similarity-weighted warm start.
    pub fn warm_start(
        &self,
        context: &RunContext,
        k: usize,
        space: &ParameterSpace,
    ) -> SimResult<HashMap<String, f64>> {
        self.store.warm_start(context, k, space)
    }

    /// Fits the transferable performance model over the recorded runs.
    pub fn fit(&mut self) -> SimResult<()> {
        let mut model = PerformanceModel::new(self.ridge)?;
        model.fit(&self.store)?;
        self.model = Some(model);
        Ok(())
    }

    /// Recommends starting parameters for `context` by meta-optimising the fitted
    /// performance model over `n_candidates` random configurations.
    ///
    /// Requires [`MetaLearner::fit`] to have been called.
    pub fn recommend<R: RngExt>(
        &self,
        context: &RunContext,
        space: &ParameterSpace,
        n_candidates: usize,
        rng: &mut R,
    ) -> SimResult<HashMap<String, f64>> {
        if n_candidates == 0 {
            return Err(SimulationError::InvalidParameter(
                "n_candidates must be greater than zero".to_string(),
            ));
        }
        let model = self.model.as_ref().ok_or_else(|| {
            SimulationError::ExecutionError("meta-learner has not been fitted".to_string())
        })?;

        let objective = self.store.objective();
        let mut best_params = space.random_named(rng)?;
        let mut best_score = worst_objective_value(objective);
        for _ in 0..n_candidates {
            let candidate = space.random_named(rng)?;
            let predicted = model.predict(context, &candidate)?;
            if is_improvement(objective, predicted, best_score) {
                best_score = predicted;
                best_params = candidate;
            }
        }
        Ok(best_params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{SeedableRng, rngs::StdRng};

    fn space() -> ParameterSpace {
        ParameterSpace::new()
            .with_dimension("lr", 0.0, 1.0)
            .unwrap()
            .with_dimension("reg", 0.0, 1.0)
            .unwrap()
    }

    fn record(ctx: f64, lr: f64, reg: f64, score: f64) -> RunRecord {
        let mut params = HashMap::new();
        params.insert("lr".to_string(), lr);
        params.insert("reg".to_string(), reg);
        RunRecord::new(RunContext::new(vec![ctx]), params, score)
    }

    #[test]
    fn test_store_basics_and_empty_warm_start() {
        let mut store = MetaLearningStore::new(Objective::Maximize);
        assert!(store.is_empty());
        assert!(matches!(store.objective(), Objective::Maximize));
        // Warm start on empty store errors.
        let ctx = RunContext::new(vec![1.0]);
        assert!(store.warm_start(&ctx, 1, &space()).is_err());
        store.record(record(1.0, 0.5, 0.5, 1.0));
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn test_warm_start_prefers_similar_context() {
        let mut store = MetaLearningStore::new(Objective::Maximize);
        // Small-context problems liked lr=0.1; large-context problems liked lr=0.9.
        store.record(record(0.0, 0.1, 0.2, 5.0));
        store.record(record(0.0, 0.15, 0.25, 4.5));
        store.record(record(10.0, 0.9, 0.8, 5.0));
        store.record(record(10.0, 0.85, 0.75, 4.5));

        let space = space();
        // A new small-context problem should be warm-started near lr=0.1.
        let near_small = store
            .warm_start(&RunContext::new(vec![0.5]), 2, &space)
            .unwrap();
        assert!(near_small["lr"] < 0.4, "lr {}", near_small["lr"]);

        // A new large-context problem should be warm-started near lr=0.9.
        let near_large = store
            .warm_start(&RunContext::new(vec![9.5]), 2, &space)
            .unwrap();
        assert!(near_large["lr"] > 0.6, "lr {}", near_large["lr"]);
    }

    #[test]
    fn test_warm_start_validation() {
        let mut store = MetaLearningStore::new(Objective::Maximize);
        store.record(record(1.0, 0.5, 0.5, 1.0));
        // Wrong k.
        assert!(
            store
                .warm_start(&RunContext::new(vec![1.0]), 0, &space())
                .is_err()
        );
        // Wrong context dimension.
        assert!(
            store
                .warm_start(&RunContext::new(vec![1.0, 2.0]), 1, &space())
                .is_err()
        );
    }

    #[test]
    fn test_performance_model_predicts_linear_score() {
        let mut store = MetaLearningStore::new(Objective::Maximize);
        // Ground-truth score = 2*ctx + 3*lr - 1*reg.
        let mut rng = StdRng::seed_from_u64(3);
        for _ in 0..40 {
            let ctx: f64 = rng.random_range(0.0..5.0);
            let lr: f64 = rng.random_range(0.0..1.0);
            let reg: f64 = rng.random_range(0.0..1.0);
            store.record(record(ctx, lr, reg, 2.0 * ctx + 3.0 * lr - reg));
        }
        let mut model = PerformanceModel::new(1e-6).unwrap();
        model.fit(&store).unwrap();
        assert!(model.is_fitted());
        assert_eq!(model.param_names(), &["lr".to_string(), "reg".to_string()]);

        let mut params = HashMap::new();
        params.insert("lr".to_string(), 0.5);
        params.insert("reg".to_string(), 0.5);
        let predicted = model.predict(&RunContext::new(vec![2.0]), &params).unwrap();
        let truth = 2.0 * 2.0 + 3.0 * 0.5 - 0.5;
        assert!(
            (predicted - truth).abs() < 0.2,
            "predicted {predicted}, truth {truth}"
        );
    }

    #[test]
    fn test_performance_model_validation() {
        assert!(PerformanceModel::new(-1.0).is_err());
        let empty = MetaLearningStore::new(Objective::Maximize);
        let mut model = PerformanceModel::new(0.1).unwrap();
        assert!(model.fit(&empty).is_err());
        // Predict before fit errors.
        let m2 = PerformanceModel::new(0.1).unwrap();
        assert!(
            m2.predict(&RunContext::new(vec![1.0]), &HashMap::new())
                .is_err()
        );
    }

    #[test]
    fn test_meta_learner_recommend_maximizes_model() {
        let mut learner = MetaLearner::new(Objective::Maximize, 1e-6).unwrap();
        // Score increases with lr; recommend should favour high lr.
        let mut rng = StdRng::seed_from_u64(8);
        for _ in 0..50 {
            let ctx: f64 = rng.random_range(0.0..2.0);
            let lr: f64 = rng.random_range(0.0..1.0);
            let reg: f64 = rng.random_range(0.0..1.0);
            learner.record(record(ctx, lr, reg, 4.0 * lr - reg));
        }
        // Recommend before fit errors.
        assert!(
            learner
                .recommend(&RunContext::new(vec![1.0]), &space(), 10, &mut rng)
                .is_err()
        );

        learner.fit().unwrap();
        assert!(learner.model().is_some());
        let recommended = learner
            .recommend(&RunContext::new(vec![1.0]), &space(), 300, &mut rng)
            .unwrap();
        assert!(recommended["lr"] > 0.6, "lr {}", recommended["lr"]);
        assert_eq!(learner.num_runs(), 50);
    }

    #[test]
    fn test_meta_learner_record_invalidates_model() {
        let mut learner = MetaLearner::new(Objective::Minimize, 1e-3).unwrap();
        learner.record(record(1.0, 0.5, 0.5, 1.0));
        learner.record(record(2.0, 0.4, 0.6, 0.8));
        learner.fit().unwrap();
        assert!(learner.model().is_some());
        // Recording new data invalidates the fitted model.
        learner.record(record(3.0, 0.3, 0.7, 0.6));
        assert!(learner.model().is_none());
        assert!(MetaLearner::new(Objective::Minimize, -1.0).is_err());
    }
}
