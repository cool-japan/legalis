//! Automated scenario generation.
//!
//! Generates batches of parameter configurations ("scenarios") that cover a
//! [`ParameterSpace`] systematically and diversely:
//!
//! - [`FactorialDesign`] — exhaustive combinatorial (full-factorial) sweeps over
//!   evenly spaced levels per dimension.
//! - [`LatinHypercubeSampler`] — stratified Latin-hypercube sampling, which
//!   spreads `n` samples so each dimension is covered evenly with far fewer
//!   points than a factorial grid.
//! - [`HaltonSequence`] — a deterministic low-discrepancy (Sobol-like)
//!   quasi-random sequence based on the radical-inverse function, giving more
//!   uniform space-filling coverage than pseudo-random sampling.
//! - [`NoveltySearch`] — novelty-seeking generation that greedily selects the
//!   candidates most behaviourally distinct from an archive of prior scenarios.
//!
//! [`AutoScenarioGenerator`] is a convenience facade exposing all four methods
//! over a single space.

use super::{ParameterSpace, euclidean_distance};
use crate::{SimResult, SimulationError};
use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single generated scenario: a labelled parameter configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedScenario {
    /// Sequential identifier within a generation batch.
    pub id: usize,
    /// Human-readable label describing how the scenario was produced.
    pub label: String,
    /// The parameter configuration.
    pub parameters: HashMap<String, f64>,
}

impl GeneratedScenario {
    /// Creates a generated scenario.
    pub fn new(id: usize, label: impl Into<String>, parameters: HashMap<String, f64>) -> Self {
        Self {
            id,
            label: label.into(),
            parameters,
        }
    }
}

/// Full-factorial combinatorial design over evenly spaced levels per dimension.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactorialDesign {
    levels: usize,
    max_combinations: usize,
}

impl FactorialDesign {
    /// Creates a factorial design with `levels` evenly spaced values per dimension.
    ///
    /// `levels == 1` places a single point at each dimension's centre.
    pub fn new(levels: usize) -> SimResult<Self> {
        if levels == 0 {
            return Err(SimulationError::InvalidParameter(
                "factorial design requires at least one level".to_string(),
            ));
        }
        Ok(Self {
            levels,
            max_combinations: 1_000_000,
        })
    }

    /// Sets the maximum number of combinations the design may generate.
    pub fn with_max_combinations(mut self, max_combinations: usize) -> Self {
        self.max_combinations = max_combinations;
        self
    }

    fn levels_for(&self, lower: f64, upper: f64) -> Vec<f64> {
        if self.levels == 1 {
            return vec![0.5 * (lower + upper)];
        }
        let step = (upper - lower) / (self.levels - 1) as f64;
        (0..self.levels).map(|i| lower + i as f64 * step).collect()
    }

    /// Generates the full grid of scenarios for `space`.
    pub fn generate(&self, space: &ParameterSpace) -> SimResult<Vec<GeneratedScenario>> {
        if space.is_empty() {
            return Err(SimulationError::ConfigurationError(
                "factorial design requires a non-empty parameter space".to_string(),
            ));
        }
        let dims = space.dimensions();

        // Guard against combinatorial explosion.
        let mut total: u128 = 1;
        for _ in 0..dims {
            total = total.saturating_mul(self.levels as u128);
            if total > self.max_combinations as u128 {
                return Err(SimulationError::InvalidParameter(format!(
                    "factorial design would generate more than {} combinations",
                    self.max_combinations
                )));
            }
        }

        let values_per_dim: Vec<Vec<f64>> = space
            .bounds()
            .iter()
            .map(|b| self.levels_for(b.lower, b.upper))
            .collect();

        let mut scenarios = Vec::with_capacity(total as usize);
        let mut indices = vec![0usize; dims];
        let mut id = 0usize;
        loop {
            let mut parameters = HashMap::with_capacity(dims);
            for (d, name) in space.names().iter().enumerate() {
                parameters.insert(name.clone(), values_per_dim[d][indices[d]]);
            }
            scenarios.push(GeneratedScenario::new(
                id,
                format!("factorial-{id}"),
                parameters,
            ));
            id += 1;

            // Increment the mixed-radix counter; stop when it overflows.
            let mut d = 0;
            loop {
                indices[d] += 1;
                if indices[d] < values_per_dim[d].len() {
                    break;
                }
                indices[d] = 0;
                d += 1;
                if d == dims {
                    return Ok(scenarios);
                }
            }
        }
    }
}

/// Stratified Latin-hypercube sampler.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LatinHypercubeSampler {
    jitter: bool,
}

impl Default for LatinHypercubeSampler {
    fn default() -> Self {
        Self { jitter: true }
    }
}

impl LatinHypercubeSampler {
    /// Creates a sampler that jitters samples within each stratum.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a sampler that places samples at stratum centres (deterministic
    /// stratification, randomised only by the per-dimension permutations).
    pub fn centered() -> Self {
        Self { jitter: false }
    }

    fn permutation<R: RngExt>(n: usize, rng: &mut R) -> Vec<usize> {
        let mut perm: Vec<usize> = (0..n).collect();
        // Fisher-Yates shuffle.
        for i in (1..n).rev() {
            let j = rng.random_range(0..=i);
            perm.swap(i, j);
        }
        perm
    }

    /// Draws `n` Latin-hypercube samples over `space`.
    pub fn sample<R: RngExt>(
        &self,
        space: &ParameterSpace,
        n: usize,
        rng: &mut R,
    ) -> SimResult<Vec<GeneratedScenario>> {
        if space.is_empty() {
            return Err(SimulationError::ConfigurationError(
                "Latin-hypercube sampling requires a non-empty parameter space".to_string(),
            ));
        }
        if n == 0 {
            return Err(SimulationError::InvalidParameter(
                "sample count must be greater than zero".to_string(),
            ));
        }
        let dims = space.dimensions();
        // One independent permutation of strata per dimension.
        let permutations: Vec<Vec<usize>> = (0..dims).map(|_| Self::permutation(n, rng)).collect();

        let mut scenarios = Vec::with_capacity(n);
        for i in 0..n {
            let mut unit = Vec::with_capacity(dims);
            for permutation in &permutations {
                let offset = if self.jitter {
                    rng.random_range(0.0..1.0)
                } else {
                    0.5
                };
                unit.push((permutation[i] as f64 + offset) / n as f64);
            }
            let parameters = space.denormalize_named(&unit)?;
            scenarios.push(GeneratedScenario::new(i, format!("lhs-{i}"), parameters));
        }
        Ok(scenarios)
    }
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut i = 2u64;
    while i * i <= n {
        if n.is_multiple_of(i) {
            return false;
        }
        i += 1;
    }
    true
}

fn first_primes(count: usize) -> Vec<u64> {
    let mut primes = Vec::with_capacity(count);
    let mut candidate = 2u64;
    while primes.len() < count {
        if is_prime(candidate) {
            primes.push(candidate);
        }
        candidate += 1;
    }
    primes
}

/// Radical inverse of `index` in the given `base` (the building block of the
/// Halton sequence).
fn radical_inverse(mut index: u64, base: u64) -> f64 {
    let mut result = 0.0;
    let mut fraction = 1.0 / base as f64;
    while index > 0 {
        result += fraction * (index % base) as f64;
        index /= base;
        fraction /= base as f64;
    }
    result
}

/// A deterministic low-discrepancy (Sobol-like) Halton quasi-random sequence.
///
/// Each dimension uses the radical inverse in a distinct prime base, producing
/// highly uniform space-filling coverage. The sequence is reproducible: the same
/// configuration always yields the same points.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HaltonSequence {
    primes: Vec<u64>,
    index: u64,
}

impl HaltonSequence {
    /// Creates a Halton sequence over `dimension` dimensions.
    pub fn new(dimension: usize) -> SimResult<Self> {
        if dimension == 0 {
            return Err(SimulationError::InvalidParameter(
                "Halton sequence requires at least one dimension".to_string(),
            ));
        }
        Ok(Self {
            primes: first_primes(dimension),
            // Skip index 0 (which maps to the origin in every base).
            index: 1,
        })
    }

    /// Returns the number of dimensions.
    pub fn dimension(&self) -> usize {
        self.primes.len()
    }

    /// Resets the sequence to its first point.
    pub fn reset(&mut self) {
        self.index = 1;
    }

    /// Returns the next unit-cube point in the sequence.
    pub fn next_unit(&mut self) -> Vec<f64> {
        let point = self
            .primes
            .iter()
            .map(|&base| radical_inverse(self.index, base))
            .collect();
        self.index += 1;
        point
    }

    /// Draws `n` low-discrepancy scenarios over `space`.
    ///
    /// The space dimensionality must match the sequence dimensionality.
    pub fn sample(
        &mut self,
        space: &ParameterSpace,
        n: usize,
    ) -> SimResult<Vec<GeneratedScenario>> {
        if space.dimensions() != self.dimension() {
            return Err(SimulationError::InvalidParameter(format!(
                "space has {} dimensions, Halton sequence has {}",
                space.dimensions(),
                self.dimension()
            )));
        }
        if n == 0 {
            return Err(SimulationError::InvalidParameter(
                "sample count must be greater than zero".to_string(),
            ));
        }
        let mut scenarios = Vec::with_capacity(n);
        for i in 0..n {
            let unit = self.next_unit();
            let parameters = space.denormalize_named(&unit)?;
            scenarios.push(GeneratedScenario::new(i, format!("halton-{i}"), parameters));
        }
        Ok(scenarios)
    }
}

/// Novelty-seeking scenario generator.
///
/// Maintains an archive of previously visited points (in unit space) and selects
/// new candidates that maximise the mean distance to their `k` nearest archived
/// neighbours, encouraging behavioural diversity rather than convergence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoveltySearch {
    space: ParameterSpace,
    archive: Vec<Vec<f64>>,
    k: usize,
}

impl NoveltySearch {
    /// Creates a novelty searcher using `k` nearest neighbours for scoring.
    pub fn new(space: ParameterSpace, k: usize) -> SimResult<Self> {
        if space.is_empty() {
            return Err(SimulationError::ConfigurationError(
                "novelty search requires a non-empty parameter space".to_string(),
            ));
        }
        if k == 0 {
            return Err(SimulationError::InvalidParameter(
                "novelty neighbourhood size must be at least one".to_string(),
            ));
        }
        Ok(Self {
            space,
            archive: Vec::new(),
            k,
        })
    }

    /// Returns the number of archived points.
    pub fn archive_size(&self) -> usize {
        self.archive.len()
    }

    /// Returns whether the archive is empty.
    pub fn is_empty(&self) -> bool {
        self.archive.is_empty()
    }

    /// Adds a named configuration to the archive.
    pub fn add(&mut self, parameters: &HashMap<String, f64>) {
        self.archive.push(self.space.normalize_named(parameters));
    }

    /// Mean distance from `point` to its `k` nearest neighbours in `reference`.
    ///
    /// Returns the maximum unit-cube distance when there are no references, so an
    /// empty reference set treats every point as maximally novel.
    fn novelty_against(&self, point: &[f64], reference: &[Vec<f64>]) -> f64 {
        if reference.is_empty() {
            return (self.space.dimensions() as f64).sqrt();
        }
        let mut distances: Vec<f64> = reference
            .iter()
            .map(|r| euclidean_distance(point, r))
            .collect();
        distances.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let k = self.k.min(distances.len());
        distances.iter().take(k).sum::<f64>() / k as f64
    }

    /// Computes the novelty score of a named configuration against the archive.
    pub fn novelty(&self, parameters: &HashMap<String, f64>) -> f64 {
        let point = self.space.normalize_named(parameters);
        self.novelty_against(&point, &self.archive)
    }

    /// Generates `n_select` novel scenarios from `n_candidates` random candidates.
    ///
    /// Selection is greedy: each chosen point maximises novelty against the
    /// archive plus the points already chosen in this batch. Selected points are
    /// added to the archive.
    pub fn generate<R: RngExt>(
        &mut self,
        n_candidates: usize,
        n_select: usize,
        rng: &mut R,
    ) -> SimResult<Vec<GeneratedScenario>> {
        if n_select == 0 || n_candidates == 0 {
            return Err(SimulationError::InvalidParameter(
                "candidate and selection counts must be greater than zero".to_string(),
            ));
        }
        if n_select > n_candidates {
            return Err(SimulationError::InvalidParameter(
                "cannot select more scenarios than candidates".to_string(),
            ));
        }

        let mut candidates: Vec<Vec<f64>> = Vec::with_capacity(n_candidates);
        for _ in 0..n_candidates {
            candidates.push(self.space.random_unit(rng)?);
        }

        let mut selected: Vec<Vec<f64>> = Vec::with_capacity(n_select);
        let mut scenarios = Vec::with_capacity(n_select);
        let mut used = vec![false; n_candidates];

        for id in 0..n_select {
            // Reference set is the archive plus points chosen so far this batch.
            let mut reference = self.archive.clone();
            reference.extend(selected.iter().cloned());

            let mut best_idx = usize::MAX;
            let mut best_novelty = f64::NEG_INFINITY;
            for (idx, candidate) in candidates.iter().enumerate() {
                if used[idx] {
                    continue;
                }
                let novelty = self.novelty_against(candidate, &reference);
                if novelty > best_novelty {
                    best_novelty = novelty;
                    best_idx = idx;
                }
            }

            used[best_idx] = true;
            let chosen = candidates[best_idx].clone();
            let parameters = self.space.denormalize_named(&chosen)?;
            scenarios.push(GeneratedScenario::new(
                id,
                format!("novelty-{id}"),
                parameters,
            ));
            selected.push(chosen);
        }

        // Commit the batch to the archive for future generations.
        self.archive.extend(selected);
        Ok(scenarios)
    }
}

/// Convenience facade over all scenario-generation strategies for one space.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoScenarioGenerator {
    space: ParameterSpace,
}

impl AutoScenarioGenerator {
    /// Creates a generator over `space`.
    pub fn new(space: ParameterSpace) -> Self {
        Self { space }
    }

    /// Returns the underlying parameter space.
    pub fn space(&self) -> &ParameterSpace {
        &self.space
    }

    /// Generates a full-factorial grid with `levels` levels per dimension.
    pub fn factorial(&self, levels: usize) -> SimResult<Vec<GeneratedScenario>> {
        FactorialDesign::new(levels)?.generate(&self.space)
    }

    /// Generates `n` Latin-hypercube scenarios.
    pub fn latin_hypercube<R: RngExt>(
        &self,
        n: usize,
        rng: &mut R,
    ) -> SimResult<Vec<GeneratedScenario>> {
        LatinHypercubeSampler::new().sample(&self.space, n, rng)
    }

    /// Generates `n` low-discrepancy Halton scenarios.
    pub fn halton(&self, n: usize) -> SimResult<Vec<GeneratedScenario>> {
        HaltonSequence::new(self.space.dimensions())?.sample(&self.space, n)
    }

    /// Generates `n_select` novelty-seeking scenarios from `n_candidates`.
    pub fn novelty<R: RngExt>(
        &self,
        n_candidates: usize,
        n_select: usize,
        rng: &mut R,
    ) -> SimResult<Vec<GeneratedScenario>> {
        NoveltySearch::new(self.space.clone(), 3)?.generate(n_candidates, n_select, rng)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{SeedableRng, rngs::StdRng};

    fn space_2d() -> ParameterSpace {
        ParameterSpace::new()
            .with_dimension("a", 0.0, 1.0)
            .unwrap()
            .with_dimension("b", 10.0, 20.0)
            .unwrap()
    }

    #[test]
    fn test_factorial_full_grid() {
        let space = space_2d();
        let design = FactorialDesign::new(3).unwrap();
        let scenarios = design.generate(&space).unwrap();
        // 3 levels x 2 dims = 9 scenarios.
        assert_eq!(scenarios.len(), 9);
        // Corners present: (0,10) and (1,20).
        assert!(
            scenarios.iter().any(
                |s| (s.parameters["a"]).abs() < 1e-9 && (s.parameters["b"] - 10.0).abs() < 1e-9
            )
        );
        assert!(
            scenarios
                .iter()
                .any(|s| (s.parameters["a"] - 1.0).abs() < 1e-9
                    && (s.parameters["b"] - 20.0).abs() < 1e-9)
        );
    }

    #[test]
    fn test_factorial_guards_and_single_level() {
        let space = space_2d();
        assert!(FactorialDesign::new(0).is_err());
        // Explosion guard.
        let design = FactorialDesign::new(100).unwrap().with_max_combinations(50);
        assert!(design.generate(&space).is_err());
        // Single level places dimension centres.
        let single = FactorialDesign::new(1).unwrap().generate(&space).unwrap();
        assert_eq!(single.len(), 1);
        assert!((single[0].parameters["a"] - 0.5).abs() < 1e-9);
        assert!((single[0].parameters["b"] - 15.0).abs() < 1e-9);
    }

    #[test]
    fn test_latin_hypercube_stratification() {
        let space = space_2d();
        let mut rng = StdRng::seed_from_u64(1);
        let n = 10;
        let scenarios = LatinHypercubeSampler::new()
            .sample(&space, n, &mut rng)
            .unwrap();
        assert_eq!(scenarios.len(), n);
        // Each of the n strata along dimension "a" (width 1) must be hit once.
        let mut strata = vec![false; n];
        for s in &scenarios {
            let stratum = ((s.parameters["a"]) * n as f64).floor() as usize;
            let stratum = stratum.min(n - 1);
            strata[stratum] = true;
        }
        assert!(
            strata.iter().all(|&hit| hit),
            "LHS did not cover all strata"
        );
        assert!(
            LatinHypercubeSampler::new()
                .sample(&space, 0, &mut rng)
                .is_err()
        );
    }

    #[test]
    fn test_halton_deterministic_and_uniform() {
        let space = space_2d();
        let mut seq1 = HaltonSequence::new(2).unwrap();
        let mut seq2 = HaltonSequence::new(2).unwrap();
        let s1 = seq1.sample(&space, 16).unwrap();
        let s2 = seq2.sample(&space, 16).unwrap();
        // Deterministic: identical sequences.
        for (a, b) in s1.iter().zip(s2.iter()) {
            assert!((a.parameters["a"] - b.parameters["a"]).abs() < 1e-12);
        }
        // Dimension mismatch rejected.
        let mut seq3 = HaltonSequence::new(3).unwrap();
        assert!(seq3.sample(&space, 4).is_err());
        // First base-2 radical inverse value is 0.5.
        let mut seq4 = HaltonSequence::new(1).unwrap();
        assert!((seq4.next_unit()[0] - 0.5).abs() < 1e-12);
    }

    #[test]
    fn test_novelty_search_spreads_points() {
        let space = space_2d();
        let mut rng = StdRng::seed_from_u64(99);
        let mut search = NoveltySearch::new(space, 2).unwrap();
        assert!(search.is_empty());
        let batch = search.generate(40, 5, &mut rng).unwrap();
        assert_eq!(batch.len(), 5);
        assert_eq!(search.archive_size(), 5);

        // A point far from the archive scores more novel than a near duplicate.
        let first = &batch[0].parameters;
        let near = first.clone();
        let novelty_near = search.novelty(&near);
        let mut far = HashMap::new();
        far.insert("a".to_string(), 0.5);
        far.insert("b".to_string(), 15.0);
        let _ = search.novelty(&far);
        // The duplicate of an archived point should have very low novelty.
        assert!(novelty_near < (2.0_f64).sqrt());

        assert!(search.generate(2, 5, &mut rng).is_err());
    }

    #[test]
    fn test_auto_generator_facade() {
        let space = space_2d();
        let mut rng = StdRng::seed_from_u64(5);
        let generator = AutoScenarioGenerator::new(space);
        assert_eq!(generator.space().dimensions(), 2);
        assert_eq!(generator.factorial(2).unwrap().len(), 4);
        assert_eq!(generator.latin_hypercube(8, &mut rng).unwrap().len(), 8);
        assert_eq!(generator.halton(8).unwrap().len(), 8);
        assert_eq!(generator.novelty(20, 4, &mut rng).unwrap().len(), 4);
    }

    #[test]
    fn test_primes_and_radical_inverse() {
        assert_eq!(first_primes(5), vec![2, 3, 5, 7, 11]);
        assert!(!is_prime(1));
        assert!(is_prime(13));
        // Base-3 radical inverse of 1 is 1/3.
        assert!((radical_inverse(1, 3) - 1.0 / 3.0).abs() < 1e-12);
    }
}
