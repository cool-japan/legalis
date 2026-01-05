//! Benchmarks for the porting engine.

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use legalis_core::{Effect, EffectType, Statute};
use legalis_i18n::{CulturalParams, Jurisdiction, LegalSystem, Locale};
use legalis_porting::{
    AgreementType, EquivalenceMapping, PortedStatute, PortingEngine, PortingOptions,
    TermReplacement,
};
use std::hint::black_box;

fn create_jurisdiction_jp() -> Jurisdiction {
    Jurisdiction::new("JP", "Japan", Locale::new("ja").with_country("JP"))
        .with_legal_system(LegalSystem::CivilLaw)
        .with_cultural_params(CulturalParams::japan())
}

fn create_jurisdiction_us() -> Jurisdiction {
    Jurisdiction::new("US", "United States", Locale::new("en").with_country("US"))
        .with_legal_system(LegalSystem::CommonLaw)
        .with_cultural_params(CulturalParams::for_country("US"))
}

fn create_sample_statutes(count: usize) -> Vec<Statute> {
    (0..count)
        .map(|i| {
            Statute::new(
                format!("statute-{}", i),
                format!("Test Statute {}", i),
                Effect::new(EffectType::Grant, format!("Grant right {}", i)),
            )
        })
        .collect()
}

fn bench_port_statute_simple(c: &mut Criterion) {
    let engine = PortingEngine::new(create_jurisdiction_jp(), create_jurisdiction_us());
    let statute = Statute::new(
        "test",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test right"),
    );
    let options = PortingOptions::default();

    c.bench_function("port_statute_simple", |b| {
        b.iter(|| {
            black_box(engine.port_statute(&statute, &options).unwrap());
        });
    });
}

fn bench_port_statute_with_cultural_params(c: &mut Criterion) {
    let engine = PortingEngine::new(create_jurisdiction_jp(), create_jurisdiction_us());
    let statute = Statute::new(
        "adult-rights",
        "Adult Rights Statute",
        Effect::new(EffectType::Grant, "Full legal capacity"),
    );
    let options = PortingOptions {
        apply_cultural_params: true,
        ..Default::default()
    };

    c.bench_function("port_statute_with_cultural_params", |b| {
        b.iter(|| {
            black_box(engine.port_statute(&statute, &options).unwrap());
        });
    });
}

fn bench_generate_compatibility_report(c: &mut Criterion) {
    let engine = PortingEngine::new(create_jurisdiction_jp(), create_jurisdiction_us());
    let mut group = c.benchmark_group("compatibility_report");

    for count in [1, 5, 10, 20, 50].iter() {
        let statutes = create_sample_statutes(*count);
        group.bench_with_input(BenchmarkId::from_parameter(count), count, |b, _| {
            b.iter(|| {
                black_box(engine.generate_report(&statutes));
            });
        });
    }

    group.finish();
}

fn bench_conflict_detection(c: &mut Criterion) {
    let engine = PortingEngine::new(create_jurisdiction_jp(), create_jurisdiction_us());
    let statute = Statute::new(
        "test",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test"),
    );

    c.bench_function("conflict_detection", |b| {
        b.iter(|| {
            black_box(engine.detect_conflicts(&statute));
        });
    });
}

fn bench_semantic_validation(c: &mut Criterion) {
    let engine = PortingEngine::new(create_jurisdiction_jp(), create_jurisdiction_us());
    let statute = Statute::new(
        "test",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test"),
    );
    let options = PortingOptions::default();
    let ported = engine.port_statute(&statute, &options).unwrap();

    c.bench_function("semantic_validation", |b| {
        b.iter(|| {
            black_box(engine.validate_semantics(&statute, &ported));
        });
    });
}

fn bench_risk_assessment(c: &mut Criterion) {
    let engine = PortingEngine::new(create_jurisdiction_jp(), create_jurisdiction_us());
    let statute = Statute::new(
        "test",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test"),
    );
    let options = PortingOptions {
        apply_cultural_params: true,
        ..Default::default()
    };
    let ported = engine.port_statute(&statute, &options).unwrap();

    c.bench_function("risk_assessment", |b| {
        b.iter(|| {
            black_box(engine.assess_risks(&ported));
        });
    });
}

fn bench_batch_port(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut group = c.benchmark_group("batch_port");
    group.sample_size(20);

    for count in [5, 10, 20, 50].iter() {
        let engine = PortingEngine::new(create_jurisdiction_jp(), create_jurisdiction_us());
        let statutes = create_sample_statutes(*count);
        let options = PortingOptions {
            apply_cultural_params: true,
            generate_report: true,
            detect_conflicts: true,
            validate_semantics: true,
            ..Default::default()
        };

        group.bench_with_input(BenchmarkId::from_parameter(count), count, |b, _| {
            b.to_async(&rt).iter(|| async {
                black_box(engine.batch_port(&statutes, &options).await.unwrap())
            });
        });
    }

    group.finish();
}

fn bench_term_replacement(c: &mut Criterion) {
    let replacements = vec![
        TermReplacement {
            source_term: "成人".to_string(),
            target_term: "adult".to_string(),
            context: None,
            confidence: 0.95,
        },
        TermReplacement {
            source_term: "未成年".to_string(),
            target_term: "minor".to_string(),
            context: None,
            confidence: 0.95,
        },
    ];

    let engine = PortingEngine::new(create_jurisdiction_jp(), create_jurisdiction_us())
        .with_term_replacements(replacements);

    let statute = Statute::new(
        "test",
        "成人 Rights for 未成年",
        Effect::new(EffectType::Grant, "Test"),
    );

    c.bench_function("term_replacement", |b| {
        b.iter(|| {
            black_box(engine.apply_term_replacement(&mut statute.clone()));
        });
    });
}

fn bench_contextual_adjustment(c: &mut Criterion) {
    let engine = PortingEngine::new(create_jurisdiction_jp(), create_jurisdiction_us());
    let statute = Statute::new(
        "test",
        "Fine Payment Law",
        Effect::new(EffectType::Obligation, "Pay fine"),
    );

    c.bench_function("contextual_adjustment", |b| {
        b.iter(|| {
            black_box(engine.adjust_parameters_contextually(&statute));
        });
    });
}

fn bench_find_similar_statutes(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let engine = PortingEngine::new(create_jurisdiction_jp(), create_jurisdiction_us());
    let statute = Statute::new(
        "test",
        "Adult Rights Law",
        Effect::new(EffectType::Grant, "Test"),
    );
    let candidates = create_sample_statutes(100);

    c.bench_function("find_similar_statutes_100", |b| {
        b.to_async(&rt).iter(|| async {
            black_box(engine.find_similar_statutes(&statute, &candidates).await)
        });
    });
}

fn bench_workflow_operations(c: &mut Criterion) {
    let engine = PortingEngine::new(create_jurisdiction_jp(), create_jurisdiction_us());

    c.bench_function("create_workflow", |b| {
        b.iter(|| {
            black_box(engine.create_workflow("test-statute".to_string()));
        });
    });

    let workflow = engine.create_workflow("test-statute".to_string());
    c.bench_function("advance_workflow", |b| {
        b.iter(|| {
            let mut wf = workflow.clone();
            black_box(engine.advance_workflow(&mut wf).ok());
        });
    });
}

fn bench_version_operations(c: &mut Criterion) {
    let engine = PortingEngine::new(create_jurisdiction_jp(), create_jurisdiction_us());
    let statute = Statute::new(
        "test",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test"),
    );
    let options = PortingOptions::default();
    let ported = engine.port_statute(&statute, &options).unwrap();

    c.bench_function("create_versioned_statute", |b| {
        b.iter(|| {
            black_box(engine.create_versioned_statute(
                ported.clone(),
                1,
                "user".to_string(),
                "V1".to_string(),
            ));
        });
    });

    let v1 =
        engine.create_versioned_statute(ported.clone(), 1, "user".to_string(), "V1".to_string());
    let v2 =
        engine.create_versioned_statute(ported.clone(), 2, "user".to_string(), "V2".to_string());

    c.bench_function("compare_versions", |b| {
        b.iter(|| {
            black_box(engine.compare_versions(&v1, &v2));
        });
    });
}

fn bench_bilateral_agreement(c: &mut Criterion) {
    let engine = PortingEngine::new(create_jurisdiction_jp(), create_jurisdiction_us());

    c.bench_function("create_bilateral_agreement", |b| {
        b.iter(|| {
            black_box(engine.create_bilateral_agreement(AgreementType::MutualRecognition));
        });
    });
}

fn bench_regulatory_equivalence(c: &mut Criterion) {
    let mappings = (0..100)
        .map(|i| EquivalenceMapping {
            source_regulation: format!("reg-{}", i),
            target_regulation: format!("us-reg-{}", i),
            equivalence_score: 0.9,
            differences: vec!["Minor differences".to_string()],
            notes: "Equivalent".to_string(),
        })
        .collect();

    let engine = PortingEngine::new(create_jurisdiction_jp(), create_jurisdiction_us())
        .with_equivalence_mappings(mappings);

    let statute = Statute::new(
        "reg-50",
        "Test Regulation",
        Effect::new(EffectType::Grant, "Test"),
    );

    c.bench_function("find_regulatory_equivalence", |b| {
        b.iter(|| {
            black_box(engine.find_regulatory_equivalence(&statute));
        });
    });
}

fn bench_compliance_check(c: &mut Criterion) {
    let engine = PortingEngine::new(create_jurisdiction_jp(), create_jurisdiction_us());
    let statute = Statute::new(
        "test",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test"),
    );
    let options = PortingOptions {
        apply_cultural_params: true,
        ..Default::default()
    };
    let ported = engine.port_statute(&statute, &options).unwrap();

    c.bench_function("compliance_check", |b| {
        b.iter(|| {
            black_box(engine.check_compliance(&ported));
        });
    });
}

fn bench_batch_compliance_check(c: &mut Criterion) {
    let engine = PortingEngine::new(create_jurisdiction_jp(), create_jurisdiction_us());
    let mut group = c.benchmark_group("batch_compliance_check");

    for count in [10, 50, 100].iter() {
        let statutes = create_sample_statutes(*count);
        let options = PortingOptions {
            apply_cultural_params: true,
            ..Default::default()
        };
        let ported: Vec<PortedStatute> = statutes
            .iter()
            .map(|s| engine.port_statute(s, &options).unwrap())
            .collect();

        group.bench_with_input(BenchmarkId::from_parameter(count), count, |b, _| {
            b.iter(|| {
                black_box(engine.batch_check_compliance(&ported));
            });
        });
    }

    group.finish();
}

fn bench_compliance_summary(c: &mut Criterion) {
    let engine = PortingEngine::new(create_jurisdiction_jp(), create_jurisdiction_us());
    let statutes = create_sample_statutes(50);
    let options = PortingOptions {
        apply_cultural_params: true,
        ..Default::default()
    };
    let ported: Vec<PortedStatute> = statutes
        .iter()
        .map(|s| engine.port_statute(s, &options).unwrap())
        .collect();
    let results = engine.batch_check_compliance(&ported);

    c.bench_function("compliance_summary", |b| {
        b.iter(|| {
            black_box(engine.generate_compliance_summary(&results));
        });
    });
}

fn bench_expert_review_workflow(c: &mut Criterion) {
    let engine = PortingEngine::new(create_jurisdiction_jp(), create_jurisdiction_us());
    let statute = Statute::new(
        "test",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test"),
    );
    let options = PortingOptions::default();
    let ported = engine.port_statute(&statute, &options).unwrap();

    c.bench_function("submit_for_review", |b| {
        b.iter(|| {
            black_box(engine.submit_for_review(ported.clone()));
        });
    });
}

criterion_group!(
    benches,
    bench_port_statute_simple,
    bench_port_statute_with_cultural_params,
    bench_generate_compatibility_report,
    bench_conflict_detection,
    bench_semantic_validation,
    bench_risk_assessment,
    bench_batch_port,
    bench_term_replacement,
    bench_contextual_adjustment,
    bench_find_similar_statutes,
    bench_workflow_operations,
    bench_version_operations,
    bench_bilateral_agreement,
    bench_regulatory_equivalence,
    bench_compliance_check,
    bench_batch_compliance_check,
    bench_compliance_summary,
    bench_expert_review_workflow,
);

criterion_main!(benches);
