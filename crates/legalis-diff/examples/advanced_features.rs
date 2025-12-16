//! Example demonstrating advanced diff features:
//! - Statistical analysis
//! - Caching and optimization
//! - Advanced diff algorithms
//! - Incremental diffing

use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};
use legalis_diff::{
    algorithms::{diff_conditions_myers, diff_conditions_patience},
    diff,
    optimization::{BatchDiffer, IncrementalDiffer},
    statistics::{
        aggregate_statistics, compute_statistics, summarize_aggregate, summarize_statistics,
    },
};

fn create_statute_version(version: u32) -> Statute {
    let age_threshold = 18 + (version - 1);
    let income_limit = 3_000_000u64 + ((version - 1) * 500_000) as u64;

    Statute::new(
        "welfare-benefit-v1",
        format!("Welfare Benefit Program V{}", version),
        Effect::new(
            EffectType::Grant,
            format!("Monthly benefit of {} yen", 50_000 + version * 5_000),
        ),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: age_threshold,
    })
    .with_precondition(Condition::Income {
        operator: ComparisonOp::LessOrEqual,
        value: income_limit,
    })
}

fn main() {
    println!("=== ADVANCED DIFF FEATURES DEMO ===\n");

    // 1. Statistical Analysis
    println!("1. STATISTICAL ANALYSIS");
    println!("{}", "-".repeat(50));

    let v1 = create_statute_version(1);
    let v2 = create_statute_version(2);
    let v3 = create_statute_version(3);

    let diff1 = diff(&v1, &v2).unwrap();
    let diff2 = diff(&v2, &v3).unwrap();

    let stats1 = compute_statistics(&diff1);
    println!("Statistics for V1 -> V2:");
    println!("{}", summarize_statistics(&stats1));

    let agg_stats = aggregate_statistics(&[diff1.clone(), diff2.clone()]);
    println!("\nAggregate Statistics:");
    println!("{}", summarize_aggregate(&agg_stats));

    // 2. Caching and Batch Processing
    println!("\n2. CACHING AND BATCH PROCESSING");
    println!("{}", "-".repeat(50));

    let mut batch_differ = BatchDiffer::new(100);

    let pairs = vec![
        (create_statute_version(1), create_statute_version(2)),
        (create_statute_version(2), create_statute_version(3)),
        (create_statute_version(1), create_statute_version(2)), // Duplicate - will use cache
    ];

    let batch_results = batch_differ.compute_batch(&pairs).unwrap();
    println!("Computed {} diffs", batch_results.len());

    let cache_stats = batch_differ.cache_stats();
    println!("Cache Statistics:");
    println!("  Size: {}/{}", cache_stats.size, cache_stats.max_size);
    println!("  Hits: {}", cache_stats.hits);
    println!("  Misses: {}", cache_stats.misses);
    println!("  Hit Rate: {:.2}%", cache_stats.hit_rate * 100.0);

    // 3. Incremental Diffing
    println!("\n3. INCREMENTAL DIFFING");
    println!("{}", "-".repeat(50));

    let mut incremental = IncrementalDiffer::new();

    for version in 1..=5 {
        let statute = create_statute_version(version);
        if let Some(diff) = incremental.add_version(statute).unwrap() {
            println!(
                "V{} -> V{}: {} changes",
                version - 1,
                version,
                diff.changes.len()
            );
        }
    }

    println!(
        "\nTotal diffs accumulated: {}",
        incremental.get_diffs().len()
    );

    // 4. Advanced Diff Algorithms
    println!("\n4. ADVANCED DIFF ALGORITHMS");
    println!("{}", "-".repeat(50));

    let old_conditions = vec![
        Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        },
        Condition::Income {
            operator: ComparisonOp::LessOrEqual,
            value: 3_000_000,
        },
    ];

    let new_conditions = vec![
        Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 21,
        },
        Condition::Income {
            operator: ComparisonOp::LessOrEqual,
            value: 4_000_000,
        },
    ];

    let myers_result = diff_conditions_myers(&old_conditions, &new_conditions);
    println!("Myers Algorithm:");
    println!("  Edit Distance: {}", myers_result.edit_distance);
    println!("  Operations: {}", myers_result.ops.len());

    let patience_result = diff_conditions_patience(&old_conditions, &new_conditions);
    println!("\nPatience Algorithm:");
    println!("  Edit Distance: {}", patience_result.edit_distance);
    println!("  Operations: {}", patience_result.ops.len());

    // 5. Performance Comparison
    println!("\n5. PERFORMANCE COMPARISON");
    println!("{}", "-".repeat(50));

    let start = std::time::Instant::now();
    for _ in 0..1000 {
        let _ = diff(&v1, &v2);
    }
    let uncached_duration = start.elapsed();

    let mut batch = BatchDiffer::new(10);
    let start = std::time::Instant::now();
    for _ in 0..1000 {
        let _ = batch.compute_batch(&[(v1.clone(), v2.clone())]);
    }
    let cached_duration = start.elapsed();

    println!("1000 diffs without cache: {:?}", uncached_duration);
    println!("1000 diffs with cache: {:?}", cached_duration);
    println!(
        "Speedup: {:.2}x",
        uncached_duration.as_secs_f64() / cached_duration.as_secs_f64()
    );

    println!("\n=== DEMO COMPLETE ===");
}
