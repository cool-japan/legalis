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

criterion_group!(
    benches,
    bench_diff_small,
    bench_diff_medium,
    bench_diff_large,
    bench_diff_scaling,
    bench_analysis,
    bench_identical_statutes,
    bench_precondition_changes
);
criterion_main!(benches);
