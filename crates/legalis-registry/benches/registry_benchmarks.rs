use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use legalis_core::{Effect, EffectType, Statute};
use legalis_registry::{Pagination, SearchQuery, StatuteEntry, StatuteRegistry, StatuteStatus};

fn test_statute(id: &str) -> Statute {
    Statute::new(
        id,
        format!("Test Statute {}", id),
        Effect::new(EffectType::Grant, "Test effect"),
    )
}

fn create_registry_with_statutes(count: usize) -> StatuteRegistry {
    let mut registry = StatuteRegistry::new();
    for i in 0..count {
        let statute_id = format!("statute-{:06}", i);
        let entry = StatuteEntry::new(test_statute(&statute_id), "JP")
            .with_tag(if i % 2 == 0 { "civil" } else { "criminal" })
            .with_status(if i % 3 == 0 {
                StatuteStatus::Active
            } else {
                StatuteStatus::Draft
            });
        registry.register(entry).unwrap();
    }
    registry
}

fn bench_register(c: &mut Criterion) {
    c.bench_function("register_statute", |b| {
        b.iter(|| {
            let mut registry = StatuteRegistry::new();
            let entry = StatuteEntry::new(test_statute("test"), "JP");
            registry.register(black_box(entry)).unwrap();
        });
    });
}

fn bench_get_with_cache(c: &mut Criterion) {
    let mut registry = create_registry_with_statutes(1000);

    c.bench_function("get_with_cache", |b| {
        b.iter(|| {
            // Access same statute repeatedly (should hit cache)
            registry.get(black_box("statute-000500"));
        });
    });
}

fn bench_get_without_cache(c: &mut Criterion) {
    let registry = create_registry_with_statutes(1000);

    c.bench_function("get_without_cache", |b| {
        b.iter(|| {
            // Use uncached access
            registry.get_uncached(black_box("statute-000500"));
        });
    });
}

fn bench_query_by_tag(c: &mut Criterion) {
    let registry = create_registry_with_statutes(1000);

    c.bench_function("query_by_tag", |b| {
        b.iter(|| {
            registry.query_by_tag(black_box("civil"));
        });
    });
}

fn bench_fuzzy_search(c: &mut Criterion) {
    let registry = create_registry_with_statutes(1000);

    c.bench_function("fuzzy_search", |b| {
        b.iter(|| {
            registry.fuzzy_search(black_box("statute-000"), 10);
        });
    });
}

fn bench_full_text_search(c: &mut Criterion) {
    let registry = create_registry_with_statutes(1000);

    c.bench_function("full_text_search", |b| {
        b.iter(|| {
            registry.full_text_search(black_box("statute"));
        });
    });
}

fn bench_advanced_search(c: &mut Criterion) {
    let registry = create_registry_with_statutes(1000);

    c.bench_function("advanced_search", |b| {
        b.iter(|| {
            let query = SearchQuery::new()
                .with_tag("civil")
                .with_jurisdiction("JP")
                .with_status(StatuteStatus::Active);
            registry.search(black_box(&query));
        });
    });
}

fn bench_pagination(c: &mut Criterion) {
    let registry = create_registry_with_statutes(1000);

    c.bench_function("list_paged", |b| {
        b.iter(|| {
            registry.list_paged(black_box(Pagination::new(0, 50)));
        });
    });
}

fn bench_batch_register(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_register");

    for size in [10, 50, 100, 500].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut registry = StatuteRegistry::new();
                let entries: Vec<_> = (0..size)
                    .map(|i| StatuteEntry::new(test_statute(&format!("statute-{}", i)), "JP"))
                    .collect();
                registry.batch_register(black_box(entries));
            });
        });
    }

    group.finish();
}

fn bench_version_management(c: &mut Criterion) {
    c.bench_function("update_statute", |b| {
        b.iter(|| {
            let mut registry = StatuteRegistry::new();
            let entry = StatuteEntry::new(test_statute("statute-1"), "JP");
            registry.register(entry).unwrap();

            // Create multiple versions
            for _ in 0..10 {
                registry
                    .update(black_box("statute-1"), test_statute("statute-1"))
                    .unwrap();
            }
        });
    });
}

criterion_group!(
    benches,
    bench_register,
    bench_get_with_cache,
    bench_get_without_cache,
    bench_query_by_tag,
    bench_fuzzy_search,
    bench_full_text_search,
    bench_advanced_search,
    bench_pagination,
    bench_batch_register,
    bench_version_management
);

criterion_main!(benches);
