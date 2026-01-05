# legalis-sim

Simulation engine for Legalis-RS.

## Overview

This crate provides an ECS-like (Entity Component System) simulation engine for testing legal statutes against populations of agents. It uses Tokio for async parallel execution, enabling high-throughput law application simulations.

## Features

- Async parallel simulation using Tokio
- Configurable population generation
- Comprehensive metrics collection
- Per-statute and aggregate statistics

## Usage

### Basic Simulation

```rust
use legalis_sim::{SimEngine, PopulationBuilder};
use legalis_core::{Statute, Condition, Effect, EffectType, ComparisonOp};

// Define statutes
let statute = Statute::new(
    "voting-rights",
    "Voting Rights Act",
    Effect::new(EffectType::Grant, "Right to vote"),
)
.with_precondition(Condition::Age {
    operator: ComparisonOp::GreaterOrEqual,
    value: 18,
});

// Create population
let population = PopulationBuilder::new()
    .generate_random(10_000)
    .build();

// Run simulation
let engine = SimEngine::new(vec![statute], population);
let metrics = engine.run_simulation().await;

println!("{}", metrics.summary());
```

### Custom Entities

```rust
use legalis_core::BasicEntity;

let mut citizen = BasicEntity::new();
citizen.set_attribute("age", "35".to_string());
citizen.set_attribute("income", "5000000".to_string());
citizen.set_attribute("citizenship", "JP".to_string());

let population = PopulationBuilder::new()
    .add_entity(citizen)
    .generate_random(999)
    .build();
```

## Simulation Engine

```rust
pub struct SimEngine {
    statutes: Vec<Statute>,
    population: Vec<Arc<dyn LegalEntity>>,
}

impl SimEngine {
    pub fn new(statutes: Vec<Statute>, population: Vec<Box<dyn LegalEntity>>) -> Self;
    pub fn population_size(&self) -> usize;
    pub fn statute_count(&self) -> usize;
    pub async fn run_simulation(&self) -> SimulationMetrics;
    pub fn apply_law(agent: &dyn LegalEntity, law: &Statute) -> LegalResult<Effect>;
}
```

## Metrics

### SimulationMetrics

```rust
pub struct SimulationMetrics {
    pub total_applications: usize,
    pub deterministic_count: usize,
    pub discretion_count: usize,
    pub void_count: usize,
    pub statute_metrics: HashMap<String, StatuteMetrics>,
    pub discretion_agents: Vec<Uuid>,
}

impl SimulationMetrics {
    pub fn deterministic_ratio(&self) -> f64;
    pub fn discretion_ratio(&self) -> f64;
    pub fn summary(&self) -> String;
}
```

### StatuteMetrics

```rust
pub struct StatuteMetrics {
    pub total: usize,
    pub deterministic: usize,
    pub discretion: usize,
    pub void: usize,
}

impl StatuteMetrics {
    pub fn effectiveness(&self) -> f64;  // deterministic / total
    pub fn ambiguity(&self) -> f64;      // discretion / total
}
```

## Example Output

```
=== Simulation Summary ===
Total applications: 10000
Deterministic: 7523 (75.2%)
Discretionary: 1834 (18.3%)
Void: 643

=== Per-Statute Breakdown ===
voting-rights: D=7523 / J=0 / V=2477
housing-subsidy: D=0 / J=1834 / V=8166
```

## Condition Evaluation

The engine evaluates conditions with three possible outcomes:

- **True**: Condition is satisfied
- **False**: Condition is not satisfied
- **Indeterminate**: Cannot be evaluated (missing data, custom conditions)

Indeterminate results trigger `JudicialDiscretion` in the `LegalResult`.

## License

MIT OR Apache-2.0
