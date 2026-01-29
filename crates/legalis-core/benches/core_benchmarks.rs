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

/// Benchmark statute serialization/deserialization
#[cfg(feature = "serde")]
fn bench_serialization(c: &mut Criterion) {
    let simple_statute = Statute::new(
        "serialize-test",
        "Serialization Test",
        Effect::grant("Test permission"),
    )
    .with_version(1);

    let complex_statute = Statute::new(
        "complex-serialize",
        "Complex Serialization Test",
        Effect::obligation("Must comply"),
    )
    .with_precondition(
        Condition::age(ComparisonOp::GreaterThan, 18)
            .and(Condition::income(ComparisonOp::LessThan, 50000)),
    )
    .with_temporal_validity(
        TemporalValidity::new().with_effective_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
    )
    .with_jurisdiction("US")
    .with_version(1);

    c.bench_function("serialize_statute_simple", |b| {
        b.iter(|| black_box(serde_json::to_string(&simple_statute).unwrap()))
    });

    c.bench_function("serialize_statute_complex", |b| {
        b.iter(|| black_box(serde_json::to_string(&complex_statute).unwrap()))
    });

    let simple_json = serde_json::to_string(&simple_statute).unwrap();
    let complex_json = serde_json::to_string(&complex_statute).unwrap();

    c.bench_function("deserialize_statute_simple", |b| {
        b.iter(|| black_box(serde_json::from_str::<Statute>(&simple_json).unwrap()))
    });

    c.bench_function("deserialize_statute_complex", |b| {
        b.iter(|| black_box(serde_json::from_str::<Statute>(&complex_json).unwrap()))
    });
}

/// Benchmark statute cloning
fn bench_statute_clone(c: &mut Criterion) {
    let simple =
        Statute::new("clone-test", "Clone Test", Effect::grant("Permission")).with_version(1);

    let complex = Statute::new(
        "complex-clone",
        "Complex Clone",
        Effect::obligation("Comply"),
    )
    .with_precondition(
        Condition::age(ComparisonOp::GreaterThan, 18)
            .and(Condition::income(ComparisonOp::LessThan, 50000))
            .or(Condition::has_attribute("exempt")),
    )
    .with_temporal_validity(
        TemporalValidity::new().with_effective_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
    )
    .with_jurisdiction("US")
    .with_version(1);

    c.bench_function("clone_statute_simple", |b| {
        b.iter(|| black_box(simple.clone()))
    });

    c.bench_function("clone_statute_complex", |b| {
        b.iter(|| black_box(complex.clone()))
    });
}

/// Benchmark condition evaluation depth
fn bench_condition_depth(c: &mut Criterion) {
    let mut group = c.benchmark_group("condition_depth");

    for depth in [2, 5, 10, 15].iter() {
        let mut cond = Condition::age(ComparisonOp::GreaterThan, 18);
        for i in 0..*depth {
            cond = cond.and(Condition::income(ComparisonOp::LessThan, 50000 + i * 1000));
        }

        group.bench_with_input(BenchmarkId::new("depth", depth), depth, |b, _| {
            b.iter(|| black_box(cond.depth()))
        });

        group.bench_with_input(BenchmarkId::new("count", depth), depth, |b, _| {
            b.iter(|| black_box(cond.count_conditions()))
        });
    }

    group.finish();
}

/// Benchmark effect composition
fn bench_effect_composition(c: &mut Criterion) {
    c.bench_function("effect_with_multiple_parameters", |b| {
        b.iter(|| {
            black_box(
                Effect::grant("Benefit")
                    .with_parameter("amount", "1000")
                    .with_parameter("currency", "USD")
                    .with_parameter("year", "2025")
                    .with_parameter("category", "tax_credit")
                    .with_parameter("subsection", "401k"),
            )
        })
    });

    let effect = Effect::grant("Test")
        .with_parameter("p1", "v1")
        .with_parameter("p2", "v2")
        .with_parameter("p3", "v3")
        .with_parameter("p4", "v4")
        .with_parameter("p5", "v5");

    c.bench_function("effect_iterate_parameters", |b| {
        b.iter(|| {
            let count = effect.parameters.len();
            black_box(count)
        })
    });
}

/// Benchmark statute version operations
fn bench_statute_versions(c: &mut Criterion) {
    let statute =
        Statute::new("version-test", "Version Test", Effect::grant("Permission")).with_version(1);

    c.bench_function("statute_with_version", |b| {
        b.iter(|| black_box(statute.clone().with_version(42)))
    });

    c.bench_function("statute_version_check", |b| {
        b.iter(|| black_box(statute.version))
    });
}

/// Benchmark jurisdiction operations
fn bench_jurisdiction_ops(c: &mut Criterion) {
    let statute = Statute::new("jurisdiction-test", "Test", Effect::grant("Permission"))
        .with_jurisdiction("US")
        .with_version(1);

    c.bench_function("statute_with_jurisdiction", |b| {
        b.iter(|| black_box(statute.clone().with_jurisdiction("JP")))
    });

    c.bench_function("statute_check_jurisdiction", |b| {
        b.iter(|| black_box(statute.jurisdiction.as_ref()))
    });

    c.bench_function("statute_version_check_multiple", |b| {
        b.iter(|| {
            let v1 = statute.version;
            let v2 = statute.version;
            black_box((v1, v2))
        })
    });
}

/// Benchmark complex condition patterns
fn bench_complex_conditions(c: &mut Criterion) {
    // Deeply nested AND conditions
    let deeply_nested_and = Condition::age(ComparisonOp::GreaterThan, 18)
        .and(Condition::income(ComparisonOp::LessThan, 50000))
        .and(Condition::has_attribute("citizen"))
        .and(Condition::attribute_equals("status", "active"))
        .and(Condition::has_attribute("employed"));

    c.bench_function("condition_deeply_nested_and", |b| {
        b.iter(|| black_box(deeply_nested_and.clone()))
    });

    // Deeply nested OR conditions
    let deeply_nested_or = Condition::age(ComparisonOp::LessThan, 25)
        .or(Condition::has_attribute("student"))
        .or(Condition::has_attribute("veteran"))
        .or(Condition::has_attribute("disabled"))
        .or(Condition::income(ComparisonOp::LessThan, 20000));

    c.bench_function("condition_deeply_nested_or", |b| {
        b.iter(|| black_box(deeply_nested_or.clone()))
    });

    // Mixed AND/OR conditions
    let mixed = Condition::age(ComparisonOp::GreaterThan, 18)
        .and(
            Condition::income(ComparisonOp::LessThan, 50000).or(Condition::has_attribute("exempt")),
        )
        .and(
            Condition::attribute_equals("status", "active")
                .or(Condition::attribute_equals("status", "pending")),
        );

    c.bench_function("condition_mixed_and_or", |b| {
        b.iter(|| black_box(mixed.clone()))
    });

    // Negated conditions
    let negated = Condition::has_attribute("criminal_record")
        .not()
        .and(Condition::has_attribute("good_standing").not().not());

    c.bench_function("condition_negated", |b| {
        b.iter(|| black_box(negated.clone()))
    });
}

/// Benchmark multiple statute operations
fn bench_statute_collection_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("statute_collections");

    for size in [10, 100, 500, 1000].iter() {
        let statutes: Vec<_> = (0..*size)
            .map(|i| {
                Statute::new(
                    format!("statute-{}", i),
                    format!("Statute {}", i),
                    Effect::grant("Permission"),
                )
                .with_jurisdiction(if i % 2 == 0 { "US" } else { "JP" })
                .with_version(1)
            })
            .collect();

        group.bench_with_input(
            BenchmarkId::new("filter_by_jurisdiction", size),
            &statutes,
            |b, stats| {
                b.iter(|| {
                    let filtered: Vec<_> = stats
                        .iter()
                        .filter(|s| s.jurisdiction.as_deref() == Some("US"))
                        .collect();
                    black_box(filtered)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("find_by_id", size),
            &statutes,
            |b, stats| {
                b.iter(|| {
                    let found = stats.iter().find(|s| s.id == "statute-500");
                    black_box(found)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("validate_all", size),
            &statutes,
            |b, stats| {
                b.iter(|| {
                    let results: Vec<_> = stats.iter().map(|s| s.validate()).collect();
                    black_box(results)
                })
            },
        );
    }

    group.finish();
}

/// Benchmark temporal validity complex scenarios
fn bench_temporal_complex(c: &mut Criterion) {
    let validity_with_both = TemporalValidity::new()
        .with_effective_date(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
        .with_expiry_date(NaiveDate::from_ymd_opt(2030, 12, 31).unwrap());

    let validity_no_expiry =
        TemporalValidity::new().with_effective_date(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap());

    let validity_no_effective =
        TemporalValidity::new().with_expiry_date(NaiveDate::from_ymd_opt(2030, 12, 31).unwrap());

    c.bench_function("temporal_is_active_with_both", |b| {
        b.iter(|| {
            black_box(validity_with_both.is_active(NaiveDate::from_ymd_opt(2025, 6, 15).unwrap()))
        })
    });

    c.bench_function("temporal_is_active_no_expiry", |b| {
        b.iter(|| {
            black_box(validity_no_expiry.is_active(NaiveDate::from_ymd_opt(2025, 6, 15).unwrap()))
        })
    });

    c.bench_function("temporal_is_active_no_effective", |b| {
        b.iter(|| {
            black_box(
                validity_no_effective.is_active(NaiveDate::from_ymd_opt(2025, 6, 15).unwrap()),
            )
        })
    });
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
    bench_serialization,
    bench_statute_clone,
    bench_condition_depth,
    bench_effect_composition,
    bench_statute_versions,
    bench_jurisdiction_ops,
    bench_complex_conditions,
    bench_statute_collection_ops,
    bench_temporal_complex,
);
criterion_main!(benches);
