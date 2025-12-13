//! Benchmarks for the simulation engine.

use criterion::{Criterion, criterion_group, criterion_main};
use legalis_core::{
    BasicEntity, ComparisonOp, Condition, Effect, EffectType, LegalEntity, Statute,
};
use legalis_sim::{PopulationBuilder, SimEngine};
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

criterion_group!(
    benches,
    bench_condition_evaluation,
    bench_complex_condition_evaluation,
    bench_population_creation,
    bench_simulation_throughput,
    bench_statute_scaling,
    bench_population_scaling,
);

criterion_main!(benches);
