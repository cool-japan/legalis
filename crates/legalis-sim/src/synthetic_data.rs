//! Synthetic Data Generation Module
//!
//! This module provides tools for generating synthetic populations with:
//! - GAN-based entity generation
//! - Privacy-preserving techniques (differential privacy)
//! - Demographic-consistent data synthesis
//! - Realistic income/wealth distributions
//! - Geographic-aware population generation
use crate::error::{SimResult, SimulationError};
use legalis_core::{BasicEntity, LegalEntity};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// GAN-based entity generator for creating synthetic populations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GANEntityGenerator {
    /// Latent dimension for the generator
    latent_dim: usize,
    /// Output dimension (number of features)
    output_dim: usize,
    /// Generator weights (simplified)
    generator_weights: Vec<f64>,
    /// Training iterations completed
    training_iterations: usize,
    /// Learning rate
    learning_rate: f64,
}

impl GANEntityGenerator {
    /// Create a new GAN entity generator
    pub fn new(latent_dim: usize, output_dim: usize) -> Self {
        let total_weights = latent_dim * output_dim;
        let mut rng = rand::rng();
        let generator_weights: Vec<f64> = (0..total_weights)
            .map(|_| rng.random_range(-0.1..0.1))
            .collect();

        Self {
            latent_dim,
            output_dim,
            generator_weights,
            training_iterations: 0,
            learning_rate: 0.01,
        }
    }

    /// Train the GAN on real data samples
    pub fn train(&mut self, real_samples: &[Vec<f64>], iterations: usize) -> SimResult<()> {
        if real_samples.is_empty() {
            return Err(SimulationError::InvalidParameter(
                "Real samples cannot be empty".to_string(),
            ));
        }

        for iter in 0..iterations {
            // Simplified training: adjust weights based on real data distribution
            let mut rng = rand::rng();

            // Generate fake sample
            let noise: Vec<f64> = (0..self.latent_dim)
                .map(|_| rng.random_range(-1.0..1.0))
                .collect();
            let fake_sample = self.generate_from_noise(&noise);

            // Get a random real sample
            let real_sample = &real_samples[rng.random_range(0..real_samples.len())];

            // Update weights to make fake sample more like real sample
            for (i, &real_val) in real_sample.iter().enumerate().take(self.output_dim) {
                let error = real_val - fake_sample[i];
                // Update weights (simplified gradient descent)
                for (j, &noise_val) in noise.iter().enumerate().take(self.latent_dim) {
                    let weight_idx = i * self.latent_dim + j;
                    if weight_idx < self.generator_weights.len() {
                        self.generator_weights[weight_idx] +=
                            self.learning_rate * error * noise_val;
                    }
                }
            }

            self.training_iterations = iter + 1;
        }

        Ok(())
    }

    /// Generate a synthetic sample from noise
    #[allow(clippy::needless_range_loop)]
    fn generate_from_noise(&self, noise: &[f64]) -> Vec<f64> {
        let mut output = vec![0.0; self.output_dim];

        for (i, out_val) in output.iter_mut().enumerate() {
            let mut sum = 0.0;
            for (j, &noise_val) in noise.iter().enumerate().take(self.latent_dim) {
                let weight_idx = i * self.latent_dim + j;
                if weight_idx < self.generator_weights.len() {
                    sum += self.generator_weights[weight_idx] * noise_val;
                }
            }
            // Tanh activation
            *out_val = sum.tanh();
        }

        output
    }

    /// Generate a synthetic entity
    pub fn generate(&self) -> Vec<f64> {
        let mut rng = rand::rng();
        let noise: Vec<f64> = (0..self.latent_dim)
            .map(|_| rng.random_range(-1.0..1.0))
            .collect();
        self.generate_from_noise(&noise)
    }

    /// Generate multiple synthetic entities
    pub fn generate_batch(&self, count: usize) -> Vec<Vec<f64>> {
        (0..count).map(|_| self.generate()).collect()
    }

    /// Get training progress
    pub fn training_iterations(&self) -> usize {
        self.training_iterations
    }
}

/// Privacy-preserving synthetic population generator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyPreservingGenerator {
    /// Epsilon for differential privacy (smaller = more privacy)
    epsilon: f64,
    /// Delta for differential privacy
    delta: f64,
    /// Noise scale for Laplace mechanism
    noise_scale: f64,
}

impl PrivacyPreservingGenerator {
    /// Create a new privacy-preserving generator
    pub fn new(epsilon: f64, delta: f64) -> Self {
        Self {
            epsilon,
            delta,
            noise_scale: 1.0 / epsilon,
        }
    }

    /// Add differential privacy noise to a value
    pub fn add_noise(&self, value: f64) -> f64 {
        let mut rng = rand::rng();
        let u: f64 = rng.random_range(-0.5..0.5);
        // Laplace noise
        let noise = -self.noise_scale * u.signum() * (1.0 - 2.0 * u.abs()).ln();
        value + noise
    }

    /// Add noise to a vector of values
    pub fn add_noise_vector(&self, values: &[f64]) -> Vec<f64> {
        values.iter().map(|&v| self.add_noise(v)).collect()
    }

    /// Generate synthetic population with privacy guarantees
    pub fn generate_private_population(
        &self,
        real_data: &[Vec<f64>],
        count: usize,
    ) -> SimResult<Vec<Vec<f64>>> {
        if real_data.is_empty() {
            return Err(SimulationError::InvalidParameter(
                "Real data cannot be empty".to_string(),
            ));
        }

        let dim = real_data[0].len();

        // Compute noisy statistics
        let mut means = vec![0.0; dim];
        let mut stds = vec![0.0; dim];

        for i in 0..dim {
            let values: Vec<f64> = real_data.iter().map(|v| v[i]).collect();
            let mean = values.iter().sum::<f64>() / values.len() as f64;
            let variance =
                values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / values.len() as f64;
            let std = variance.sqrt();

            means[i] = self.add_noise(mean);
            stds[i] = self.add_noise(std).abs(); // Standard deviation must be positive
        }

        // Generate synthetic data from noisy statistics
        let mut rng = rand::rng();
        let synthetic_data: Vec<Vec<f64>> = (0..count)
            .map(|_| {
                (0..dim)
                    .map(|i| {
                        // Sample from normal distribution with noisy parameters
                        let u1: f64 = rng.random_range(0.0..1.0);
                        let u2: f64 = rng.random_range(0.0..1.0);
                        let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
                        means[i] + stds[i] * z
                    })
                    .collect()
            })
            .collect();

        Ok(synthetic_data)
    }

    /// Get privacy budget parameters
    pub fn epsilon(&self) -> f64 {
        self.epsilon
    }

    pub fn delta(&self) -> f64 {
        self.delta
    }
}

/// Demographic constraint for data synthesis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemographicConstraint {
    /// Attribute name
    pub attribute: String,
    /// Target distribution (value -> probability)
    pub distribution: HashMap<String, f64>,
    /// Tolerance for constraint satisfaction
    pub tolerance: f64,
}

impl DemographicConstraint {
    /// Create a new demographic constraint
    pub fn new(attribute: String, distribution: HashMap<String, f64>, tolerance: f64) -> Self {
        Self {
            attribute,
            distribution,
            tolerance,
        }
    }

    /// Check if a population satisfies this constraint
    pub fn is_satisfied(&self, population: &[HashMap<String, String>]) -> bool {
        if population.is_empty() {
            return false;
        }

        let mut counts: HashMap<String, usize> = HashMap::new();
        for entity in population {
            if let Some(value) = entity.get(&self.attribute) {
                *counts.entry(value.clone()).or_insert(0) += 1;
            }
        }

        let total = population.len() as f64;
        for (value, &target_prob) in &self.distribution {
            let actual_prob = *counts.get(value).unwrap_or(&0) as f64 / total;
            if (actual_prob - target_prob).abs() > self.tolerance {
                return false;
            }
        }

        true
    }
}

/// Demographic-consistent data synthesizer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemographicSynthesizer {
    /// Constraints to satisfy
    constraints: Vec<DemographicConstraint>,
    /// Maximum iterations for constraint satisfaction
    max_iterations: usize,
}

impl DemographicSynthesizer {
    /// Create a new demographic synthesizer
    pub fn new(constraints: Vec<DemographicConstraint>) -> Self {
        Self {
            constraints,
            max_iterations: 1000,
        }
    }

    /// Generate synthetic population satisfying demographic constraints
    pub fn synthesize(&self, count: usize) -> SimResult<Vec<HashMap<String, String>>> {
        let mut rng = rand::rng();
        let mut population = Vec::new();

        for _ in 0..count {
            let mut entity: HashMap<String, String> = HashMap::new();

            for constraint in &self.constraints {
                // Sample from the target distribution
                let roll: f64 = rng.random_range(0.0..1.0);
                let mut cumulative = 0.0;
                let mut selected_value = String::new();

                for (value, &prob) in &constraint.distribution {
                    cumulative += prob;
                    if roll <= cumulative {
                        selected_value = value.clone();
                        break;
                    }
                }

                if selected_value.is_empty() && !constraint.distribution.is_empty() {
                    // Fallback: pick the first value
                    selected_value = constraint.distribution.keys().next().unwrap().clone();
                }

                entity.insert(constraint.attribute.clone(), selected_value);
            }

            population.push(entity);
        }

        // Verify constraints
        let mut iterations = 0;
        while iterations < self.max_iterations {
            let all_satisfied = self.constraints.iter().all(|c| c.is_satisfied(&population));
            if all_satisfied {
                break;
            }
            iterations += 1;

            // Regenerate one entity to improve constraint satisfaction
            let idx = rng.random_range(0..population.len());
            let mut entity: HashMap<String, String> = HashMap::new();

            for constraint in &self.constraints {
                let roll: f64 = rng.random_range(0.0..1.0);
                let mut cumulative = 0.0;
                let mut selected_value = String::new();

                for (value, &prob) in &constraint.distribution {
                    cumulative += prob;
                    if roll <= cumulative {
                        selected_value = value.clone();
                        break;
                    }
                }

                if selected_value.is_empty() && !constraint.distribution.is_empty() {
                    selected_value = constraint.distribution.keys().next().unwrap().clone();
                }

                entity.insert(constraint.attribute.clone(), selected_value);
            }

            population[idx] = entity;
        }

        Ok(population)
    }

    /// Get constraints
    pub fn constraints(&self) -> &[DemographicConstraint] {
        &self.constraints
    }
}

/// Income/wealth distribution model
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DistributionModel {
    /// Log-normal distribution (realistic for income)
    LogNormal { mean: f64, std_dev: f64 },
    /// Pareto distribution (power law, realistic for wealth)
    Pareto { scale: f64, shape: f64 },
    /// Exponential distribution
    Exponential { lambda: f64 },
}

/// Income/wealth distribution generator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomeWealthGenerator {
    /// Distribution model to use
    model: DistributionModel,
}

impl IncomeWealthGenerator {
    /// Create a new income/wealth generator
    pub fn new(model: DistributionModel) -> Self {
        Self { model }
    }

    /// Generate a single income/wealth value
    pub fn generate(&self) -> f64 {
        let mut rng = rand::rng();

        match self.model {
            DistributionModel::LogNormal { mean, std_dev } => {
                // Box-Muller transform for normal distribution
                let u1: f64 = rng.random_range(0.0..1.0);
                let u2: f64 = rng.random_range(0.0..1.0);
                let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
                (mean + std_dev * z).exp()
            }
            DistributionModel::Pareto { scale, shape } => {
                let u: f64 = rng.random_range(0.0..1.0);
                scale * (1.0 - u).powf(-1.0 / shape)
            }
            DistributionModel::Exponential { lambda } => {
                let u: f64 = rng.random_range(0.0..1.0);
                -(1.0 - u).ln() / lambda
            }
        }
    }

    /// Generate multiple income/wealth values
    pub fn generate_batch(&self, count: usize) -> Vec<f64> {
        (0..count).map(|_| self.generate()).collect()
    }

    /// Get statistics for generated data
    pub fn statistics(&self, data: &[f64]) -> (f64, f64, f64) {
        if data.is_empty() {
            return (0.0, 0.0, 0.0);
        }

        let mean = data.iter().sum::<f64>() / data.len() as f64;
        let variance = data.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / data.len() as f64;
        let std_dev = variance.sqrt();

        let mut sorted = data.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median = if sorted.len().is_multiple_of(2) {
            (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) / 2.0
        } else {
            sorted[sorted.len() / 2]
        };

        (mean, std_dev, median)
    }
}

/// Geographic distribution model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeographicDistribution {
    /// Cluster centers (latitude, longitude)
    clusters: Vec<(f64, f64)>,
    /// Cluster weights (probabilities)
    weights: Vec<f64>,
    /// Spread around cluster centers
    spread: f64,
}

impl GeographicDistribution {
    /// Create a new geographic distribution
    pub fn new(clusters: Vec<(f64, f64)>, weights: Vec<f64>, spread: f64) -> SimResult<Self> {
        if clusters.len() != weights.len() {
            return Err(SimulationError::InvalidParameter(
                "Clusters and weights must have the same length".to_string(),
            ));
        }

        let total_weight: f64 = weights.iter().sum();
        if (total_weight - 1.0).abs() > 0.01 {
            return Err(SimulationError::InvalidParameter(
                "Weights must sum to 1.0".to_string(),
            ));
        }

        Ok(Self {
            clusters,
            weights,
            spread,
        })
    }

    /// Generate a geographic location
    pub fn generate(&self) -> (f64, f64) {
        let mut rng = rand::rng();

        // Select cluster
        let roll: f64 = rng.random_range(0.0..1.0);
        let mut cumulative = 0.0;
        let mut selected_cluster = (0.0, 0.0);

        for (i, &weight) in self.weights.iter().enumerate() {
            cumulative += weight;
            if roll <= cumulative {
                selected_cluster = self.clusters[i];
                break;
            }
        }

        // Add noise around cluster center
        let u1: f64 = rng.random_range(0.0..1.0);
        let u2: f64 = rng.random_range(0.0..1.0);
        let z1 = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        let z2 = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).sin();

        let lat = selected_cluster.0 + self.spread * z1;
        let lon = selected_cluster.1 + self.spread * z2;

        (lat, lon)
    }

    /// Generate multiple geographic locations
    pub fn generate_batch(&self, count: usize) -> Vec<(f64, f64)> {
        (0..count).map(|_| self.generate()).collect()
    }

    /// Get cluster information
    pub fn clusters(&self) -> &[(f64, f64)] {
        &self.clusters
    }

    pub fn weights(&self) -> &[f64] {
        &self.weights
    }

    pub fn spread(&self) -> f64 {
        self.spread
    }
}

/// Comprehensive synthetic population generator
#[derive(Debug)]
pub struct SyntheticPopulationGenerator {
    /// GAN generator for entity features
    gan: Option<GANEntityGenerator>,
    /// Privacy-preserving generator
    privacy: Option<PrivacyPreservingGenerator>,
    /// Demographic synthesizer
    demographic: Option<DemographicSynthesizer>,
    /// Income/wealth generator
    income: Option<IncomeWealthGenerator>,
    /// Geographic distribution
    geographic: Option<GeographicDistribution>,
}

impl SyntheticPopulationGenerator {
    /// Create a new synthetic population generator
    pub fn new() -> Self {
        Self {
            gan: None,
            privacy: None,
            demographic: None,
            income: None,
            geographic: None,
        }
    }

    /// Set GAN generator
    pub fn with_gan(mut self, gan: GANEntityGenerator) -> Self {
        self.gan = Some(gan);
        self
    }

    /// Set privacy-preserving generator
    pub fn with_privacy(mut self, privacy: PrivacyPreservingGenerator) -> Self {
        self.privacy = Some(privacy);
        self
    }

    /// Set demographic synthesizer
    pub fn with_demographic(mut self, demographic: DemographicSynthesizer) -> Self {
        self.demographic = Some(demographic);
        self
    }

    /// Set income/wealth generator
    pub fn with_income(mut self, income: IncomeWealthGenerator) -> Self {
        self.income = Some(income);
        self
    }

    /// Set geographic distribution
    pub fn with_geographic(mut self, geographic: GeographicDistribution) -> Self {
        self.geographic = Some(geographic);
        self
    }

    /// Generate synthetic population
    pub fn generate(&self, count: usize) -> SimResult<Vec<BasicEntity>> {
        let mut entities = Vec::new();

        // Generate demographic attributes
        let demographic_data = if let Some(ref demo) = self.demographic {
            demo.synthesize(count)?
        } else {
            vec![HashMap::new(); count]
        };

        // Generate income/wealth
        let income_data = if let Some(ref income_gen) = self.income {
            income_gen.generate_batch(count)
        } else {
            vec![0.0; count]
        };

        // Generate geographic locations
        let geographic_data = if let Some(ref geo) = self.geographic {
            geo.generate_batch(count)
        } else {
            vec![(0.0, 0.0); count]
        };

        // Generate entities
        for i in 0..count {
            let mut entity = BasicEntity::new();

            // Add demographic attributes
            for (key, value) in &demographic_data[i] {
                entity.set_attribute(key, value.clone());
            }

            // Add income
            entity.set_attribute("income", income_data[i].to_string());

            // Add geographic location
            entity.set_attribute("latitude", geographic_data[i].0.to_string());
            entity.set_attribute("longitude", geographic_data[i].1.to_string());

            entities.push(entity);
        }

        Ok(entities)
    }
}

impl Default for SyntheticPopulationGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gan_generator_creation() {
        let generator = GANEntityGenerator::new(10, 5);
        assert_eq!(generator.latent_dim, 10);
        assert_eq!(generator.output_dim, 5);
        assert_eq!(generator.generator_weights.len(), 50);
    }

    #[test]
    fn test_gan_generate() {
        let generator = GANEntityGenerator::new(10, 5);
        let sample = generator.generate();
        assert_eq!(sample.len(), 5);
        // Values should be between -1 and 1 due to tanh activation
        for &val in &sample {
            assert!((-1.0..=1.0).contains(&val));
        }
    }

    #[test]
    fn test_gan_generate_batch() {
        let generator = GANEntityGenerator::new(10, 5);
        let batch = generator.generate_batch(100);
        assert_eq!(batch.len(), 100);
        for sample in &batch {
            assert_eq!(sample.len(), 5);
        }
    }

    #[test]
    fn test_gan_training() {
        let mut generator = GANEntityGenerator::new(10, 5);
        let real_samples: Vec<Vec<f64>> = (0..50).map(|_| vec![0.5, 0.3, 0.1, -0.2, 0.4]).collect();

        let result = generator.train(&real_samples, 10);
        assert!(result.is_ok());
        assert_eq!(generator.training_iterations(), 10);
    }

    #[test]
    fn test_privacy_preserving_generator() {
        let generator = PrivacyPreservingGenerator::new(1.0, 0.0001);
        assert_eq!(generator.epsilon(), 1.0);
        assert_eq!(generator.delta(), 0.0001);
        assert_eq!(generator.noise_scale, 1.0);
    }

    #[test]
    fn test_add_noise() {
        let generator = PrivacyPreservingGenerator::new(1.0, 0.0001);
        let value = 100.0;
        let noisy_value = generator.add_noise(value);
        // Noise should change the value, but not drastically
        assert!((noisy_value - value).abs() < 100.0);
    }

    #[test]
    fn test_add_noise_vector() {
        let generator = PrivacyPreservingGenerator::new(1.0, 0.0001);
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let noisy = generator.add_noise_vector(&values);
        assert_eq!(noisy.len(), values.len());
    }

    #[test]
    fn test_generate_private_population() {
        let generator = PrivacyPreservingGenerator::new(1.0, 0.0001);
        let real_data: Vec<Vec<f64>> = (0..100)
            .map(|i| vec![i as f64, (i * 2) as f64, (i * 3) as f64])
            .collect();

        let result = generator.generate_private_population(&real_data, 50);
        assert!(result.is_ok());
        let synthetic = result.unwrap();
        assert_eq!(synthetic.len(), 50);
        for sample in &synthetic {
            assert_eq!(sample.len(), 3);
        }
    }

    #[test]
    fn test_demographic_constraint() {
        let mut dist = HashMap::new();
        dist.insert("male".to_string(), 0.5);
        dist.insert("female".to_string(), 0.5);

        let constraint = DemographicConstraint::new("gender".to_string(), dist, 0.1);
        assert_eq!(constraint.attribute, "gender");
        assert_eq!(constraint.tolerance, 0.1);
    }

    #[test]
    fn test_demographic_constraint_satisfaction() {
        let mut dist = HashMap::new();
        dist.insert("male".to_string(), 0.5);
        dist.insert("female".to_string(), 0.5);

        let constraint = DemographicConstraint::new("gender".to_string(), dist, 0.1);

        let mut population = Vec::new();
        for _ in 0..50 {
            let mut entity = HashMap::new();
            entity.insert("gender".to_string(), "male".to_string());
            population.push(entity);
        }
        for _ in 0..50 {
            let mut entity = HashMap::new();
            entity.insert("gender".to_string(), "female".to_string());
            population.push(entity);
        }

        assert!(constraint.is_satisfied(&population));
    }

    #[test]
    fn test_demographic_synthesizer() {
        let mut dist = HashMap::new();
        dist.insert("male".to_string(), 0.5);
        dist.insert("female".to_string(), 0.5);

        let constraint = DemographicConstraint::new("gender".to_string(), dist, 0.1);
        let synthesizer = DemographicSynthesizer::new(vec![constraint]);

        let result = synthesizer.synthesize(100);
        assert!(result.is_ok());
        let population = result.unwrap();
        assert_eq!(population.len(), 100);
    }

    #[test]
    fn test_income_lognormal() {
        let generator = IncomeWealthGenerator::new(DistributionModel::LogNormal {
            mean: 10.0,
            std_dev: 0.5,
        });

        let income = generator.generate();
        assert!(income > 0.0);
    }

    #[test]
    fn test_income_pareto() {
        let generator = IncomeWealthGenerator::new(DistributionModel::Pareto {
            scale: 20000.0,
            shape: 1.5,
        });

        let wealth = generator.generate();
        assert!(wealth >= 20000.0);
    }

    #[test]
    fn test_income_exponential() {
        let generator =
            IncomeWealthGenerator::new(DistributionModel::Exponential { lambda: 0.001 });

        let value = generator.generate();
        assert!(value >= 0.0);
    }

    #[test]
    fn test_income_batch() {
        let generator = IncomeWealthGenerator::new(DistributionModel::LogNormal {
            mean: 10.0,
            std_dev: 0.5,
        });

        let batch = generator.generate_batch(100);
        assert_eq!(batch.len(), 100);
        for &income in &batch {
            assert!(income > 0.0);
        }
    }

    #[test]
    fn test_income_statistics() {
        let generator = IncomeWealthGenerator::new(DistributionModel::LogNormal {
            mean: 10.0,
            std_dev: 0.5,
        });

        let batch = generator.generate_batch(1000);
        let (mean, std_dev, median) = generator.statistics(&batch);

        assert!(mean > 0.0);
        assert!(std_dev > 0.0);
        assert!(median > 0.0);
    }

    #[test]
    fn test_geographic_distribution() {
        let clusters = vec![(40.7128, -74.0060), (34.0522, -118.2437)]; // NYC, LA
        let weights = vec![0.6, 0.4];

        let result = GeographicDistribution::new(clusters.clone(), weights.clone(), 1.0);
        assert!(result.is_ok());

        let dist = result.unwrap();
        assert_eq!(dist.clusters(), &clusters);
        assert_eq!(dist.weights(), &weights);
        assert_eq!(dist.spread(), 1.0);
    }

    #[test]
    fn test_geographic_generation() {
        let clusters = vec![(40.7128, -74.0060)];
        let weights = vec![1.0];

        let dist = GeographicDistribution::new(clusters, weights, 0.5).unwrap();
        let (lat, lon) = dist.generate();

        // Should be close to NYC
        assert!((lat - 40.7128).abs() < 5.0);
        assert!((lon + 74.0060).abs() < 5.0);
    }

    #[test]
    fn test_geographic_batch() {
        let clusters = vec![(40.7128, -74.0060)];
        let weights = vec![1.0];

        let dist = GeographicDistribution::new(clusters, weights, 0.5).unwrap();
        let batch = dist.generate_batch(100);

        assert_eq!(batch.len(), 100);
        for (lat, lon) in &batch {
            assert!((lat - 40.7128).abs() < 10.0);
            assert!((lon + 74.0060).abs() < 10.0);
        }
    }

    #[test]
    fn test_synthetic_population_generator_basic() {
        let generator = SyntheticPopulationGenerator::new();
        let result = generator.generate(10);
        assert!(result.is_ok());
        let entities = result.unwrap();
        assert_eq!(entities.len(), 10);
    }

    #[test]
    fn test_synthetic_population_with_income() {
        let income_gen = IncomeWealthGenerator::new(DistributionModel::LogNormal {
            mean: 10.0,
            std_dev: 0.5,
        });

        let generator = SyntheticPopulationGenerator::new().with_income(income_gen);
        let result = generator.generate(10);
        assert!(result.is_ok());
        let entities = result.unwrap();

        for entity in &entities {
            let income = entity.get_attribute("income");
            assert!(income.is_some());
        }
    }

    #[test]
    fn test_synthetic_population_with_geographic() {
        let clusters = vec![(40.7128, -74.0060)];
        let weights = vec![1.0];
        let geo_dist = GeographicDistribution::new(clusters, weights, 0.5).unwrap();

        let generator = SyntheticPopulationGenerator::new().with_geographic(geo_dist);
        let result = generator.generate(10);
        assert!(result.is_ok());
        let entities = result.unwrap();

        for entity in &entities {
            assert!(entity.get_attribute("latitude").is_some());
            assert!(entity.get_attribute("longitude").is_some());
        }
    }

    #[test]
    fn test_synthetic_population_comprehensive() {
        // Set up all generators
        let mut dist = HashMap::new();
        dist.insert("employed".to_string(), 0.7);
        dist.insert("unemployed".to_string(), 0.3);

        let constraint = DemographicConstraint::new("employment".to_string(), dist, 0.15);
        let demo_synth = DemographicSynthesizer::new(vec![constraint]);

        let income_gen = IncomeWealthGenerator::new(DistributionModel::LogNormal {
            mean: 10.5,
            std_dev: 0.6,
        });

        let clusters = vec![(40.7128, -74.0060), (34.0522, -118.2437)];
        let weights = vec![0.5, 0.5];
        let geo_dist = GeographicDistribution::new(clusters, weights, 1.0).unwrap();

        let generator = SyntheticPopulationGenerator::new()
            .with_demographic(demo_synth)
            .with_income(income_gen)
            .with_geographic(geo_dist);

        let result = generator.generate(100);
        assert!(result.is_ok());
        let entities = result.unwrap();
        assert_eq!(entities.len(), 100);

        // Verify all entities have required attributes
        for entity in &entities {
            assert!(entity.get_attribute("employment").is_some());
            assert!(entity.get_attribute("income").is_some());
            assert!(entity.get_attribute("latitude").is_some());
            assert!(entity.get_attribute("longitude").is_some());
        }
    }
}
