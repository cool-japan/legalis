//! Benchmarks for the simulation engine.

use chrono::NaiveDate;
use criterion::{Criterion, criterion_group, criterion_main};
use legalis_core::{
    BasicEntity, ComparisonOp, Condition, Effect, EffectType, LegalEntity, Statute,
};
use legalis_sim::{
    BehavioralProfile, ComplianceContext, ComplianceModel, DecisionStrategy, DemographicProfile,
    PopulationBuilder, PopulationGenerator, RelationshipGraph, RelationshipType, SimEngine,
    TemporalSimBuilder, TemporalStatute, TimeStep,
};
use std::hint::black_box;

fn create_sample_statutes(count: usize) -> Vec<Statute> {
    (0..count)
        .map(|i| {
            Statute::new(
                format!("statute-{}", i),
                format!("Test Statute {}", i),
                Effect::new(EffectType::Grant, format!("Grant right {}", i)),
            )
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18 + (i as u32 % 10),
            })
        })
        .collect()
}

fn create_population(size: usize) -> Vec<Box<dyn LegalEntity>> {
    PopulationBuilder::new().generate_random(size).build()
}

fn bench_condition_evaluation(c: &mut Criterion) {
    let statute = Statute::new(
        "adult-rights",
        "Adult Rights",
        Effect::new(EffectType::Grant, "Full legal capacity"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    });

    let mut entity = BasicEntity::new();
    entity.set_attribute("age", "25".to_string());

    c.bench_function("condition_evaluation_simple", |b| {
        b.iter(|| {
            black_box(SimEngine::apply_law(&entity, &statute));
        });
    });
}

fn bench_complex_condition_evaluation(c: &mut Criterion) {
    let statute = Statute::new(
        "complex-rights",
        "Complex Rights",
        Effect::new(EffectType::Grant, "Complex grant"),
    )
    .with_precondition(Condition::And(
        Box::new(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        }),
        Box::new(Condition::Or(
            Box::new(Condition::Income {
                operator: ComparisonOp::GreaterThan,
                value: 50000,
            }),
            Box::new(Condition::HasAttribute {
                key: "citizen".to_string(),
            }),
        )),
    ));

    let mut entity = BasicEntity::new();
    entity.set_attribute("age", "25".to_string());
    entity.set_attribute("income", "60000".to_string());

    c.bench_function("condition_evaluation_complex", |b| {
        b.iter(|| {
            black_box(SimEngine::apply_law(&entity, &statute));
        });
    });
}

fn bench_population_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("population_creation");

    for size in [100, 1000, 10000].iter() {
        group.bench_function(format!("size_{}", size), |b| {
            b.iter(|| {
                black_box(create_population(*size));
            });
        });
    }

    group.finish();
}

fn bench_simulation_throughput(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut group = c.benchmark_group("simulation_throughput");

    // Small simulation
    group.bench_function("small_100x5", |b| {
        let statutes = create_sample_statutes(5);
        let population = create_population(100);
        let engine = SimEngine::new(statutes, population);

        b.to_async(&rt).iter(|| async {
            black_box(engine.run_simulation().await);
        });
    });

    // Medium simulation
    group.bench_function("medium_500x10", |b| {
        let statutes = create_sample_statutes(10);
        let population = create_population(500);
        let engine = SimEngine::new(statutes, population);

        b.to_async(&rt).iter(|| async {
            black_box(engine.run_simulation().await);
        });
    });

    group.finish();
}

fn bench_statute_scaling(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut group = c.benchmark_group("statute_scaling");
    group.sample_size(20);

    for statute_count in [1, 5, 10, 20].iter() {
        group.bench_function(format!("statutes_{}", statute_count), |b| {
            b.to_async(&rt).iter(|| async {
                let statutes = create_sample_statutes(*statute_count);
                let population = create_population(100);
                let engine = SimEngine::new(statutes, population);
                black_box(engine.run_simulation().await)
            });
        });
    }

    group.finish();
}

fn bench_population_scaling(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut group = c.benchmark_group("population_scaling");
    group.sample_size(20);

    for pop_size in [50, 100, 200, 500].iter() {
        group.bench_function(format!("population_{}", pop_size), |b| {
            b.to_async(&rt).iter(|| async {
                let statutes = create_sample_statutes(5);
                let population = create_population(*pop_size);
                let engine = SimEngine::new(statutes, population);
                black_box(engine.run_simulation().await)
            });
        });
    }

    group.finish();
}

fn bench_behavioral_decisions(c: &mut Criterion) {
    let mut group = c.benchmark_group("behavioral_decisions");

    let context = ComplianceContext {
        statute_id: "test".to_string(),
        legal_result: legalis_core::LegalResult::Deterministic(Effect::new(
            EffectType::Obligation,
            "Pay tax",
        )),
        enforcement_probability: 0.7,
        penalty_severity: 100.0,
        evasion_benefit: 50.0,
        compliance_cost: 10.0,
        social_norm: 0.8,
    };

    // Benchmark different decision strategies
    for strategy in [
        DecisionStrategy::Rational,
        DecisionStrategy::BoundedRational,
        DecisionStrategy::RuleFollowing,
        DecisionStrategy::Opportunistic,
    ] {
        group.bench_function(format!("{:?}", strategy), |b| {
            let profile = BehavioralProfile::new(strategy);
            let mut model = ComplianceModel::new(profile);
            b.iter(|| black_box(model.decide(&context)));
        });
    }

    group.finish();
}

fn bench_population_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("population_generation");

    for size in [100, 500, 1000, 5000] {
        group.bench_function(format!("realistic_{}", size), |b| {
            let profile = DemographicProfile::us_2024();
            let generator = PopulationGenerator::new(profile, size);
            b.iter(|| black_box(generator.generate()));
        });
    }

    group.finish();
}

fn bench_relationship_queries(c: &mut Criterion) {
    use legalis_sim::Relationship;
    use uuid::Uuid;

    let mut group = c.benchmark_group("relationship_queries");

    // Create a graph with 1000 entities and random relationships
    let mut graph = RelationshipGraph::new();
    let entities: Vec<Uuid> = (0..1000).map(|_| Uuid::new_v4()).collect();

    for i in 0..entities.len() - 1 {
        if i % 3 == 0 {
            graph.add_relationship(Relationship::new(
                entities[i],
                entities[i + 1],
                RelationshipType::Parent,
            ));
        }
    }

    group.bench_function("get_relationships", |b| {
        b.iter(|| {
            black_box(graph.get_relationships(entities[0], RelationshipType::Parent));
        });
    });

    group.bench_function("find_connected_2_degrees", |b| {
        b.iter(|| {
            black_box(graph.find_connected(entities[0], 2));
        });
    });

    group.bench_function("find_connected_5_degrees", |b| {
        b.iter(|| {
            black_box(graph.find_connected(entities[0], 5));
        });
    });

    group.finish();
}

fn bench_temporal_simulation(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut group = c.benchmark_group("temporal_simulation");
    group.sample_size(10);

    let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();

    let statute = Statute::new(
        "test",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    });

    // Monthly steps
    group.bench_function("monthly_100_agents", |b| {
        b.to_async(&rt).iter(|| async {
            let entities = PopulationGenerator::new(DemographicProfile::us_2024(), 100).generate();
            let mut engine = TemporalSimBuilder::new(start, end)
                .with_time_step(TimeStep::Month)
                .add_statute(TemporalStatute::new(statute.clone()))
                .add_population(entities)
                .build()
                .await;

            black_box(engine.run().await)
        });
    });

    // Yearly steps
    group.bench_function("yearly_500_agents", |b| {
        b.to_async(&rt).iter(|| async {
            let entities = PopulationGenerator::new(DemographicProfile::us_2024(), 500).generate();
            let mut engine = TemporalSimBuilder::new(start, end)
                .with_time_step(TimeStep::Year)
                .add_statute(TemporalStatute::new(statute.clone()))
                .add_population(entities)
                .build()
                .await;

            black_box(engine.run().await)
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_condition_evaluation,
    bench_complex_condition_evaluation,
    bench_population_creation,
    bench_simulation_throughput,
    bench_statute_scaling,
    bench_population_scaling,
    bench_behavioral_decisions,
    bench_population_generation,
    bench_relationship_queries,
    bench_temporal_simulation,
);

criterion_main!(benches);
