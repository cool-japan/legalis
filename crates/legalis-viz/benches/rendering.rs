//! Rendering performance benchmarks for legalis-viz.

use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};
use legalis_viz::{DecisionTree, DependencyGraph, PopulationChart, Theme, Timeline, TimelineEvent};
use std::time::Instant;

fn benchmark<F: FnOnce()>(name: &str, f: F) {
    let start = Instant::now();
    f();
    let duration = start.elapsed();
    println!("{}: {:?}", name, duration);
}

fn create_complex_statute() -> Statute {
    Statute::new(
        "complex-statute",
        "Complex Test Statute",
        Effect::new(EffectType::Grant, "Complex effect"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    })
    .with_precondition(Condition::Income {
        operator: ComparisonOp::LessThan,
        value: 50000,
    })
    .with_precondition(Condition::HasAttribute {
        key: "citizenship".to_string(),
    })
    .with_discretion("Review additional circumstances")
}

fn bench_decision_tree_creation() {
    benchmark("Decision tree creation", || {
        let statute = create_complex_statute();
        let _tree = DecisionTree::from_statute(&statute).unwrap();
    });
}

fn bench_decision_tree_ascii() {
    let statute = create_complex_statute();
    let tree = DecisionTree::from_statute(&statute).unwrap();

    benchmark("Decision tree ASCII export", || {
        let _ascii = tree.to_ascii();
    });
}

fn bench_decision_tree_mermaid() {
    let statute = create_complex_statute();
    let tree = DecisionTree::from_statute(&statute).unwrap();

    benchmark("Decision tree Mermaid export", || {
        let _mermaid = tree.to_mermaid();
    });
}

fn bench_decision_tree_svg() {
    let statute = create_complex_statute();
    let tree = DecisionTree::from_statute(&statute).unwrap();

    benchmark("Decision tree SVG export", || {
        let _svg = tree.to_svg();
    });
}

fn bench_decision_tree_html() {
    let statute = create_complex_statute();
    let tree = DecisionTree::from_statute(&statute).unwrap();

    benchmark("Decision tree HTML export", || {
        let _html = tree.to_html();
    });
}

#[cfg(feature = "png-export")]
fn bench_decision_tree_png() {
    let statute = create_complex_statute();
    let tree = DecisionTree::from_statute(&statute).unwrap();

    benchmark("Decision tree PNG export", || {
        let _png = tree.to_png().unwrap();
    });
}

fn bench_dependency_graph_large() {
    let mut graph = DependencyGraph::new();

    benchmark("Dependency graph creation (100 nodes)", || {
        for i in 0..100 {
            graph.add_statute(&format!("statute-{}", i));
            if i > 0 {
                graph.add_dependency(
                    &format!("statute-{}", i - 1),
                    &format!("statute-{}", i),
                    "references",
                );
            }
        }
    });

    benchmark("Dependency graph SVG export (100 nodes)", || {
        let _svg = graph.to_svg();
    });

    benchmark("Dependency graph HTML export (100 nodes)", || {
        let _html = graph.to_html();
    });
}

fn bench_population_chart() {
    let mut chart = PopulationChart::new("Performance Test");

    benchmark("Population chart data addition", || {
        for i in 0..100 {
            chart.add_data(&format!("Category-{}", i), i * 10);
        }
        chart.calculate_percentages();
    });

    benchmark("Population chart ASCII export", || {
        let _ascii = chart.to_ascii();
    });

    benchmark("Population chart HTML export", || {
        let _html = chart.to_html();
    });
}

fn bench_timeline() {
    let mut timeline = Timeline::new();

    benchmark("Timeline event addition", || {
        for i in 0..100 {
            timeline.add_event(
                &format!("2024-{:02}-01", (i % 12) + 1),
                TimelineEvent::Enacted {
                    statute_id: format!("statute-{}", i),
                    title: format!("Statute Title {}", i),
                },
            );
        }
    });

    benchmark("Timeline Mermaid export", || {
        let _mermaid = timeline.to_mermaid();
    });

    benchmark("Timeline HTML export", || {
        let _html = timeline.to_html();
    });
}

fn bench_theme_application() {
    let statute = create_complex_statute();
    let tree = DecisionTree::from_statute(&statute).unwrap();

    let themes = [
        Theme::light(),
        Theme::dark(),
        Theme::high_contrast(),
        Theme::colorblind_friendly(),
    ];

    for (i, theme) in themes.iter().enumerate() {
        benchmark(&format!("SVG with theme {}", i), || {
            let _svg = tree.to_svg_with_theme(theme);
        });
    }
}

fn main() {
    println!("=== Legalis-Viz Rendering Performance Benchmarks ===\n");

    println!("Decision Tree Benchmarks:");
    bench_decision_tree_creation();
    bench_decision_tree_ascii();
    bench_decision_tree_mermaid();
    bench_decision_tree_svg();
    bench_decision_tree_html();

    #[cfg(feature = "png-export")]
    {
        bench_decision_tree_png();
    }

    println!("\nDependency Graph Benchmarks:");
    bench_dependency_graph_large();

    println!("\nPopulation Chart Benchmarks:");
    bench_population_chart();

    println!("\nTimeline Benchmarks:");
    bench_timeline();

    println!("\nTheme Application Benchmarks:");
    bench_theme_application();

    println!("\n=== Benchmarks Complete ===");
}
