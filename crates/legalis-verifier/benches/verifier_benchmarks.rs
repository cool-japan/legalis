//! Performance benchmarks for legalis-verifier critical operations.
//!
//! Run with: cargo bench

use chrono::NaiveDate;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute, TemporalValidity};
use legalis_verifier::{
    ConstitutionalPrinciple, PrincipleCheck, StatuteVerifier, VerificationBudget,
    analyze_complexity, analyze_coverage, analyze_quality, analyze_regulatory_impact,
    analyze_statute_statistics, batch_verify, compare_statutes, detect_duplicates,
    detect_statute_conflicts, semantic_similarity, verify_integrity,
};
use std::hint::black_box;

/// Helper to create a simple statute for benchmarking
fn create_simple_statute(id: &str) -> Statute {
    Statute::new(
        id,
        "Simple Statute",
        Effect::new(EffectType::Grant, "Grant permission"),
    )
    .with_precondition(Condition::age(ComparisonOp::GreaterThan, 18))
    .with_jurisdiction("US")
    .with_version(1)
}

/// Helper to create a complex statute for benchmarking
fn create_complex_statute(id: &str) -> Statute {
    Statute::new(
        id,
        "Complex Tax Regulation Statute",
        Effect::new(EffectType::Obligation, "Must file quarterly taxes"),
    )
    .with_precondition(
        Condition::age(ComparisonOp::GreaterOrEqual, 18).and(
            Condition::income(ComparisonOp::GreaterThan, 50000)
                .or(Condition::has_attribute("self_employed"))
                .and(Condition::attribute_equals("tax_status", "resident").not()),
        ),
    )
    .with_temporal_validity(
        TemporalValidity::new()
            .with_effective_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
            .with_expiry_date(NaiveDate::from_ymd_opt(2030, 12, 31).unwrap()),
    )
    .with_jurisdiction("US")
    .with_version(1)
}

/// Benchmark basic statute verification
fn bench_statute_verification(c: &mut Criterion) {
    let verifier = StatuteVerifier::new();
    let simple_statute = create_simple_statute("simple-001");
    let complex_statute = create_complex_statute("complex-001");

    c.bench_function("verify_simple_statute", |b| {
        b.iter(|| black_box(verifier.verify(std::slice::from_ref(&simple_statute))))
    });

    c.bench_function("verify_complex_statute", |b| {
        b.iter(|| black_box(verifier.verify(std::slice::from_ref(&complex_statute))))
    });
}

/// Benchmark circular reference detection
fn bench_circular_reference_detection(c: &mut Criterion) {
    let verifier = StatuteVerifier::new();

    // Create statutes with circular references using Custom conditions
    let stat1 = Statute::new(
        "stat-1",
        "Statute 1",
        Effect::new(EffectType::Grant, "Grant permission"),
    )
    .with_precondition(Condition::custom("statute:stat-2"))
    .with_version(1);

    let stat2 = Statute::new(
        "stat-2",
        "Statute 2",
        Effect::new(EffectType::Grant, "Grant permission"),
    )
    .with_precondition(Condition::custom("statute:stat-3"))
    .with_version(1);

    let stat3 = Statute::new(
        "stat-3",
        "Statute 3",
        Effect::new(EffectType::Grant, "Grant permission"),
    )
    .with_precondition(Condition::custom("statute:stat-1")) // Circular!
    .with_version(1);

    let statutes = vec![stat1, stat2, stat3];

    c.bench_function("detect_circular_references", |b| {
        b.iter(|| black_box(verifier.verify(&statutes)))
    });
}

/// Benchmark constitutional principle checking
fn bench_constitutional_checks(c: &mut Criterion) {
    let verifier = StatuteVerifier::with_principles(vec![ConstitutionalPrinciple {
        id: "equality".to_string(),
        name: "Equality".to_string(),
        description: "Equal protection under the law".to_string(),
        check: PrincipleCheck::EqualityCheck,
    }]);

    let statute = create_complex_statute("equality-test");

    c.bench_function("check_equality_principle", |b| {
        b.iter(|| black_box(verifier.verify(std::slice::from_ref(&statute))))
    });
}

/// Benchmark caching performance
fn bench_caching(c: &mut Criterion) {
    let mut group = c.benchmark_group("caching");

    let statute = create_complex_statute("cache-test");

    // Benchmark without caching
    let verifier_no_cache = StatuteVerifier::new();
    group.bench_function("verify_no_cache", |b| {
        b.iter(|| black_box(verifier_no_cache.verify(std::slice::from_ref(&statute))))
    });

    // Benchmark with caching (first call)
    let verifier_with_cache = StatuteVerifier::new().with_caching();
    group.bench_function("verify_with_cache_first", |b| {
        b.iter(|| {
            verifier_with_cache.clear_cache();
            black_box(verifier_with_cache.verify(std::slice::from_ref(&statute)))
        })
    });

    // Benchmark with caching (cached call)
    verifier_with_cache.verify(std::slice::from_ref(&statute)); // Prime the cache
    group.bench_function("verify_with_cache_hit", |b| {
        b.iter(|| black_box(verifier_with_cache.verify(std::slice::from_ref(&statute))))
    });

    group.finish();
}

/// Benchmark complexity analysis
fn bench_complexity_analysis(c: &mut Criterion) {
    let simple = create_simple_statute("simple");
    let complex = create_complex_statute("complex");

    c.bench_function("complexity_simple", |b| {
        b.iter(|| black_box(analyze_complexity(&simple)))
    });

    c.bench_function("complexity_complex", |b| {
        b.iter(|| black_box(analyze_complexity(&complex)))
    });
}

/// Benchmark coverage analysis
fn bench_coverage_analysis(c: &mut Criterion) {
    let mut group = c.benchmark_group("coverage_analysis");

    for size in [10, 50, 100, 500].iter() {
        let statutes: Vec<_> = (0..*size)
            .map(|i| {
                if i % 2 == 0 {
                    create_simple_statute(&format!("stat-{}", i))
                } else {
                    create_complex_statute(&format!("stat-{}", i))
                }
            })
            .collect();

        group.bench_with_input(
            BenchmarkId::new("coverage_analysis", size),
            &statutes,
            |b, stats| b.iter(|| black_box(analyze_coverage(stats))),
        );
    }

    group.finish();
}

/// Benchmark semantic similarity
fn bench_semantic_similarity(c: &mut Criterion) {
    let stat1 = create_simple_statute("stat-1");
    let stat2 = create_simple_statute("stat-2");
    let stat3 = create_complex_statute("stat-3");

    c.bench_function("similarity_identical", |b| {
        b.iter(|| black_box(semantic_similarity(&stat1, &stat2)))
    });

    c.bench_function("similarity_different", |b| {
        b.iter(|| black_box(semantic_similarity(&stat1, &stat3)))
    });
}

/// Benchmark conflict detection
fn bench_conflict_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("conflict_detection");

    for size in [10, 50, 100].iter() {
        let statutes: Vec<_> = (0..*size)
            .map(|i| {
                let mut statute = if i % 3 == 0 {
                    create_simple_statute(&format!("stat-{}", i))
                } else {
                    create_complex_statute(&format!("stat-{}", i))
                };

                // Add some overlapping jurisdictions
                if i % 5 == 0 {
                    statute = statute.with_jurisdiction("US");
                }

                statute
            })
            .collect();

        group.bench_with_input(
            BenchmarkId::new("detect_conflicts", size),
            &statutes,
            |b, stats| b.iter(|| black_box(detect_statute_conflicts(stats))),
        );
    }

    group.finish();
}

/// Benchmark quality analysis
fn bench_quality_analysis(c: &mut Criterion) {
    let simple = create_simple_statute("simple");
    let complex = create_complex_statute("complex");

    c.bench_function("quality_simple", |b| {
        b.iter(|| black_box(analyze_quality(&simple)))
    });

    c.bench_function("quality_complex", |b| {
        b.iter(|| black_box(analyze_quality(&complex)))
    });
}

/// Benchmark statute comparison
fn bench_statute_comparison(c: &mut Criterion) {
    let old_statute = create_simple_statute("statute-v1");
    let mut new_statute = create_simple_statute("statute-v1");
    new_statute.title = "Updated Statute Title".to_string();
    new_statute
        .preconditions
        .push(Condition::income(ComparisonOp::LessThan, 75000));

    c.bench_function("compare_statutes", |b| {
        b.iter(|| black_box(compare_statutes(&old_statute, &new_statute)))
    });
}

/// Benchmark batch verification
fn bench_batch_verification(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_verification");
    let verifier = StatuteVerifier::new();

    for size in [10, 50, 100].iter() {
        let statutes: Vec<_> = (0..*size)
            .map(|i| create_simple_statute(&format!("stat-{}", i)))
            .collect();

        group.bench_with_input(
            BenchmarkId::new("batch_verify", size),
            &statutes,
            |b, stats| b.iter(|| black_box(batch_verify(stats, &verifier))),
        );
    }

    group.finish();
}

/// Benchmark duplicate detection
fn bench_duplicate_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("duplicate_detection");

    for size in [10, 50, 100].iter() {
        let statutes: Vec<_> = (0..*size)
            .map(|i| {
                // Create some similar statutes
                if i % 3 == 0 {
                    create_simple_statute(&format!("stat-{}", i))
                } else {
                    create_complex_statute(&format!("stat-{}", i))
                }
            })
            .collect();

        group.bench_with_input(
            BenchmarkId::new("detect_duplicates", size),
            &statutes,
            |b, stats| b.iter(|| black_box(detect_duplicates(stats, 0.7))),
        );
    }

    group.finish();
}

/// Benchmark regulatory impact analysis
fn bench_regulatory_impact(c: &mut Criterion) {
    let simple = create_simple_statute("simple");
    let complex = create_complex_statute("complex");

    c.bench_function("impact_simple", |b| {
        b.iter(|| black_box(analyze_regulatory_impact(&simple)))
    });

    c.bench_function("impact_complex", |b| {
        b.iter(|| black_box(analyze_regulatory_impact(&complex)))
    });
}

/// Benchmark statistics analysis
fn bench_statistics_analysis(c: &mut Criterion) {
    let mut group = c.benchmark_group("statistics_analysis");

    for size in [10, 50, 100, 500].iter() {
        let statutes: Vec<_> = (0..*size)
            .map(|i| {
                if i % 2 == 0 {
                    create_simple_statute(&format!("stat-{}", i))
                } else {
                    create_complex_statute(&format!("stat-{}", i))
                }
            })
            .collect();

        group.bench_with_input(
            BenchmarkId::new("analyze_statistics", size),
            &statutes,
            |b, stats| b.iter(|| black_box(analyze_statute_statistics(stats))),
        );
    }

    group.finish();
}

/// Benchmark integrity verification
fn bench_integrity_verification(c: &mut Criterion) {
    let mut group = c.benchmark_group("integrity_verification");

    for size in [10, 50, 100].iter() {
        let statutes: Vec<_> = (0..*size)
            .map(|i| create_simple_statute(&format!("stat-{}", i)))
            .collect();

        group.bench_with_input(
            BenchmarkId::new("verify_integrity", size),
            &statutes,
            |b, stats| b.iter(|| black_box(verify_integrity(stats))),
        );
    }

    group.finish();
}

/// Benchmark verification budget
fn bench_verification_budget(c: &mut Criterion) {
    let verifier = StatuteVerifier::new();
    let statute = create_complex_statute("budgeted");

    let unlimited_budget = VerificationBudget::unlimited();
    let limited_budget = VerificationBudget {
        max_statutes: Some(100),
        max_checks: Some(10),
        max_time_ms: Some(5000),
    };

    c.bench_function("verify_unlimited_budget", |b| {
        b.iter(|| {
            black_box(verifier.verify_with_budget(std::slice::from_ref(&statute), unlimited_budget))
        })
    });

    c.bench_function("verify_limited_budget", |b| {
        b.iter(|| {
            black_box(verifier.verify_with_budget(std::slice::from_ref(&statute), limited_budget))
        })
    });
}

/// Benchmark report generation
fn bench_report_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("report_generation");
    let verifier = StatuteVerifier::new();

    for size in [10, 50, 100].iter() {
        let statutes: Vec<_> = (0..*size)
            .map(|i| create_complex_statute(&format!("stat-{}", i)))
            .collect();

        let result = verifier.verify(&statutes);

        group.bench_with_input(
            BenchmarkId::new("json_report", size),
            &result,
            |b, result| {
                b.iter(|| {
                    let json = serde_json::to_string(result).unwrap();
                    black_box(json)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("format_errors", size),
            &result,
            |b, result| {
                b.iter(|| {
                    let formatted: Vec<_> =
                        result.errors.iter().map(|e| format!("{}", e)).collect();
                    black_box(formatted)
                })
            },
        );
    }

    group.finish();
}

/// Benchmark SMT solver operations
#[cfg(feature = "smt-solver")]
fn bench_smt_solver(c: &mut Criterion) {
    use legalis_verifier::SmtVerifier;

    let mut group = c.benchmark_group("smt_solver");

    let simple_statute = create_simple_statute("smt-simple");
    let complex_statute = create_complex_statute("smt-complex");

    group.bench_function("smt_verify_simple", |b| {
        b.iter(|| {
            let mut verifier = SmtVerifier::new();
            if let Some(precondition) = &simple_statute.precondition {
                black_box(verifier.is_satisfiable(precondition))
            } else {
                Ok(true)
            }
        })
    });

    group.bench_function("smt_verify_complex", |b| {
        b.iter(|| {
            let mut verifier = SmtVerifier::new();
            if let Some(precondition) = &complex_statute.precondition {
                black_box(verifier.is_satisfiable(precondition))
            } else {
                Ok(true)
            }
        })
    });

    // Benchmark satisfiability checking
    group.bench_function("smt_check_satisfiability", |b| {
        b.iter(|| {
            let mut verifier = SmtVerifier::new();
            if let Some(precondition) = &complex_statute.precondition {
                black_box(verifier.is_satisfiable(precondition))
            } else {
                Ok(true)
            }
        })
    });

    group.finish();
}

/// Benchmark parallel verification
fn bench_parallel_verification(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel_verification");

    for size in [50, 100, 250].iter() {
        let statutes: Vec<_> = (0..*size)
            .map(|i| create_simple_statute(&format!("parallel-{}", i)))
            .collect();

        group.bench_with_input(
            BenchmarkId::new("sequential", size),
            &statutes,
            |b, stats| {
                let verifier = StatuteVerifier::new();
                b.iter(|| {
                    for stat in stats {
                        verifier.verify(std::slice::from_ref(stat));
                    }
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("batch_parallel", size),
            &statutes,
            |b, stats| {
                let verifier = StatuteVerifier::new();
                b.iter(|| black_box(batch_verify(stats, &verifier)))
            },
        );
    }

    group.finish();
}

/// Benchmark verification with different error counts
fn bench_error_accumulation(c: &mut Criterion) {
    let mut group = c.benchmark_group("error_accumulation");

    // Create statutes with intentional errors
    for error_count in [0, 5, 10, 25].iter() {
        let mut statutes = vec![create_simple_statute("base")];

        // Add statutes with circular references (errors)
        for i in 0..*error_count {
            let stat = Statute::new(
                format!("error-{}", i),
                "Error Statute",
                Effect::new(EffectType::Grant, "Grant"),
            )
            .with_precondition(Condition::custom(format!(
                "statute:error-{}",
                (i + 1) % error_count
            )))
            .with_version(1);
            statutes.push(stat);
        }

        group.bench_with_input(
            BenchmarkId::new("verify_with_errors", error_count),
            &statutes,
            |b, stats| {
                let verifier = StatuteVerifier::new();
                b.iter(|| black_box(verifier.verify(stats)))
            },
        );
    }

    group.finish();
}

/// Benchmark incremental verification
fn bench_incremental_verification(c: &mut Criterion) {
    let verifier = StatuteVerifier::new().with_caching();
    let base_statutes: Vec<_> = (0..50)
        .map(|i| create_simple_statute(&format!("base-{}", i)))
        .collect();

    // Prime the cache
    verifier.verify(&base_statutes);

    c.bench_function("incremental_single_change", |b| {
        b.iter(|| {
            let mut modified = base_statutes.clone();
            modified.push(create_simple_statute("new-statute"));
            black_box(verifier.verify(&modified))
        })
    });

    c.bench_function("incremental_multiple_changes", |b| {
        b.iter(|| {
            let mut modified = base_statutes.clone();
            for i in 0..5 {
                modified.push(create_simple_statute(&format!("new-{}", i)));
            }
            black_box(verifier.verify(&modified))
        })
    });
}

/// Benchmark verification result operations
fn bench_result_operations(c: &mut Criterion) {
    let verifier = StatuteVerifier::new();
    let statute = create_complex_statute("result-test");
    let result = verifier.verify(std::slice::from_ref(&statute));

    c.bench_function("result_clone", |b| b.iter(|| black_box(result.clone())));

    c.bench_function("result_error_count", |b| {
        b.iter(|| black_box(result.errors.len()))
    });

    c.bench_function("result_warning_count", |b| {
        b.iter(|| black_box(result.warnings.len()))
    });
}

/// Benchmark verification metrics
fn bench_verification_metrics(c: &mut Criterion) {
    let statutes: Vec<_> = (0..100)
        .map(|i| {
            if i % 3 == 0 {
                create_simple_statute(&format!("stat-{}", i))
            } else {
                create_complex_statute(&format!("stat-{}", i))
            }
        })
        .collect();

    c.bench_function("calculate_metrics", |b| {
        b.iter(|| {
            let stats = analyze_statute_statistics(&statutes);
            black_box(stats)
        })
    });

    c.bench_function("calculate_coverage", |b| {
        b.iter(|| {
            let coverage = analyze_coverage(&statutes);
            black_box(coverage)
        })
    });
}

/// Benchmark deep dependency analysis
fn bench_dependency_analysis(c: &mut Criterion) {
    let mut group = c.benchmark_group("dependency_analysis");

    for depth in [2, 5, 10].iter() {
        // Create statutes with dependency chains
        let mut statutes = Vec::new();
        for i in 0..*depth {
            let precond = if i == 0 {
                Condition::age(ComparisonOp::GreaterThan, 18)
            } else {
                Condition::custom(format!("statute:dep-{}", i - 1))
            };

            let stat = Statute::new(
                format!("dep-{}", i),
                format!("Dependent Statute {}", i),
                Effect::grant("Permission"),
            )
            .with_precondition(precond)
            .with_version(1);

            statutes.push(stat);
        }

        group.bench_with_input(
            BenchmarkId::new("analyze_dependencies", depth),
            &statutes,
            |b, stats| {
                let verifier = StatuteVerifier::new();
                b.iter(|| black_box(verifier.verify(stats)))
            },
        );
    }

    group.finish();
}

/// Benchmark verification with different complexity levels
fn bench_complexity_levels(c: &mut Criterion) {
    let mut group = c.benchmark_group("complexity_levels");

    let simple = create_simple_statute("simple");
    let medium = Statute::new("medium", "Medium", Effect::grant("Permission"))
        .with_precondition(
            Condition::age(ComparisonOp::GreaterThan, 18)
                .and(Condition::income(ComparisonOp::LessThan, 50000)),
        )
        .with_version(1);
    let complex = create_complex_statute("complex");

    group.bench_function("verify_simple", |b| {
        let verifier = StatuteVerifier::new();
        b.iter(|| black_box(verifier.verify(std::slice::from_ref(&simple))))
    });

    group.bench_function("verify_medium", |b| {
        let verifier = StatuteVerifier::new();
        b.iter(|| black_box(verifier.verify(std::slice::from_ref(&medium))))
    });

    group.bench_function("verify_complex", |b| {
        let verifier = StatuteVerifier::new();
        b.iter(|| black_box(verifier.verify(std::slice::from_ref(&complex))))
    });

    group.finish();
}

/// Benchmark cross-statute analysis
fn bench_cross_statute_analysis(c: &mut Criterion) {
    let mut group = c.benchmark_group("cross_statute_analysis");

    for size in [10, 25, 50].iter() {
        let statutes: Vec<_> = (0..*size)
            .map(|i| create_complex_statute(&format!("stat-{}", i)))
            .collect();

        group.bench_with_input(
            BenchmarkId::new("detect_all_conflicts", size),
            &statutes,
            |b, stats| b.iter(|| black_box(detect_statute_conflicts(stats))),
        );

        group.bench_with_input(
            BenchmarkId::new("find_all_duplicates", size),
            &statutes,
            |b, stats| b.iter(|| black_box(detect_duplicates(stats, 0.8))),
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_statute_verification,
    bench_circular_reference_detection,
    bench_constitutional_checks,
    bench_caching,
    bench_complexity_analysis,
    bench_coverage_analysis,
    bench_semantic_similarity,
    bench_conflict_detection,
    bench_quality_analysis,
    bench_statute_comparison,
    bench_batch_verification,
    bench_duplicate_detection,
    bench_regulatory_impact,
    bench_statistics_analysis,
    bench_integrity_verification,
    bench_verification_budget,
    bench_report_generation,
    bench_parallel_verification,
    bench_error_accumulation,
    bench_incremental_verification,
    bench_result_operations,
    bench_verification_metrics,
    bench_dependency_analysis,
    bench_complexity_levels,
    bench_cross_statute_analysis,
);

#[cfg(feature = "smt-solver")]
criterion_group!(smt_benches, bench_smt_solver);

#[cfg(feature = "smt-solver")]
criterion_main!(benches, smt_benches);

#[cfg(not(feature = "smt-solver"))]
criterion_main!(benches);
