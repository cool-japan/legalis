use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use legalis_interop::{LegalConverter, LegalFormat};

fn catala_import_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("catala_import");

    let small_source = r#"
declaration scope VotingRights:
  context input content Input
  context output content Output

scope VotingRights:
  definition output.eligible equals
    input.age >= 18
"#;

    let medium_source = r#"
declaration scope TaxBenefit:
  context input content Input
  context output content Output

scope TaxBenefit:
  definition output.eligible equals
    input.age >= 65 and input.income <= 50000

declaration scope ChildBenefit:
  context input content Input
  context output content Output

scope ChildBenefit:
  definition output.amount equals
    if input.children >= 1 then 100 else 0
"#;

    group.throughput(Throughput::Bytes(small_source.len() as u64));
    group.bench_with_input(
        BenchmarkId::new("small", small_source.len()),
        &small_source,
        |b, source| {
            let mut converter = LegalConverter::new();
            b.iter(|| {
                converter
                    .import(black_box(source), LegalFormat::Catala)
                    .unwrap()
            })
        },
    );

    group.throughput(Throughput::Bytes(medium_source.len() as u64));
    group.bench_with_input(
        BenchmarkId::new("medium", medium_source.len()),
        &medium_source,
        |b, source| {
            let mut converter = LegalConverter::new();
            b.iter(|| {
                converter
                    .import(black_box(source), LegalFormat::Catala)
                    .unwrap()
            })
        },
    );

    group.finish();
}

fn l4_import_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("l4_import");

    let small_source = "RULE VotingAge WHEN age >= 18 THEN Person MAY vote";
    let medium_source = r#"
RULE VotingAge WHEN age >= 18 THEN Person MAY vote
RULE DrivingAge WHEN age >= 16 THEN Person MAY drive
RULE AlcoholAge WHEN age >= 21 THEN Person MAY purchase_alcohol
"#;

    group.throughput(Throughput::Bytes(small_source.len() as u64));
    group.bench_with_input(
        BenchmarkId::new("small", small_source.len()),
        &small_source,
        |b, source| {
            let mut converter = LegalConverter::new();
            b.iter(|| {
                converter
                    .import(black_box(source), LegalFormat::L4)
                    .unwrap()
            })
        },
    );

    group.throughput(Throughput::Bytes(medium_source.len() as u64));
    group.bench_with_input(
        BenchmarkId::new("medium", medium_source.len()),
        &medium_source,
        |b, source| {
            let mut converter = LegalConverter::new();
            b.iter(|| {
                converter
                    .import(black_box(source), LegalFormat::L4)
                    .unwrap()
            })
        },
    );

    group.finish();
}

fn stipula_import_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("stipula_import");

    let small_source = "agreement SimpleContract(Alice, Bob) { }";
    let medium_source = r#"
agreement RentalContract(Landlord, Tenant) {
    val rent = 1000
    val deposit = 2000
}

agreement ServiceContract(Provider, Client) {
    val fee = 500
}
"#;

    group.throughput(Throughput::Bytes(small_source.len() as u64));
    group.bench_with_input(
        BenchmarkId::new("small", small_source.len()),
        &small_source,
        |b, source| {
            let mut converter = LegalConverter::new();
            b.iter(|| {
                converter
                    .import(black_box(source), LegalFormat::Stipula)
                    .unwrap()
            })
        },
    );

    group.throughput(Throughput::Bytes(medium_source.len() as u64));
    group.bench_with_input(
        BenchmarkId::new("medium", medium_source.len()),
        &medium_source,
        |b, source| {
            let mut converter = LegalConverter::new();
            b.iter(|| {
                converter
                    .import(black_box(source), LegalFormat::Stipula)
                    .unwrap()
            })
        },
    );

    group.finish();
}

fn conversion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("format_conversion");

    let catala_source = r#"
declaration scope VotingRights:
  context input content Input
  context output content Output

scope VotingRights:
  definition output.eligible equals
    input.age >= 18
"#;

    group.bench_function("catala_to_l4", |b| {
        let mut converter = LegalConverter::new();
        b.iter(|| {
            converter
                .convert(
                    black_box(catala_source),
                    LegalFormat::Catala,
                    LegalFormat::L4,
                )
                .unwrap()
        })
    });

    group.bench_function("catala_to_stipula", |b| {
        let mut converter = LegalConverter::new();
        b.iter(|| {
            converter
                .convert(
                    black_box(catala_source),
                    LegalFormat::Catala,
                    LegalFormat::Stipula,
                )
                .unwrap()
        })
    });

    group.bench_function("catala_to_akoma_ntoso", |b| {
        let mut converter = LegalConverter::new();
        b.iter(|| {
            converter
                .convert(
                    black_box(catala_source),
                    LegalFormat::Catala,
                    LegalFormat::AkomaNtoso,
                )
                .unwrap()
        })
    });

    group.finish();
}

fn caching_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("caching");

    let source = r#"
declaration scope Test:
  context input content Input
  context output content Output
"#;

    group.bench_function("without_cache", |b| {
        let mut converter = LegalConverter::new();
        b.iter(|| {
            converter
                .convert(black_box(source), LegalFormat::Catala, LegalFormat::L4)
                .unwrap()
        })
    });

    group.bench_function("with_cache_first", |b| {
        let mut converter = LegalConverter::with_cache(100);
        b.iter(|| {
            converter
                .convert(black_box(source), LegalFormat::Catala, LegalFormat::L4)
                .unwrap()
        })
    });

    group.finish();
}

fn batch_conversion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_conversion");

    let sources: Vec<(String, LegalFormat)> = vec![
        (
            "declaration scope Test1:\n  context input content integer".to_string(),
            LegalFormat::Catala,
        ),
        (
            "declaration scope Test2:\n  context input content integer".to_string(),
            LegalFormat::Catala,
        ),
        (
            "declaration scope Test3:\n  context input content integer".to_string(),
            LegalFormat::Catala,
        ),
        (
            "agreement Test4(A, B) { }".to_string(),
            LegalFormat::Stipula,
        ),
        (
            "agreement Test5(A, B) { }".to_string(),
            LegalFormat::Stipula,
        ),
    ];

    group.bench_function("sequential", |b| {
        let mut converter = LegalConverter::new();
        b.iter(|| {
            converter
                .batch_convert(black_box(&sources), LegalFormat::L4)
                .unwrap()
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    catala_import_benchmark,
    l4_import_benchmark,
    stipula_import_benchmark,
    conversion_benchmark,
    caching_benchmark,
    batch_conversion_benchmark
);
criterion_main!(benches);
