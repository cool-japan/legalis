use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use legalis_core::{Effect, EffectType, Statute};
use legalis_registry::{Pagination, SearchQuery, StatuteEntry, StatuteRegistry, StatuteStatus};
use std::hint::black_box;

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

fn bench_cache_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_performance");

    let mut registry = create_registry_with_statutes(1000);

    // Benchmark cache hits
    group.bench_function("cache_hit_repeated", |b| {
        b.iter(|| {
            // Access same items repeatedly (should all be cached)
            for i in 0..10 {
                registry.get(black_box(&format!("statute-{:06}", i)));
            }
        });
    });

    // Benchmark cache misses
    group.bench_function("cache_miss_sequential", |b| {
        b.iter(|| {
            // Access different items each time (cache misses)
            for i in 900..910 {
                registry.get_uncached(black_box(&format!("statute-{:06}", i)));
            }
        });
    });

    group.finish();
}

fn bench_version_history_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("version_history");

    for versions in [5, 10, 25, 50].iter() {
        let mut registry = StatuteRegistry::new();
        let entry = StatuteEntry::new(test_statute("versioned-statute"), "JP");
        registry.register(entry).unwrap();

        // Create multiple versions
        for v in 1..=*versions {
            let mut updated_statute = test_statute("versioned-statute");
            updated_statute.title = format!("Version {}", v);
            registry
                .update("versioned-statute", updated_statute)
                .unwrap();
        }

        group.bench_with_input(
            BenchmarkId::new("get_multiple_versions", versions),
            versions,
            |b, &versions| {
                b.iter(|| {
                    for v in 1..=versions {
                        registry.get_version(black_box("versioned-statute"), v).ok();
                    }
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("get_specific_version", versions),
            versions,
            |b, _| {
                b.iter(|| {
                    let _ = registry.get_version(black_box("versioned-statute"), *versions / 2);
                });
            },
        );
    }

    group.finish();
}

fn bench_bulk_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("bulk_operations");

    for size in [50, 100, 250, 500].iter() {
        let entries: Vec<_> = (0..*size)
            .map(|i| {
                StatuteEntry::new(test_statute(&format!("bulk-{}", i)), "JP")
                    .with_tag(if i % 2 == 0 { "civil" } else { "criminal" })
            })
            .collect();

        group.bench_with_input(
            BenchmarkId::new("bulk_insert", size),
            &entries,
            |b, entries| {
                b.iter(|| {
                    let mut registry = StatuteRegistry::new();
                    registry.batch_register(black_box(entries.clone()));
                });
            },
        );
    }

    // Bulk update operations
    for size in [10, 50, 100].iter() {
        group.bench_with_input(BenchmarkId::new("bulk_update", size), size, |b, &size| {
            b.iter(|| {
                let mut registry = create_registry_with_statutes(size);
                for i in 0..size {
                    let statute_id = format!("statute-{:06}", i);
                    registry.update(&statute_id, test_statute(&statute_id)).ok();
                }
            });
        });
    }

    group.finish();
}

fn bench_query_combinations(c: &mut Criterion) {
    let registry = create_registry_with_statutes(1000);

    c.bench_function("query_multiple_tags", |b| {
        b.iter(|| {
            let mut results = registry.query_by_tag(black_box("civil"));
            results.extend(registry.query_by_tag(black_box("criminal")));
            black_box(results)
        });
    });

    c.bench_function("query_by_jurisdiction", |b| {
        b.iter(|| {
            let results: Vec<_> = registry
                .list()
                .into_iter()
                .filter(|e| e.jurisdiction == "JP")
                .collect();
            black_box(results)
        });
    });

    c.bench_function("query_by_status", |b| {
        b.iter(|| {
            let results: Vec<_> = registry
                .list()
                .into_iter()
                .filter(|e| e.status == StatuteStatus::Active)
                .collect();
            black_box(results)
        });
    });
}

fn bench_search_variations(c: &mut Criterion) {
    let registry = create_registry_with_statutes(1000);

    let mut group = c.benchmark_group("search_variations");

    // Short queries
    group.bench_function("fuzzy_search_short", |b| {
        b.iter(|| {
            registry.fuzzy_search(black_box("sta"), 10);
        });
    });

    // Medium queries
    group.bench_function("fuzzy_search_medium", |b| {
        b.iter(|| {
            registry.fuzzy_search(black_box("statute-00"), 10);
        });
    });

    // Long queries
    group.bench_function("fuzzy_search_long", |b| {
        b.iter(|| {
            registry.fuzzy_search(black_box("statute-000500"), 10);
        });
    });

    // Different result limits
    for limit in [5, 10, 25, 50].iter() {
        group.bench_with_input(
            BenchmarkId::new("fuzzy_limit", limit),
            limit,
            |b, &limit| {
                b.iter(|| {
                    registry.fuzzy_search(black_box("statute"), limit);
                });
            },
        );
    }

    group.finish();
}

fn bench_pagination_variations(c: &mut Criterion) {
    let registry = create_registry_with_statutes(1000);
    let mut group = c.benchmark_group("pagination");

    for page_size in [10, 25, 50, 100].iter() {
        group.bench_with_input(
            BenchmarkId::new("first_page", page_size),
            page_size,
            |b, &page_size| {
                b.iter(|| {
                    registry.list_paged(black_box(Pagination::new(0, page_size)));
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("middle_page", page_size),
            page_size,
            |b, &page_size| {
                b.iter(|| {
                    registry.list_paged(black_box(Pagination::new(500, page_size)));
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("last_page", page_size),
            page_size,
            |b, &page_size| {
                b.iter(|| {
                    registry.list_paged(black_box(Pagination::new(950, page_size)));
                });
            },
        );
    }

    group.finish();
}

fn bench_registry_size_impact(c: &mut Criterion) {
    let mut group = c.benchmark_group("registry_size_impact");

    for size in [100, 500, 1000, 5000].iter() {
        group.bench_with_input(
            BenchmarkId::new("count_statutes", size),
            size,
            |b, &size| {
                let registry = create_registry_with_statutes(size);
                b.iter(|| black_box(registry.count()));
            },
        );

        group.bench_with_input(BenchmarkId::new("list_all", size), size, |b, &size| {
            let registry = create_registry_with_statutes(size);
            b.iter(|| black_box(registry.list()));
        });
    }

    group.finish();
}

fn bench_concurrent_access(c: &mut Criterion) {
    let mut registry = create_registry_with_statutes(1000);

    c.bench_function("concurrent_reads", |b| {
        b.iter(|| {
            // Simulate multiple concurrent reads
            let results: Vec<_> = (0..10)
                .map(|i| registry.get(black_box(&format!("statute-{:06}", i * 100))))
                .collect();
            black_box(results)
        });
    });
}

fn bench_registry_iteration(c: &mut Criterion) {
    let registry = create_registry_with_statutes(1000);

    c.bench_function("iterate_all_entries", |b| {
        b.iter(|| {
            let count = registry.list().len();
            black_box(count)
        });
    });

    c.bench_function("iterate_filtered", |b| {
        b.iter(|| {
            let count = registry
                .list()
                .into_iter()
                .filter(|e| e.status == StatuteStatus::Active)
                .count();
            black_box(count)
        });
    });
}

fn bench_statute_lookup_patterns(c: &mut Criterion) {
    let mut registry = create_registry_with_statutes(1000);
    let mut group = c.benchmark_group("lookup_patterns");

    // Sequential access
    group.bench_function("sequential_access", |b| {
        b.iter(|| {
            for i in 0..10 {
                registry.get(black_box(&format!("statute-{:06}", i)));
            }
        });
    });

    // Random access
    group.bench_function("random_access", |b| {
        b.iter(|| {
            for i in [42, 789, 123, 456, 890, 234, 567, 901, 345, 678].iter() {
                registry.get(black_box(&format!("statute-{:06}", i)));
            }
        });
    });

    // Repeated access (cache test)
    group.bench_function("repeated_access", |b| {
        b.iter(|| {
            for _ in 0..10 {
                registry.get(black_box("statute-000500"));
            }
        });
    });

    group.finish();
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
    bench_version_management,
    bench_cache_performance,
    bench_version_history_operations,
    bench_bulk_operations,
    bench_query_combinations,
    bench_search_variations,
    bench_pagination_variations,
    bench_registry_size_impact,
    bench_concurrent_access,
    bench_registry_iteration,
    bench_statute_lookup_patterns,
);

criterion_main!(benches);
