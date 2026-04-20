#![cfg(test)]
use super::*;
use legalis_core::{ComparisonOp, Effect, EffectType};

#[test]
fn test_decision_tree_from_statute() {
    let statute = Statute::new(
        "adult-rights",
        "Adult Rights Act",
        Effect::new(EffectType::Grant, "Full legal capacity"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    });
    let tree = DecisionTree::from_statute(&statute).unwrap();
    assert!(tree.node_count() > 0);
}
#[test]
fn test_mermaid_export() {
    let statute = Statute::new(
        "test",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let mermaid = tree.to_mermaid();
    assert!(mermaid.contains("flowchart TD"));
}
#[test]
fn test_dependency_graph() {
    let mut graph = DependencyGraph::new();
    graph.add_dependency("statute-a", "statute-b", "references");
    graph.add_dependency("statute-b", "statute-c", "amends");
    let mermaid = graph.to_mermaid();
    assert!(mermaid.contains("statute-a"));
    assert!(mermaid.contains("statute-b"));
}
#[test]
fn test_ascii_export() {
    let statute = Statute::new(
        "adult-rights",
        "Adult Rights Act",
        Effect::new(EffectType::Grant, "Full legal capacity"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    });
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let ascii = tree.to_ascii();
    assert!(ascii.contains("Adult Rights Act"));
    assert!(ascii.contains("Age"));
}
#[test]
fn test_box_export() {
    let statute = Statute::new(
        "test-statute",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let box_output = tree.to_box();
    assert!(box_output.contains("Test Statute"));
    assert!(box_output.contains("┌"));
    assert!(box_output.contains("└"));
}
#[test]
fn test_ascii_with_discretion() {
    let statute = Statute::new(
        "discretionary",
        "Discretionary Statute",
        Effect::new(EffectType::Grant, "Some right"),
    )
    .with_discretion("Consider circumstances");
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let ascii = tree.to_ascii();
    assert!(ascii.contains("Discretionary"));
    assert!(ascii.contains("🔴"));
}
#[test]
fn test_plantuml_export() {
    let statute = Statute::new(
        "test",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let plantuml = tree.to_plantuml();
    assert!(plantuml.contains("@startuml"));
    assert!(plantuml.contains("@enduml"));
    assert!(plantuml.contains("Test Statute"));
}
#[test]
fn test_dependency_graph_plantuml() {
    let mut graph = DependencyGraph::new();
    graph.add_dependency("statute-a", "statute-b", "references");
    graph.add_dependency("statute-b", "statute-c", "amends");
    let plantuml = graph.to_plantuml();
    assert!(plantuml.contains("@startuml"));
    assert!(plantuml.contains("statute-a"));
    assert!(plantuml.contains("references"));
}
#[test]
fn test_html_export() {
    let statute = Statute::new(
        "test",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let html = tree.to_html();
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("d3.v7.min.js"));
    assert!(html.contains("Test Statute"));
}
#[test]
fn test_dependency_graph_html() {
    let mut graph = DependencyGraph::new();
    graph.add_dependency("statute-a", "statute-b", "references");
    let html = graph.to_html();
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("statute-a"));
    assert!(html.contains("d3.forceSimulation"));
}
#[test]
fn test_timeline_ascii() {
    let mut timeline = Timeline::new();
    timeline.add_event(
        "2020-01-01",
        TimelineEvent::Enacted {
            statute_id: "test-law".to_string(),
            title: "Test Law".to_string(),
        },
    );
    timeline.add_event(
        "2021-06-15",
        TimelineEvent::Amended {
            statute_id: "test-law".to_string(),
            description: "Added provision X".to_string(),
        },
    );
    let ascii = timeline.to_ascii();
    assert!(ascii.contains("Timeline of Legal Events"));
    assert!(ascii.contains("2020-01-01"));
    assert!(ascii.contains("Test Law"));
}
#[test]
fn test_timeline_mermaid() {
    let mut timeline = Timeline::new();
    timeline.add_event(
        "2020-01-01",
        TimelineEvent::Enacted {
            statute_id: "test-law".to_string(),
            title: "Test Law".to_string(),
        },
    );
    let mermaid = timeline.to_mermaid();
    assert!(mermaid.contains("gantt"));
    assert!(mermaid.contains("test-law"));
}
#[test]
fn test_timeline_html() {
    let mut timeline = Timeline::new();
    timeline.add_event(
        "2020-01-01",
        TimelineEvent::Enacted {
            statute_id: "test-law".to_string(),
            title: "Test Law".to_string(),
        },
    );
    let html = timeline.to_html();
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("Legal Timeline"));
    assert!(html.contains("2020-01-01"));
}
#[test]
fn test_theme_light() {
    let theme = Theme::light();
    assert_eq!(theme.background_color, "#ffffff");
    assert_eq!(theme.text_color, "#333333");
}
#[test]
fn test_theme_dark() {
    let theme = Theme::dark();
    assert_eq!(theme.background_color, "#1a1a1a");
    assert_eq!(theme.text_color, "#e0e0e0");
}
#[test]
fn test_html_with_custom_theme() {
    let statute = Statute::new(
        "test",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let theme = Theme::dark();
    let html = tree.to_html_with_theme(&theme);
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains(&theme.background_color));
    assert!(html.contains("Test Statute"));
}
#[test]
fn test_annotation_creation() {
    let annotation = Annotation::new("ann1", "node-1", "This is a test annotation")
        .with_citation("Smith v. Jones, 123 U.S. 456 (2020)")
        .with_author("Judge Smith")
        .with_date("2020-01-01")
        .with_type(AnnotationType::CaseLaw);
    assert_eq!(annotation.id, "ann1");
    assert_eq!(annotation.target, "node-1");
    assert_eq!(annotation.text, "This is a test annotation");
    assert_eq!(
        annotation.citation,
        Some("Smith v. Jones, 123 U.S. 456 (2020)".to_string())
    );
    assert!(matches!(
        annotation.annotation_type,
        AnnotationType::CaseLaw
    ));
}
#[test]
fn test_decision_tree_with_annotations() {
    let statute = Statute::new(
        "test",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let mut tree = DecisionTree::from_statute(&statute).unwrap();
    let annotation = Annotation::new("ann1", "test", "Judicial interpretation note")
        .with_type(AnnotationType::Interpretation);
    tree.add_annotation(annotation);
    assert_eq!(tree.annotations().len(), 1);
    let annotations_for_test = tree.annotations_for("test");
    assert_eq!(annotations_for_test.len(), 1);
    assert_eq!(annotations_for_test[0].text, "Judicial interpretation note");
}
#[test]
fn test_ascii_with_annotations() {
    let statute = Statute::new(
        "test",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let mut tree = DecisionTree::from_statute(&statute).unwrap();
    let annotation =
        Annotation::new("ann1", "test", "Important note").with_citation("Example citation");
    tree.add_annotation(annotation);
    let ascii = tree.to_ascii();
    assert!(ascii.contains("Annotations:"));
    assert!(ascii.contains("Important note"));
    assert!(ascii.contains("Example citation"));
}
#[test]
fn test_layout_config_default() {
    let config = LayoutConfig::default();
    assert_eq!(config.width, 960);
    assert_eq!(config.height, 600);
    assert!(!config.enable_clustering);
}
#[test]
fn test_layout_config_large_graph() {
    let config = LayoutConfig::large_graph();
    assert_eq!(config.width, 1920);
    assert_eq!(config.height, 1080);
    assert!(config.enable_clustering);
    assert_eq!(config.max_nodes, Some(100));
}
#[test]
fn test_dependency_graph_with_layout() {
    let layout = LayoutConfig::large_graph();
    let mut graph = DependencyGraph::with_layout(layout);
    for i in 0..10 {
        graph.add_statute(&format!("statute-{}", i));
    }
    assert_eq!(graph.node_count(), 10);
    assert!(!graph.is_large_graph());
}
#[test]
fn test_large_graph_detection() {
    let layout = LayoutConfig {
        width: 800,
        height: 600,
        node_spacing: 100,
        enable_clustering: true,
        max_nodes: Some(5),
    };
    let mut graph = DependencyGraph::with_layout(layout);
    for i in 0..10 {
        graph.add_statute(&format!("statute-{}", i));
    }
    assert!(graph.is_large_graph());
}
#[test]
fn test_population_chart_ascii() {
    let mut chart = PopulationChart::new("Test Distribution");
    chart.add_data("Eligible", 150);
    chart.add_data("Ineligible", 50);
    chart.add_data("Pending", 25);
    chart.calculate_percentages();
    let ascii = chart.to_ascii();
    assert!(ascii.contains("Test Distribution"));
    assert!(ascii.contains("Eligible"));
    assert!(ascii.contains("150"));
}
#[test]
fn test_population_chart_html() {
    let mut chart = PopulationChart::new("Simulation Results");
    chart.add_data("Approved", 100);
    chart.add_data("Denied", 30);
    let html = chart.to_html();
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("chart.js"));
    assert!(html.contains("Simulation Results"));
}
#[test]
fn test_population_chart_time_series() {
    let mut chart = PopulationChart::new("Population Over Time");
    let data_t1 = vec![
        PopulationDataPoint {
            category: "Approved".to_string(),
            count: 50,
            percentage: None,
        },
        PopulationDataPoint {
            category: "Denied".to_string(),
            count: 20,
            percentage: None,
        },
    ];
    let data_t2 = vec![
        PopulationDataPoint {
            category: "Approved".to_string(),
            count: 75,
            percentage: None,
        },
        PopulationDataPoint {
            category: "Denied".to_string(),
            count: 25,
            percentage: None,
        },
    ];
    chart.add_time_point("2020-01-01", data_t1);
    chart.add_time_point("2020-02-01", data_t2);
    let html = chart.time_series_to_html();
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("2020-01-01"));
    assert!(html.contains("Approved"));
}
#[test]
fn test_population_percentages() {
    let mut chart = PopulationChart::new("Test");
    chart.add_data("A", 50);
    chart.add_data("B", 50);
    chart.calculate_percentages();
    assert_eq!(chart.data[0].percentage, Some(50.0));
    assert_eq!(chart.data[1].percentage, Some(50.0));
}
#[test]
fn test_decision_tree_svg() {
    let statute = Statute::new(
        "test",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let svg = tree.to_svg();
    assert!(svg.contains("<svg"));
    assert!(svg.contains("</svg>"));
    assert!(svg.contains("Test Statute"));
}
#[test]
fn test_dependency_graph_svg() {
    let mut graph = DependencyGraph::new();
    graph.add_dependency("statute-a", "statute-b", "references");
    graph.add_dependency("statute-b", "statute-c", "amends");
    let svg = graph.to_svg();
    assert!(svg.contains("<svg"));
    assert!(svg.contains("</svg>"));
    assert!(svg.contains("statute-a"));
}
#[test]
fn test_svg_with_custom_theme() {
    let statute = Statute::new(
        "test",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let theme = Theme::dark();
    let svg = tree.to_svg_with_theme(&theme);
    assert!(svg.contains("<svg"));
    assert!(svg.contains(&theme.background_color));
}
#[test]
#[cfg(feature = "png-export")]
fn test_png_export() {
    let statute = Statute::new(
        "test",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let png_data = tree.to_png();
    assert!(png_data.is_ok());
    assert!(!png_data.unwrap().is_empty());
}
#[test]
#[cfg(feature = "png-export")]
fn test_dependency_graph_png() {
    let mut graph = DependencyGraph::new();
    graph.add_dependency("statute-a", "statute-b", "references");
    let png_data = graph.to_png();
    assert!(png_data.is_ok());
    assert!(!png_data.unwrap().is_empty());
}
#[test]
fn test_drill_down_html() {
    let statute = Statute::new(
        "test",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let html = tree.to_html();
    assert!(html.contains("Interactive"));
    assert!(html.contains("drill down"));
    assert!(html.contains("details"));
    assert!(html.contains("click"));
}
#[test]
fn test_renderer_registry() {
    let registry = RendererRegistry::new();
    assert!(registry.renderers.is_empty());
}
#[test]
fn test_live_visualization() {
    let mut live_viz = LiveVisualization::new("Test Live Viz");
    let event = UpdateEvent::PopulationUpdate {
        category: "Eligible".to_string(),
        count: 100,
        timestamp: "2024-01-01".to_string(),
    };
    live_viz.process_update(event);
    assert_eq!(live_viz.update_history().len(), 1);
}
#[test]
fn test_live_visualization_dependency_update() {
    let mut live_viz = LiveVisualization::new("Test");
    let event = UpdateEvent::DependencyAdded {
        from_statute: "statute-a".to_string(),
        to_statute: "statute-b".to_string(),
        relation: "references".to_string(),
    };
    live_viz.process_update(event);
    assert_eq!(live_viz.dependency_graph.node_count(), 2);
}
#[test]
fn test_live_html_export() {
    let live_viz = LiveVisualization::new("Test");
    let html = live_viz.to_live_html("ws://localhost:8080");
    assert!(html.contains("WebSocket"));
    assert!(html.contains("ws://localhost:8080"));
    assert!(html.contains("Live Visualization Dashboard"));
}
#[test]
fn test_update_event_serialization() {
    let event = UpdateEvent::PopulationUpdate {
        category: "Test".to_string(),
        count: 50,
        timestamp: "2024-01-01".to_string(),
    };
    let json = serde_json::to_string(&event).unwrap();
    assert!(json.contains("PopulationUpdate"));
    assert!(json.contains("Test"));
}
#[test]
fn test_theme_colorblind_friendly() {
    let theme = Theme::colorblind_friendly();
    assert_eq!(theme.condition_color, "#0173b2");
    assert_eq!(theme.discretion_color, "#de8f05");
    assert_eq!(theme.outcome_color, "#029e73");
}
#[test]
fn test_dependency_graph_svg_with_theme() {
    let mut graph = DependencyGraph::new();
    graph.add_dependency("statute-a", "statute-b", "references");
    let theme = Theme::high_contrast();
    let svg = graph.to_svg_with_theme(&theme);
    assert!(svg.contains("<svg"));
    assert!(svg.contains(&theme.background_color));
}
#[test]
fn test_all_output_formats_decision_tree() {
    let statute = Statute::new(
        "comprehensive-test",
        "Comprehensive Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    });
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let dot = tree.to_dot();
    assert!(!dot.is_empty());
    let ascii = tree.to_ascii();
    assert!(ascii.contains("Comprehensive Test Statute"));
    let box_format = tree.to_box();
    assert!(box_format.contains("┌"));
    let mermaid = tree.to_mermaid();
    assert!(mermaid.contains("flowchart TD"));
    let plantuml = tree.to_plantuml();
    assert!(plantuml.contains("@startuml"));
    let svg = tree.to_svg();
    assert!(svg.contains("<svg"));
    let html = tree.to_html();
    assert!(html.contains("<!DOCTYPE html>"));
}
#[test]
fn test_all_output_formats_dependency_graph() {
    let mut graph = DependencyGraph::new();
    graph.add_dependency("statute-1", "statute-2", "references");
    graph.add_dependency("statute-2", "statute-3", "amends");
    let dot = graph.to_dot();
    assert!(!dot.is_empty());
    let mermaid = graph.to_mermaid();
    assert!(mermaid.contains("flowchart LR"));
    let plantuml = graph.to_plantuml();
    assert!(plantuml.contains("@startuml"));
    let svg = graph.to_svg();
    assert!(svg.contains("<svg"));
    let html = graph.to_html();
    assert!(html.contains("<!DOCTYPE html>"));
}
#[test]
fn test_layout_config_compact() {
    let config = LayoutConfig::compact();
    assert_eq!(config.width, 800);
    assert_eq!(config.height, 400);
    assert_eq!(config.node_spacing, 50);
    assert_eq!(config.max_nodes, Some(50));
}
#[test]
fn test_live_visualization_clear_history() {
    let mut live_viz = LiveVisualization::new("Test");
    let event = UpdateEvent::StatisticsUpdate {
        metric: "test_metric".to_string(),
        value: 42.5,
    };
    live_viz.process_update(event);
    assert_eq!(live_viz.update_history().len(), 1);
    live_viz.clear_history();
    assert_eq!(live_viz.update_history().len(), 0);
}
#[test]
fn test_presentation_exporter_creation() {
    let exporter = PresentationExporter::new();
    assert_eq!(exporter.slides.len(), 0);
}
#[test]
fn test_presentation_exporter_with_theme() {
    let theme = Theme::dark();
    let exporter = PresentationExporter::new().with_theme(theme.clone());
    assert_eq!(exporter.theme.background_color, theme.background_color);
}
#[test]
fn test_presentation_add_decision_tree_slide() {
    let statute = Statute::new(
        "test-statute",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    });
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let mut exporter = PresentationExporter::new();
    exporter.add_decision_tree_slide("Test Decision Tree", &tree);
    assert_eq!(exporter.slides.len(), 1);
    assert_eq!(exporter.slides[0].title, "Test Decision Tree");
}
#[test]
fn test_presentation_add_dependency_graph_slide() {
    let mut graph = DependencyGraph::new();
    graph.add_dependency("statute-a", "statute-b", "references");
    let mut exporter = PresentationExporter::new();
    exporter.add_dependency_graph_slide("Test Dependency Graph", &graph);
    assert_eq!(exporter.slides.len(), 1);
    assert_eq!(exporter.slides[0].title, "Test Dependency Graph");
}
#[test]
fn test_presentation_to_pptx() {
    let statute = Statute::new(
        "test-statute",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let mut exporter = PresentationExporter::new();
    exporter.add_decision_tree_slide("Test Slide", &tree);
    let pptx = exporter.to_pptx().unwrap();
    assert!(pptx.contains("<?xml version=\"1.0\""));
    assert!(pptx.contains("<p:presentation"));
    assert!(pptx.contains("<p:sldIdLst>"));
}
#[test]
fn test_presentation_to_keynote() {
    let statute = Statute::new(
        "test-statute",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let mut exporter = PresentationExporter::new();
    exporter.add_decision_tree_slide("Test Slide", &tree);
    let keynote = exporter.to_keynote().unwrap();
    assert!(keynote.contains("<?xml version=\"1.0\""));
    assert!(keynote.contains("<key version="));
    assert!(keynote.contains("<slides>"));
    assert!(keynote.contains("<title>Test Slide</title>"));
}
#[test]
fn test_presentation_to_animated_html() {
    let statute = Statute::new(
        "test-statute",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let mut exporter = PresentationExporter::new();
    exporter.add_decision_tree_slide("Slide 1", &tree);
    exporter.add_decision_tree_slide("Slide 2", &tree);
    let html = exporter.to_animated_html();
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("Animated Presentation"));
    assert!(html.contains("Slide 1"));
    assert!(html.contains("Slide 2"));
    assert!(html.contains("nextSlide"));
    assert!(html.contains("previousSlide"));
    assert!(html.contains("@keyframes fadeIn"));
}
#[test]
fn test_document_embedder_creation() {
    let embedder = DocumentEmbedder::new();
    assert_eq!(
        embedder.theme.background_color,
        Theme::default().background_color
    );
}
#[test]
fn test_document_embedder_with_theme() {
    let theme = Theme::dark();
    let embedder = DocumentEmbedder::new().with_theme(theme.clone());
    assert_eq!(embedder.theme.background_color, theme.background_color);
}
#[test]
fn test_embed_in_markdown() {
    let statute = Statute::new(
        "test-statute",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let embedder = DocumentEmbedder::new();
    let markdown = embedder.embed_in_markdown(&tree);
    assert!(markdown.starts_with("![Decision Tree](data:image/svg+xml;base64,"));
    assert!(markdown.contains("base64"));
}
#[test]
fn test_embed_in_latex() {
    let statute = Statute::new(
        "test-statute",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let embedder = DocumentEmbedder::new();
    let latex = embedder.embed_in_latex(&tree);
    assert!(latex.contains("\\begin{figure}"));
    assert!(latex.contains("\\begin{tikzpicture}"));
    assert!(latex.contains("\\end{tikzpicture}"));
    assert!(latex.contains("\\caption{Decision Tree Visualization}"));
}
#[test]
fn test_embed_in_rst() {
    let statute = Statute::new(
        "test-statute",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let embedder = DocumentEmbedder::new();
    let rst = embedder.embed_in_rst(&tree);
    assert!(rst.starts_with(".. image:: data:image/svg+xml;base64,"));
    assert!(rst.contains(":alt: Decision Tree"));
    assert!(rst.contains(":align: center"));
}
#[test]
fn test_embed_in_asciidoc() {
    let statute = Statute::new(
        "test-statute",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let embedder = DocumentEmbedder::new();
    let asciidoc = embedder.embed_in_asciidoc(&tree);
    assert!(asciidoc.starts_with("image::data:image/svg+xml;base64,"));
    assert!(asciidoc.contains("[Decision Tree,align=center]"));
}
#[test]
fn test_embed_as_iframe() {
    let statute = Statute::new(
        "test-statute",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let embedder = DocumentEmbedder::new();
    let iframe = embedder.embed_as_iframe(&tree, 800, 600);
    assert!(iframe.starts_with("<iframe"));
    assert!(iframe.contains("width=\"800\""));
    assert!(iframe.contains("height=\"600\""));
    assert!(iframe.contains("data:text/html;base64,"));
}
#[test]
fn test_visual_regression_test_passed() {
    let baseline = "Line 1\nLine 2\nLine 3";
    let actual = "Line 1\nLine 2\nLine 3";
    let test = VisualRegressionTest::new("test1", baseline, actual);
    assert!(test.passed);
    assert_eq!(test.differences.len(), 0);
}
#[test]
fn test_visual_regression_test_failed() {
    let baseline = "Line 1\nLine 2\nLine 3";
    let actual = "Line 1\nLine X\nLine 3";
    let test = VisualRegressionTest::new("test1", baseline, actual);
    assert!(!test.passed);
    assert!(!test.differences.is_empty());
}
#[test]
fn test_visual_regression_test_report() {
    let baseline = "Line 1\nLine 2";
    let actual = "Line 1\nLine X";
    let test = VisualRegressionTest::new("test1", baseline, actual);
    let report = test.report();
    assert!(report.contains("Visual Regression Test: test1"));
    assert!(report.contains("Status: FAILED"));
    assert!(report.contains("Differences found:"));
}
#[test]
fn test_visual_regression_suite() {
    let mut suite = VisualRegressionSuite::new();
    let test1 = VisualRegressionTest::new("test1", "data1", "data1");
    let test2 = VisualRegressionTest::new("test2", "data2", "different");
    suite.add_test(test1);
    suite.add_test(test2);
    let summary = suite.run();
    assert!(summary.contains("Total tests: 2"));
    assert!(summary.contains("Passed: 1"));
    assert!(summary.contains("Failed: 1"));
    assert!(!suite.all_passed());
}
#[test]
fn test_visual_regression_suite_all_passed() {
    let mut suite = VisualRegressionSuite::new();
    let test1 = VisualRegressionTest::new("test1", "data1", "data1");
    let test2 = VisualRegressionTest::new("test2", "data2", "data2");
    suite.add_test(test1);
    suite.add_test(test2);
    assert!(suite.all_passed());
}
#[test]
fn test_base64_encode() {
    let data = "Hello, World!";
    let encoded = base64_encode(data);
    assert!(!encoded.is_empty());
    assert_eq!(encoded, "SGVsbG8sIFdvcmxkIQ==");
}
#[test]
fn test_animation_types() {
    let animation = Animation {
        target: "element1".to_string(),
        animation_type: AnimationType::FadeIn,
        duration_ms: 500,
        delay_ms: 0,
    };
    assert_eq!(animation.duration_ms, 500);
    assert_eq!(animation.delay_ms, 0);
}
#[test]
fn test_slide_content_types() {
    let slide = Slide {
        title: "Test Slide".to_string(),
        content: SlideContent::Text("Some text".to_string()),
        animations: Vec::new(),
        notes: Some("Speaker notes".to_string()),
    };
    assert_eq!(slide.title, "Test Slide");
    assert!(slide.notes.is_some());
}
#[test]
fn test_statute_diff_visualizer_creation() {
    let visualizer = StatuteDiffVisualizer::new();
    assert_eq!(visualizer.theme.background_color, "#ffffff");
}
#[test]
fn test_statute_diff_visualizer_with_theme() {
    let visualizer = StatuteDiffVisualizer::new().with_theme(Theme::dark());
    assert_eq!(visualizer.theme.background_color, "#1a1a1a");
}
#[test]
fn test_statute_diff_to_html_empty() {
    use legalis_core::StatuteDiff;
    let diff = StatuteDiff {
        statute_id: "test-statute".to_string(),
        changes: vec![],
    };
    let visualizer = StatuteDiffVisualizer::new();
    let html = visualizer.to_html(&diff);
    assert!(html.contains("test-statute"));
    assert!(html.contains("No changes detected"));
    assert!(html.contains("<style>"));
}
#[test]
fn test_statute_diff_to_html_with_changes() {
    use legalis_core::{StatuteChange, StatuteDiff};
    let diff = StatuteDiff {
        statute_id: "test-statute".to_string(),
        changes: vec![
            StatuteChange::TitleChanged {
                old: "Old Title".to_string(),
                new: "New Title".to_string(),
            },
            StatuteChange::VersionChanged { old: 1, new: 2 },
        ],
    };
    let visualizer = StatuteDiffVisualizer::new();
    let html = visualizer.to_html(&diff);
    assert!(html.contains("test-statute"));
    assert!(html.contains("Title Changed"));
    assert!(html.contains("Version Changed"));
    assert!(html.contains("<table"));
}
#[test]
fn test_statute_diff_to_mermaid() {
    use legalis_core::{StatuteChange, StatuteDiff};
    let diff = StatuteDiff {
        statute_id: "test-statute".to_string(),
        changes: vec![StatuteChange::VersionChanged { old: 1, new: 2 }],
    };
    let visualizer = StatuteDiffVisualizer::new();
    let mermaid = visualizer.to_mermaid(&diff);
    assert!(mermaid.contains("flowchart LR"));
    assert!(mermaid.contains("test-statute"));
    assert!(mermaid.contains("Changes"));
}
#[test]
fn test_statute_diff_to_ascii() {
    use legalis_core::{StatuteChange, StatuteDiff};
    let diff = StatuteDiff {
        statute_id: "test-statute".to_string(),
        changes: vec![StatuteChange::TitleChanged {
            old: "Old".to_string(),
            new: "New".to_string(),
        }],
    };
    let visualizer = StatuteDiffVisualizer::new();
    let ascii = visualizer.to_ascii(&diff);
    assert!(ascii.contains("test-statute"));
    assert!(ascii.contains("1."));
}
#[test]
fn test_reasoning_chain_visualizer_creation() {
    let visualizer = ReasoningChainVisualizer::new();
    assert_eq!(visualizer.theme.background_color, "#ffffff");
}
#[test]
fn test_reasoning_chain_visualizer_with_theme() {
    let visualizer = ReasoningChainVisualizer::new().with_theme(Theme::colorblind_friendly());
    assert_eq!(visualizer.theme.root_color, "#999999");
}
#[test]
fn test_reasoning_chain_to_html() {
    use legalis_core::{LegalExplanation, ReasoningStep};
    let explanation = LegalExplanation {
        outcome: Effect::new(EffectType::Grant, "Tax credit"),
        applicable_statutes: vec!["statute-1".to_string()],
        satisfied_conditions: vec!["Age >= 18".to_string()],
        unsatisfied_conditions: vec![],
        confidence: 0.95,
        reasoning_chain: vec![ReasoningStep {
            step: 1,
            description: "Check age requirement".to_string(),
            statute_id: Some("statute-1".to_string()),
            condition: Some("Age >= 18".to_string()),
            result: legalis_core::StepResult::Satisfied,
        }],
    };
    let visualizer = ReasoningChainVisualizer::new();
    let html = visualizer.to_html(&explanation);
    assert!(html.contains("Tax credit"));
    assert!(html.contains("95"));
    assert!(html.contains("statute-1"));
    assert!(html.contains("Age >= 18"));
    assert!(html.contains("Check age requirement"));
}
#[test]
fn test_reasoning_chain_to_mermaid() {
    use legalis_core::{LegalExplanation, ReasoningStep};
    let explanation = LegalExplanation {
        outcome: Effect::new(EffectType::Grant, "Benefit"),
        applicable_statutes: vec!["statute-1".to_string()],
        satisfied_conditions: vec![],
        unsatisfied_conditions: vec![],
        confidence: 0.8,
        reasoning_chain: vec![ReasoningStep {
            step: 1,
            description: "Verify conditions".to_string(),
            statute_id: Some("statute-1".to_string()),
            condition: None,
            result: legalis_core::StepResult::Applied,
        }],
    };
    let visualizer = ReasoningChainVisualizer::new();
    let mermaid = visualizer.to_mermaid(&explanation);
    assert!(mermaid.contains("flowchart TD"));
    assert!(mermaid.contains("statute-1"));
    assert!(mermaid.contains("80"));
}
#[test]
fn test_reasoning_chain_to_ascii() {
    use legalis_core::{LegalExplanation, ReasoningStep};
    let explanation = LegalExplanation {
        outcome: Effect::new(EffectType::Grant, "Grant"),
        applicable_statutes: vec!["statute-a".to_string()],
        satisfied_conditions: vec!["Condition A".to_string()],
        unsatisfied_conditions: vec!["Condition B".to_string()],
        confidence: 0.75,
        reasoning_chain: vec![ReasoningStep {
            step: 1,
            description: "Step one".to_string(),
            statute_id: None,
            condition: Some("Test condition".to_string()),
            result: legalis_core::StepResult::Satisfied,
        }],
    };
    let visualizer = ReasoningChainVisualizer::new();
    let ascii = visualizer.to_ascii(&explanation);
    assert!(ascii.contains("Grant"));
    assert!(ascii.contains("75"));
    assert!(ascii.contains("statute-a"));
    assert!(ascii.contains("Condition A"));
    assert!(ascii.contains("Condition B"));
    assert!(ascii.contains("Step one"));
}
#[test]
fn test_audit_trail_visualizer_creation() {
    let visualizer = AuditTrailVisualizer::new();
    assert_eq!(visualizer.theme.background_color, "#ffffff");
}
#[test]
fn test_audit_trail_visualizer_with_theme() {
    let visualizer = AuditTrailVisualizer::new().with_theme(Theme::high_contrast());
    assert_eq!(visualizer.theme.background_color, "#ffffff");
}
#[test]
fn test_audit_trail_to_html_empty() {
    use legalis_core::EvaluationAuditTrail;
    let trail = EvaluationAuditTrail::new();
    let visualizer = AuditTrailVisualizer::new();
    let html = visualizer.to_html(&trail);
    assert!(html.contains("Evaluation Audit Trail"));
    assert!(html.contains("No evaluation records"));
}
#[test]
fn test_audit_trail_to_html_with_records() {
    use legalis_core::EvaluationAuditTrail;
    let mut trail = EvaluationAuditTrail::new();
    trail.record("Age >= 18".to_string(), true, 100);
    trail.record("Income < 50000".to_string(), false, 150);
    let visualizer = AuditTrailVisualizer::new();
    let html = visualizer.to_html(&trail);
    assert!(html.contains("Total Evaluations"));
    assert!(html.contains("Age >= 18"));
    assert!(html.contains("Income < 50000"));
    assert!(html.contains("Pass Rate"));
    assert!(html.contains("Average Duration"));
}
#[test]
fn test_audit_trail_to_ascii() {
    use legalis_core::EvaluationAuditTrail;
    let mut trail = EvaluationAuditTrail::new();
    trail.record("Condition A".to_string(), true, 50);
    trail.record("Condition B".to_string(), true, 75);
    trail.record("Condition C".to_string(), false, 60);
    let visualizer = AuditTrailVisualizer::new();
    let ascii = visualizer.to_ascii(&trail);
    assert!(ascii.contains("Evaluation Audit Trail"));
    assert!(ascii.contains("Total Evaluations: 3"));
    assert!(ascii.contains("Condition A"));
    assert!(ascii.contains("Condition B"));
    assert!(ascii.contains("Condition C"));
    assert!(ascii.contains("Pass Rate"));
    assert!(ascii.contains("66.7%"));
}
#[test]
fn test_statute_diff_default() {
    let visualizer1 = StatuteDiffVisualizer::new();
    let visualizer2 = StatuteDiffVisualizer::default();
    assert_eq!(
        visualizer1.theme.background_color,
        visualizer2.theme.background_color
    );
}
#[test]
fn test_reasoning_chain_default() {
    let visualizer1 = ReasoningChainVisualizer::new();
    let visualizer2 = ReasoningChainVisualizer::default();
    assert_eq!(
        visualizer1.theme.background_color,
        visualizer2.theme.background_color
    );
}
#[test]
fn test_audit_trail_default() {
    let visualizer1 = AuditTrailVisualizer::new();
    let visualizer2 = AuditTrailVisualizer::default();
    assert_eq!(
        visualizer1.theme.background_color,
        visualizer2.theme.background_color
    );
}
#[test]
fn test_format_change_type() {
    use legalis_core::StatuteChange;
    assert_eq!(
        format_change_type(&StatuteChange::IdChanged {
            old: "a".to_string(),
            new: "b".to_string()
        }),
        "ID Changed"
    );
    assert_eq!(
        format_change_type(&StatuteChange::TitleChanged {
            old: "a".to_string(),
            new: "b".to_string()
        }),
        "Title Changed"
    );
    assert_eq!(
        format_change_type(&StatuteChange::TemporalValidityChanged),
        "Temporal Validity Changed"
    );
}
#[test]
fn test_interactive_config_default() {
    let config = InteractiveConfig::default();
    assert!(config.enable_zoom_pan);
    assert!(config.enable_tooltips);
    assert!(config.enable_click_expand);
    assert!(config.enable_search);
    assert!(config.enable_minimap);
    assert_eq!(config.initial_zoom, 1.0);
    assert_eq!(config.min_zoom, 0.1);
    assert_eq!(config.max_zoom, 5.0);
    assert_eq!(config.minimap_size, (200, 150));
}
#[test]
fn test_interactive_visualizer_creation() {
    let visualizer = InteractiveVisualizer::new();
    assert_eq!(visualizer.theme.background_color, "#ffffff");
    assert!(visualizer.config.enable_zoom_pan);
}
#[test]
fn test_interactive_visualizer_with_theme() {
    let visualizer = InteractiveVisualizer::new().with_theme(Theme::dark());
    assert_eq!(visualizer.theme.background_color, "#1a1a1a");
}
#[test]
fn test_interactive_visualizer_with_config() {
    let config = InteractiveConfig {
        enable_minimap: false,
        initial_zoom: 2.0,
        ..Default::default()
    };
    let visualizer = InteractiveVisualizer::new().with_config(config);
    assert!(!visualizer.config.enable_minimap);
    assert_eq!(visualizer.config.initial_zoom, 2.0);
}
#[test]
fn test_interactive_html_generation() {
    let statute = Statute::new(
        "test-1",
        "Test Statute",
        Effect::new(EffectType::Grant, "benefit"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    });
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let visualizer = InteractiveVisualizer::new();
    let html = visualizer.to_interactive_html(&tree);
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("Interactive decision-tree Visualization"));
    assert!(html.contains("zoom-in"));
    assert!(html.contains("zoom-out"));
    assert!(html.contains("search-box"));
    assert!(html.contains("minimap"));
    assert!(html.contains("enableZoomPan"));
    assert!(html.contains("enableTooltips"));
    assert!(html.contains("enableClickExpand"));
    assert!(html.contains("enableSearch"));
    assert!(html.contains("enableMinimap"));
}
#[test]
fn test_interactive_html_graph() {
    let mut graph = DependencyGraph::new();
    graph.add_statute("statute-1");
    graph.add_statute("statute-2");
    graph.add_dependency("statute-2", "statute-1", "depends-on");
    let visualizer = InteractiveVisualizer::new();
    let html = visualizer.to_interactive_html_graph(&graph);
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("Interactive dependency-graph Visualization"));
    assert!(html.contains("zoom-controls"));
    assert!(html.contains("search-controls"));
}
#[test]
fn test_interactive_config_disabled_features() {
    let config = InteractiveConfig {
        enable_zoom_pan: false,
        enable_tooltips: false,
        enable_click_expand: false,
        enable_search: false,
        enable_minimap: false,
        initial_zoom: 1.0,
        min_zoom: 0.5,
        max_zoom: 3.0,
        minimap_size: (100, 100),
    };
    let statute = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "test"));
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let visualizer = InteractiveVisualizer::new().with_config(config);
    let html = visualizer.to_interactive_html(&tree);
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("enableZoomPan: false"));
    assert!(html.contains("enableTooltips: false"));
    assert!(html.contains("enableClickExpand: false"));
    assert!(html.contains("enableSearch: false"));
    assert!(html.contains("enableMinimap: false"));
}
#[test]
fn test_interactive_visualizer_default() {
    let visualizer1 = InteractiveVisualizer::new();
    let visualizer2 = InteractiveVisualizer::default();
    assert_eq!(
        visualizer1.theme.background_color,
        visualizer2.theme.background_color
    );
    assert_eq!(
        visualizer1.config.initial_zoom,
        visualizer2.config.initial_zoom
    );
}
#[test]
fn test_3d_config_default() {
    let config = ThreeDConfig::default();
    assert!(!config.enable_vr);
    assert!(!config.enable_ar);
    assert!(config.force_directed);
    assert!(config.depth_coloring);
    assert_eq!(config.camera_fov, 75.0);
    assert_eq!(config.node_size, 1.0);
    assert_eq!(config.edge_thickness, 0.1);
    assert_eq!(config.force_strength, 0.5);
    assert_eq!(config.auto_rotate_speed, 10.0);
}
#[test]
fn test_3d_visualizer_creation() {
    let visualizer = ThreeDVisualizer::new();
    assert_eq!(visualizer.theme.background_color, "#ffffff");
    assert!(visualizer.config.force_directed);
}
#[test]
fn test_3d_visualizer_with_theme() {
    let visualizer = ThreeDVisualizer::new().with_theme(Theme::dark());
    assert_eq!(visualizer.theme.background_color, "#1a1a1a");
}
#[test]
fn test_3d_visualizer_with_config() {
    let config = ThreeDConfig {
        enable_vr: true,
        enable_ar: true,
        force_directed: false,
        depth_coloring: false,
        ..Default::default()
    };
    let visualizer = ThreeDVisualizer::new().with_config(config);
    assert!(visualizer.config.enable_vr);
    assert!(visualizer.config.enable_ar);
    assert!(!visualizer.config.force_directed);
    assert!(!visualizer.config.depth_coloring);
}
#[test]
fn test_3d_html_graph_generation() {
    let mut graph = DependencyGraph::new();
    graph.add_statute("statute-1");
    graph.add_statute("statute-2");
    graph.add_statute("statute-3");
    graph.add_dependency("statute-2", "statute-1", "depends-on");
    graph.add_dependency("statute-3", "statute-2", "depends-on");
    let visualizer = ThreeDVisualizer::new();
    let html = visualizer.to_3d_html_graph(&graph);
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("3D Dependency Graph Visualization"));
    assert!(html.contains("three.min.js"));
    assert!(html.contains("reset-camera"));
    assert!(html.contains("toggle-rotation"));
    assert!(html.contains("const nodes = ["));
    assert!(html.contains("const edges = ["));
    assert!(html.contains("enableVR"));
    assert!(html.contains("enableAR"));
    assert!(html.contains("forceDirected"));
    assert!(html.contains("depthColoring"));
}
#[test]
fn test_3d_html_timeline_generation() {
    let mut timeline = Timeline::new();
    timeline.add_event(
        "2020-01-01",
        TimelineEvent::Enacted {
            statute_id: "law-1".to_string(),
            title: "Event 1".to_string(),
        },
    );
    timeline.add_event(
        "2020-02-01",
        TimelineEvent::Amended {
            statute_id: "law-1".to_string(),
            description: "Event 2".to_string(),
        },
    );
    timeline.add_event(
        "2020-03-01",
        TimelineEvent::Repealed {
            statute_id: "law-1".to_string(),
        },
    );
    let visualizer = ThreeDVisualizer::new();
    let html = visualizer.to_3d_html_timeline(&timeline);
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("3D Timeline Visualization"));
    assert!(html.contains("three.min.js"));
    assert!(html.contains("isTimeline: true"));
    assert!(html.contains("2020-01-01"));
    assert!(html.contains("2020-02-01"));
    assert!(html.contains("2020-03-01"));
}
#[test]
fn test_3d_vr_ar_buttons() {
    let config = ThreeDConfig {
        enable_vr: true,
        enable_ar: true,
        force_directed: true,
        depth_coloring: true,
        camera_fov: 75.0,
        node_size: 1.0,
        edge_thickness: 0.1,
        force_strength: 0.5,
        auto_rotate_speed: 10.0,
    };
    let mut graph = DependencyGraph::new();
    graph.add_statute("test");
    let visualizer = ThreeDVisualizer::new().with_config(config);
    let html = visualizer.to_3d_html_graph(&graph);
    assert!(html.contains("enter-vr"));
    assert!(html.contains("enter-ar"));
    assert!(html.contains("VRButton.js"));
}
#[test]
fn test_3d_force_directed_layout() {
    let config = ThreeDConfig {
        enable_vr: false,
        enable_ar: false,
        force_directed: true,
        depth_coloring: false,
        camera_fov: 75.0,
        node_size: 2.0,
        edge_thickness: 0.2,
        force_strength: 0.8,
        auto_rotate_speed: 5.0,
    };
    let mut graph = DependencyGraph::new();
    graph.add_statute("node1");
    graph.add_statute("node2");
    let visualizer = ThreeDVisualizer::new().with_config(config);
    let html = visualizer.to_3d_html_graph(&graph);
    assert!(html.contains("forceDirected: true"));
    assert!(html.contains("reset-forces"));
    assert!(html.contains("nodeSize: 2"));
    assert!(html.contains("edgeThickness: 0.2"));
    assert!(html.contains("forceStrength: 0.8"));
    assert!(html.contains("autoRotateSpeed: 5"));
}
#[test]
fn test_3d_depth_based_coloring() {
    let config = ThreeDConfig {
        enable_vr: false,
        enable_ar: false,
        force_directed: false,
        depth_coloring: true,
        camera_fov: 75.0,
        node_size: 1.0,
        edge_thickness: 0.1,
        force_strength: 0.5,
        auto_rotate_speed: 0.0,
    };
    let mut graph = DependencyGraph::new();
    graph.add_statute("root");
    graph.add_statute("child");
    graph.add_dependency("child", "root", "depends-on");
    let visualizer = ThreeDVisualizer::new().with_config(config);
    let html = visualizer.to_3d_html_graph(&graph);
    assert!(html.contains("depthColoring: true"));
    assert!(html.contains("const hue = (node.depth * 60) % 360"));
}
#[test]
fn test_3d_visualizer_default() {
    let visualizer1 = ThreeDVisualizer::new();
    let visualizer2 = ThreeDVisualizer::default();
    assert_eq!(
        visualizer1.theme.background_color,
        visualizer2.theme.background_color
    );
    assert_eq!(visualizer1.config.camera_fov, visualizer2.config.camera_fov);
}
#[test]
fn test_accessibility_config_default() {
    let config = AccessibilityConfig::default();
    assert!(config.wcag_aa_compliant);
    assert!(config.enable_screen_reader);
    assert!(config.enable_keyboard_nav);
    assert!(!config.high_contrast_mode);
    assert!(!config.reduced_motion);
    assert_eq!(config.min_font_size, 16.0);
    assert_eq!(config.focus_color, "#005fcc");
    assert_eq!(config.tab_index_start, 0);
}
#[test]
fn test_accessibility_config_screen_reader_optimized() {
    let config = AccessibilityConfig::screen_reader_optimized();
    assert!(config.wcag_aa_compliant);
    assert!(config.enable_screen_reader);
    assert!(config.enable_keyboard_nav);
    assert!(config.high_contrast_mode);
    assert!(config.reduced_motion);
    assert_eq!(config.min_font_size, 18.0);
}
#[test]
fn test_accessibility_config_reduced_motion() {
    let config = AccessibilityConfig::reduced_motion();
    assert!(config.reduced_motion);
    assert!(config.wcag_aa_compliant);
}
#[test]
fn test_accessibility_config_high_contrast() {
    let config = AccessibilityConfig::high_contrast();
    assert!(config.high_contrast_mode);
    assert_eq!(config.min_font_size, 18.0);
}
#[test]
fn test_accessibility_enhancer_creation() {
    let enhancer = AccessibilityEnhancer::new();
    assert!(enhancer.config.wcag_aa_compliant);
    assert_eq!(enhancer.theme.background_color, "#ffffff");
}
#[test]
fn test_accessibility_enhancer_with_config() {
    let config = AccessibilityConfig::high_contrast();
    let enhancer = AccessibilityEnhancer::new().with_config(config);
    assert!(enhancer.config.high_contrast_mode);
}
#[test]
fn test_accessibility_enhancer_with_theme() {
    let enhancer = AccessibilityEnhancer::new().with_theme(Theme::dark());
    assert_eq!(enhancer.theme.background_color, "#1a1a1a");
}
#[test]
fn test_accessibility_enhancer_with_high_contrast_theme() {
    let config = AccessibilityConfig::high_contrast();
    let enhancer = AccessibilityEnhancer::new()
        .with_config(config)
        .with_theme(Theme::light());
    assert_eq!(enhancer.theme.background_color, "#ffffff");
    assert_eq!(enhancer.theme.text_color, "#000000");
}
#[test]
fn test_aria_label_for_root_node() {
    let enhancer = AccessibilityEnhancer::new();
    let node = DecisionNode::Root {
        statute_id: "test-1".to_string(),
        title: "Test Statute".to_string(),
    };
    let label = enhancer.aria_label_for_node(&node);
    assert!(label.contains("Root node"));
    assert!(label.contains("Test Statute"));
    assert!(label.contains("test-1"));
}
#[test]
fn test_aria_label_for_condition_node() {
    let enhancer = AccessibilityEnhancer::new();
    let node = DecisionNode::Condition {
        description: "Age >= 18".to_string(),
        is_discretionary: false,
    };
    let label = enhancer.aria_label_for_node(&node);
    assert!(label.contains("Condition"));
    assert!(label.contains("Age >= 18"));
}
#[test]
fn test_aria_label_for_discretionary_condition() {
    let enhancer = AccessibilityEnhancer::new();
    let node = DecisionNode::Condition {
        description: "Good moral character".to_string(),
        is_discretionary: true,
    };
    let label = enhancer.aria_label_for_node(&node);
    assert!(label.contains("Discretionary condition"));
    assert!(label.contains("Good moral character"));
}
#[test]
fn test_aria_label_for_outcome_node() {
    let enhancer = AccessibilityEnhancer::new();
    let node = DecisionNode::Outcome {
        description: "Eligible for benefits".to_string(),
    };
    let label = enhancer.aria_label_for_node(&node);
    assert!(label.contains("Outcome"));
    assert!(label.contains("Eligible for benefits"));
}
#[test]
fn test_aria_label_for_discretion_node() {
    let enhancer = AccessibilityEnhancer::new();
    let node = DecisionNode::Discretion {
        issue: "Exceptional circumstances".to_string(),
        hint: Some("Consider case history".to_string()),
    };
    let label = enhancer.aria_label_for_node(&node);
    assert!(label.contains("Discretionary decision"));
    assert!(label.contains("Exceptional circumstances"));
    assert!(label.contains("Hint"));
    assert!(label.contains("Consider case history"));
}
#[test]
fn test_aria_role_for_nodes() {
    let enhancer = AccessibilityEnhancer::new();
    let root = DecisionNode::Root {
        statute_id: "test".to_string(),
        title: "Test".to_string(),
    };
    assert_eq!(enhancer.aria_role_for_node(&root), "landmark");
    let condition = DecisionNode::Condition {
        description: "Test".to_string(),
        is_discretionary: false,
    };
    assert_eq!(enhancer.aria_role_for_node(&condition), "listitem");
    let outcome = DecisionNode::Outcome {
        description: "Test".to_string(),
    };
    assert_eq!(enhancer.aria_role_for_node(&outcome), "status");
    let discretion = DecisionNode::Discretion {
        issue: "Test".to_string(),
        hint: None,
    };
    assert_eq!(enhancer.aria_role_for_node(&discretion), "alert");
}
#[test]
fn test_keyboard_nav_script_enabled() {
    let enhancer = AccessibilityEnhancer::new();
    let script = enhancer.keyboard_nav_script();
    assert!(script.contains("Keyboard navigation support"));
    assert!(script.contains("Tab"));
    assert!(script.contains("ArrowUp"));
    assert!(script.contains("ArrowDown"));
    assert!(script.contains("Home"));
    assert!(script.contains("End"));
    assert!(script.contains(&enhancer.config.focus_color));
}
#[test]
fn test_keyboard_nav_script_disabled() {
    let config = AccessibilityConfig {
        enable_keyboard_nav: false,
        ..Default::default()
    };
    let enhancer = AccessibilityEnhancer::new().with_config(config);
    let script = enhancer.keyboard_nav_script();
    assert!(script.is_empty());
}
#[test]
fn test_screen_reader_enhancements_enabled() {
    let enhancer = AccessibilityEnhancer::new();
    let enhancements = enhancer.screen_reader_enhancements();
    assert!(enhancements.contains("Navigation Instructions"));
    assert!(enhancements.contains("Tab"));
    assert!(enhancements.contains("sr-only"));
    assert!(enhancements.contains("complementary"));
}
#[test]
fn test_screen_reader_enhancements_disabled() {
    let config = AccessibilityConfig {
        enable_screen_reader: false,
        ..Default::default()
    };
    let enhancer = AccessibilityEnhancer::new().with_config(config);
    let enhancements = enhancer.screen_reader_enhancements();
    assert!(enhancements.is_empty());
}
#[test]
fn test_reduced_motion_css_enabled() {
    let config = AccessibilityConfig::reduced_motion();
    let enhancer = AccessibilityEnhancer::new().with_config(config);
    let css = enhancer.reduced_motion_css();
    assert!(css.contains("prefers-reduced-motion"));
    assert!(css.contains("animation-duration"));
    assert!(css.contains("0.01ms"));
}
#[test]
fn test_reduced_motion_css_disabled() {
    let enhancer = AccessibilityEnhancer::new();
    let css = enhancer.reduced_motion_css();
    assert!(css.is_empty());
}
#[test]
fn test_high_contrast_css_enabled() {
    let config = AccessibilityConfig::high_contrast();
    let enhancer = AccessibilityEnhancer::new().with_config(config);
    let css = enhancer.high_contrast_css();
    assert!(css.contains("High contrast mode"));
    assert!(css.contains("font-size"));
    assert!(css.contains("18px"));
    assert!(css.contains(".node"));
    assert!(css.contains(".edge"));
}
#[test]
fn test_high_contrast_css_disabled() {
    let enhancer = AccessibilityEnhancer::new();
    let css = enhancer.high_contrast_css();
    assert!(css.is_empty());
}
#[test]
fn test_enhance_html_adds_lang() {
    let enhancer = AccessibilityEnhancer::new();
    let html = "<html><head></head><body></body></html>";
    let enhanced = enhancer.enhance_html(html);
    assert!(enhanced.contains(r#"lang="en""#));
}
#[test]
fn test_enhance_html_adds_viewport() {
    let enhancer = AccessibilityEnhancer::new();
    let html = "<html><head></head><body></body></html>";
    let enhanced = enhancer.enhance_html(html);
    assert!(enhanced.contains("viewport"));
    assert!(enhanced.contains("width=device-width"));
}
#[test]
fn test_enhance_html_preserves_existing_lang() {
    let enhancer = AccessibilityEnhancer::new();
    let html = r#"<html lang="fr"><head></head><body></body></html>"#;
    let enhanced = enhancer.enhance_html(html);
    assert!(enhanced.contains(r#"lang="fr""#));
}
#[test]
fn test_validate_contrast_good() {
    let enhancer = AccessibilityEnhancer::new();
    assert!(enhancer.validate_contrast("#000000", "#ffffff"));
    assert!(enhancer.validate_contrast("#0000aa", "#ffffff"));
}
#[test]
fn test_validate_contrast_bad() {
    let enhancer = AccessibilityEnhancer::new();
    assert!(!enhancer.validate_contrast("#cccccc", "#ffffff"));
    assert!(!enhancer.validate_contrast("#ffff00", "#ffffff"));
}
#[test]
fn test_validate_contrast_invalid_color() {
    let enhancer = AccessibilityEnhancer::new();
    assert!(!enhancer.validate_contrast("invalid", "#ffffff"));
    assert!(!enhancer.validate_contrast("#fff", "#ffffff"));
}
#[test]
fn test_accessible_html_decision_tree() {
    let statute = Statute::new(
        "test-1",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let enhancer = AccessibilityEnhancer::new();
    let html = enhancer.to_accessible_html(&tree);
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains(r#"lang="en""#));
    assert!(html.contains("viewport"));
    assert!(html.contains("Navigation Instructions"));
}
#[test]
fn test_accessible_html_dependency_graph() {
    let mut graph = DependencyGraph::new();
    graph.add_statute("statute-1");
    graph.add_statute("statute-2");
    graph.add_dependency("statute-2", "statute-1", "depends-on");
    let enhancer = AccessibilityEnhancer::new();
    let html = enhancer.to_accessible_html_graph(&graph);
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains(r#"lang="en""#));
    assert!(html.contains("viewport"));
}
#[test]
fn test_accessible_html_with_all_features() {
    let config = AccessibilityConfig::screen_reader_optimized();
    let enhancer = AccessibilityEnhancer::new().with_config(config);
    let statute = Statute::new(
        "test-1",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let html = enhancer.to_accessible_html(&tree);
    assert!(html.contains(r#"lang="en""#));
    assert!(html.contains("viewport"));
    assert!(html.contains("Navigation Instructions"));
    assert!(html.contains("Keyboard navigation support"));
    assert!(html.contains("High contrast mode"));
    assert!(html.contains("prefers-reduced-motion"));
}
#[test]
fn test_accessibility_enhancer_default() {
    let enhancer1 = AccessibilityEnhancer::new();
    let enhancer2 = AccessibilityEnhancer::default();
    assert_eq!(
        enhancer1.config.wcag_aa_compliant,
        enhancer2.config.wcag_aa_compliant
    );
    assert_eq!(
        enhancer1.theme.background_color,
        enhancer2.theme.background_color
    );
}
#[test]
fn test_export_format_types() {
    let formats = [
        ExportFormat::AnimatedGif,
        ExportFormat::Mp4,
        ExportFormat::WebM,
        ExportFormat::PrintPdf,
        ExportFormat::VectorPdf,
        ExportFormat::Poster,
    ];
    assert_eq!(formats.len(), 6);
}
#[test]
fn test_poster_config_default() {
    let config = PosterConfig::default();
    assert_eq!(config.width, 841);
    assert_eq!(config.height, 1189);
    assert_eq!(config.dpi, 300);
    assert_eq!(config.paper_size, "A0");
    assert_eq!(config.orientation, "portrait");
}
#[test]
fn test_poster_config_a0() {
    let config = PosterConfig::a0();
    assert_eq!(config.width, 841);
    assert_eq!(config.height, 1189);
    assert_eq!(config.paper_size, "A0");
}
#[test]
fn test_poster_config_a1() {
    let config = PosterConfig::a1();
    assert_eq!(config.width, 594);
    assert_eq!(config.height, 841);
    assert_eq!(config.paper_size, "A1");
}
#[test]
fn test_poster_config_a2() {
    let config = PosterConfig::a2();
    assert_eq!(config.width, 420);
    assert_eq!(config.height, 594);
    assert_eq!(config.paper_size, "A2");
}
#[test]
fn test_poster_config_24x36() {
    let config = PosterConfig::poster_24x36();
    assert_eq!(config.width, 610);
    assert_eq!(config.height, 914);
    assert_eq!(config.paper_size, "24x36");
}
#[test]
fn test_poster_config_landscape() {
    let config = PosterConfig::a0().landscape();
    assert_eq!(config.width, 1189);
    assert_eq!(config.height, 841);
    assert_eq!(config.orientation, "landscape");
}
#[test]
fn test_poster_config_with_dpi() {
    let config = PosterConfig::a0().with_dpi(600);
    assert_eq!(config.dpi, 600);
}
#[test]
fn test_animated_gif_config_default() {
    let config = AnimatedGifConfig::default();
    assert_eq!(config.fps, 30);
    assert_eq!(config.duration, 10);
    assert_eq!(config.loop_count, 0);
    assert_eq!(config.width, 1920);
    assert_eq!(config.height, 1080);
    assert_eq!(config.quality, 80);
}
#[test]
fn test_animated_gif_config_with_fps() {
    let config = AnimatedGifConfig::new().with_fps(60);
    assert_eq!(config.fps, 60);
}
#[test]
fn test_animated_gif_config_with_duration() {
    let config = AnimatedGifConfig::new().with_duration(5);
    assert_eq!(config.duration, 5);
}
#[test]
fn test_animated_gif_config_with_loop_count() {
    let config = AnimatedGifConfig::new().with_loop_count(5);
    assert_eq!(config.loop_count, 5);
}
#[test]
fn test_animated_gif_config_with_size() {
    let config = AnimatedGifConfig::new().with_size(1280, 720);
    assert_eq!(config.width, 1280);
    assert_eq!(config.height, 720);
}
#[test]
fn test_animated_gif_config_with_quality() {
    let config = AnimatedGifConfig::new().with_quality(90);
    assert_eq!(config.quality, 90);
}
#[test]
fn test_animated_gif_config_quality_clamped() {
    let config = AnimatedGifConfig::new().with_quality(150);
    assert_eq!(config.quality, 100);
}
#[test]
fn test_video_config_default() {
    let config = VideoConfig::default();
    assert_eq!(config.fps, 30);
    assert_eq!(config.duration, 10);
    assert_eq!(config.width, 1920);
    assert_eq!(config.height, 1080);
    assert_eq!(config.bitrate, 5000);
    assert_eq!(config.codec, "h264");
}
#[test]
fn test_video_config_hd_1080p() {
    let config = VideoConfig::hd_1080p();
    assert_eq!(config.width, 1920);
    assert_eq!(config.height, 1080);
    assert_eq!(config.bitrate, 8000);
}
#[test]
fn test_video_config_hd_720p() {
    let config = VideoConfig::hd_720p();
    assert_eq!(config.width, 1280);
    assert_eq!(config.height, 720);
    assert_eq!(config.bitrate, 5000);
}
#[test]
fn test_video_config_uhd_4k() {
    let config = VideoConfig::uhd_4k();
    assert_eq!(config.width, 3840);
    assert_eq!(config.height, 2160);
    assert_eq!(config.bitrate, 20000);
}
#[test]
fn test_video_config_with_codec() {
    let config = VideoConfig::new().with_codec("vp9");
    assert_eq!(config.codec, "vp9");
}
#[test]
fn test_video_config_with_bitrate() {
    let config = VideoConfig::new().with_bitrate(10000);
    assert_eq!(config.bitrate, 10000);
}
#[test]
fn test_video_config_with_duration() {
    let config = VideoConfig::new().with_duration(20);
    assert_eq!(config.duration, 20);
}
#[test]
fn test_pdf_config_default() {
    let config = PdfConfig::default();
    assert_eq!(config.width, 210.0);
    assert_eq!(config.height, 297.0);
    assert_eq!(config.margin, 10.0);
    assert!(config.vector);
    assert_eq!(config.dpi, 300);
    assert!(config.print_optimized);
}
#[test]
fn test_pdf_config_a4() {
    let config = PdfConfig::a4();
    assert_eq!(config.width, 210.0);
    assert_eq!(config.height, 297.0);
}
#[test]
fn test_pdf_config_a3() {
    let config = PdfConfig::a3();
    assert_eq!(config.width, 297.0);
    assert_eq!(config.height, 420.0);
}
#[test]
fn test_pdf_config_letter() {
    let config = PdfConfig::letter();
    assert_eq!(config.width, 215.9);
    assert_eq!(config.height, 279.4);
}
#[test]
fn test_pdf_config_tabloid() {
    let config = PdfConfig::tabloid();
    assert_eq!(config.width, 279.4);
    assert_eq!(config.height, 431.8);
}
#[test]
fn test_pdf_config_landscape() {
    let config = PdfConfig::a4().landscape();
    assert_eq!(config.width, 297.0);
    assert_eq!(config.height, 210.0);
}
#[test]
fn test_pdf_config_vector() {
    let config = PdfConfig::new().vector();
    assert!(config.vector);
}
#[test]
fn test_pdf_config_raster() {
    let config = PdfConfig::new().raster();
    assert!(!config.vector);
}
#[test]
fn test_pdf_config_print_optimized() {
    let config = PdfConfig::new().print_optimized();
    assert!(config.print_optimized);
}
#[test]
fn test_pdf_config_screen_optimized() {
    let config = PdfConfig::new().screen_optimized();
    assert!(!config.print_optimized);
    assert_eq!(config.dpi, 96);
}
#[test]
fn test_pdf_config_with_dpi() {
    let config = PdfConfig::new().with_dpi(600);
    assert_eq!(config.dpi, 600);
}
#[test]
fn test_pdf_config_with_margin() {
    let config = PdfConfig::new().with_margin(20.0);
    assert_eq!(config.margin, 20.0);
}
#[test]
fn test_advanced_exporter_creation() {
    let exporter = AdvancedExporter::new();
    assert_eq!(exporter.theme.background_color, "#ffffff");
}
#[test]
fn test_advanced_exporter_with_theme() {
    let exporter = AdvancedExporter::new().with_theme(Theme::dark());
    assert_eq!(exporter.theme.background_color, "#1a1a1a");
}
#[test]
fn test_advanced_exporter_default() {
    let exporter1 = AdvancedExporter::new();
    let exporter2 = AdvancedExporter::default();
    assert_eq!(
        exporter1.theme.background_color,
        exporter2.theme.background_color
    );
}
#[test]
fn test_to_animated_gif() {
    let statute = Statute::new(
        "test-1",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let exporter = AdvancedExporter::new();
    let config = AnimatedGifConfig::new().with_fps(2).with_duration(1);
    let frames = exporter.to_animated_gif(&tree, config);
    assert_eq!(frames.len(), 2);
    assert!(frames[0].contains("<svg"));
}
#[test]
fn test_graph_to_animated_gif() {
    let mut graph = DependencyGraph::new();
    graph.add_statute("statute-1");
    graph.add_statute("statute-2");
    graph.add_dependency("statute-2", "statute-1", "depends-on");
    let exporter = AdvancedExporter::new();
    let config = AnimatedGifConfig::new().with_fps(2).with_duration(1);
    let frames = exporter.graph_to_animated_gif(&graph, config);
    assert_eq!(frames.len(), 2);
    assert!(frames[0].contains("<svg"));
}
#[test]
fn test_to_video_frames() {
    let statute = Statute::new(
        "test-1",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let exporter = AdvancedExporter::new();
    let config = VideoConfig::new().with_fps(2).with_duration(1);
    let frames = exporter.to_video_frames(&tree, config);
    assert_eq!(frames.len(), 2);
    assert!(frames[0].contains("<svg"));
}
#[test]
fn test_graph_to_video_frames() {
    let mut graph = DependencyGraph::new();
    graph.add_statute("statute-1");
    let exporter = AdvancedExporter::new();
    let config = VideoConfig::hd_720p().with_fps(2).with_duration(1);
    let frames = exporter.graph_to_video_frames(&graph, config);
    assert_eq!(frames.len(), 2);
    assert!(frames[0].contains("<svg"));
}
