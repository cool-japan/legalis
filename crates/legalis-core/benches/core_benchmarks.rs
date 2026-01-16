//! Performance benchmarks for legalis-core critical operations.
//!
//! Run with: cargo bench

use chrono::NaiveDate;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use legalis_core::{
    BasicEntity, Case, CaseDatabase, ComparisonOp, Condition, Court, Effect, EffectType,
    LegalEntity, Statute, TemporalValidity,
};
use std::hint::black_box;

/// Benchmark statute validation
fn bench_statute_validate(c: &mut Criterion) {
    let valid_statute = Statute::new(
        "tax-law-2025",
        "Income Tax Credit",
        Effect::new(EffectType::Grant, "Tax credit of $1000"),
    )
    .with_precondition(Condition::income(ComparisonOp::LessThan, 50000))
    .with_jurisdiction("US")
    .with_version(1);

    c.bench_function("statute_validate_valid", |b| {
        b.iter(|| black_box(valid_statute.validate()))
    });

    let complex_statute = Statute::new(
        "complex-law-2025",
        "Complex Legal Provision",
        Effect::new(EffectType::Obligation, "Must comply with requirements"),
    )
    .with_precondition(
        Condition::age(ComparisonOp::GreaterOrEqual, 18).and(
            Condition::income(ComparisonOp::LessThan, 75000)
                .or(Condition::has_attribute("special_status")),
        ),
    )
    .with_temporal_validity(
        TemporalValidity::new()
            .with_effective_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
            .with_expiry_date(NaiveDate::from_ymd_opt(2030, 12, 31).unwrap()),
    )
    .with_jurisdiction("US")
    .with_version(2);

    c.bench_function("statute_validate_complex", |b| {
        b.iter(|| black_box(complex_statute.validate()))
    });
}

/// Benchmark statute builder pattern
fn bench_statute_builder(c: &mut Criterion) {
    c.bench_function("statute_builder_simple", |b| {
        b.iter(|| {
            black_box(
                Statute::new(
                    "law-001",
                    "Simple Law",
                    Effect::new(EffectType::Grant, "Grant permission"),
                )
                .with_version(1),
            )
        })
    });

    c.bench_function("statute_builder_complex", |b| {
        b.iter(|| {
            black_box(
                Statute::new(
                    "law-002",
                    "Complex Law",
                    Effect::new(EffectType::Obligation, "Must comply"),
                )
                .with_precondition(
                    Condition::age(ComparisonOp::GreaterThan, 18)
                        .and(Condition::income(ComparisonOp::LessThan, 50000)),
                )
                .with_temporal_validity(
                    TemporalValidity::new()
                        .with_effective_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
                )
                .with_jurisdiction("US")
                .with_version(1),
            )
        })
    });
}

/// Benchmark case database operations
fn bench_case_database(c: &mut Criterion) {
    let mut group = c.benchmark_group("case_database");

    for size in [10, 100, 1000].iter() {
        // Create database with N cases
        let mut db = CaseDatabase::new();
        for i in 0..*size {
            let case = Case::new(
                format!("Case {}, {} F.3d {} (2020)", i, i + 100, i + 500),
                format!("Case {}", i),
                2020,
                Court::Trial,
                "US",
            )
            .with_facts(format!("Facts of case {}", i))
            .with_holding(format!("Holding of case {}", i))
            .with_ratio(format!("Ratio of case {}", i))
            .with_date(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap());
            db.add_case(case);
        }

        group.bench_with_input(
            BenchmarkId::new("cases_by_jurisdiction", size),
            size,
            |b, _| b.iter(|| black_box(db.cases_by_jurisdiction("US"))),
        );

        group.bench_with_input(
            BenchmarkId::new("cases_by_year_range", size),
            size,
            |b, _| b.iter(|| black_box(db.cases_by_year_range(2018, 2022))),
        );

        group.bench_with_input(BenchmarkId::new("cases_by_court", size), size, |b, _| {
            b.iter(|| black_box(db.cases_by_court(&Court::Trial)))
        });

        group.bench_with_input(BenchmarkId::new("all_cases", size), size, |b, _| {
            b.iter(|| black_box(db.all_cases()))
        });

        group.bench_with_input(BenchmarkId::new("iter_count", size), size, |b, _| {
            b.iter(|| black_box(db.iter().count()))
        });
    }

    group.finish();
}

/// Benchmark condition construction
fn bench_condition_construction(c: &mut Criterion) {
    c.bench_function("condition_age_simple", |b| {
        b.iter(|| black_box(Condition::age(ComparisonOp::GreaterThan, 18)))
    });

    c.bench_function("condition_income_simple", |b| {
        b.iter(|| black_box(Condition::income(ComparisonOp::LessThan, 50000)))
    });

    c.bench_function("condition_has_attribute", |b| {
        b.iter(|| black_box(Condition::has_attribute("citizen")))
    });

    c.bench_function("condition_attribute_equals", |b| {
        b.iter(|| black_box(Condition::attribute_equals("status", "active")))
    });

    c.bench_function("condition_custom", |b| {
        b.iter(|| black_box(Condition::custom("custom_condition")))
    });
}

/// Benchmark condition combinators
fn bench_condition_combinators(c: &mut Criterion) {
    let simple = Condition::age(ComparisonOp::GreaterThan, 18);

    c.bench_function("condition_is_compound_simple", |b| {
        b.iter(|| black_box(simple.is_compound()))
    });

    c.bench_function("condition_is_simple", |b| {
        b.iter(|| black_box(simple.is_simple()))
    });

    let compound = Condition::age(ComparisonOp::GreaterThan, 18)
        .and(Condition::income(ComparisonOp::LessThan, 50000));

    c.bench_function("condition_is_compound_compound", |b| {
        b.iter(|| black_box(compound.is_compound()))
    });

    c.bench_function("condition_count_simple", |b| {
        b.iter(|| black_box(simple.count_conditions()))
    });

    c.bench_function("condition_count_compound", |b| {
        b.iter(|| black_box(compound.count_conditions()))
    });

    c.bench_function("condition_depth_simple", |b| {
        b.iter(|| black_box(simple.depth()))
    });

    c.bench_function("condition_depth_compound", |b| {
        b.iter(|| black_box(compound.depth()))
    });

    // Fluent builders
    c.bench_function("condition_fluent_and", |b| {
        b.iter(|| {
            black_box(
                Condition::age(ComparisonOp::GreaterThan, 18)
                    .and(Condition::income(ComparisonOp::LessThan, 50000)),
            )
        })
    });

    c.bench_function("condition_fluent_or", |b| {
        b.iter(|| {
            black_box(
                Condition::age(ComparisonOp::GreaterThan, 21)
                    .or(Condition::has_attribute("special_permit")),
            )
        })
    });

    c.bench_function("condition_fluent_not", |b| {
        b.iter(|| black_box(Condition::has_attribute("criminal_record").not()))
    });

    c.bench_function("condition_fluent_complex", |b| {
        b.iter(|| {
            black_box(
                Condition::age(ComparisonOp::GreaterThan, 21)
                    .and(Condition::income(ComparisonOp::LessThan, 50000))
                    .or(Condition::has_attribute("veteran")),
            )
        })
    });
}

/// Benchmark comparison operator methods
fn bench_comparison_op(c: &mut Criterion) {
    c.bench_function("comparison_op_inverse", |b| {
        b.iter(|| black_box(ComparisonOp::GreaterThan.inverse()))
    });

    c.bench_function("comparison_op_is_equality", |b| {
        b.iter(|| black_box(ComparisonOp::Equal.is_equality()))
    });

    c.bench_function("comparison_op_is_ordering", |b| {
        b.iter(|| black_box(ComparisonOp::GreaterThan.is_ordering()))
    });
}

/// Benchmark temporal validity checks
fn bench_temporal_validity(c: &mut Criterion) {
    let validity = TemporalValidity::new()
        .with_effective_date(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
        .with_expiry_date(NaiveDate::from_ymd_opt(2030, 12, 31).unwrap());

    let check_date = NaiveDate::from_ymd_opt(2025, 6, 15).unwrap();

    c.bench_function("temporal_validity_is_active", |b| {
        b.iter(|| black_box(validity.is_active(black_box(check_date))))
    });

    c.bench_function("temporal_validity_has_effective_date", |b| {
        b.iter(|| black_box(validity.has_effective_date()))
    });

    c.bench_function("temporal_validity_has_expiry_date", |b| {
        b.iter(|| black_box(validity.has_expiry_date()))
    });

    c.bench_function("temporal_validity_is_pending", |b| {
        b.iter(|| {
            black_box(validity.is_pending(black_box(NaiveDate::from_ymd_opt(2019, 1, 1).unwrap())))
        })
    });

    c.bench_function("temporal_validity_has_expired", |b| {
        b.iter(|| {
            black_box(validity.has_expired(black_box(NaiveDate::from_ymd_opt(2031, 1, 1).unwrap())))
        })
    });
}

/// Benchmark effect operations
fn bench_effect_operations(c: &mut Criterion) {
    let effect = Effect::grant("Tax credit")
        .with_parameter("amount", "1000")
        .with_parameter("year", "2025");

    c.bench_function("effect_get_parameter", |b| {
        b.iter(|| black_box(effect.get_parameter("amount")))
    });

    c.bench_function("effect_has_parameter", |b| {
        b.iter(|| black_box(effect.has_parameter("amount")))
    });

    c.bench_function("effect_parameter_count", |b| {
        b.iter(|| black_box(effect.parameter_count()))
    });

    c.bench_function("effect_construction_grant", |b| {
        b.iter(|| black_box(Effect::grant("Permission granted")))
    });

    c.bench_function("effect_construction_revoke", |b| {
        b.iter(|| black_box(Effect::revoke("License revoked")))
    });

    c.bench_function("effect_construction_obligation", |b| {
        b.iter(|| black_box(Effect::obligation("Must file taxes")))
    });

    c.bench_function("effect_construction_prohibition", |b| {
        b.iter(|| black_box(Effect::prohibition("Cannot operate without license")))
    });

    c.bench_function("effect_with_parameter", |b| {
        b.iter(|| {
            black_box(
                Effect::grant("Benefit")
                    .with_parameter("amount", "500")
                    .with_parameter("type", "credit"),
            )
        })
    });
}

/// Benchmark entity operations
fn bench_entity_operations(c: &mut Criterion) {
    let mut entity = BasicEntity::new();
    entity.set_attribute("age", "25".to_string());
    entity.set_attribute("income", "45000".to_string());
    entity.set_attribute("citizen", "true".to_string());

    c.bench_function("entity_get_attribute", |b| {
        b.iter(|| black_box(entity.get_attribute("age")))
    });

    c.bench_function("entity_set_attribute", |b| {
        b.iter(|| {
            let mut e = BasicEntity::new();
            e.set_attribute("test", "value".to_string());
        })
    });

    c.bench_function("entity_id", |b| b.iter(|| black_box(entity.id())));

    c.bench_function("entity_new", |b| b.iter(|| black_box(BasicEntity::new())));
}

criterion_group!(
    benches,
    bench_statute_validate,
    bench_statute_builder,
    bench_case_database,
    bench_condition_construction,
    bench_condition_combinators,
    bench_comparison_op,
    bench_temporal_validity,
    bench_effect_operations,
    bench_entity_operations,
);
criterion_main!(benches);
