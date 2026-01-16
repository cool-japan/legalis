//! Performance benchmarks for diff operations.

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};
use legalis_diff::{analysis::analyze_changes, diff};
use std::hint::black_box;

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

// Benchmarks for audit trail
fn bench_audit_trail_creation(c: &mut Criterion) {
    let old = create_statute_with_preconditions("test", "Old", 10);
    let new = create_statute_with_preconditions("test", "New", 10);
    let diff_result = diff(&old, &new).unwrap();

    c.bench_function("audit_trail_creation", |b| {
        b.iter(|| {
            use legalis_diff::audit::{AuditTrail, ChangeAttribution};
            let attr = ChangeAttribution::new("Test User");
            let _ = AuditTrail::new(black_box(diff_result.clone()), black_box(attr));
        });
    });
}

fn bench_audit_lifecycle_transitions(c: &mut Criterion) {
    let old = create_statute_with_preconditions("test", "Old", 10);
    let new = create_statute_with_preconditions("test", "New", 10);
    let diff_result = diff(&old, &new).unwrap();

    use legalis_diff::audit::{AuditTrail, ChangeAttribution, ChangeLifecycle};
    let attr = ChangeAttribution::new("Test User");
    let mut audit = AuditTrail::new(diff_result, attr);

    c.bench_function("audit_lifecycle_transition", |b| {
        b.iter(|| {
            audit.transition_to(
                black_box(ChangeLifecycle::UnderReview),
                black_box("Reviewer"),
                None,
            );
        });
    });
}

// Benchmarks for streaming diff
fn bench_streaming_diff(c: &mut Criterion) {
    use legalis_diff::streaming::StreamingDiffer;

    let mut group = c.benchmark_group("streaming_diff");
    for size in [10, 50, 100, 500].iter() {
        let old = create_statute_with_preconditions("test", "Old", *size);
        let new = create_statute_with_preconditions("test", "New", *size);
        let differ = StreamingDiffer::new();

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let _ = differ.stream_diff(black_box(&old), black_box(&new));
            });
        });
    }
    group.finish();
}

fn bench_memory_efficient_diff(c: &mut Criterion) {
    use legalis_diff::streaming::MemoryEfficientDiffer;

    let old = create_statute_with_preconditions("test", "Old", 200);
    let new = create_statute_with_preconditions("test", "New", 200);
    let differ = MemoryEfficientDiffer::new();

    c.bench_function("memory_efficient_diff_200", |b| {
        b.iter(|| {
            let _ = differ.diff(black_box(&old), black_box(&new));
        });
    });
}

fn bench_incremental_updater(c: &mut Criterion) {
    use legalis_diff::streaming::IncrementalUpdater;

    let v1 = create_statute_with_preconditions("test", "V1", 50);
    let v2 = create_statute_with_preconditions("test", "V2", 50);
    let mut updater = IncrementalUpdater::new();

    c.bench_function("incremental_updater", |b| {
        b.iter(|| {
            let _ = updater.update(black_box(&v1), black_box(&v2));
        });
    });
}

// Benchmarks for export formats
fn bench_export_unified_diff(c: &mut Criterion) {
    let old = create_statute_with_preconditions("test", "Old", 20);
    let new = create_statute_with_preconditions("test", "New", 20);
    let diff_result = diff(&old, &new).unwrap();

    c.bench_function("export_unified_diff", |b| {
        b.iter(|| {
            use legalis_diff::export::generate_unified_diff;
            let _ = generate_unified_diff(black_box(&diff_result));
        });
    });
}

fn bench_export_latex(c: &mut Criterion) {
    let old = create_statute_with_preconditions("test", "Old", 20);
    let new = create_statute_with_preconditions("test", "New", 20);
    let diff_result = diff(&old, &new).unwrap();

    c.bench_function("export_latex_redline", |b| {
        b.iter(|| {
            use legalis_diff::export::generate_latex_redline;
            let _ = generate_latex_redline(black_box(&diff_result));
        });
    });
}

fn bench_export_changelog(c: &mut Criterion) {
    let old = create_statute_with_preconditions("test", "Old", 20);
    let new = create_statute_with_preconditions("test", "New", 20);
    let diff_result = diff(&old, &new).unwrap();

    c.bench_function("export_changelog", |b| {
        b.iter(|| {
            use legalis_diff::export::generate_changelog;
            let _ = generate_changelog(black_box(&diff_result), "1.0.0");
        });
    });
}

fn bench_export_pdf_content(c: &mut Criterion) {
    let old = create_statute_with_preconditions("test", "Old", 20);
    let new = create_statute_with_preconditions("test", "New", 20);
    let diff_result = diff(&old, &new).unwrap();

    c.bench_function("export_pdf_content", |b| {
        b.iter(|| {
            use legalis_diff::export::{PdfConfig, generate_pdf_content};
            let config = PdfConfig::default();
            let _ = generate_pdf_content(black_box(&diff_result), black_box(&config));
        });
    });
}

// Benchmarks for integration
fn bench_webhook_payload_creation(c: &mut Criterion) {
    let old = create_statute_with_preconditions("test", "Old", 20);
    let new = create_statute_with_preconditions("test", "New", 20);
    let diff_result = diff(&old, &new).unwrap();

    c.bench_function("webhook_payload_creation", |b| {
        b.iter(|| {
            use legalis_diff::integration::{WebhookEvent, WebhookPayload};
            let _ = WebhookPayload::new(
                black_box(WebhookEvent::DiffCreated),
                black_box(diff_result.clone()),
            );
        });
    });
}

fn bench_cicd_trigger_evaluation(c: &mut Criterion) {
    let old = create_statute_with_preconditions("test", "Old", 20);
    let mut new = old.clone();
    new.effect = Effect::new(EffectType::Revoke, "Revoke");
    let diff_result = diff(&old, &new).unwrap();

    c.bench_function("cicd_trigger_evaluation", |b| {
        b.iter(|| {
            use legalis_diff::integration::{CiCdTrigger, TriggerCondition};
            let trigger = CiCdTrigger::new("test", TriggerCondition::BreakingChange);
            let _ = trigger.should_trigger(black_box(&diff_result));
        });
    });
}

fn bench_pr_comment_generation(c: &mut Criterion) {
    let old = create_statute_with_preconditions("test", "Old", 20);
    let new = create_statute_with_preconditions("test", "New", 20);
    let diff_result = diff(&old, &new).unwrap();

    c.bench_function("pr_comment_generation", |b| {
        b.iter(|| {
            use legalis_diff::integration::{Platform, PullRequestIntegration};
            let pr =
                PullRequestIntegration::new(42, "owner/repo", Platform::GitHub, "main", "feature");
            let _ = pr.generate_comment(black_box(&diff_result));
        });
    });
}

// Benchmarks for new v0.4.0 features

fn bench_cloud_storage_ops(c: &mut Criterion) {
    use legalis_diff::cloud::{CloudStorage, CloudStorageConfig};

    let old = create_statute_with_preconditions("test", "Old", 20);
    let new = create_statute_with_preconditions("test", "New", 20);
    let diff_result = diff(&old, &new).unwrap();

    let mut storage = CloudStorage::new(CloudStorageConfig::default());

    c.bench_function("cloud_storage_store_retrieve", |b| {
        b.iter(|| {
            storage.store("bench-key", black_box(&diff_result)).unwrap();
            let _ = storage.retrieve("bench-key");
        });
    });
}

fn bench_distributed_batch(c: &mut Criterion) {
    use legalis_diff::distributed::{NodeConfig, distributed_diff_batch};

    let nodes = vec![NodeConfig {
        node_id: "bench-node".to_string(),
        address: "localhost:8000".to_string(),
        max_concurrent_tasks: 8,
        capacity: 1000,
    }];

    let pairs: Vec<_> = (0..50)
        .map(|i| {
            let id = format!("statute-{}", i);
            (
                create_statute_with_preconditions(&id, "Old", 10),
                create_statute_with_preconditions(&id, "New", 10),
            )
        })
        .collect();

    c.bench_function("distributed_diff_batch_50", |b| {
        b.iter(|| {
            let _ = distributed_diff_batch(black_box(&pairs), black_box(&nodes));
        });
    });
}

fn bench_gpu_batch_compute(c: &mut Criterion) {
    use legalis_diff::gpu::{GpuConfig, GpuDiffEngine};

    let mut engine = GpuDiffEngine::new(GpuConfig::default());

    let pairs: Vec<_> = (0..50)
        .map(|i| {
            let id = format!("statute-{}", i);
            (
                create_statute_with_preconditions(&id, "Old", 10),
                create_statute_with_preconditions(&id, "New", 10),
            )
        })
        .collect();

    c.bench_function("gpu_batch_compute_50", |b| {
        b.iter(|| {
            let _ = engine.batch_compute(black_box(&pairs));
        });
    });
}

fn bench_llm_analysis(c: &mut Criterion) {
    use legalis_diff::llm::{LlmAnalyzer, LlmConfig};

    let old = create_statute_with_preconditions("test", "Old", 20);
    let new = create_statute_with_preconditions("test", "New", 20);
    let diff_result = diff(&old, &new).unwrap();

    let mut analyzer = LlmAnalyzer::new(LlmConfig::default());

    c.bench_function("llm_full_analysis", |b| {
        b.iter(|| {
            let _ = analyzer.explain_diff(black_box(&diff_result));
            let _ = analyzer.detect_intent(black_box(&diff_result));
            let _ = analyzer.categorize_changes(black_box(&diff_result));
            let _ = analyzer.predict_impact(black_box(&diff_result));
        });
    });
}

fn bench_quantum_fingerprinting(c: &mut Criterion) {
    use legalis_diff::quantum::QuantumFingerprint;

    let statute = create_statute_with_preconditions("test", "Title", 50);

    c.bench_function("quantum_fingerprint_creation", |b| {
        b.iter(|| {
            let _ = QuantumFingerprint::new(black_box(&statute), 64);
        });
    });
}

fn bench_quantum_similarity(c: &mut Criterion) {
    use legalis_diff::quantum::{QuantumSimilarityConfig, quantum_similarity};

    let s1 = create_statute_with_preconditions("test", "Title1", 50);
    let s2 = create_statute_with_preconditions("test", "Title2", 50);
    let config = QuantumSimilarityConfig::default();

    c.bench_function("quantum_similarity_computation", |b| {
        b.iter(|| {
            let _ = quantum_similarity(black_box(&s1), black_box(&s2), black_box(&config));
        });
    });
}

fn bench_quantum_batch_similarity(c: &mut Criterion) {
    use legalis_diff::quantum::{QuantumBatchSimilarity, QuantumSimilarityConfig};

    let statutes: Vec<_> = (0..20)
        .map(|i| create_statute_with_preconditions(&format!("law-{}", i), "Title", 10))
        .collect();

    let mut batch = QuantumBatchSimilarity::new(QuantumSimilarityConfig::default());

    c.bench_function("quantum_similarity_matrix_20", |b| {
        b.iter(|| {
            let _ = batch.similarity_matrix(black_box(&statutes));
        });
    });
}

fn bench_comprehensive_pipeline(c: &mut Criterion) {
    use legalis_diff::integration_examples::ComprehensiveDiffPipeline;

    let mut pipeline = ComprehensiveDiffPipeline::new();

    let pairs: Vec<_> = (0..10)
        .map(|i| {
            let id = format!("statute-{}", i);
            (
                create_statute_with_preconditions(&id, "Old", 10),
                create_statute_with_preconditions(&id, "New", 10),
            )
        })
        .collect();

    c.bench_function("comprehensive_pipeline_10", |b| {
        b.iter(|| {
            let _ = pipeline.process_batch(black_box(&pairs));
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
    bench_validate_rollbacks_parallel,
    bench_audit_trail_creation,
    bench_audit_lifecycle_transitions,
    bench_streaming_diff,
    bench_memory_efficient_diff,
    bench_incremental_updater,
    bench_export_unified_diff,
    bench_export_latex,
    bench_export_changelog,
    bench_export_pdf_content,
    bench_webhook_payload_creation,
    bench_cicd_trigger_evaluation,
    bench_pr_comment_generation,
    bench_cloud_storage_ops,
    bench_distributed_batch,
    bench_gpu_batch_compute,
    bench_llm_analysis,
    bench_quantum_fingerprinting,
    bench_quantum_similarity,
    bench_quantum_batch_similarity,
    bench_comprehensive_pipeline
);
criterion_main!(benches);
