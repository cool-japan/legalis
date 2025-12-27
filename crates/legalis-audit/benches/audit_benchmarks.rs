//! Performance benchmarks for legalis-audit.
//!
//! Run with: cargo bench

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use legalis_audit::{
    Actor, AuditRecord, AuditTrail, DecisionContext, DecisionResult, EventType,
};
use std::collections::HashMap;
use uuid::Uuid;

/// Creates a test audit record.
fn create_test_record() -> AuditRecord {
    AuditRecord::new(
        EventType::AutomaticDecision,
        Actor::System {
            component: "benchmark".to_string(),
        },
        "statute-benchmark".to_string(),
        Uuid::new_v4(),
        DecisionContext {
            attributes: {
                let mut attrs = HashMap::new();
                attrs.insert("key1".to_string(), "value1".to_string());
                attrs.insert("key2".to_string(), "value2".to_string());
                attrs
            },
            metadata: HashMap::new(),
            evaluated_conditions: Vec::new(),
        },
        DecisionResult::Deterministic {
            effect_applied: "approved".to_string(),
            parameters: HashMap::new(),
        },
        None,
    )
}

/// Benchmarks single record insertion.
fn bench_single_insert(c: &mut Criterion) {
    c.bench_function("single_insert", |b| {
        b.iter(|| {
            let mut trail = AuditTrail::new();
            let record = create_test_record();
            black_box(trail.record(record).unwrap());
        });
    });
}

/// Benchmarks bulk record insertion.
fn bench_bulk_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("bulk_insert");

    for size in [10, 100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut trail = AuditTrail::new();
                for _ in 0..size {
                    let record = create_test_record();
                    black_box(trail.record(record).unwrap());
                }
            });
        });
    }

    group.finish();
}

/// Benchmarks integrity verification.
fn bench_verify_integrity(c: &mut Criterion) {
    let mut group = c.benchmark_group("verify_integrity");

    for size in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut trail = AuditTrail::new();
            for _ in 0..size {
                let record = create_test_record();
                trail.record(record).unwrap();
            }

            b.iter(|| {
                black_box(trail.verify_integrity().unwrap());
            });
        });
    }

    group.finish();
}

/// Benchmarks query by statute.
fn bench_query_by_statute(c: &mut Criterion) {
    let mut group = c.benchmark_group("query_by_statute");

    for size in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut trail = AuditTrail::new();
            for i in 0..size {
                let mut record = create_test_record();
                record.statute_id = format!("statute-{}", i % 10); // 10 different statutes
                trail.record(record).unwrap();
            }

            b.iter(|| {
                black_box(trail.query_by_statute("statute-5").unwrap());
            });
        });
    }

    group.finish();
}

/// Benchmarks query by subject.
fn bench_query_by_subject(c: &mut Criterion) {
    let mut group = c.benchmark_group("query_by_subject");

    for size in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let subjects: Vec<_> = (0..10).map(|_| Uuid::new_v4()).collect();
            let mut trail = AuditTrail::new();

            for i in 0..size {
                let mut record = create_test_record();
                record.subject_id = subjects[i % 10];
                trail.record(record).unwrap();
            }

            let target_subject = subjects[5];

            b.iter(|| {
                black_box(trail.query_by_subject(target_subject).unwrap());
            });
        });
    }

    group.finish();
}

/// Benchmarks compliance report generation.
fn bench_generate_report(c: &mut Criterion) {
    let mut group = c.benchmark_group("generate_report");

    for size in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut trail = AuditTrail::new();
            for _ in 0..size {
                let record = create_test_record();
                trail.record(record).unwrap();
            }

            b.iter(|| {
                black_box(trail.generate_report().unwrap());
            });
        });
    }

    group.finish();
}

/// Benchmarks Merkle tree construction.
fn bench_merkle_tree(c: &mut Criterion) {
    let mut group = c.benchmark_group("merkle_tree");

    for size in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut trail = AuditTrail::new();
            for _ in 0..size {
                let record = create_test_record();
                trail.record(record).unwrap();
            }

            b.iter(|| {
                black_box(trail.build_merkle_tree().unwrap());
            });
        });
    }

    group.finish();
}

/// Benchmarks pattern analysis.
fn bench_analyze_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("analyze_patterns");

    for size in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut trail = AuditTrail::new();
            for i in 0..size {
                let mut record = create_test_record();
                record.statute_id = format!("statute-{}", i % 10);
                trail.record(record).unwrap();
            }

            b.iter(|| {
                black_box(trail.analyze_patterns().unwrap());
            });
        });
    }

    group.finish();
}

/// Benchmarks CSV export.
fn bench_export_csv(c: &mut Criterion) {
    let mut group = c.benchmark_group("export_csv");

    for size in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut trail = AuditTrail::new();
            for _ in 0..size {
                let record = create_test_record();
                trail.record(record).unwrap();
            }

            b.iter(|| {
                let mut output = Vec::new();
                black_box(trail.export_csv(&mut output).unwrap());
            });
        });
    }

    group.finish();
}

/// Benchmarks JSON export.
fn bench_export_json(c: &mut Criterion) {
    let mut group = c.benchmark_group("export_json");

    for size in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut trail = AuditTrail::new();
            for _ in 0..size {
                let record = create_test_record();
                trail.record(record).unwrap();
            }

            b.iter(|| {
                black_box(trail.export_json().unwrap());
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_single_insert,
    bench_bulk_insert,
    bench_verify_integrity,
    bench_query_by_statute,
    bench_query_by_subject,
    bench_generate_report,
    bench_merkle_tree,
    bench_analyze_patterns,
    bench_export_csv,
    bench_export_json,
);

criterion_main!(benches);
