//! Performance benchmarks for diff operations.

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};
use legalis_diff::{analysis::analyze_changes, diff};

fn create_statute_with_preconditions(id: &str, title: &str, precondition_count: usize) -> Statute {
    let mut statute = Statute::new(id, title, Effect::new(EffectType::Grant, "Test effect"));

    for i in 0..precondition_count {
        statute.preconditions.push(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18 + (i as u32),
        });
    }

    statute
}

fn bench_diff_small(c: &mut Criterion) {
    let old = create_statute_with_preconditions("test-statute", "Old Title", 5);
    let new = create_statute_with_preconditions("test-statute", "New Title", 5);

    c.bench_function("diff_small_statute", |b| {
        b.iter(|| {
            let _ = diff(black_box(&old), black_box(&new));
        });
    });
}

fn bench_diff_medium(c: &mut Criterion) {
    let old = create_statute_with_preconditions("test-statute", "Old Title", 20);
    let new = create_statute_with_preconditions("test-statute", "New Title", 20);

    c.bench_function("diff_medium_statute", |b| {
        b.iter(|| {
            let _ = diff(black_box(&old), black_box(&new));
        });
    });
}

fn bench_diff_large(c: &mut Criterion) {
    let old = create_statute_with_preconditions("test-statute", "Old Title", 100);
    let new = create_statute_with_preconditions("test-statute", "New Title", 100);

    c.bench_function("diff_large_statute", |b| {
        b.iter(|| {
            let _ = diff(black_box(&old), black_box(&new));
        });
    });
}

fn bench_diff_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("diff_scaling");

    for size in [1, 5, 10, 20, 50, 100].iter() {
        let old = create_statute_with_preconditions("test-statute", "Old Title", *size);
        let new = create_statute_with_preconditions("test-statute", "New Title", *size);

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let _ = diff(black_box(&old), black_box(&new));
            });
        });
    }

    group.finish();
}

fn bench_analysis(c: &mut Criterion) {
    let old = create_statute_with_preconditions("test-statute", "Old Title", 20);
    let mut new = create_statute_with_preconditions("test-statute", "New Title", 20);
    new.preconditions[0] = Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 21,
    };

    let diff_result = diff(&old, &new).unwrap();

    c.bench_function("analyze_changes", |b| {
        b.iter(|| {
            let _ = analyze_changes(black_box(&diff_result));
        });
    });
}

fn bench_identical_statutes(c: &mut Criterion) {
    let statute = create_statute_with_preconditions("test-statute", "Title", 20);

    c.bench_function("diff_identical_statutes", |b| {
        b.iter(|| {
            let _ = diff(black_box(&statute), black_box(&statute));
        });
    });
}

fn bench_precondition_changes(c: &mut Criterion) {
    let mut group = c.benchmark_group("precondition_changes");

    // Test with different numbers of changed preconditions
    for change_count in [1, 5, 10, 20].iter() {
        let old = create_statute_with_preconditions("test-statute", "Title", 20);
        let mut new = old.clone();

        // Modify some preconditions
        for i in 0..*change_count {
            if i < new.preconditions.len() {
                new.preconditions[i] = Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 20 + (i as u32),
                };
            }
        }

        group.bench_with_input(
            BenchmarkId::from_parameter(change_count),
            change_count,
            |b, _| {
                b.iter(|| {
                    let _ = diff(black_box(&old), black_box(&new));
                });
            },
        );
    }

    group.finish();
}

fn bench_parallel_diff_pairs(c: &mut Criterion) {
    let pairs: Vec<_> = (0..100)
        .map(|i| {
            let id = format!("statute-{}", i);
            (
                create_statute_with_preconditions(&id, "Old Title", 10),
                create_statute_with_preconditions(&id, "New Title", 10),
            )
        })
        .collect();

    c.bench_function("parallel_diff_pairs_100", |b| {
        b.iter(|| {
            use legalis_diff::parallel::parallel_diff_pairs;
            let _ = parallel_diff_pairs(black_box(&pairs));
        });
    });
}

fn bench_validate_batch(c: &mut Criterion) {
    let old = create_statute_with_preconditions("test", "Old", 20);
    let new = create_statute_with_preconditions("test", "New", 20);
    let diff_result = diff(&old, &new).unwrap();

    let diffs: Vec<_> = std::iter::repeat_n(diff_result, 100).collect();

    c.bench_function("validate_batch_100", |b| {
        b.iter(|| {
            use legalis_diff::validation::validate_batch;
            let _ = validate_batch(black_box(&diffs));
        });
    });
}

fn bench_validate_batch_parallel(c: &mut Criterion) {
    let old = create_statute_with_preconditions("test", "Old", 20);
    let new = create_statute_with_preconditions("test", "New", 20);
    let diff_result = diff(&old, &new).unwrap();

    let diffs: Vec<_> = std::iter::repeat_n(diff_result, 100).collect();

    c.bench_function("validate_batch_parallel_100", |b| {
        b.iter(|| {
            use legalis_diff::validation::validate_batch_parallel;
            let _ = validate_batch_parallel(black_box(&diffs));
        });
    });
}

fn bench_rollback_generation(c: &mut Criterion) {
    let old = create_statute_with_preconditions("test", "Old", 20);
    let mut new = create_statute_with_preconditions("test", "New", 20);
    new.preconditions[0] = Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 25,
    };

    let forward_diff = diff(&old, &new).unwrap();

    c.bench_function("generate_rollback", |b| {
        b.iter(|| {
            use legalis_diff::rollback::generate_rollback;
            let _ = generate_rollback(black_box(&forward_diff));
        });
    });
}

fn bench_rollback_analysis(c: &mut Criterion) {
    let old = create_statute_with_preconditions("test", "Old", 20);
    let mut new = create_statute_with_preconditions("test", "New", 20);
    new.preconditions[0] = Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 25,
    };

    let forward_diff = diff(&old, &new).unwrap();

    c.bench_function("analyze_rollback", |b| {
        b.iter(|| {
            use legalis_diff::rollback::analyze_rollback;
            let _ = analyze_rollback(black_box(&forward_diff));
        });
    });
}

fn bench_parallel_generate_rollbacks(c: &mut Criterion) {
    let pairs: Vec<_> = (0..100)
        .map(|i| {
            let id = format!("statute-{}", i);
            (
                create_statute_with_preconditions(&id, "Old Title", 10),
                create_statute_with_preconditions(&id, "New Title", 10),
            )
        })
        .collect();

    let forward_diffs: Vec<_> = pairs
        .iter()
        .map(|(old, new)| diff(old, new).unwrap())
        .collect();

    c.bench_function("parallel_generate_rollbacks_100", |b| {
        b.iter(|| {
            use legalis_diff::rollback::parallel_generate_rollbacks;
            let _ = parallel_generate_rollbacks(black_box(&forward_diffs));
        });
    });
}

fn bench_parallel_analyze_rollbacks(c: &mut Criterion) {
    let pairs: Vec<_> = (0..100)
        .map(|i| {
            let id = format!("statute-{}", i);
            (
                create_statute_with_preconditions(&id, "Old Title", 10),
                create_statute_with_preconditions(&id, "New Title", 10),
            )
        })
        .collect();

    let forward_diffs: Vec<_> = pairs
        .iter()
        .map(|(old, new)| diff(old, new).unwrap())
        .collect();

    c.bench_function("parallel_analyze_rollbacks_100", |b| {
        b.iter(|| {
            use legalis_diff::rollback::parallel_analyze_rollbacks;
            let _ = parallel_analyze_rollbacks(black_box(&forward_diffs));
        });
    });
}

fn bench_compute_rollback_statistics(c: &mut Criterion) {
    let pairs: Vec<_> = (0..100)
        .map(|i| {
            let id = format!("statute-{}", i);
            (
                create_statute_with_preconditions(&id, "Old Title", 10),
                create_statute_with_preconditions(&id, "New Title", 10),
            )
        })
        .collect();

    let forward_diffs: Vec<_> = pairs
        .iter()
        .map(|(old, new)| diff(old, new).unwrap())
        .collect();

    use legalis_diff::rollback::parallel_analyze_rollbacks;
    let analyses = parallel_analyze_rollbacks(&forward_diffs);

    c.bench_function("compute_rollback_statistics_100", |b| {
        b.iter(|| {
            use legalis_diff::rollback::compute_rollback_statistics;
            let _ = compute_rollback_statistics(black_box(&analyses));
        });
    });
}

fn bench_validate_rollbacks_parallel(c: &mut Criterion) {
    let pairs: Vec<_> = (0..100)
        .map(|i| {
            let id = format!("statute-{}", i);
            (
                create_statute_with_preconditions(&id, "Old Title", 10),
                create_statute_with_preconditions(&id, "New Title", 10),
            )
        })
        .collect();

    let forward_diffs: Vec<_> = pairs
        .iter()
        .map(|(old, new)| diff(old, new).unwrap())
        .collect();

    use legalis_diff::rollback::parallel_generate_rollbacks;
    let rollback_diffs = parallel_generate_rollbacks(&forward_diffs);

    c.bench_function("validate_rollbacks_parallel_100", |b| {
        b.iter(|| {
            use legalis_diff::validation::validate_rollbacks_parallel;
            let _ =
                validate_rollbacks_parallel(black_box(&forward_diffs), black_box(&rollback_diffs));
        });
    });
}

criterion_group!(
    benches,
    bench_diff_small,
    bench_diff_medium,
    bench_diff_large,
    bench_diff_scaling,
    bench_analysis,
    bench_identical_statutes,
    bench_precondition_changes,
    bench_parallel_diff_pairs,
    bench_validate_batch,
    bench_validate_batch_parallel,
    bench_rollback_generation,
    bench_rollback_analysis,
    bench_parallel_generate_rollbacks,
    bench_parallel_analyze_rollbacks,
    bench_compute_rollback_statistics,
    bench_validate_rollbacks_parallel
);
criterion_main!(benches);
