//! Performance benchmarks for Legalis CLI.
//!
//! These benchmarks measure the performance of key CLI operations.

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};
use legalis_diff::diff;
use legalis_dsl::{DslPrinter, LegalDslParser, PrinterConfig};
use legalis_verifier::{StatuteVerifier, analyze_complexity};
use legalis_viz::DecisionTree;

/// Create a simple test statute for benchmarking.
fn create_benchmark_statute(id: &str) -> Statute {
    Statute::new(
        id,
        "Benchmark Test Act",
        Effect::new(EffectType::Grant, "Test right"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    })
    .with_jurisdiction("US")
}

/// Create a complex statute for benchmarking.
fn create_complex_benchmark_statute(id: &str) -> Statute {
    Statute::new(
        id,
        "Complex Benchmark Act",
        Effect::new(EffectType::Grant, "Complex right"),
    )
    .with_precondition(Condition::And(
        Box::new(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 21,
        }),
        Box::new(Condition::Or(
            Box::new(Condition::Income {
                operator: ComparisonOp::GreaterThan,
                value: 30000,
            }),
            Box::new(Condition::HasAttribute {
                key: "citizen".to_string(),
            }),
        )),
    ))
    .with_discretion("Consider individual circumstances")
    .with_version(2)
}

/// DSL for benchmarking parsing.
fn benchmark_dsl() -> &'static str {
    r#"
STATUTE benchmark-test: "Benchmark Test Act" {
    WHEN AGE >= 21 AND (INCOME > 30000 OR HAS citizen)
    THEN GRANT "Complex right"
    DISCRETION "Consider individual circumstances"
}
"#
}

/// Benchmark DSL parsing.
fn bench_parsing(c: &mut Criterion) {
    let parser = LegalDslParser::new();
    let dsl = benchmark_dsl();

    c.bench_function("parse_simple_dsl", |b| {
        b.iter(|| parser.parse_statute(black_box(dsl)).unwrap())
    });

    // Benchmark parsing multiple statutes
    let multi_dsl = format!("{}\n{}\n{}", dsl, dsl, dsl);
    c.bench_function("parse_multiple_dsl", |b| {
        b.iter(|| parser.parse_statutes(black_box(&multi_dsl)).unwrap())
    });
}

/// Benchmark statute verification.
fn bench_verification(c: &mut Criterion) {
    let verifier = StatuteVerifier::new();

    // Single statute verification
    let statute = create_benchmark_statute("bench-verify");
    c.bench_function("verify_single_statute", |b| {
        b.iter(|| verifier.verify(black_box(&[statute.clone()])))
    });

    // Multiple statutes verification
    let statutes: Vec<_> = (0..10)
        .map(|i| create_benchmark_statute(&format!("bench-{}", i)))
        .collect();

    c.bench_function("verify_10_statutes", |b| {
        b.iter(|| verifier.verify(black_box(&statutes)))
    });

    // Complex statute verification
    let complex = create_complex_benchmark_statute("bench-complex");
    c.bench_function("verify_complex_statute", |b| {
        b.iter(|| verifier.verify(black_box(&[complex.clone()])))
    });
}

/// Benchmark complexity analysis.
fn bench_complexity(c: &mut Criterion) {
    let simple = create_benchmark_statute("bench-simple");
    let complex = create_complex_benchmark_statute("bench-complex");

    c.bench_function("complexity_simple", |b| {
        b.iter(|| analyze_complexity(black_box(&simple)))
    });

    c.bench_function("complexity_complex", |b| {
        b.iter(|| analyze_complexity(black_box(&complex)))
    });

    // Benchmark batch complexity analysis
    let statutes: Vec<_> = (0..10)
        .map(|i| create_complex_benchmark_statute(&format!("bench-{}", i)))
        .collect();

    c.bench_function("complexity_batch_10", |b| {
        b.iter(|| {
            for statute in &statutes {
                analyze_complexity(black_box(statute));
            }
        })
    });
}

/// Benchmark visualization generation.
fn bench_visualization(c: &mut Criterion) {
    let statute = create_complex_benchmark_statute("bench-viz");

    c.bench_function("viz_create_tree", |b| {
        b.iter(|| DecisionTree::from_statute(black_box(&statute)).unwrap())
    });

    let tree = DecisionTree::from_statute(&statute).unwrap();

    c.bench_function("viz_mermaid", |b| b.iter(|| tree.to_mermaid()));

    c.bench_function("viz_dot", |b| b.iter(|| tree.to_dot()));

    c.bench_function("viz_ascii", |b| b.iter(|| tree.to_ascii()));
}

/// Benchmark diff generation.
fn bench_diff(c: &mut Criterion) {
    let old_statute = create_benchmark_statute("bench-diff");
    let new_statute = create_complex_benchmark_statute("bench-diff");

    c.bench_function("diff_generate", |b| {
        b.iter(|| diff(black_box(&old_statute), black_box(&new_statute)).unwrap())
    });
}

/// Benchmark printing/formatting.
fn bench_printing(c: &mut Criterion) {
    let statute = create_complex_benchmark_statute("bench-print");
    let printer = DslPrinter::with_config(PrinterConfig::default());

    c.bench_function("print_default", |b| {
        b.iter(|| printer.format(black_box(&statute)))
    });

    let compact_printer = DslPrinter::with_config(PrinterConfig::compact());
    c.bench_function("print_compact", |b| {
        b.iter(|| compact_printer.format(black_box(&statute)))
    });

    let verbose_printer = DslPrinter::with_config(PrinterConfig::verbose());
    c.bench_function("print_verbose", |b| {
        b.iter(|| verbose_printer.format(black_box(&statute)))
    });

    // Benchmark batch printing
    let statutes: Vec<_> = (0..10)
        .map(|i| create_benchmark_statute(&format!("bench-{}", i)))
        .collect();

    c.bench_function("print_batch_10", |b| {
        b.iter(|| printer.format_batch(black_box(&statutes)))
    });
}

/// Benchmark serialization.
fn bench_serialization(c: &mut Criterion) {
    let statute = create_complex_benchmark_statute("bench-serialize");

    c.bench_function("serialize_json", |b| {
        b.iter(|| serde_json::to_string(black_box(&statute)).unwrap())
    });

    c.bench_function("serialize_yaml", |b| {
        b.iter(|| serde_yaml::to_string(black_box(&statute)).unwrap())
    });

    // Benchmark deserialization
    let json = serde_json::to_string(&statute).unwrap();
    c.bench_function("deserialize_json", |b| {
        b.iter(|| serde_json::from_str::<Statute>(black_box(&json)).unwrap())
    });

    let yaml = serde_yaml::to_string(&statute).unwrap();
    c.bench_function("deserialize_yaml", |b| {
        b.iter(|| serde_yaml::from_str::<Statute>(black_box(&yaml)).unwrap())
    });
}

/// Benchmark scaling with different input sizes.
fn bench_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("verification_scaling");
    let verifier = StatuteVerifier::new();

    for size in [1, 10, 50, 100].iter() {
        let statutes: Vec<_> = (0..*size)
            .map(|i| create_benchmark_statute(&format!("bench-{}", i)))
            .collect();

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &_size| {
            b.iter(|| verifier.verify(black_box(&statutes)))
        });
    }

    group.finish();
}

/// Benchmark parsing scaling.
fn bench_parsing_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("parsing_scaling");
    let parser = LegalDslParser::new();
    let dsl = benchmark_dsl();

    for count in [1, 5, 10, 20].iter() {
        let multi_dsl: String = (0..*count).map(|_| dsl).collect::<Vec<_>>().join("\n");

        group.bench_with_input(BenchmarkId::from_parameter(count), count, |b, &_count| {
            b.iter(|| parser.parse_statutes(black_box(&multi_dsl)).unwrap())
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_parsing,
    bench_verification,
    bench_complexity,
    bench_visualization,
    bench_diff,
    bench_printing,
    bench_serialization,
    bench_scaling,
    bench_parsing_scaling
);
criterion_main!(benches);
