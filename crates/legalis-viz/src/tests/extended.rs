#![cfg(test)]
use super::*;
use legalis_core::{Effect, EffectType};

#[test]
fn test_timeline_narrative_view_creation() {
    let view = TimelineNarrativeView::new("Case Timeline");
    assert_eq!(view.title, "Case Timeline");
    assert!(view.show_captions);
}
#[test]
fn test_timeline_narrative_view_html() {
    let events = vec![
        NarrativeEvent::new(
            "2024-01-15",
            "First Event",
            "This is the first event narrative",
        ),
        NarrativeEvent::new(
            "2024-02-20",
            "Second Event",
            "This is the second event narrative",
        ),
    ];
    let view = TimelineNarrativeView::new("Legal Timeline");
    let html = view.to_html(&events);
    assert!(html.contains("Legal Timeline"));
    assert!(html.contains("2024-01-15"));
    assert!(html.contains("First Event"));
    assert!(html.contains("This is the first event narrative"));
    assert!(html.contains("Second Event"));
}
#[test]
fn test_narrative_event_creation() {
    let event = NarrativeEvent::new("2024-03-10", "Event Title", "Narrative text");
    assert_eq!(event.date, "2024-03-10");
    assert_eq!(event.title, "Event Title");
    assert_eq!(event.narrative, "Narrative text");
}
#[test]
fn test_guided_exploration_tour_creation() {
    let tour = GuidedExplorationTour::new("Legal Concepts Tour");
    assert_eq!(tour.title, "Legal Concepts Tour");
    assert!(!tour.auto_advance);
    assert_eq!(tour.advance_delay, 5000);
}
#[test]
fn test_guided_exploration_tour_auto_advance() {
    let tour = GuidedExplorationTour::new("Tour").with_auto_advance(3000);
    assert!(tour.auto_advance);
    assert_eq!(tour.advance_delay, 3000);
}
#[test]
fn test_guided_exploration_tour_html() {
    let stops = vec![
        TourStop::new("Introduction", "Welcome to the tour"),
        TourStop::new("Main Concept", "This is the main idea").with_visual("Diagram"),
        TourStop::new("Conclusion", "Thank you"),
    ];
    let tour = GuidedExplorationTour::new("Test Tour");
    let html = tour.to_html(&stops);
    assert!(html.contains("Test Tour"));
    assert!(html.contains("Introduction"));
    assert!(html.contains("Welcome to the tour"));
    assert!(html.contains("Main Concept"));
    assert!(html.contains("Diagram"));
    assert!(html.contains("Step 1 of 3"));
}
#[test]
fn test_tour_stop_creation() {
    let stop = TourStop::new("Stop 1", "Description").with_visual("Visual element");
    assert_eq!(stop.title, "Stop 1");
    assert_eq!(stop.description, "Description");
    assert!(stop.visual.is_some());
}
#[test]
fn test_educational_walkthrough_creation() {
    let walkthrough = EducationalWalkthrough::new("Learn Legal Concepts");
    assert_eq!(walkthrough.title, "Learn Legal Concepts");
    assert!(walkthrough.include_quiz);
}
#[test]
fn test_educational_walkthrough_without_quiz() {
    let walkthrough = EducationalWalkthrough::new("Walkthrough").without_quiz();
    assert!(!walkthrough.include_quiz);
}
#[test]
fn test_lesson_creation() {
    let lesson = Lesson::new("Introduction to Contracts")
        .with_content("Contracts are agreements between parties")
        .with_content("They must have consideration")
        .with_example("Example: A buys from B for $100")
        .with_takeaway("Contracts require mutual agreement");
    assert_eq!(lesson.title, "Introduction to Contracts");
    assert_eq!(lesson.content.len(), 2);
    assert!(lesson.example.is_some());
    assert!(lesson.key_takeaway.is_some());
}
#[test]
fn test_quiz_question_creation() {
    let quiz = QuizQuestion::new(
        "What is a contract?",
        vec![
            "An agreement".to_string(),
            "A law".to_string(),
            "A statute".to_string(),
        ],
        0,
    );
    assert_eq!(quiz.question, "What is a contract?");
    assert_eq!(quiz.options.len(), 3);
    assert_eq!(quiz.correct_index, 0);
}
#[test]
fn test_educational_walkthrough_html() {
    let lessons = vec![
        Lesson::new("Lesson 1")
            .with_content("Content paragraph 1")
            .with_example("Example text")
            .with_quiz(QuizQuestion::new(
                "Test question?",
                vec!["Answer A".to_string(), "Answer B".to_string()],
                1,
            ))
            .with_takeaway("Key point to remember"),
        Lesson::new("Lesson 2").with_content("More content"),
    ];
    let walkthrough = EducationalWalkthrough::new("Legal Education");
    let html = walkthrough.to_html(&lessons);
    assert!(html.contains("Legal Education"));
    assert!(html.contains("Lesson 1"));
    assert!(html.contains("Content paragraph 1"));
    assert!(html.contains("Example text"));
    assert!(html.contains("Test question?"));
    assert!(html.contains("Answer A"));
    assert!(html.contains("Key point to remember"));
}
#[test]
fn test_scrollytelling_config_default() {
    let config1 = ScrollytellingConfig::new();
    let config2 = ScrollytellingConfig::default();
    assert_eq!(config1.enable_animations, config2.enable_animations);
}
#[test]
fn test_legal_history_scrollytelling_default() {
    let scrolly = LegalHistoryScrollytelling::default();
    assert_eq!(scrolly.title, "Legal History");
}
#[test]
fn test_case_story_generator_default() {
    let generator = CaseStoryGenerator::default();
    assert!(generator.include_timeline);
}
#[test]
fn test_timeline_narrative_view_default() {
    let view = TimelineNarrativeView::default();
    assert_eq!(view.title, "Timeline");
}
#[test]
fn test_guided_exploration_tour_default() {
    let tour = GuidedExplorationTour::default();
    assert_eq!(tour.title, "Guided Tour");
}
#[test]
fn test_educational_walkthrough_default() {
    let walkthrough = EducationalWalkthrough::default();
    assert_eq!(walkthrough.title, "Educational Walkthrough");
}
#[test]
fn test_key_player_serialization() {
    let player = KeyPlayer {
        name: "John Doe".to_string(),
        role: "Plaintiff".to_string(),
    };
    let json = serde_json::to_string(&player).unwrap();
    assert!(json.contains("John Doe"));
    assert!(json.contains("Plaintiff"));
}
#[test]
fn test_timeline_story_event_serialization() {
    let event = TimelineStoryEvent {
        date: "2024-01-01".to_string(),
        description: "Event occurred".to_string(),
    };
    let json = serde_json::to_string(&event).unwrap();
    assert!(json.contains("2024-01-01"));
    assert!(json.contains("Event occurred"));
}
#[test]
fn test_looking_glass_visualizer_creation() {
    let visualizer = LookingGlassVisualizer::new("Test Hologram");
    assert_eq!(visualizer.title, "Test Hologram");
    assert_eq!(visualizer.config.view_count, 45);
}
#[test]
fn test_looking_glass_config_default() {
    let config = LookingGlassConfig::default();
    assert!(config.enable_quilt);
    assert_eq!(config.view_count, 45);
    assert_eq!(config.quilt_width, 4096);
    assert_eq!(config.quilt_height, 4096);
    assert!(config.enable_depth_mapping);
}
#[test]
fn test_looking_glass_visualizer_with_config() {
    let config = LookingGlassConfig {
        enable_quilt: false,
        view_count: 30,
        quilt_width: 2048,
        quilt_height: 2048,
        enable_depth_mapping: false,
        fov: 20.0,
        depth_range: (0.5, 50.0),
    };
    let visualizer = LookingGlassVisualizer::new("Custom").with_config(config.clone());
    assert_eq!(visualizer.config.view_count, 30);
    assert_eq!(visualizer.config.quilt_width, 2048);
}
#[test]
fn test_looking_glass_visualizer_html_generation() {
    let mut graph = DependencyGraph::new();
    graph.add_statute("test-1");
    let visualizer = LookingGlassVisualizer::new("Holographic Test");
    let html = visualizer.to_holographic_html(&graph);
    assert!(html.contains("Holographic Test"));
    assert!(html.contains("Looking Glass Display"));
    assert!(html.contains("holoplay-core"));
    assert!(html.contains("THREE.Scene"));
}
#[test]
fn test_looking_glass_visualizer_default() {
    let visualizer = LookingGlassVisualizer::default();
    assert_eq!(visualizer.title, "Holographic Visualization");
}
#[test]
fn test_holographic_statute_model_creation() {
    let model = HolographicStatuteModel::new();
    assert_eq!(model.config.layer_count, 5);
    assert!(model.config.enable_rotation);
}
#[test]
fn test_holographic_model_config_default() {
    let config = HolographicModelConfig::default();
    assert!(config.enable_layers);
    assert_eq!(config.layer_count, 5);
    assert!(config.enable_rotation);
    assert_eq!(config.rotation_speed, 15.0);
    assert!(config.enable_interaction);
}
#[test]
fn test_holographic_statute_model_with_config() {
    let config = HolographicModelConfig {
        enable_layers: false,
        layer_count: 3,
        enable_rotation: false,
        rotation_speed: 10.0,
        enable_interaction: false,
    };
    let model = HolographicStatuteModel::new().with_config(config.clone());
    assert_eq!(model.config.layer_count, 3);
    assert!(!model.config.enable_rotation);
}
#[test]
fn test_holographic_statute_model_html() {
    let statute = Statute::new(
        "test-1",
        "Test Statute",
        Effect::new(EffectType::Grant, "Grants permission"),
    );
    let model = HolographicStatuteModel::new();
    let html = model.to_holographic_model(&statute);
    assert!(html.contains("Test Statute"));
    assert!(html.contains("Holographic Statute Model"));
    assert!(html.contains("THREE.Scene"));
    assert!(html.contains("PlaneGeometry"));
}
#[test]
fn test_holographic_statute_model_default() {
    let model = HolographicStatuteModel::default();
    assert_eq!(model.config.layer_count, 5);
}
#[test]
fn test_3d_print_exporter_creation() {
    let exporter = ThreeDPrintExporter::new();
    assert_eq!(exporter.config.format, "STL");
    assert_eq!(exporter.config.scale, 1.0);
}
#[test]
fn test_print_export_config_default() {
    let config = PrintExportConfig::default();
    assert_eq!(config.format, "STL");
    assert_eq!(config.scale, 1.0);
    assert_eq!(config.base_thickness, 2.0);
    assert_eq!(config.wall_thickness, 1.0);
    assert!(!config.generate_supports);
}
#[test]
fn test_3d_print_exporter_to_stl() {
    let statute = Statute::new(
        "test-1",
        "Test Statute",
        Effect::new(EffectType::Grant, "Grants permission"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let exporter = ThreeDPrintExporter::new();
    let stl = exporter.to_stl(&tree);
    assert!(stl.contains("solid DecisionTree"));
    assert!(stl.contains("facet normal"));
    assert!(stl.contains("vertex"));
    assert!(stl.contains("endsolid DecisionTree"));
}
#[test]
fn test_3d_print_exporter_to_obj() {
    let mut graph = DependencyGraph::new();
    graph.add_statute("test-1");
    let exporter = ThreeDPrintExporter::new();
    let obj = exporter.to_obj(&graph);
    assert!(obj.contains("# OBJ file"));
    assert!(obj.contains("# Vertices:"));
    assert!(obj.contains("v "));
    assert!(obj.contains("f "));
}
#[test]
fn test_3d_print_exporter_to_3mf() {
    let statute = Statute::new(
        "test-1",
        "Test Statute",
        Effect::new(EffectType::Grant, "Grants permission"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let exporter = ThreeDPrintExporter::new();
    let mf = exporter.to_3mf(&tree);
    assert!(mf.contains("<?xml version"));
    assert!(mf.contains("<model"));
    assert!(mf.contains("<mesh>"));
    assert!(mf.contains("<vertices>"));
    assert!(mf.contains("<triangles>"));
}
#[test]
fn test_3d_print_exporter_with_config() {
    let config = PrintExportConfig {
        format: "OBJ".to_string(),
        scale: 2.0,
        base_thickness: 3.0,
        wall_thickness: 1.5,
        generate_supports: true,
    };
    let exporter = ThreeDPrintExporter::new().with_config(config.clone());
    assert_eq!(exporter.config.format, "OBJ");
    assert_eq!(exporter.config.scale, 2.0);
}
#[test]
fn test_3d_print_exporter_default() {
    let exporter = ThreeDPrintExporter::default();
    assert_eq!(exporter.config.format, "STL");
}
#[test]
fn test_volumetric_renderer_creation() {
    let renderer = VolumetricRenderer::new("Volumetric Test");
    assert_eq!(renderer.title, "Volumetric Test");
    assert_eq!(renderer.config.sample_steps, 128);
}
#[test]
fn test_volumetric_config_default() {
    let config = VolumetricConfig::default();
    assert!(config.enable_ray_marching);
    assert_eq!(config.sample_steps, 128);
    assert_eq!(config.density_threshold, 0.1);
    assert!(config.enable_lighting);
    assert_eq!(config.transfer_function, "linear");
}
#[test]
fn test_volumetric_renderer_with_config() {
    let config = VolumetricConfig {
        enable_ray_marching: false,
        sample_steps: 256,
        density_threshold: 0.2,
        enable_lighting: false,
        transfer_function: "cubic".to_string(),
    };
    let renderer = VolumetricRenderer::new("Custom").with_config(config.clone());
    assert_eq!(renderer.config.sample_steps, 256);
    assert_eq!(renderer.config.transfer_function, "cubic");
}
#[test]
fn test_volumetric_renderer_html() {
    let mut graph = DependencyGraph::new();
    graph.add_statute("test-1");
    let renderer = VolumetricRenderer::new("Volumetric Viz");
    let html = renderer.to_volumetric_html(&graph);
    assert!(html.contains("Volumetric Viz"));
    assert!(html.contains("Volumetric Rendering"));
    assert!(html.contains("Steps: 128"));
    assert!(html.contains("THREE.Scene"));
    assert!(html.contains("SphereGeometry"));
}
#[test]
fn test_volumetric_renderer_default() {
    let renderer = VolumetricRenderer::default();
    assert_eq!(renderer.title, "Volumetric Visualization");
}
#[test]
fn test_holographic_gesture_controller_creation() {
    let controller = HolographicGestureController::new("Gesture Test");
    assert_eq!(controller.title, "Gesture Test");
    assert!(controller.config.enable_hand_tracking);
}
#[test]
fn test_gesture_config_default() {
    let config = GestureConfig::default();
    assert!(config.enable_hand_tracking);
    assert!(config.enable_pinch);
    assert!(config.enable_swipe);
    assert!(config.enable_rotation);
    assert_eq!(config.sensitivity, 0.7);
}
#[test]
fn test_holographic_gesture_controller_with_config() {
    let config = GestureConfig {
        enable_hand_tracking: false,
        enable_pinch: false,
        enable_swipe: true,
        enable_rotation: false,
        sensitivity: 0.5,
    };
    let controller = HolographicGestureController::new("Custom").with_config(config.clone());
    assert_eq!(controller.config.sensitivity, 0.5);
    assert!(!controller.config.enable_pinch);
}
#[test]
fn test_holographic_gesture_controller_html() {
    let statute = Statute::new(
        "test-1",
        "Test Statute",
        Effect::new(EffectType::Grant, "Grants permission"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let controller = HolographicGestureController::new("Gesture Control");
    let html = controller.to_gesture_html(&tree);
    assert!(html.contains("Gesture Control"));
    assert!(html.contains("Gesture Control Active"));
    assert!(html.contains("Pinch to zoom"));
    assert!(html.contains("THREE.Scene"));
    assert!(html.contains("gestureState"));
}
#[test]
fn test_holographic_gesture_controller_default() {
    let controller = HolographicGestureController::default();
    assert_eq!(
        controller.title,
        "Gesture-Controlled Holographic Visualization"
    );
}
#[test]
fn test_looking_glass_config_serialization() {
    let config = LookingGlassConfig::default();
    let json = serde_json::to_string(&config).unwrap();
    assert!(json.contains("enable_quilt"));
    assert!(json.contains("view_count"));
}
#[test]
fn test_holographic_model_config_serialization() {
    let config = HolographicModelConfig::default();
    let json = serde_json::to_string(&config).unwrap();
    assert!(json.contains("enable_layers"));
    assert!(json.contains("layer_count"));
}
#[test]
fn test_print_export_config_serialization() {
    let config = PrintExportConfig::default();
    let json = serde_json::to_string(&config).unwrap();
    assert!(json.contains("format"));
    assert!(json.contains("scale"));
}
#[test]
fn test_volumetric_config_serialization() {
    let config = VolumetricConfig::default();
    let json = serde_json::to_string(&config).unwrap();
    assert!(json.contains("enable_ray_marching"));
    assert!(json.contains("sample_steps"));
}
#[test]
fn test_gesture_config_serialization() {
    let config = GestureConfig::default();
    let json = serde_json::to_string(&config).unwrap();
    assert!(json.contains("enable_hand_tracking"));
    assert!(json.contains("sensitivity"));
}
#[test]
fn test_jurisdictional_statute_creation() {
    let statute = Statute::new(
        "adult-rights",
        "Adult Rights Act",
        Effect::new(EffectType::Grant, "Grants adult rights"),
    );
    let js = JurisdictionalStatute::new("US", "United States", statute);
    assert_eq!(js.jurisdiction, "US");
    assert_eq!(js.jurisdiction_name, "United States");
    assert_eq!(js.statute.id, "adult-rights");
    assert!(js.metadata.is_empty());
}
#[test]
fn test_jurisdictional_statute_with_metadata() {
    let statute = Statute::new(
        "test",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let js = JurisdictionalStatute::new("JP", "Japan", statute)
        .with_metadata("enacted", "2020")
        .with_metadata("status", "active");
    assert_eq!(js.metadata.len(), 2);
    assert_eq!(js.metadata.get("enacted"), Some(&"2020".to_string()));
    assert_eq!(js.metadata.get("status"), Some(&"active".to_string()));
}
#[test]
fn test_jurisdictional_difference_creation() {
    let diff = JurisdictionalDifference::new(
        "age_requirement",
        "Different age requirements across jurisdictions",
    );
    assert_eq!(diff.aspect, "age_requirement");
    assert_eq!(diff.severity, 0.5);
    assert!(diff.values.is_empty());
}
#[test]
fn test_jurisdictional_difference_with_values() {
    let diff = JurisdictionalDifference::new("age", "Age requirement differs")
        .with_value("US", "18 years")
        .with_value("JP", "20 years")
        .with_value("DE", "18 years")
        .with_severity(0.7);
    assert_eq!(diff.values.len(), 3);
    assert_eq!(diff.values.get("US"), Some(&"18 years".to_string()));
    assert_eq!(diff.values.get("JP"), Some(&"20 years".to_string()));
    assert_eq!(diff.severity, 0.7);
}
#[test]
fn test_jurisdictional_difference_severity_clamping() {
    let diff1 = JurisdictionalDifference::new("test", "test").with_severity(1.5);
    assert_eq!(diff1.severity, 1.0);
    let diff2 = JurisdictionalDifference::new("test", "test").with_severity(-0.5);
    assert_eq!(diff2.severity, 0.0);
}
#[test]
fn test_cross_jurisdictional_comparison_creation() {
    let comparison = CrossJurisdictionalComparison::new("Adult Rights Comparison");
    assert_eq!(comparison.title, "Adult Rights Comparison");
    assert!(comparison.statutes.is_empty());
    assert!(comparison.differences.is_empty());
    assert!(comparison.synchronized_nav);
}
#[test]
fn test_cross_jurisdictional_comparison_default() {
    let comparison = CrossJurisdictionalComparison::default();
    assert_eq!(comparison.title, "Jurisdictional Comparison");
}
#[test]
fn test_cross_jurisdictional_comparison_add_statute() {
    let mut comparison = CrossJurisdictionalComparison::new("Test");
    let statute1 = Statute::new("test1", "Test 1", Effect::new(EffectType::Grant, "Test"));
    let js1 = JurisdictionalStatute::new("US", "United States", statute1);
    comparison.add_statute(js1);
    assert_eq!(comparison.statutes.len(), 1);
    assert_eq!(comparison.statutes[0].jurisdiction, "US");
}
#[test]
fn test_cross_jurisdictional_comparison_add_difference() {
    let mut comparison = CrossJurisdictionalComparison::new("Test");
    let diff = JurisdictionalDifference::new("age", "Age differs")
        .with_value("US", "18")
        .with_value("JP", "20");
    comparison.add_difference(diff);
    assert_eq!(comparison.differences.len(), 1);
    assert_eq!(comparison.differences[0].aspect, "age");
}
#[test]
fn test_cross_jurisdictional_comparison_with_theme() {
    let comparison = CrossJurisdictionalComparison::new("Test").with_theme(Theme::dark());
    assert_eq!(comparison.theme.background_color, "#1a1a1a");
}
#[test]
fn test_cross_jurisdictional_comparison_with_synchronized_nav() {
    let comparison1 = CrossJurisdictionalComparison::new("Test").with_synchronized_nav(true);
    assert!(comparison1.synchronized_nav);
    let comparison2 = CrossJurisdictionalComparison::new("Test").with_synchronized_nav(false);
    assert!(!comparison2.synchronized_nav);
}
#[test]
fn test_cross_jurisdictional_comparison_side_by_side_html() {
    let mut comparison = CrossJurisdictionalComparison::new("Adult Rights Comparison");
    let statute_us = Statute::new(
        "us-adult",
        "US Adult Rights",
        Effect::new(EffectType::Grant, "Grants rights at 18"),
    );
    let js_us = JurisdictionalStatute::new("US", "United States", statute_us)
        .with_metadata("enacted", "1971");
    let statute_jp = Statute::new(
        "jp-adult",
        "Japan Adult Rights",
        Effect::new(EffectType::Grant, "Grants rights at 20"),
    );
    let js_jp =
        JurisdictionalStatute::new("JP", "Japan", statute_jp).with_metadata("enacted", "2022");
    comparison.add_statute(js_us);
    comparison.add_statute(js_jp);
    let diff = JurisdictionalDifference::new("age_requirement", "Age of majority differs")
        .with_value("US", "18 years")
        .with_value("JP", "20 years")
        .with_severity(0.6);
    comparison.add_difference(diff);
    let html = comparison.to_side_by_side_html();
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("Adult Rights Comparison"));
    assert!(html.contains("United States"));
    assert!(html.contains("Japan"));
    assert!(html.contains("us-adult"));
    assert!(html.contains("jp-adult"));
    assert!(html.contains("age_requirement"));
    assert!(html.contains("18 years"));
    assert!(html.contains("20 years"));
    assert!(html.contains("jurisdiction-column"));
    assert!(html.contains("differences-section"));
}
#[test]
fn test_cross_jurisdictional_comparison_side_by_side_html_no_differences() {
    let mut comparison = CrossJurisdictionalComparison::new("Test Comparison");
    let statute = Statute::new(
        "test",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test"),
    );
    let js = JurisdictionalStatute::new("US", "United States", statute);
    comparison.add_statute(js);
    let html = comparison.to_side_by_side_html();
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("Test Comparison"));
    assert!(html.contains("United States"));
    assert!(!html.contains("Key Differences"));
}
#[test]
fn test_cross_jurisdictional_comparison_synchronized_navigation_script() {
    let comparison1 = CrossJurisdictionalComparison::new("Test").with_synchronized_nav(true);
    let html1 = comparison1.to_side_by_side_html();
    assert!(html1.contains("addEventListener('scroll'"));
    assert!(html1.contains("scrollRatio"));
    let comparison2 = CrossJurisdictionalComparison::new("Test").with_synchronized_nav(false);
    let html2 = comparison2.to_side_by_side_html();
    assert!(!html2.contains("addEventListener('scroll'"));
}
#[test]
fn test_cross_jurisdictional_comparison_heatmap_html() {
    let mut comparison = CrossJurisdictionalComparison::new("Rights Comparison");
    let statute_us = Statute::new(
        "us-rights",
        "US Rights",
        Effect::new(EffectType::Grant, "US rights"),
    );
    let js_us = JurisdictionalStatute::new("US", "United States", statute_us);
    let statute_jp = Statute::new(
        "jp-rights",
        "JP Rights",
        Effect::new(EffectType::Grant, "JP rights"),
    );
    let js_jp = JurisdictionalStatute::new("JP", "Japan", statute_jp);
    let statute_de = Statute::new(
        "de-rights",
        "DE Rights",
        Effect::new(EffectType::Grant, "DE rights"),
    );
    let js_de = JurisdictionalStatute::new("DE", "Germany", statute_de);
    comparison.add_statute(js_us);
    comparison.add_statute(js_jp);
    comparison.add_statute(js_de);
    let diff1 = JurisdictionalDifference::new("age", "Age requirement")
        .with_value("US", "18")
        .with_value("JP", "20")
        .with_value("DE", "18")
        .with_severity(0.3);
    let diff2 = JurisdictionalDifference::new("citizenship", "Citizenship requirement")
        .with_value("US", "Yes")
        .with_value("JP", "No")
        .with_value("DE", "EU only")
        .with_severity(0.8);
    comparison.add_difference(diff1);
    comparison.add_difference(diff2);
    let html = comparison.to_heatmap_html();
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("Jurisdictional Heatmap"));
    assert!(html.contains("heatmap-container"));
    assert!(html.contains("US"));
    assert!(html.contains("JP"));
    assert!(html.contains("DE"));
    assert!(html.contains("age"));
    assert!(html.contains("citizenship"));
    assert!(html.contains("heatmap-low"));
    assert!(html.contains("heatmap-high"));
}
#[test]
fn test_cross_jurisdictional_comparison_heatmap_severity_classes() {
    let mut comparison = CrossJurisdictionalComparison::new("Test");
    let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));
    let js = JurisdictionalStatute::new("US", "United States", statute);
    comparison.add_statute(js);
    let diff_low = JurisdictionalDifference::new("low", "Low severity")
        .with_value("US", "Low")
        .with_severity(0.2);
    let diff_medium = JurisdictionalDifference::new("medium", "Medium severity")
        .with_value("US", "Medium")
        .with_severity(0.5);
    let diff_high = JurisdictionalDifference::new("high", "High severity")
        .with_value("US", "High")
        .with_severity(0.9);
    comparison.add_difference(diff_low);
    comparison.add_difference(diff_medium);
    comparison.add_difference(diff_high);
    let html = comparison.to_heatmap_html();
    assert!(html.contains("heatmap-low"));
    assert!(html.contains("heatmap-medium"));
    assert!(html.contains("heatmap-high"));
}
#[test]
fn test_cross_jurisdictional_comparison_heatmap_missing_values() {
    let mut comparison = CrossJurisdictionalComparison::new("Test");
    let statute_us = Statute::new("us", "US", Effect::new(EffectType::Grant, "US"));
    let js_us = JurisdictionalStatute::new("US", "United States", statute_us);
    let statute_jp = Statute::new("jp", "JP", Effect::new(EffectType::Grant, "JP"));
    let js_jp = JurisdictionalStatute::new("JP", "Japan", statute_jp);
    comparison.add_statute(js_us);
    comparison.add_statute(js_jp);
    let diff = JurisdictionalDifference::new("test", "Test difference")
        .with_value("US", "Available")
        .with_severity(0.5);
    comparison.add_difference(diff);
    let html = comparison.to_heatmap_html();
    assert!(html.contains("Available"));
    assert!(html.contains("N/A"));
}
#[test]
fn test_jurisdictional_statute_serialization() {
    let statute = Statute::new(
        "test",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test"),
    );
    let js =
        JurisdictionalStatute::new("US", "United States", statute).with_metadata("year", "2020");
    let json = serde_json::to_string(&js).unwrap();
    assert!(json.contains("US"));
    assert!(json.contains("United States"));
    assert!(json.contains("test"));
    assert!(json.contains("year"));
}
#[test]
fn test_jurisdictional_difference_serialization() {
    let diff = JurisdictionalDifference::new("age", "Age differs")
        .with_value("US", "18")
        .with_value("JP", "20")
        .with_severity(0.7);
    let json = serde_json::to_string(&diff).unwrap();
    assert!(json.contains("age"));
    assert!(json.contains("18"));
    assert!(json.contains("20"));
    assert!(json.contains("0.7"));
}
#[test]
fn test_legal_concept_creation() {
    let concept = LegalConcept::new("c1", "Privacy Right", "Right to privacy", "rights");
    assert_eq!(concept.id, "c1");
    assert_eq!(concept.name, "Privacy Right");
    assert_eq!(concept.description, "Right to privacy");
    assert_eq!(concept.category, "rights");
    assert!(concept.statute_ids.is_empty());
    assert!(concept.metadata.is_empty());
}
#[test]
fn test_legal_concept_add_statute() {
    let mut concept = LegalConcept::new("c1", "Privacy", "Privacy rights", "rights");
    concept.add_statute("s1");
    concept.add_statute("s2");
    assert_eq!(concept.statute_ids.len(), 2);
    assert_eq!(concept.statute_ids[0], "s1");
    assert_eq!(concept.statute_ids[1], "s2");
}
#[test]
fn test_legal_concept_with_metadata() {
    let concept = LegalConcept::new("c1", "Privacy", "Privacy rights", "rights")
        .with_metadata("jurisdiction", "US")
        .with_metadata("enacted", "2020");
    assert_eq!(concept.metadata.len(), 2);
    assert_eq!(
        concept.metadata.get("jurisdiction"),
        Some(&"US".to_string())
    );
    assert_eq!(concept.metadata.get("enacted"), Some(&"2020".to_string()));
}
#[test]
fn test_concept_relation_type_label() {
    assert_eq!(ConceptRelationType::IsA.label(), "is a");
    assert_eq!(ConceptRelationType::PartOf.label(), "part of");
    assert_eq!(ConceptRelationType::Requires.label(), "requires");
    assert_eq!(ConceptRelationType::ConflictsWith.label(), "conflicts with");
    assert_eq!(ConceptRelationType::Enables.label(), "enables");
    assert_eq!(ConceptRelationType::RelatedTo.label(), "related to");
    assert_eq!(ConceptRelationType::Supersedes.label(), "supersedes");
    assert_eq!(ConceptRelationType::Implements.label(), "implements");
}
#[test]
fn test_concept_relation_type_color() {
    assert_eq!(ConceptRelationType::IsA.color(), "#3498db");
    assert_eq!(ConceptRelationType::PartOf.color(), "#2ecc71");
    assert_eq!(ConceptRelationType::Requires.color(), "#e74c3c");
    assert_eq!(ConceptRelationType::ConflictsWith.color(), "#c0392b");
}
#[test]
fn test_concept_relationship_creation() {
    let rel = ConceptRelationship::new("c1", "c2", ConceptRelationType::IsA);
    assert_eq!(rel.from_id, "c1");
    assert_eq!(rel.to_id, "c2");
    assert_eq!(rel.relation_type, ConceptRelationType::IsA);
    assert_eq!(rel.strength, 1.0);
    assert!(rel.description.is_empty());
}
#[test]
fn test_concept_relationship_with_description() {
    let rel = ConceptRelationship::new("c1", "c2", ConceptRelationType::Requires)
        .with_description("Requires for validity");
    assert_eq!(rel.description, "Requires for validity");
}
#[test]
fn test_concept_relationship_with_strength() {
    let rel =
        ConceptRelationship::new("c1", "c2", ConceptRelationType::RelatedTo).with_strength(0.7);
    assert_eq!(rel.strength, 0.7);
    let rel_high =
        ConceptRelationship::new("c1", "c2", ConceptRelationType::IsA).with_strength(1.5);
    assert_eq!(rel_high.strength, 1.0);
    let rel_low =
        ConceptRelationship::new("c1", "c2", ConceptRelationType::IsA).with_strength(-0.5);
    assert_eq!(rel_low.strength, 0.0);
}
#[test]
fn test_concept_relationship_graph_creation() {
    let graph = ConceptRelationshipGraph::new("Legal Concepts");
    assert_eq!(graph.title, "Legal Concepts");
    assert!(graph.concepts.is_empty());
    assert!(graph.relationships.is_empty());
}
#[test]
fn test_concept_relationship_graph_add_concept() {
    let mut graph = ConceptRelationshipGraph::new("Test");
    let concept = LegalConcept::new("c1", "Privacy", "Privacy rights", "rights");
    graph.add_concept(concept);
    assert_eq!(graph.concepts.len(), 1);
    assert_eq!(graph.concepts[0].id, "c1");
}
#[test]
fn test_concept_relationship_graph_add_relationship() {
    let mut graph = ConceptRelationshipGraph::new("Test");
    let rel = ConceptRelationship::new("c1", "c2", ConceptRelationType::IsA);
    graph.add_relationship(rel);
    assert_eq!(graph.relationships.len(), 1);
    assert_eq!(graph.relationships[0].from_id, "c1");
}
#[test]
fn test_concept_relationship_graph_html() {
    let mut graph = ConceptRelationshipGraph::new("Legal Network");
    let c1 = LegalConcept::new("c1", "Privacy", "Privacy rights", "rights");
    let c2 = LegalConcept::new("c2", "Data Protection", "Data protection laws", "rights");
    graph.add_concept(c1);
    graph.add_concept(c2);
    graph.add_relationship(ConceptRelationship::new(
        "c1",
        "c2",
        ConceptRelationType::RelatedTo,
    ));
    let html = graph.to_html();
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("Legal Network"));
    assert!(html.contains("Privacy"));
    assert!(html.contains("Data Protection"));
    assert!(html.contains("d3js.org"));
    assert!(html.contains("forceSimulation"));
}
#[test]
fn test_concept_relationship_graph_mermaid() {
    let mut graph = ConceptRelationshipGraph::new("Test");
    let c1 = LegalConcept::new("c1", "Privacy", "Privacy rights", "rights");
    let c2 = LegalConcept::new("c2", "Security", "Security measures", "obligations");
    graph.add_concept(c1);
    graph.add_concept(c2);
    graph.add_relationship(ConceptRelationship::new(
        "c1",
        "c2",
        ConceptRelationType::Requires,
    ));
    let mermaid = graph.to_mermaid();
    assert!(mermaid.contains("graph TD"));
    assert!(mermaid.contains("c1[\"Privacy\"]"));
    assert!(mermaid.contains("c2[\"Security\"]"));
    assert!(mermaid.contains("c1 -->|requires| c2"));
}
#[test]
fn test_statute_concept_mapping_creation() {
    let mapping = StatuteConceptMapping::new("s1", "GDPR Article 5");
    assert_eq!(mapping.statute_id, "s1");
    assert_eq!(mapping.statute_name, "GDPR Article 5");
    assert!(mapping.concept_ids.is_empty());
    assert!(mapping.confidence_scores.is_empty());
}
#[test]
fn test_statute_concept_mapping_add_concept() {
    let mut mapping = StatuteConceptMapping::new("s1", "Privacy Act");
    mapping.add_concept("c1", 0.9);
    mapping.add_concept("c2", 0.7);
    assert_eq!(mapping.concept_ids.len(), 2);
    assert_eq!(mapping.concept_ids[0], "c1");
    assert_eq!(mapping.confidence("c1"), 0.9);
    assert_eq!(mapping.confidence("c2"), 0.7);
    assert_eq!(mapping.confidence("c3"), 0.0);
}
#[test]
fn test_statute_concept_mapping_confidence_clamping() {
    let mut mapping = StatuteConceptMapping::new("s1", "Test");
    mapping.add_concept("c1", 1.5);
    mapping.add_concept("c2", -0.5);
    assert_eq!(mapping.confidence("c1"), 1.0);
    assert_eq!(mapping.confidence("c2"), 0.0);
}
#[test]
fn test_ontology_based_visualizer_creation() {
    let viz = OntologyBasedVisualizer::new();
    assert_eq!(viz.theme.background_color, "#ffffff");
}
#[test]
fn test_ontology_based_visualizer_with_theme() {
    let viz = OntologyBasedVisualizer::new().with_theme(Theme::dark());
    assert_eq!(viz.theme.background_color, "#1a1a1a");
}
#[test]
fn test_ontology_based_visualizer_html() {
    let viz = OntologyBasedVisualizer::new();
    let mut graph = ConceptRelationshipGraph::new("Ontology");
    let c1 = LegalConcept::new("c1", "Legal Right", "A legal right", "rights");
    graph.add_concept(c1);
    let html = viz.to_html(&graph);
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("Ontology"));
    assert!(html.contains("ontology-layer"));
    assert!(html.contains("ontology-root"));
}
#[test]
fn test_ontology_based_visualizer_tree_html() {
    let viz = OntologyBasedVisualizer::new();
    let mut graph = ConceptRelationshipGraph::new("Test Ontology");
    let c1 = LegalConcept::new("c1", "Privacy", "Privacy concept", "rights");
    graph.add_concept(c1);
    let html = viz.to_tree_html(&graph);
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("Test Ontology"));
    assert!(html.contains("Privacy"));
    assert!(html.contains("tree-node"));
}
#[test]
fn test_semantic_search_highlighter_creation() {
    let highlighter = SemanticSearchHighlighter::new("privacy");
    assert_eq!(highlighter.query, "privacy");
    assert!(highlighter.matches.is_empty());
    assert_eq!(highlighter.highlight_color, "#ffeb3b");
}
#[test]
fn test_semantic_search_highlighter_with_color() {
    let highlighter = SemanticSearchHighlighter::new("test").with_color("#ff0000");
    assert_eq!(highlighter.highlight_color, "#ff0000");
}
#[test]
fn test_semantic_search_highlighter_search() {
    let mut graph = ConceptRelationshipGraph::new("Test");
    let c1 = LegalConcept::new("c1", "Privacy Right", "Protects privacy", "rights");
    let c2 = LegalConcept::new("c2", "Data Security", "Ensures security", "obligations");
    let c3 = LegalConcept::new("c3", "Privacy Policy", "Privacy guidelines", "procedures");
    graph.add_concept(c1);
    graph.add_concept(c2);
    graph.add_concept(c3);
    let mut highlighter = SemanticSearchHighlighter::new("privacy");
    highlighter.search(&graph);
    assert_eq!(highlighter.matches.len(), 2);
    assert!(highlighter.matches.contains(&"c1".to_string()));
    assert!(highlighter.matches.contains(&"c3".to_string()));
    assert!(!highlighter.matches.contains(&"c2".to_string()));
}
#[test]
fn test_semantic_search_highlighter_relevance_scoring() {
    let mut graph = ConceptRelationshipGraph::new("Test");
    let c1 = LegalConcept::new("c1", "Privacy", "About privacy", "rights");
    graph.add_concept(c1);
    let mut highlighter = SemanticSearchHighlighter::new("privacy");
    highlighter.search(&graph);
    assert_eq!(highlighter.relevance_scores.get("c1"), Some(&1.0));
}
#[test]
fn test_semantic_search_highlighter_highlighted_html() {
    let mut graph = ConceptRelationshipGraph::new("Test");
    let c1 = LegalConcept::new("c1", "Privacy", "Privacy concept", "rights");
    graph.add_concept(c1);
    let mut highlighter = SemanticSearchHighlighter::new("privacy");
    highlighter.search(&graph);
    let html = highlighter.to_highlighted_html(&graph);
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("highlights"));
    assert!(html.contains("#ffeb3b"));
}
#[test]
fn test_concept_hierarchy_tree_creation() {
    let concept = LegalConcept::new("c1", "Legal Right", "A legal right", "rights");
    let tree = ConceptHierarchyTree::new(concept);
    assert_eq!(tree.root.id, "c1");
    assert_eq!(tree.root.name, "Legal Right");
    assert!(tree.children.is_empty());
}
#[test]
fn test_concept_hierarchy_tree_add_child() {
    let root = LegalConcept::new("c1", "Right", "General right", "rights");
    let mut tree = ConceptHierarchyTree::new(root);
    let child_concept = LegalConcept::new("c2", "Privacy Right", "Privacy right", "rights");
    let child_tree = ConceptHierarchyTree::new(child_concept);
    tree.add_child(child_tree);
    assert_eq!(tree.children.len(), 1);
    assert_eq!(tree.children[0].root.id, "c2");
}
#[test]
fn test_concept_hierarchy_tree_from_graph() {
    let mut graph = ConceptRelationshipGraph::new("Test");
    let c1 = LegalConcept::new("c1", "Right", "General right", "rights");
    let c2 = LegalConcept::new("c2", "Privacy Right", "Privacy right", "rights");
    let c3 = LegalConcept::new("c3", "Data Privacy", "Data privacy", "rights");
    graph.add_concept(c1);
    graph.add_concept(c2);
    graph.add_concept(c3);
    graph.add_relationship(ConceptRelationship::new(
        "c2",
        "c1",
        ConceptRelationType::IsA,
    ));
    graph.add_relationship(ConceptRelationship::new(
        "c3",
        "c2",
        ConceptRelationType::IsA,
    ));
    let tree = ConceptHierarchyTree::from_graph(&graph, "c1").unwrap();
    assert_eq!(tree.root.id, "c1");
    assert_eq!(tree.children.len(), 1);
    assert_eq!(tree.children[0].root.id, "c2");
    assert_eq!(tree.children[0].children.len(), 1);
    assert_eq!(tree.children[0].children[0].root.id, "c3");
}
#[test]
fn test_concept_hierarchy_tree_html() {
    let concept = LegalConcept::new("c1", "Privacy", "Privacy concept", "rights");
    let tree = ConceptHierarchyTree::new(concept);
    let html = tree.to_html();
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("Concept Hierarchy"));
    assert!(html.contains("Privacy"));
    assert!(html.contains("concept-box"));
    assert!(html.contains("concept-name"));
}
#[test]
fn test_concept_hierarchy_tree_mermaid() {
    let root = LegalConcept::new("c1", "Right", "General right", "rights");
    let mut tree = ConceptHierarchyTree::new(root);
    let child = LegalConcept::new("c2", "Privacy Right", "Privacy right", "rights");
    let child_tree = ConceptHierarchyTree::new(child);
    tree.add_child(child_tree);
    let mermaid = tree.to_mermaid();
    assert!(mermaid.contains("graph TD"));
    assert!(mermaid.contains("c1[\"Right\"]"));
    assert!(mermaid.contains("c2[\"Privacy Right\"]"));
    assert!(mermaid.contains("c1 --> c2"));
}
#[test]
fn test_legal_concept_serialization() {
    let concept = LegalConcept::new("c1", "Privacy", "Privacy right", "rights")
        .with_metadata("jurisdiction", "US");
    let json = serde_json::to_string(&concept).unwrap();
    assert!(json.contains("c1"));
    assert!(json.contains("Privacy"));
    assert!(json.contains("rights"));
    assert!(json.contains("jurisdiction"));
}
#[test]
fn test_concept_relationship_serialization() {
    let rel = ConceptRelationship::new("c1", "c2", ConceptRelationType::IsA).with_strength(0.8);
    let json = serde_json::to_string(&rel).unwrap();
    assert!(json.contains("c1"));
    assert!(json.contains("c2"));
    assert!(json.contains("0.8"));
}
#[test]
fn test_concept_relationship_graph_serialization() {
    let mut graph = ConceptRelationshipGraph::new("Test");
    let c1 = LegalConcept::new("c1", "Privacy", "Privacy concept", "rights");
    graph.add_concept(c1);
    let json = serde_json::to_string(&graph).unwrap();
    assert!(json.contains("Test"));
    assert!(json.contains("c1"));
    assert!(json.contains("Privacy"));
}
#[test]
fn test_statute_change_event_creation() {
    let event = StatuteChangeEvent::new(
        "evt-1",
        "statute-1",
        "Test Statute",
        "2024-01-15T10:00:00Z",
        "amended",
        "Added new section",
        "2.0",
    );
    assert_eq!(event.id, "evt-1");
    assert_eq!(event.statute_id, "statute-1");
    assert_eq!(event.statute_name, "Test Statute");
    assert_eq!(event.timestamp, "2024-01-15T10:00:00Z");
    assert_eq!(event.change_type, "amended");
    assert_eq!(event.description, "Added new section");
    assert_eq!(event.version, "2.0");
    assert_eq!(event.metric_value, None);
}
#[test]
fn test_statute_change_event_with_metric() {
    let event = StatuteChangeEvent::new(
        "evt-1",
        "statute-1",
        "Test Statute",
        "2024-01-15T10:00:00Z",
        "amended",
        "Modified 5 sections",
        "2.0",
    )
    .with_metric(5.0);
    assert_eq!(event.metric_value, Some(5.0));
}
#[test]
fn test_statute_time_series_creation() {
    let series = StatuteTimeSeries::new("Test Time Series");
    assert_eq!(series.title, "Test Time Series");
    assert_eq!(series.events.len(), 0);
    assert!(series.show_metrics);
}
#[test]
fn test_statute_time_series_add_event() {
    let mut series = StatuteTimeSeries::new("Test Series");
    let event = StatuteChangeEvent::new(
        "evt-1",
        "statute-1",
        "Test Statute",
        "2024-01-15T10:00:00Z",
        "enacted",
        "Initial version",
        "1.0",
    );
    series.add_event(event);
    assert_eq!(series.events.len(), 1);
    assert_eq!(series.events[0].change_type, "enacted");
}
#[test]
fn test_statute_time_series_with_theme() {
    let series = StatuteTimeSeries::new("Test").with_theme(Theme::dark());
    assert_eq!(series.theme.background_color, "#1a1a1a");
}
#[test]
fn test_statute_time_series_with_show_metrics() {
    let series = StatuteTimeSeries::new("Test").with_show_metrics(false);
    assert!(!series.show_metrics);
}
#[test]
fn test_statute_time_series_html_generation() {
    let mut series = StatuteTimeSeries::new("Statute Changes");
    let event = StatuteChangeEvent::new(
        "evt-1",
        "statute-1",
        "Test Statute",
        "2024-01-15T10:00:00Z",
        "amended",
        "Updated section 3",
        "2.0",
    )
    .with_metric(3.0);
    series.add_event(event);
    let html = series.to_html();
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("Statute Changes"));
    assert!(html.contains("d3.v7.min.js"));
    assert!(html.contains("Test Statute"));
}
#[test]
fn test_statute_time_series_mermaid() {
    let mut series = StatuteTimeSeries::new("Statute Changes");
    let event = StatuteChangeEvent::new(
        "evt-1",
        "statute-1",
        "Test Statute",
        "2024-01-15T10:00:00Z",
        "enacted",
        "Initial version",
        "1.0",
    );
    series.add_event(event);
    let mermaid = series.to_mermaid();
    assert!(mermaid.contains("timeline"));
    assert!(mermaid.contains("Statute Changes"));
    assert!(mermaid.contains("Test Statute"));
    assert!(mermaid.contains("enacted"));
}
#[test]
fn test_legal_evolution_timeline_creation() {
    let timeline = LegalEvolutionTimeline::new("statute-1", "Test Statute");
    assert_eq!(timeline.statute_id, "statute-1");
    assert_eq!(timeline.statute_name, "Test Statute");
    assert_eq!(timeline.title, "Evolution of Test Statute");
    assert_eq!(timeline.events.len(), 0);
}
#[test]
fn test_legal_evolution_timeline_add_event() {
    let mut timeline = LegalEvolutionTimeline::new("statute-1", "Test Statute");
    let event = StatuteChangeEvent::new(
        "evt-1",
        "statute-1",
        "Test Statute",
        "2024-01-15T10:00:00Z",
        "enacted",
        "Initial enactment",
        "1.0",
    );
    timeline.add_event(event);
    assert_eq!(timeline.events.len(), 1);
}
#[test]
fn test_legal_evolution_timeline_html() {
    let mut timeline = LegalEvolutionTimeline::new("statute-1", "Test Statute");
    let event = StatuteChangeEvent::new(
        "evt-1",
        "statute-1",
        "Test Statute",
        "2024-01-15T10:00:00Z",
        "enacted",
        "Initial enactment",
        "1.0",
    );
    timeline.add_event(event);
    let html = timeline.to_html();
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("Evolution of Test Statute"));
    assert!(html.contains("timeline"));
    assert!(html.contains("enacted"));
}
#[test]
fn test_legal_evolution_timeline_mermaid() {
    let mut timeline = LegalEvolutionTimeline::new("statute-1", "Test Statute");
    let event = StatuteChangeEvent::new(
        "evt-1",
        "statute-1",
        "Test Statute",
        "2024-01-15T10:00:00Z",
        "enacted",
        "Initial enactment",
        "1.0",
    );
    timeline.add_event(event);
    let mermaid = timeline.to_mermaid();
    assert!(mermaid.contains("graph LR"));
    assert!(mermaid.contains("enacted"));
    assert!(mermaid.contains("1.0"));
}
#[test]
fn test_amendment_impact_creation() {
    let impact = AmendmentImpact::new(
        "amend-1",
        "statute-1",
        "Test Statute",
        "2024-01-15T10:00:00Z",
        "Major amendment",
        5,
        3,
        0.8,
    );
    assert_eq!(impact.id, "amend-1");
    assert_eq!(impact.statute_id, "statute-1");
    assert_eq!(impact.sections_affected, 5);
    assert_eq!(impact.downstream_statutes, 3);
    assert_eq!(impact.severity, 0.8);
    assert_eq!(impact.affected_population, None);
}
#[test]
fn test_amendment_impact_with_affected_population() {
    let impact = AmendmentImpact::new(
        "amend-1",
        "statute-1",
        "Test Statute",
        "2024-01-15T10:00:00Z",
        "Major amendment",
        5,
        3,
        0.8,
    )
    .with_affected_population(100000);
    assert_eq!(impact.affected_population, Some(100000));
}
#[test]
fn test_amendment_impact_severity_clamping() {
    let impact1 = AmendmentImpact::new(
        "amend-1",
        "statute-1",
        "Test",
        "2024-01-15",
        "Test",
        5,
        3,
        1.5,
    );
    assert_eq!(impact1.severity, 1.0);
    let impact2 = AmendmentImpact::new(
        "amend-2",
        "statute-2",
        "Test",
        "2024-01-15",
        "Test",
        5,
        3,
        -0.5,
    );
    assert_eq!(impact2.severity, 0.0);
}
#[test]
fn test_amendment_impact_analysis_creation() {
    let analysis = AmendmentImpactAnalysis::new("Impact Analysis");
    assert_eq!(analysis.title, "Impact Analysis");
    assert_eq!(analysis.amendments.len(), 0);
}
#[test]
fn test_amendment_impact_analysis_add_amendment() {
    let mut analysis = AmendmentImpactAnalysis::new("Impact Analysis");
    let impact = AmendmentImpact::new(
        "amend-1",
        "statute-1",
        "Test Statute",
        "2024-01-15T10:00:00Z",
        "Major amendment",
        5,
        3,
        0.8,
    );
    analysis.add_amendment(impact);
    assert_eq!(analysis.amendments.len(), 1);
}
#[test]
fn test_amendment_impact_analysis_html() {
    let mut analysis = AmendmentImpactAnalysis::new("Impact Analysis");
    let impact = AmendmentImpact::new(
        "amend-1",
        "statute-1",
        "Test Statute",
        "2024-01-15T10:00:00Z",
        "Major amendment",
        5,
        3,
        0.8,
    );
    analysis.add_amendment(impact);
    let html = analysis.to_html();
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("Impact Analysis"));
    assert!(html.contains("Test Statute"));
    assert!(html.contains("Total Amendments"));
    assert!(html.contains("d3.v7.min.js"));
}
#[test]
fn test_amendment_impact_analysis_text_report() {
    let mut analysis = AmendmentImpactAnalysis::new("Impact Analysis");
    let impact = AmendmentImpact::new(
        "amend-1",
        "statute-1",
        "Test Statute",
        "2024-01-15T10:00:00Z",
        "Major amendment",
        5,
        3,
        0.8,
    );
    analysis.add_amendment(impact);
    let report = analysis.to_text_report();
    assert!(report.contains("Impact Analysis"));
    assert!(report.contains("Total Amendments: 1"));
    assert!(report.contains("Test Statute"));
    assert!(report.contains("Sections: 5"));
}
#[test]
fn test_legislative_trend_chart_creation() {
    let chart = LegislativeTrendChart::new("Trend Chart");
    assert_eq!(chart.title, "Trend Chart");
    assert_eq!(chart.data_points.len(), 0);
}
#[test]
fn test_legislative_trend_chart_add_data_point() {
    let mut chart = LegislativeTrendChart::new("Trend Chart");
    chart.add_data_point("2024-Q1", "Enacted", 10.0);
    chart.add_data_point("2024-Q2", "Enacted", 15.0);
    assert_eq!(chart.data_points.len(), 2);
    assert_eq!(chart.data_points[0].period, "2024-Q1");
    assert_eq!(chart.data_points[0].category, "Enacted");
    assert_eq!(chart.data_points[0].value, 10.0);
}
#[test]
fn test_legislative_trend_chart_with_chart_type() {
    let chart = LegislativeTrendChart::new("Test").with_chart_type(ChartType::Bar);
    assert_eq!(chart.title, "Test");
}
#[test]
fn test_legislative_trend_chart_html_line() {
    let mut chart = LegislativeTrendChart::new("Trend Chart").with_chart_type(ChartType::Line);
    chart.add_data_point("2024-Q1", "Enacted", 10.0);
    let html = chart.to_html();
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("Trend Chart"));
    assert!(html.contains("d3.v7.min.js"));
    assert!(html.contains("// Line chart"));
}
#[test]
fn test_legislative_trend_chart_html_bar() {
    let mut chart = LegislativeTrendChart::new("Trend Chart").with_chart_type(ChartType::Bar);
    chart.add_data_point("2024-Q1", "Enacted", 10.0);
    let html = chart.to_html();
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("// Bar chart"));
}
#[test]
fn test_legislative_trend_chart_html_area() {
    let mut chart = LegislativeTrendChart::new("Trend Chart").with_chart_type(ChartType::Area);
    chart.add_data_point("2024-Q1", "Enacted", 10.0);
    let html = chart.to_html();
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("// Area chart"));
}
#[test]
fn test_statute_version_creation() {
    let version = StatuteVersion::new("v1", "1.0", "2024-01-01", "Initial version of the statute");
    assert_eq!(version.version_id, "v1");
    assert_eq!(version.version, "1.0");
    assert_eq!(version.effective_date, "2024-01-01");
    assert_eq!(version.content, "Initial version of the statute");
    assert_eq!(version.sections.len(), 0);
    assert_eq!(version.metadata.len(), 0);
}
#[test]
fn test_statute_version_add_section() {
    let mut version = StatuteVersion::new("v1", "1.0", "2024-01-01", "Test");
    version.add_section("Section 1: Definitions");
    version.add_section("Section 2: Scope");
    assert_eq!(version.sections.len(), 2);
    assert_eq!(version.sections[0], "Section 1: Definitions");
}
#[test]
fn test_statute_version_add_metadata() {
    let mut version = StatuteVersion::new("v1", "1.0", "2024-01-01", "Test");
    version.add_metadata("author", "Legislature");
    version.add_metadata("status", "active");
    assert_eq!(version.metadata.len(), 2);
    assert_eq!(
        version.metadata.get("author"),
        Some(&"Legislature".to_string())
    );
}
#[test]
fn test_historical_comparison_view_creation() {
    let view = HistoricalComparisonView::new("Version Comparison");
    assert_eq!(view.title, "Version Comparison");
    assert_eq!(view.versions.len(), 0);
}
#[test]
fn test_historical_comparison_view_add_version() {
    let mut view = HistoricalComparisonView::new("Version Comparison");
    let version = StatuteVersion::new("v1", "1.0", "2024-01-01", "Initial version");
    view.add_version(version);
    assert_eq!(view.versions.len(), 1);
}
#[test]
fn test_historical_comparison_view_html() {
    let mut view = HistoricalComparisonView::new("Version Comparison");
    let mut v1 = StatuteVersion::new("v1", "1.0", "2024-01-01", "Initial version");
    v1.add_section("Section 1");
    v1.add_metadata("author", "Legislature");
    let mut v2 = StatuteVersion::new("v2", "2.0", "2024-06-01", "Amended version");
    v2.add_section("Section 1");
    v2.add_section("Section 2");
    view.add_version(v1);
    view.add_version(v2);
    let html = view.to_html();
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("Version Comparison"));
    assert!(html.contains("Version 1.0"));
    assert!(html.contains("Version 2.0"));
    assert!(html.contains("Initial version"));
    assert!(html.contains("Amended version"));
    assert!(html.contains("Section 1"));
    assert!(html.contains("Section 2"));
}
#[test]
fn test_historical_comparison_view_mermaid() {
    let mut view = HistoricalComparisonView::new("Version Comparison");
    let v1 = StatuteVersion::new("v1", "1.0", "2024-01-01", "Initial");
    let v2 = StatuteVersion::new("v2", "2.0", "2024-06-01", "Amended");
    view.add_version(v1);
    view.add_version(v2);
    let mermaid = view.to_mermaid();
    assert!(mermaid.contains("graph LR"));
    assert!(mermaid.contains("Version 1.0"));
    assert!(mermaid.contains("Version 2.0"));
    assert!(mermaid.contains("Amended"));
}
#[test]
fn test_statute_change_event_serialization() {
    let event = StatuteChangeEvent::new(
        "evt-1",
        "statute-1",
        "Test Statute",
        "2024-01-15T10:00:00Z",
        "amended",
        "Test",
        "2.0",
    );
    let json = serde_json::to_string(&event).unwrap();
    assert!(json.contains("evt-1"));
    assert!(json.contains("statute-1"));
    assert!(json.contains("Test Statute"));
    assert!(json.contains("amended"));
}
#[test]
fn test_trend_data_point_serialization() {
    let point = TrendDataPoint {
        period: "2024-Q1".to_string(),
        category: "Enacted".to_string(),
        value: 10.0,
    };
    let json = serde_json::to_string(&point).unwrap();
    assert!(json.contains("2024-Q1"));
    assert!(json.contains("Enacted"));
    assert!(json.contains("10"));
}
#[test]
fn test_statute_version_serialization() {
    let version = StatuteVersion::new("v1", "1.0", "2024-01-01", "Test");
    let json = serde_json::to_string(&version).unwrap();
    assert!(json.contains("v1"));
    assert!(json.contains("1.0"));
    assert!(json.contains("2024-01-01"));
}
#[test]
fn test_latex_tikz_exporter_creation() {
    let exporter = LatexTikzExporter::new();
    assert_eq!(exporter.document_class, "article");
    assert!(!exporter.standalone);
}
#[test]
fn test_latex_tikz_exporter_with_document_class() {
    let exporter = LatexTikzExporter::new().with_document_class("beamer");
    assert_eq!(exporter.document_class, "beamer");
}
#[test]
fn test_latex_tikz_exporter_with_standalone() {
    let exporter = LatexTikzExporter::new().with_standalone(true);
    assert!(exporter.standalone);
}
#[test]
fn test_latex_tikz_exporter_export_decision_tree() {
    let exporter = LatexTikzExporter::new();
    let statute = Statute::new(
        "test",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let latex = exporter.export_decision_tree(&tree);
    assert!(latex.contains("\\documentclass"));
    assert!(latex.contains("\\usepackage{tikz}"));
    assert!(latex.contains("\\begin{tikzpicture}"));
    assert!(latex.contains("\\end{tikzpicture}"));
    assert!(latex.contains("\\begin{document}"));
    assert!(latex.contains("\\end{document}"));
}
#[test]
fn test_latex_tikz_exporter_export_dependency_graph() {
    let exporter = LatexTikzExporter::new().with_standalone(true);
    let mut graph = DependencyGraph::new();
    graph.add_statute("test-1");
    graph.add_statute("test-2");
    graph.add_dependency("test-1", "test-2", "references");
    let latex = exporter.export_dependency_graph(&graph);
    assert!(latex.contains("\\documentclass[tikz,border=10pt]{standalone}"));
    assert!(latex.contains("\\graph[spring layout"));
    assert!(latex.contains("test-1"));
    assert!(latex.contains("test-2"));
}
#[test]
fn test_latex_tikz_exporter_default() {
    let exporter = LatexTikzExporter::default();
    assert_eq!(exporter.document_class, "article");
}
#[test]
fn test_graphml_exporter_creation() {
    let exporter = GraphMLExporter::new();
    assert!(exporter.include_visuals);
}
#[test]
fn test_graphml_exporter_with_visuals() {
    let exporter = GraphMLExporter::new().with_visuals(false);
    assert!(!exporter.include_visuals);
}
#[test]
fn test_graphml_exporter_export_graph() {
    let exporter = GraphMLExporter::new();
    let mut graph = DependencyGraph::new();
    graph.add_statute("statute-a");
    graph.add_statute("statute-b");
    graph.add_dependency("statute-a", "statute-b", "depends_on");
    let xml = exporter.export_graph(&graph);
    assert!(xml.contains("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
    assert!(xml.contains("<graphml"));
    assert!(xml.contains("<node id="));
    assert!(xml.contains("statute-a"));
    assert!(xml.contains("statute-b"));
    assert!(xml.contains("<key id=\"d0\""));
    assert!(xml.contains("<key id=\"d1\""));
    assert!(xml.contains("<key id=\"d2\""));
    assert!(xml.contains("d3"));
}
#[test]
fn test_graphml_exporter_export_graph_no_visuals() {
    let exporter = GraphMLExporter::new().with_visuals(false);
    let mut graph = DependencyGraph::new();
    graph.add_statute("test");
    let xml = exporter.export_graph(&graph);
    assert!(xml.contains("<graphml"));
    assert!(!xml.contains("d3"));
    assert!(!xml.contains("d4"));
}
#[test]
fn test_graphml_exporter_export_decision_tree() {
    let exporter = GraphMLExporter::new();
    let statute = Statute::new(
        "test",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let xml = exporter.export_decision_tree(&tree);
    assert!(xml.contains("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
    assert!(xml.contains("<graphml"));
    assert!(xml.contains("<node id=\"n0\">"));
    assert!(xml.contains("Decision Tree"));
}
#[test]
fn test_graphml_exporter_default() {
    let exporter = GraphMLExporter::default();
    assert!(exporter.include_visuals);
}
#[test]
fn test_cypher_exporter_creation() {
    let exporter = CypherExporter::new();
    assert!(exporter.include_indexes);
    assert!(!exporter.use_merge);
}
#[test]
fn test_cypher_exporter_with_indexes() {
    let exporter = CypherExporter::new().with_indexes(false);
    assert!(!exporter.include_indexes);
}
#[test]
fn test_cypher_exporter_with_merge() {
    let exporter = CypherExporter::new().with_merge(true);
    assert!(exporter.use_merge);
}
#[test]
fn test_cypher_exporter_export_graph() {
    let exporter = CypherExporter::new();
    let mut graph = DependencyGraph::new();
    graph.add_statute("statute-1");
    graph.add_statute("statute-2");
    graph.add_dependency("statute-1", "statute-2", "depends_on");
    let cypher = exporter.export_graph(&graph);
    assert!(cypher.contains("// Neo4j Cypher Query Export"));
    assert!(cypher.contains("CREATE INDEX statute_id"));
    assert!(cypher.contains("CREATE (s_statute_1:Statute"));
    assert!(cypher.contains("CREATE (s_statute_2:Statute"));
    assert!(cypher.contains("DEPENDS_ON"));
    assert!(cypher.contains("MATCH (s:Statute) RETURN s"));
}
#[test]
fn test_cypher_exporter_export_graph_with_merge() {
    let exporter = CypherExporter::new().with_merge(true).with_indexes(false);
    let mut graph = DependencyGraph::new();
    graph.add_statute("test");
    let cypher = exporter.export_graph(&graph);
    assert!(cypher.contains("MERGE (s_test:Statute"));
    assert!(!cypher.contains("CREATE INDEX"));
}
#[test]
fn test_cypher_exporter_export_concept_graph() {
    let exporter = CypherExporter::new();
    let mut graph = ConceptRelationshipGraph::new("Test Graph");
    let c1 = LegalConcept::new("c1", "Privacy", "Privacy concept", "rights");
    let c2 = LegalConcept::new("c2", "Consent", "Consent concept", "rights");
    graph.add_concept(c1);
    graph.add_concept(c2);
    graph.add_relationship(ConceptRelationship::new(
        "c1",
        "c2",
        ConceptRelationType::Requires,
    ));
    let cypher = exporter.export_concept_graph(&graph);
    assert!(cypher.contains("// Neo4j Cypher Query Export - Legal Concepts"));
    assert!(cypher.contains("CREATE INDEX concept_id"));
    assert!(cypher.contains("(c_c1:Concept"));
    assert!(cypher.contains("Privacy"));
    assert!(cypher.contains("Consent"));
    assert!(cypher.contains("REQUIRES"));
}
#[test]
fn test_cypher_exporter_default() {
    let exporter = CypherExporter::default();
    assert!(exporter.include_indexes);
}
#[test]
fn test_sparql_exporter_creation() {
    let exporter = SparqlExporter::new();
    assert_eq!(exporter.base_uri, "http://example.org/legalis/");
    assert!(exporter.include_prefixes);
}
#[test]
fn test_sparql_exporter_with_base_uri() {
    let exporter = SparqlExporter::new().with_base_uri("http://custom.org/");
    assert_eq!(exporter.base_uri, "http://custom.org/");
}
#[test]
fn test_sparql_exporter_with_prefixes() {
    let exporter = SparqlExporter::new().with_prefixes(false);
    assert!(!exporter.include_prefixes);
}
#[test]
fn test_sparql_exporter_export_graph() {
    let exporter = SparqlExporter::new();
    let mut graph = DependencyGraph::new();
    graph.add_statute("statute-a");
    graph.add_statute("statute-b");
    graph.add_dependency("statute-a", "statute-b", "depends_on");
    let sparql = exporter.export_graph(&graph);
    assert!(sparql.contains("# SPARQL INSERT Queries"));
    assert!(sparql.contains("PREFIX leg:"));
    assert!(sparql.contains("PREFIX rdf:"));
    assert!(sparql.contains("PREFIX rdfs:"));
    assert!(sparql.contains("INSERT DATA {"));
    assert!(sparql.contains("rdf:type leg:Statute"));
    assert!(sparql.contains("statute-a"));
    assert!(sparql.contains("statute-b"));
    assert!(sparql.contains("leg:dependsOn"));
}
