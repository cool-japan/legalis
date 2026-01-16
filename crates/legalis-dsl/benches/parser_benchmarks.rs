//! Performance benchmarks for the Legalis DSL parser.

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use legalis_dsl::{CachingParser, IncrementalParser, LegalDslParser, TextEdit};
use std::hint::black_box;

/// Simple statute for benchmarking
fn simple_statute() -> &'static str {
    r#"STATUTE test: "Test Statute" {
        WHEN AGE >= 18
        THEN GRANT "Access"
    }"#
}

/// Medium complexity statute
fn medium_statute() -> &'static str {
    r#"STATUTE voting-rights: "Voting Rights Act" {
        JURISDICTION "US-CA"
        VERSION 2
        EFFECTIVE_DATE 2024-01-01
        DEFAULT status "pending"
        WHEN AGE >= 18 AND HAS citizenship AND (INCOME <= 1000000 OR HAS special_status)
        THEN GRANT "Right to vote in federal elections"
        EXCEPTION WHEN AGE < 21 AND HAS felony_conviction "Restricted voting rights"
        DISCRETION "Consider residency requirements and registration status"
    }"#
}

/// Complex statute with many conditions
fn complex_statute() -> &'static str {
    r#"STATUTE comprehensive-law: "Comprehensive Legal Framework" {
        JURISDICTION "US-FEDERAL"
        VERSION 5
        EFFECTIVE_DATE 2024-01-01
        EXPIRY_DATE 2030-12-31

        DEFAULT status "pending"
        DEFAULT priority 1
        DEFAULT review_required true

        REQUIRES citizenship-law, residence-law, background-check

        WHEN (AGE BETWEEN 21 AND 65) AND
             HAS citizenship AND
             (INCOME >= 25000 AND INCOME <= 150000) AND
             NOT (HAS felony OR HAS misdemeanor) AND
             (HAS education OR HAS work_experience) AND
             CURRENT_DATE >= "2024-01-01" AND
             (status == "approved" OR status == "pending")
        UNLESS HAS disqualifying_factor

        THEN GRANT "Full benefits package"
        THEN OBLIGATION "Must maintain active status"

        EXCEPTION WHEN AGE < 25 AND HAS military_service "Veteran exemption for age requirement"
        EXCEPTION WHEN INCOME < 25000 AND HAS hardship_status "Income exemption for hardship cases"
        EXCEPTION WHEN HAS disability "Disability accommodation pathway"

        DISCRETION "Review board may grant exceptions for exceptional circumstances"

        AMENDMENT old-comprehensive-law VERSION 4 EFFECTIVE_DATE 2024-01-01 "Updated income thresholds and added disability accommodations"

        SUPERSEDES legacy-law-v3, interim-regulation
    }"#
}

/// Multiple statutes document
fn multi_statute_document() -> String {
    let mut doc = String::new();
    for i in 0..10 {
        doc.push_str(&format!(
            r#"STATUTE statute-{}: "Statute Number {}" {{
                JURISDICTION "TEST"
                VERSION 1
                WHEN AGE >= {} AND INCOME <= {}
                THEN GRANT "Benefit level {}"
                DISCRETION "Standard review process"
            }}

            "#,
            i,
            i,
            18 + i,
            50000 * (i + 1),
            i
        ));
    }
    doc
}

/// Large document with many statutes
fn large_document(num_statutes: usize) -> String {
    let mut doc = String::new();
    for i in 0..num_statutes {
        doc.push_str(&format!(
            r#"STATUTE law-{}: "Law {}" {{
                WHEN AGE >= {} AND HAS requirement_{}
                THEN GRANT "Access level {}"
            }}

            "#,
            i,
            i,
            18 + (i % 50),
            i % 10,
            i % 5
        ));
    }
    doc
}

fn bench_parse_simple(c: &mut Criterion) {
    let input = simple_statute();
    let parser = LegalDslParser::new();

    c.bench_function("parse_simple_statute", |b| {
        b.iter(|| parser.parse_statute(black_box(input)))
    });
}

fn bench_parse_medium(c: &mut Criterion) {
    let input = medium_statute();
    let parser = LegalDslParser::new();

    c.bench_function("parse_medium_statute", |b| {
        b.iter(|| parser.parse_statute(black_box(input)))
    });
}

fn bench_parse_complex(c: &mut Criterion) {
    let input = complex_statute();
    let parser = LegalDslParser::new();

    c.bench_function("parse_complex_statute", |b| {
        b.iter(|| parser.parse_statute(black_box(input)))
    });
}

fn bench_parse_multi_statute(c: &mut Criterion) {
    let input = multi_statute_document();
    let parser = LegalDslParser::new();

    c.bench_function("parse_10_statutes", |b| {
        b.iter(|| parser.parse_document(black_box(&input)))
    });
}

fn bench_parse_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_scaling");

    for size in [10, 50, 100, 200].iter() {
        let input = large_document(*size);
        let parser = LegalDslParser::new();

        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| parser.parse_document(black_box(&input)))
        });
    }

    group.finish();
}

fn bench_caching_parser(c: &mut Criterion) {
    let mut group = c.benchmark_group("caching");
    let input = medium_statute();

    // First parse (cache miss)
    group.bench_function("cache_miss", |b| {
        b.iter_batched(
            CachingParser::new,
            |mut parser| parser.parse_document(black_box(input)),
            criterion::BatchSize::SmallInput,
        )
    });

    // Cached parse (cache hit)
    group.bench_function("cache_hit", |b| {
        let mut parser = CachingParser::new();
        parser.parse_document(input).unwrap(); // Prime the cache

        b.iter(|| parser.parse_document(black_box(input)))
    });

    group.finish();
}

fn bench_incremental_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("incremental");

    let text = r#"STATUTE test1: "Test 1" {
        WHEN AGE >= 18
        THEN GRANT "Access"
    }

    STATUTE test2: "Test 2" {
        WHEN AGE >= 21
        THEN GRANT "Advanced Access"
    }"#;

    // Initial parse
    group.bench_function("initial_parse", |b| {
        b.iter_batched(
            IncrementalParser::new,
            |mut parser| parser.parse_initial(black_box(text)),
            criterion::BatchSize::SmallInput,
        )
    });

    // Incremental update
    group.bench_function("incremental_update", |b| {
        let mut parser = IncrementalParser::new();
        parser.parse_initial(text).unwrap();

        let edit = TextEdit::replace(text.find("18").unwrap(), text.find("18").unwrap() + 2, "21");

        b.iter(|| parser.parse_incremental(black_box(std::slice::from_ref(&edit))))
    });

    group.finish();
}

fn bench_tokenization(c: &mut Criterion) {
    let mut group = c.benchmark_group("tokenization");

    let parser = LegalDslParser::new();

    for (name, input) in [
        ("simple", simple_statute()),
        ("medium", medium_statute()),
        ("complex", complex_statute()),
    ] {
        group.bench_function(name, |b| b.iter(|| parser.tokenize(black_box(input))));
    }

    group.finish();
}

fn bench_error_recovery(c: &mut Criterion) {
    let input_with_errors = r#"
        STATUTE good1: "Good" {
            WHEN AGE >= 18
            THEN GRANT "Access"
        }

        STATUTE bad "Missing Colon" {
            WHEN AGE >= 21
            THEN GRANT "Access"
        }

        STATUTE good2: "Also Good" {
            WHEN AGE >= 25
            THEN GRANT "Access"
        }
    "#;

    let parser = LegalDslParser::new();

    c.bench_function("parse_with_recovery", |b| {
        b.iter(|| parser.parse_document_with_recovery(black_box(input_with_errors)))
    });
}

fn bench_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialization");

    let parser = LegalDslParser::new();
    let doc = parser
        .parse_document(multi_statute_document().as_str())
        .unwrap();

    group.bench_function("to_json", |b| {
        b.iter(|| legalis_dsl::to_json(black_box(&doc)))
    });

    group.bench_function("to_yaml", |b| {
        b.iter(|| legalis_dsl::to_yaml(black_box(&doc)))
    });

    let json = legalis_dsl::to_json(&doc).unwrap();
    group.bench_function("from_json", |b| {
        b.iter(|| legalis_dsl::from_json(black_box(&json)))
    });

    let yaml = legalis_dsl::to_yaml(&doc).unwrap();
    group.bench_function("from_yaml", |b| {
        b.iter(|| legalis_dsl::from_yaml(black_box(&yaml)))
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_parse_simple,
    bench_parse_medium,
    bench_parse_complex,
    bench_parse_multi_statute,
    bench_parse_scaling,
    bench_caching_parser,
    bench_incremental_parsing,
    bench_tokenization,
    bench_error_recovery,
    bench_serialization,
);

criterion_main!(benches);
