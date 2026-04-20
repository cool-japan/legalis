#![cfg(test)]
use super::*;
use legalis_core::{Effect, EffectType};

#[test]
fn test_to_print_pdf() {
    let statute = Statute::new(
        "test-1",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let exporter = AdvancedExporter::new();
    let config = PdfConfig::a4().print_optimized();
    let svg = exporter.to_print_pdf(&tree, config);
    assert!(svg.contains("<svg"));
    assert!(svg.contains("@media print"));
}
#[test]
fn test_graph_to_print_pdf() {
    let mut graph = DependencyGraph::new();
    graph.add_statute("statute-1");
    let exporter = AdvancedExporter::new();
    let config = PdfConfig::letter().print_optimized();
    let svg = exporter.graph_to_print_pdf(&graph, config);
    assert!(svg.contains("<svg"));
    assert!(svg.contains("@media print"));
}
#[test]
fn test_to_vector_pdf() {
    let statute = Statute::new(
        "test-1",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let exporter = AdvancedExporter::new();
    let config = PdfConfig::a4().vector();
    let svg = exporter.to_vector_pdf(&tree, config);
    assert!(svg.contains("<svg"));
    assert!(svg.contains("PDF Export"));
}
#[test]
fn test_graph_to_vector_pdf() {
    let mut graph = DependencyGraph::new();
    graph.add_statute("statute-1");
    let exporter = AdvancedExporter::new();
    let config = PdfConfig::a3().vector();
    let svg = exporter.graph_to_vector_pdf(&graph, config);
    assert!(svg.contains("<svg"));
    assert!(svg.contains("PDF Export"));
}
#[test]
fn test_to_poster() {
    let statute = Statute::new(
        "test-1",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let exporter = AdvancedExporter::new();
    let config = PosterConfig::a0();
    let svg = exporter.to_poster(&tree, config);
    assert!(svg.contains("<svg"));
    assert!(svg.contains("Poster"));
    assert!(svg.contains("A0"));
}
#[test]
fn test_graph_to_poster() {
    let mut graph = DependencyGraph::new();
    graph.add_statute("statute-1");
    let exporter = AdvancedExporter::new();
    let config = PosterConfig::poster_24x36().landscape();
    let svg = exporter.graph_to_poster(&graph, config);
    assert!(svg.contains("<svg"));
    assert!(svg.contains("Poster"));
    assert!(svg.contains("24x36"));
}
#[test]
fn test_format_metadata() {
    let exporter = AdvancedExporter::new();
    let gif_meta = exporter.format_metadata(ExportFormat::AnimatedGif);
    assert!(gif_meta.contains("Animated GIF"));
    let mp4_meta = exporter.format_metadata(ExportFormat::Mp4);
    assert!(mp4_meta.contains("MP4"));
    let webm_meta = exporter.format_metadata(ExportFormat::WebM);
    assert!(webm_meta.contains("WebM"));
    let print_pdf_meta = exporter.format_metadata(ExportFormat::PrintPdf);
    assert!(print_pdf_meta.contains("Print PDF"));
    let vector_pdf_meta = exporter.format_metadata(ExportFormat::VectorPdf);
    assert!(vector_pdf_meta.contains("Vector PDF"));
    let poster_meta = exporter.format_metadata(ExportFormat::Poster);
    assert!(poster_meta.contains("Poster"));
}
#[test]
fn test_animated_gif_config_builder_pattern() {
    let config = AnimatedGifConfig::new()
        .with_fps(60)
        .with_duration(5)
        .with_loop_count(3)
        .with_size(1280, 720)
        .with_quality(95);
    assert_eq!(config.fps, 60);
    assert_eq!(config.duration, 5);
    assert_eq!(config.loop_count, 3);
    assert_eq!(config.width, 1280);
    assert_eq!(config.height, 720);
    assert_eq!(config.quality, 95);
}
#[test]
fn test_video_config_builder_pattern() {
    let config = VideoConfig::hd_1080p()
        .with_codec("vp9")
        .with_bitrate(15000)
        .with_duration(30);
    assert_eq!(config.codec, "vp9");
    assert_eq!(config.bitrate, 15000);
    assert_eq!(config.duration, 30);
}
#[test]
fn test_pdf_config_builder_pattern() {
    let config = PdfConfig::a4()
        .landscape()
        .vector()
        .print_optimized()
        .with_dpi(600)
        .with_margin(15.0);
    assert_eq!(config.width, 297.0);
    assert_eq!(config.height, 210.0);
    assert!(config.vector);
    assert!(config.print_optimized);
    assert_eq!(config.dpi, 600);
    assert_eq!(config.margin, 15.0);
}
#[test]
fn test_poster_config_builder_pattern() {
    let config = PosterConfig::a1().landscape().with_dpi(450);
    assert_eq!(config.width, 841);
    assert_eq!(config.height, 594);
    assert_eq!(config.orientation, "landscape");
    assert_eq!(config.dpi, 450);
}
#[test]
fn test_streaming_data_source_creation() {
    let source = StreamingDataSource::new("test-source", "ws://localhost:8080", 1000);
    assert_eq!(source.source_id, "test-source");
    assert_eq!(source.stream_url, "ws://localhost:8080");
    assert_eq!(source.update_frequency_ms, 1000);
    assert_eq!(source.buffer_size, 1000);
}
#[test]
fn test_streaming_data_source_buffer() {
    let mut source = StreamingDataSource::new("test", "ws://localhost:8080", 1000);
    source.push_data("data1".to_string());
    source.push_data("data2".to_string());
    assert_eq!(source.buffer().len(), 2);
    source.clear_buffer();
    assert_eq!(source.buffer().len(), 0);
}
#[test]
fn test_streaming_data_source_buffer_limit() {
    let mut source =
        StreamingDataSource::new("test", "ws://localhost:8080", 1000).with_buffer_size(2);
    source.push_data("data1".to_string());
    source.push_data("data2".to_string());
    source.push_data("data3".to_string());
    assert_eq!(source.buffer().len(), 2);
    assert_eq!(source.buffer()[0], "data2");
    assert_eq!(source.buffer()[1], "data3");
}
#[test]
fn test_streaming_data_source_javascript() {
    let source = StreamingDataSource::new("test-source", "ws://localhost:8080", 1000);
    let js = source.to_javascript();
    assert!(js.contains("class StreamingDataSource"));
    assert!(js.contains("test-source"));
    assert!(js.contains("ws://localhost:8080"));
}
#[test]
fn test_collaborative_user_creation() {
    let user = CollaborativeUser::new("user1", "Alice", "#ff0000");
    assert_eq!(user.user_id, "user1");
    assert_eq!(user.display_name, "Alice");
    assert_eq!(user.color, "#ff0000");
    assert!(user.active);
}
#[test]
fn test_cursor_position_creation() {
    let user = CollaborativeUser::new("user1", "Alice", "#ff0000");
    let cursor = CursorPosition::new(user.clone(), 50.0, 75.0, 1234567890);
    assert_eq!(cursor.user.user_id, "user1");
    assert_eq!(cursor.x, 50.0);
    assert_eq!(cursor.y, 75.0);
    assert_eq!(cursor.timestamp, 1234567890);
}
#[test]
fn test_shared_annotation_creation() {
    let user = CollaborativeUser::new("user1", "Alice", "#ff0000");
    let annotation = SharedAnnotation::new(
        "annot1",
        user.clone(),
        "node-123",
        "This is a comment",
        1234567890,
    );
    assert_eq!(annotation.annotation_id, "annot1");
    assert_eq!(annotation.user.user_id, "user1");
    assert_eq!(annotation.target_id, "node-123");
    assert_eq!(annotation.content, "This is a comment");
    assert!(!annotation.resolved);
}
#[test]
fn test_shared_annotation_resolve() {
    let user = CollaborativeUser::new("user1", "Alice", "#ff0000");
    let mut annotation =
        SharedAnnotation::new("annot1", user, "node-123", "This is a comment", 1234567890);
    annotation.resolve();
    assert!(annotation.resolved);
}
#[test]
fn test_collaborative_session_creation() {
    let session = CollaborativeSession::new("session1", "ws://localhost:8080");
    assert_eq!(session.session_id, "session1");
    assert_eq!(session.websocket_url, "ws://localhost:8080");
    assert_eq!(session.active_users().len(), 0);
    assert_eq!(session.cursors().len(), 0);
    assert_eq!(session.annotations().len(), 0);
}
#[test]
fn test_collaborative_session_add_user() {
    let mut session = CollaborativeSession::new("session1", "ws://localhost:8080");
    let user = CollaborativeUser::new("user1", "Alice", "#ff0000");
    session.add_user(user.clone());
    assert_eq!(session.active_users().len(), 1);
    session.add_user(user);
    assert_eq!(session.active_users().len(), 1);
}
#[test]
fn test_collaborative_session_remove_user() {
    let mut session = CollaborativeSession::new("session1", "ws://localhost:8080");
    let user = CollaborativeUser::new("user1", "Alice", "#ff0000");
    session.add_user(user.clone());
    assert_eq!(session.active_users().len(), 1);
    session.remove_user("user1");
    assert_eq!(session.active_users().len(), 0);
}
#[test]
fn test_collaborative_session_update_cursor() {
    let mut session = CollaborativeSession::new("session1", "ws://localhost:8080");
    let user = CollaborativeUser::new("user1", "Alice", "#ff0000");
    let cursor = CursorPosition::new(user.clone(), 50.0, 75.0, 1234567890);
    session.update_cursor(cursor.clone());
    assert_eq!(session.cursors().len(), 1);
    let cursor2 = CursorPosition::new(user, 60.0, 80.0, 1234567891);
    session.update_cursor(cursor2);
    assert_eq!(session.cursors().len(), 1);
    assert_eq!(session.cursors()[0].x, 60.0);
}
#[test]
fn test_collaborative_session_add_annotation() {
    let mut session = CollaborativeSession::new("session1", "ws://localhost:8080");
    let user = CollaborativeUser::new("user1", "Alice", "#ff0000");
    let annotation =
        SharedAnnotation::new("annot1", user, "node-123", "This is a comment", 1234567890);
    session.add_annotation(annotation);
    assert_eq!(session.annotations().len(), 1);
}
#[test]
fn test_collaborative_session_remove_annotation() {
    let mut session = CollaborativeSession::new("session1", "ws://localhost:8080");
    let user = CollaborativeUser::new("user1", "Alice", "#ff0000");
    let annotation =
        SharedAnnotation::new("annot1", user, "node-123", "This is a comment", 1234567890);
    session.add_annotation(annotation);
    assert_eq!(session.annotations().len(), 1);
    session.remove_annotation("annot1");
    assert_eq!(session.annotations().len(), 0);
}
#[test]
fn test_collaborative_session_html_generation() {
    let statute = Statute::new(
        "test-1",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let session = CollaborativeSession::new("session1", "ws://localhost:8080");
    let html = session.to_collaborative_html(&tree);
    assert!(html.contains("Collaborative Visualization"));
    assert!(html.contains("session1"));
    assert!(html.contains("ws://localhost:8080"));
    assert!(html.contains("connectWebSocket"));
    assert!(html.contains("updateCursor"));
    assert!(html.contains("addAnnotation"));
}
#[test]
fn test_custom_theme_builder_creation() {
    let builder = CustomThemeBuilder::new();
    let theme = builder.build();
    assert_eq!(theme.background_color, Theme::default().background_color);
}
#[test]
fn test_custom_theme_builder_with_colors() {
    let theme = CustomThemeBuilder::new()
        .with_background_color("#000000")
        .with_text_color("#ffffff")
        .with_condition_color("#0000ff")
        .with_outcome_color("#00ff00")
        .with_discretion_color("#ff0000")
        .with_link_color("#ffff00")
        .with_root_color("#cccccc")
        .build();
    assert_eq!(theme.background_color, "#000000");
    assert_eq!(theme.text_color, "#ffffff");
    assert_eq!(theme.condition_color, "#0000ff");
    assert_eq!(theme.outcome_color, "#00ff00");
    assert_eq!(theme.discretion_color, "#ff0000");
    assert_eq!(theme.link_color, "#ffff00");
    assert_eq!(theme.root_color, "#cccccc");
}
#[test]
fn test_custom_theme_builder_with_branding() {
    let theme = CustomThemeBuilder::new()
        .with_branding("#ff0000", "#0000ff")
        .build();
    assert_eq!(theme.condition_color, "#ff0000");
    assert_eq!(theme.outcome_color, "#0000ff");
    assert_eq!(theme.link_color, "#ff0000");
}
#[test]
fn test_custom_theme_builder_with_palette() {
    let theme = CustomThemeBuilder::new()
        .with_palette("#ffffff", "#000000", "#ff0000", "#00ff00", "#0000ff")
        .build();
    assert_eq!(theme.background_color, "#ffffff");
    assert_eq!(theme.text_color, "#000000");
    assert_eq!(theme.condition_color, "#ff0000");
    assert_eq!(theme.outcome_color, "#00ff00");
    assert_eq!(theme.discretion_color, "#0000ff");
    assert_eq!(theme.link_color, "#ff0000");
}
#[test]
fn test_custom_theme_builder_from_theme() {
    let dark_theme = Theme::dark();
    let custom = CustomThemeBuilder::from_theme(dark_theme.clone())
        .with_condition_color("#123456")
        .build();
    assert_eq!(custom.background_color, dark_theme.background_color);
    assert_eq!(custom.condition_color, "#123456");
}
#[test]
fn test_custom_theme_builder_to_json() {
    let builder = CustomThemeBuilder::new()
        .with_background_color("#ffffff")
        .with_text_color("#000000");
    let json = builder.to_json().unwrap();
    assert!(json.contains("background_color"));
    assert!(json.contains("#ffffff"));
    assert!(json.contains("text_color"));
    assert!(json.contains("#000000"));
}
#[test]
fn test_custom_theme_builder_from_json() {
    let json = r##"{
        "root_color": "#f0f0f0",
        "condition_color": "#e1f5fe",
        "discretion_color": "#ffcdd2",
        "outcome_color": "#c8e6c9",
        "link_color": "#ccc",
        "background_color": "#ffffff",
        "text_color": "#333333"
    }"##;
    let builder = CustomThemeBuilder::from_json(json).unwrap();
    let theme = builder.build();
    assert_eq!(theme.background_color, "#ffffff");
    assert_eq!(theme.text_color, "#333333");
}
#[test]
fn test_custom_theme_builder_from_json_invalid() {
    let json = r##"{ "invalid": "json" }"##;
    let result = CustomThemeBuilder::from_json(json);
    assert!(result.is_err());
}
#[test]
fn test_custom_theme_builder_default() {
    let builder = CustomThemeBuilder::default();
    let theme = builder.build();
    assert_eq!(theme.background_color, Theme::default().background_color);
}
#[test]
fn test_seasonal_theme_winter() {
    let theme = SeasonalThemes::winter();
    assert!(theme.background_color.contains("f0f8ff"));
    assert!(theme.link_color.contains("668db8"));
}
#[test]
fn test_seasonal_theme_spring() {
    let theme = SeasonalThemes::spring();
    assert!(theme.background_color.contains("f1f8e9"));
    assert!(theme.link_color.contains("81c784"));
}
#[test]
fn test_seasonal_theme_summer() {
    let theme = SeasonalThemes::summer();
    assert!(theme.background_color.contains("fffaf0"));
    assert!(theme.link_color.contains("ff9800"));
}
#[test]
fn test_seasonal_theme_autumn() {
    let theme = SeasonalThemes::autumn();
    assert!(theme.background_color.contains("fff8f5"));
    assert!(theme.link_color.contains("8d6e63"));
}
#[test]
fn test_seasonal_theme_holiday() {
    let theme = SeasonalThemes::holiday();
    assert_eq!(theme.background_color, "#fafafa");
    assert_eq!(theme.link_color, "#c62828");
}
#[test]
fn test_seasonal_theme_corporate() {
    let theme = SeasonalThemes::corporate();
    assert_eq!(theme.background_color, "#fafafa");
    assert_eq!(theme.link_color, "#455a64");
}
#[test]
fn test_seasonal_theme_academic() {
    let theme = SeasonalThemes::academic();
    assert_eq!(theme.background_color, "#fafafa");
    assert_eq!(theme.link_color, "#1976d2");
}
#[test]
fn test_seasonal_theme_legal() {
    let theme = SeasonalThemes::legal();
    assert_eq!(theme.background_color, "#ffffff");
    assert_eq!(theme.link_color, "#1a237e");
    assert_eq!(theme.text_color, "#000000");
}
#[test]
fn test_css_variable_theme_creation() {
    let css_theme = CssVariableTheme::new()
        .add_variable("--primary-color", "#ff0000")
        .add_variable("--secondary-color", "#00ff00");
    assert_eq!(css_theme.variables().len(), 2);
    assert_eq!(css_theme.variables()[0].0, "--primary-color");
    assert_eq!(css_theme.variables()[0].1, "#ff0000");
}
#[test]
fn test_css_variable_theme_from_theme() {
    let theme = Theme::dark();
    let css_theme = CssVariableTheme::from_theme(&theme);
    assert_eq!(css_theme.variables().len(), 7);
    let vars: Vec<&String> = css_theme.variables().iter().map(|(name, _)| name).collect();
    assert!(vars.contains(&&"--viz-root-color".to_string()));
    assert!(vars.contains(&&"--viz-condition-color".to_string()));
}
#[test]
fn test_css_variable_theme_to_css() {
    let css_theme = CssVariableTheme::new()
        .add_variable("--primary-color", "#ff0000")
        .add_variable("--secondary-color", "#00ff00");
    let css = css_theme.to_css();
    assert!(css.contains(":root {"));
    assert!(css.contains("--primary-color: #ff0000;"));
    assert!(css.contains("--secondary-color: #00ff00;"));
}
#[test]
fn test_css_variable_theme_to_css_with_selector() {
    let css_theme = CssVariableTheme::new().add_variable("--primary-color", "#ff0000");
    let css = css_theme.to_css_with_selector(".dark-theme");
    assert!(css.contains(".dark-theme {"));
    assert!(css.contains("--primary-color: #ff0000;"));
}
#[test]
fn test_css_variable_theme_default() {
    let css_theme = CssVariableTheme::default();
    assert_eq!(css_theme.variables().len(), 0);
}
#[test]
fn test_virtualization_config_creation() {
    let config = VirtualizationConfig::new();
    assert!(config.enabled);
    assert_eq!(config.render_batch_size, 100);
    assert_eq!(config.buffer_size, 20);
    assert_eq!(config.min_item_height, 50);
    assert!(!config.dynamic_height);
}
#[test]
fn test_virtualization_config_disabled() {
    let config = VirtualizationConfig::disabled();
    assert!(!config.enabled);
}
#[test]
fn test_virtualization_config_builder() {
    let config = VirtualizationConfig::new()
        .with_batch_size(200)
        .with_buffer_size(30)
        .with_dynamic_height();
    assert_eq!(config.render_batch_size, 200);
    assert_eq!(config.buffer_size, 30);
    assert!(config.dynamic_height);
}
#[test]
fn test_virtualization_config_javascript() {
    let config = VirtualizationConfig::new();
    let js = config.to_javascript();
    assert!(js.contains("class VirtualScroller"));
    assert!(js.contains("renderBatchSize"));
    assert!(js.contains("onScroll"));
}
#[test]
fn test_virtualization_config_javascript_disabled() {
    let config = VirtualizationConfig::disabled();
    let js = config.to_javascript();
    assert_eq!(js, "");
}
#[test]
fn test_virtualization_config_default() {
    let config = VirtualizationConfig::default();
    assert!(config.enabled);
    assert_eq!(config.render_batch_size, 100);
}
#[test]
fn test_progressive_loading_config_creation() {
    let config = ProgressiveLoadingConfig::new();
    assert!(config.enabled);
    assert_eq!(config.initial_load, 50);
    assert_eq!(config.load_increment, 25);
    assert!(config.show_loading_indicator);
    assert_eq!(config.load_delay_ms, 200);
}
#[test]
fn test_progressive_loading_config_builder() {
    let config = ProgressiveLoadingConfig::new()
        .with_initial_load(100)
        .with_load_increment(50)
        .without_loading_indicator();
    assert_eq!(config.initial_load, 100);
    assert_eq!(config.load_increment, 50);
    assert!(!config.show_loading_indicator);
}
#[test]
fn test_progressive_loading_config_javascript() {
    let config = ProgressiveLoadingConfig::new();
    let js = config.to_javascript();
    assert!(js.contains("class ProgressiveLoader"));
    assert!(js.contains("loadMore"));
    assert!(js.contains("checkScroll"));
}
#[test]
fn test_progressive_loading_config_default() {
    let config = ProgressiveLoadingConfig::default();
    assert!(config.enabled);
    assert_eq!(config.initial_load, 50);
}
#[test]
fn test_level_of_detail_config_creation() {
    let config = LevelOfDetailConfig::new();
    assert!(config.enabled);
    assert_eq!(config.zoom_thresholds.len(), 4);
    assert!(config.simplify_at_low_zoom);
    assert!(config.hide_labels_at_low_zoom);
    assert!(config.aggregate_nodes);
}
#[test]
fn test_level_of_detail_config_disabled() {
    let config = LevelOfDetailConfig::disabled();
    assert!(!config.enabled);
}
#[test]
fn test_level_of_detail_config_custom_thresholds() {
    let config = LevelOfDetailConfig::new().with_zoom_thresholds(vec![0.1, 0.5, 1.0]);
    assert_eq!(config.zoom_thresholds.len(), 3);
    assert_eq!(config.zoom_thresholds[0], 0.1);
    assert_eq!(config.zoom_thresholds[1], 0.5);
    assert_eq!(config.zoom_thresholds[2], 1.0);
}
#[test]
fn test_level_of_detail_config_javascript() {
    let config = LevelOfDetailConfig::new();
    let js = config.to_javascript();
    assert!(js.contains("class LevelOfDetailRenderer"));
    assert!(js.contains("updateDetailLevel"));
    assert!(js.contains("applyDetailLevel"));
}
#[test]
fn test_level_of_detail_config_javascript_disabled() {
    let config = LevelOfDetailConfig::disabled();
    let js = config.to_javascript();
    assert_eq!(js, "");
}
#[test]
fn test_level_of_detail_config_default() {
    let config = LevelOfDetailConfig::default();
    assert!(config.enabled);
    assert_eq!(config.zoom_thresholds.len(), 4);
}
#[test]
fn test_vr_exploration_config_default() {
    let config = VRExplorationConfig::default();
    assert!(config.enable_hand_tracking);
    assert!(config.enable_teleportation);
    assert!(!config.enable_voice_commands);
    assert!(config.enable_spatial_audio);
    assert!(config.enable_haptic_feedback);
    assert_eq!(config.interaction_distance, 2.0);
    assert_eq!(config.movement_speed, 1.0);
}
#[test]
fn test_vr_statute_explorer_creation() {
    let explorer = VRStatuteExplorer::new();
    assert_eq!(explorer.theme.background_color, "#ffffff");
    assert!(explorer.config.enable_hand_tracking);
}
#[test]
fn test_vr_statute_explorer_with_theme() {
    let explorer = VRStatuteExplorer::new().with_theme(Theme::dark());
    assert_eq!(explorer.theme.background_color, "#1a1a1a");
}
#[test]
fn test_vr_statute_explorer_with_config() {
    let config = VRExplorationConfig {
        enable_hand_tracking: false,
        enable_spatial_audio: false,
        ..Default::default()
    };
    let explorer = VRStatuteExplorer::new().with_config(config);
    assert!(!explorer.config.enable_hand_tracking);
    assert!(!explorer.config.enable_spatial_audio);
}
#[test]
fn test_vr_statute_explorer_html_generation() {
    let statute = Statute::new(
        "test-1",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let explorer = VRStatuteExplorer::new();
    let html = explorer.to_vr_html(&statute);
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("VR Statute Explorer"));
    assert!(html.contains("ENTER VR"));
    assert!(html.contains("renderer.xr.enabled = true"));
    assert!(html.contains("navigator.xr.requestSession"));
    assert!(html.contains("immersive-vr"));
}
#[test]
fn test_vr_statute_explorer_spatial_audio() {
    let statute = Statute::new(
        "test-1",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let explorer = VRStatuteExplorer::new();
    let html = explorer.to_vr_html(&statute);
    assert!(html.contains("setupSpatialAudio"));
    assert!(html.contains("AudioContext"));
    assert!(html.contains("PositionalAudio"));
}
#[test]
fn test_vr_statute_explorer_haptic_feedback() {
    let statute = Statute::new(
        "test-1",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let explorer = VRStatuteExplorer::new();
    let html = explorer.to_vr_html(&statute);
    assert!(html.contains("hapticActuators"));
    assert!(html.contains("pulse"));
}
#[test]
fn test_vr_statute_explorer_default() {
    let explorer1 = VRStatuteExplorer::new();
    let explorer2 = VRStatuteExplorer::default();
    assert_eq!(
        explorer1.theme.background_color,
        explorer2.theme.background_color
    );
}
#[test]
fn test_ar_overlay_config_default() {
    let config = AROverlayConfig::default();
    assert!(config.enable_markers);
    assert!(config.enable_markerless);
    assert!(!config.enable_face_tracking);
    assert_eq!(config.marker_size, 0.15);
    assert_eq!(config.overlay_opacity, 0.9);
}
#[test]
fn test_ar_document_overlay_creation() {
    let overlay = ARDocumentOverlay::new();
    assert_eq!(overlay.theme.background_color, "#ffffff");
    assert!(overlay.config.enable_markers);
}
#[test]
fn test_ar_document_overlay_with_theme() {
    let overlay = ARDocumentOverlay::new().with_theme(Theme::dark());
    assert_eq!(overlay.theme.background_color, "#1a1a1a");
}
#[test]
fn test_ar_document_overlay_with_config() {
    let config = AROverlayConfig {
        enable_markers: false,
        enable_markerless: false,
        enable_face_tracking: true,
        marker_size: 0.2,
        overlay_opacity: 0.5,
    };
    let overlay = ARDocumentOverlay::new().with_config(config);
    assert!(!overlay.config.enable_markers);
    assert!(overlay.config.enable_face_tracking);
    assert_eq!(overlay.config.overlay_opacity, 0.5);
}
#[test]
fn test_ar_document_overlay_html_generation() {
    let statute = Statute::new(
        "test-1",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let overlay = ARDocumentOverlay::new();
    let html = overlay.to_ar_html(&statute);
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("AR Document Overlay"));
    assert!(html.contains("start-ar"));
    assert!(html.contains("camera-feed"));
    assert!(html.contains("getUserMedia"));
    assert!(html.contains("immersive-ar"));
}
#[test]
fn test_ar_document_overlay_camera_access() {
    let statute = Statute::new(
        "test-1",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let overlay = ARDocumentOverlay::new();
    let html = overlay.to_ar_html(&statute);
    assert!(html.contains("navigator.mediaDevices.getUserMedia"));
    assert!(html.contains("facingMode: 'environment'"));
}
#[test]
fn test_ar_document_overlay_default() {
    let overlay1 = ARDocumentOverlay::new();
    let overlay2 = ARDocumentOverlay::default();
    assert_eq!(
        overlay1.theme.background_color,
        overlay2.theme.background_color
    );
}
#[test]
fn test_panoramic_360_config_default() {
    let config = Panoramic360Config::default();
    assert!(config.enable_vr_mode);
    assert!(!config.enable_auto_rotation);
    assert_eq!(config.rotation_speed, 10.0);
    assert_eq!(config.field_of_view, 75.0);
    assert!(config.enable_gyroscope);
}
#[test]
fn test_panoramic_360_timeline_creation() {
    let timeline = Panoramic360Timeline::new();
    assert_eq!(timeline.theme.background_color, "#ffffff");
    assert!(timeline.config.enable_vr_mode);
}
#[test]
fn test_panoramic_360_timeline_with_theme() {
    let timeline = Panoramic360Timeline::new().with_theme(Theme::dark());
    assert_eq!(timeline.theme.background_color, "#1a1a1a");
}
#[test]
fn test_panoramic_360_timeline_with_config() {
    let config = Panoramic360Config {
        enable_vr_mode: false,
        enable_auto_rotation: true,
        rotation_speed: 20.0,
        field_of_view: 90.0,
        enable_gyroscope: false,
    };
    let timeline = Panoramic360Timeline::new().with_config(config);
    assert!(!timeline.config.enable_vr_mode);
    assert!(timeline.config.enable_auto_rotation);
    assert_eq!(timeline.config.rotation_speed, 20.0);
}
#[test]
fn test_panoramic_360_timeline_html_generation() {
    let mut timeline_data = Timeline::new();
    timeline_data.add_event(
        "2024-01-01",
        TimelineEvent::Enacted {
            statute_id: "statute-1".to_string(),
            title: "Test Statute".to_string(),
        },
    );
    let timeline = Panoramic360Timeline::new();
    let html = timeline.to_360_html(&timeline_data);
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("360° Case Timeline"));
    assert!(html.contains("SphereGeometry"));
    assert!(html.contains("BackSide"));
    assert!(html.contains("DeviceOrientationEvent"));
}
#[test]
fn test_panoramic_360_timeline_gyroscope_support() {
    let mut timeline_data = Timeline::new();
    timeline_data.add_event(
        "2024-01-01",
        TimelineEvent::Enacted {
            statute_id: "statute-1".to_string(),
            title: "Test Statute".to_string(),
        },
    );
    let timeline = Panoramic360Timeline::new();
    let html = timeline.to_360_html(&timeline_data);
    assert!(html.contains("deviceorientation"));
    assert!(html.contains("event.alpha"));
    assert!(html.contains("event.beta"));
    assert!(html.contains("event.gamma"));
}
#[test]
fn test_panoramic_360_timeline_vr_mode() {
    let mut timeline_data = Timeline::new();
    timeline_data.add_event(
        "2024-01-01",
        TimelineEvent::Enacted {
            statute_id: "statute-1".to_string(),
            title: "Test Statute".to_string(),
        },
    );
    let config = Panoramic360Config {
        enable_vr_mode: true,
        ..Default::default()
    };
    let timeline = Panoramic360Timeline::new().with_config(config);
    let html = timeline.to_360_html(&timeline_data);
    assert!(html.contains("enter-vr"));
    assert!(html.contains("renderer.xr.enabled = true"));
}
#[test]
fn test_panoramic_360_timeline_auto_rotation() {
    let mut timeline_data = Timeline::new();
    timeline_data.add_event(
        "2024-01-01",
        TimelineEvent::Enacted {
            statute_id: "statute-1".to_string(),
            title: "Test Statute".to_string(),
        },
    );
    let config = Panoramic360Config {
        enable_auto_rotation: true,
        rotation_speed: 15.0,
        ..Default::default()
    };
    let timeline = Panoramic360Timeline::new().with_config(config);
    let html = timeline.to_360_html(&timeline_data);
    assert!(html.contains("enableAutoRotation: true"));
    assert!(html.contains("rotationSpeed: 15"));
    assert!(html.contains("toggle-rotation"));
}
#[test]
fn test_panoramic_360_timeline_default() {
    let timeline1 = Panoramic360Timeline::new();
    let timeline2 = Panoramic360Timeline::default();
    assert_eq!(
        timeline1.theme.background_color,
        timeline2.theme.background_color
    );
}
#[test]
fn test_panoramic_360_timeline_event_extraction() {
    let mut timeline_data = Timeline::new();
    timeline_data.add_event(
        "2024-01-01",
        TimelineEvent::Enacted {
            statute_id: "statute-1".to_string(),
            title: "Test Statute".to_string(),
        },
    );
    timeline_data.add_event(
        "2024-02-01",
        TimelineEvent::Amended {
            statute_id: "statute-1".to_string(),
            description: "First amendment".to_string(),
        },
    );
    timeline_data.add_event(
        "2024-03-01",
        TimelineEvent::Repealed {
            statute_id: "statute-1".to_string(),
        },
    );
    let timeline = Panoramic360Timeline::new();
    let html = timeline.to_360_html(&timeline_data);
    assert!(html.contains("2024-01-01"));
    assert!(html.contains("2024-02-01"));
    assert!(html.contains("2024-03-01"));
    assert!(html.contains("Enacted"));
    assert!(html.contains("Amended"));
    assert!(html.contains("Repealed"));
}
#[test]
fn test_auto_visualization_selector_creation() {
    let selector = AutoVisualizationSelector::new();
    assert_eq!(selector.min_confidence, 0.7);
}
#[test]
fn test_auto_visualization_selector_with_min_confidence() {
    let selector = AutoVisualizationSelector::new().with_min_confidence(0.8);
    assert_eq!(selector.min_confidence, 0.8);
}
#[test]
fn test_auto_visualization_selector_recommend_small_tree() {
    let statute = Statute::new(
        "test-1",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let selector = AutoVisualizationSelector::new();
    let recommendation = selector.recommend_for_tree(&tree);
    assert_eq!(recommendation.viz_type, VisualizationType::DecisionTree);
    assert!(recommendation.confidence > 0.7);
    assert!(!recommendation.reasoning.is_empty());
    assert!(!recommendation.alternatives.is_empty());
}
#[test]
fn test_auto_visualization_selector_recommend_graph() {
    let mut graph = DependencyGraph::new();
    graph.add_dependency("statute-1", "statute-2", "references");
    let selector = AutoVisualizationSelector::new();
    let recommendation = selector.recommend_for_graph(&graph);
    assert!(recommendation.confidence > 0.7);
    assert!(!recommendation.reasoning.is_empty());
}
#[test]
fn test_auto_visualization_selector_recommend_timeline() {
    let mut timeline = Timeline::new();
    timeline.add_event(
        "2024-01-01",
        TimelineEvent::Enacted {
            statute_id: "statute-1".to_string(),
            title: "Test".to_string(),
        },
    );
    let selector = AutoVisualizationSelector::new();
    let recommendation = selector.recommend_for_timeline(&timeline);
    assert_eq!(recommendation.viz_type, VisualizationType::Timeline);
    assert!(recommendation.confidence > 0.9);
}
#[test]
fn test_auto_visualization_selector_default() {
    let selector1 = AutoVisualizationSelector::new();
    let selector2 = AutoVisualizationSelector::default();
    assert_eq!(selector1.min_confidence, selector2.min_confidence);
}
#[test]
fn test_ai_annotation_generator_creation() {
    let generator = AIAnnotationGenerator::new();
    assert!(generator.enable_complexity);
    assert!(generator.enable_patterns);
    assert_eq!(generator.min_importance, 0.5);
}
#[test]
fn test_ai_annotation_generator_without_complexity() {
    let generator = AIAnnotationGenerator::new().without_complexity();
    assert!(!generator.enable_complexity);
}
#[test]
fn test_ai_annotation_generator_without_patterns() {
    let generator = AIAnnotationGenerator::new().without_patterns();
    assert!(!generator.enable_patterns);
}
#[test]
fn test_ai_annotation_generator_with_min_importance() {
    let generator = AIAnnotationGenerator::new().with_min_importance(0.8);
    assert_eq!(generator.min_importance, 0.8);
}
#[test]
fn test_ai_annotation_generator_for_tree() {
    let statute = Statute::new(
        "test-1",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let generator = AIAnnotationGenerator::new();
    let annotations = generator.generate_for_tree(&tree);
    let _ = annotations;
}
#[test]
fn test_ai_annotation_generator_for_graph() {
    let mut graph = DependencyGraph::new();
    graph.add_dependency("statute-1", "statute-2", "references");
    let generator = AIAnnotationGenerator::new();
    let annotations = generator.generate_for_graph(&graph);
    let _ = annotations;
}
#[test]
fn test_ai_annotation_generator_default() {
    let gen1 = AIAnnotationGenerator::new();
    let gen2 = AIAnnotationGenerator::default();
    assert_eq!(gen1.min_importance, gen2.min_importance);
}
#[test]
fn test_natural_language_query_processor_creation() {
    let processor = NaturalLanguageQueryProcessor::new();
    assert!(!processor.case_sensitive);
}
#[test]
fn test_natural_language_query_processor_case_sensitive() {
    let processor = NaturalLanguageQueryProcessor::new().with_case_sensitive();
    assert!(processor.case_sensitive);
}
#[test]
fn test_natural_language_query_processor_query_outcomes() {
    let statute = Statute::new(
        "test-1",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let processor = NaturalLanguageQueryProcessor::new();
    let results = processor.query_tree(&tree, "show me outcomes");
    let _ = results;
}
#[test]
fn test_natural_language_query_processor_query_discretion() {
    let statute = Statute::new(
        "test-1",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let processor = NaturalLanguageQueryProcessor::new();
    let results = processor.query_tree(&tree, "find discretion");
    let _ = results;
}
#[test]
fn test_natural_language_query_processor_default() {
    let proc1 = NaturalLanguageQueryProcessor::new();
    let proc2 = NaturalLanguageQueryProcessor::default();
    assert_eq!(proc1.case_sensitive, proc2.case_sensitive);
}
#[test]
fn test_smart_data_highlighter_creation() {
    let highlighter = SmartDataHighlighter::new();
    assert_eq!(highlighter.highlight_color, "#ffeb3b");
    assert_eq!(highlighter.min_importance, 0.7);
}
#[test]
fn test_smart_data_highlighter_with_color() {
    let highlighter = SmartDataHighlighter::new().with_color("#ff0000".to_string());
    assert_eq!(highlighter.highlight_color, "#ff0000");
}
#[test]
fn test_smart_data_highlighter_with_min_importance() {
    let highlighter = SmartDataHighlighter::new().with_min_importance(0.9);
    assert_eq!(highlighter.min_importance, 0.9);
}
#[test]
fn test_smart_data_highlighter_highlight_tree() {
    let statute = Statute::new(
        "test-1",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let highlighter = SmartDataHighlighter::new();
    let rules = highlighter.highlight_tree(&tree);
    let _ = rules;
}
#[test]
fn test_smart_data_highlighter_highlight_graph() {
    let mut graph = DependencyGraph::new();
    graph.add_dependency("statute-1", "statute-2", "references");
    let highlighter = SmartDataHighlighter::new();
    let rules = highlighter.highlight_graph(&graph);
    let _ = rules;
}
#[test]
fn test_smart_data_highlighter_default() {
    let high1 = SmartDataHighlighter::new();
    let high2 = SmartDataHighlighter::default();
    assert_eq!(high1.highlight_color, high2.highlight_color);
}
#[test]
fn test_anomaly_detector_creation() {
    let detector = AnomalyDetector::new();
    assert_eq!(detector.sensitivity, 0.7);
}
#[test]
fn test_anomaly_detector_with_sensitivity() {
    let detector = AnomalyDetector::new().with_sensitivity(0.9);
    assert_eq!(detector.sensitivity, 0.9);
}
#[test]
fn test_anomaly_detector_detect_in_tree() {
    let statute = Statute::new(
        "test-1",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let detector = AnomalyDetector::new();
    let anomalies = detector.detect_in_tree(&tree);
    let _ = anomalies;
}
#[test]
fn test_anomaly_detector_detect_in_graph() {
    let mut graph = DependencyGraph::new();
    graph.add_dependency("statute-1", "statute-2", "references");
    let detector = AnomalyDetector::new();
    let anomalies = detector.detect_in_graph(&graph);
    let _ = anomalies;
}
#[test]
fn test_anomaly_detector_default() {
    let det1 = AnomalyDetector::new();
    let det2 = AnomalyDetector::default();
    assert_eq!(det1.sensitivity, det2.sensitivity);
}
#[test]
fn test_visualization_type_serialization() {
    let viz_type = VisualizationType::DecisionTree;
    let json = serde_json::to_string(&viz_type).unwrap();
    assert!(json.contains("DecisionTree"));
}
#[test]
fn test_annotation_category_serialization() {
    let category = AnnotationCategory::CriticalPath;
    let json = serde_json::to_string(&category).unwrap();
    assert!(json.contains("CriticalPath"));
}
#[test]
fn test_anomaly_type_serialization() {
    let anomaly_type = AnomalyType::OrphanedNode;
    let json = serde_json::to_string(&anomaly_type).unwrap();
    assert!(json.contains("OrphanedNode"));
}
#[test]
fn test_query_result_serialization() {
    let result = QueryResult {
        node_id: "node-1".to_string(),
        relevance: 0.8,
        excerpt: "test excerpt".to_string(),
        node_type: "condition".to_string(),
    };
    let json = serde_json::to_string(&result).unwrap();
    assert!(json.contains("node-1"));
    assert!(json.contains("0.8"));
}
#[test]
fn test_highlight_rule_serialization() {
    let rule = HighlightRule {
        target_id: "node-1".to_string(),
        color: "#ff0000".to_string(),
        importance: 0.9,
        reason: "Test reason".to_string(),
    };
    let json = serde_json::to_string(&rule).unwrap();
    assert!(json.contains("node-1"));
    assert!(json.contains("#ff0000"));
}
#[test]
fn test_anomaly_serialization() {
    let anomaly = Anomaly {
        anomaly_type: AnomalyType::Cycle,
        severity: 0.95,
        description: "Test anomaly".to_string(),
        location: "test-location".to_string(),
        suggestion: "Fix it".to_string(),
    };
    let json = serde_json::to_string(&anomaly).unwrap();
    assert!(json.contains("Cycle"));
    assert!(json.contains("0.95"));
}
#[test]
fn test_live_court_proceeding_creation() {
    let proceeding = LiveCourtProceeding::new("Supreme Court", "2024-001", "ws://localhost:8080");
    assert_eq!(proceeding.court_name, "Supreme Court");
    assert_eq!(proceeding.case_number, "2024-001");
    assert_eq!(proceeding.ws_url, "ws://localhost:8080");
}
#[test]
fn test_live_court_proceeding_with_theme() {
    let proceeding = LiveCourtProceeding::new("Supreme Court", "2024-001", "ws://localhost:8080")
        .with_theme(Theme::dark());
    assert_eq!(proceeding.theme.background_color, "#1a1a1a");
}
#[test]
fn test_live_court_proceeding_html_generation() {
    let events = vec![
        CourtEvent::new(
            "10:00 AM",
            CourtEventType::Opening,
            "Opening statements begin",
        )
        .with_participant("Prosecutor"),
        CourtEvent::new("10:30 AM", CourtEventType::Testimony, "Witness testimony")
            .with_participant("Witness 1"),
    ];
    let proceeding = LiveCourtProceeding::new("Supreme Court", "2024-001", "ws://localhost:8080");
    let html = proceeding.to_live_html(&events);
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("Supreme Court"));
    assert!(html.contains("2024-001"));
    assert!(html.contains("LIVE"));
    assert!(html.contains("Opening statements begin"));
    assert!(html.contains("Witness testimony"));
    assert!(html.contains("ws://localhost:8080"));
}
#[test]
fn test_live_court_proceeding_default() {
    let proceeding = LiveCourtProceeding::default();
    assert_eq!(proceeding.court_name, "Court");
    assert_eq!(proceeding.case_number, "Unknown");
}
#[test]
fn test_court_event_creation() {
    let event = CourtEvent::new("10:00 AM", CourtEventType::Ruling, "Judge issues ruling");
    assert_eq!(event.timestamp, "10:00 AM");
    assert_eq!(event.event_type, CourtEventType::Ruling);
    assert_eq!(event.description, "Judge issues ruling");
}
#[test]
fn test_court_event_with_participant() {
    let event = CourtEvent::new("10:00 AM", CourtEventType::Motion, "Motion filed")
        .with_participant("Defense Attorney")
        .with_participant("Prosecutor");
    assert_eq!(event.participants.len(), 2);
}
#[test]
fn test_court_event_type_serialization() {
    let event_type = CourtEventType::Testimony;
    let json = serde_json::to_string(&event_type).unwrap();
    assert!(json.contains("Testimony"));
}
#[test]
fn test_breaking_news_feed_creation() {
    let feed = BreakingNewsFeed::new("Legal News", "ws://localhost:8080");
    assert_eq!(feed.title, "Legal News");
    assert_eq!(feed.ws_url, "ws://localhost:8080");
    assert_eq!(feed.max_items, 50);
}
#[test]
fn test_breaking_news_feed_with_theme() {
    let feed = BreakingNewsFeed::new("Legal News", "ws://localhost:8080").with_theme(Theme::dark());
    assert_eq!(feed.theme.background_color, "#1a1a1a");
}
#[test]
fn test_breaking_news_feed_with_max_items() {
    let feed = BreakingNewsFeed::new("Legal News", "ws://localhost:8080").with_max_items(100);
    assert_eq!(feed.max_items, 100);
}
#[test]
fn test_breaking_news_feed_html_generation() {
    let news_items = vec![
        NewsItem::new(
            "Supreme Court Ruling",
            "Important case decided today",
            "Legal Times",
            "2024-01-01",
            NewsPriority::Urgent,
        )
        .with_tag("Supreme Court")
        .with_tag("Constitutional Law"),
        NewsItem::new(
            "New Legislation Proposed",
            "Bill introduced in Congress",
            "Law Gazette",
            "2024-01-02",
            NewsPriority::High,
        ),
    ];
    let feed = BreakingNewsFeed::new("Legal News", "ws://localhost:8080");
    let html = feed.to_html(&news_items);
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("Legal News"));
    assert!(html.contains("Supreme Court Ruling"));
    assert!(html.contains("New Legislation Proposed"));
    assert!(html.contains("Supreme Court"));
    assert!(html.contains("Constitutional Law"));
    assert!(html.contains("ws://localhost:8080"));
}
#[test]
fn test_breaking_news_feed_default() {
    let feed = BreakingNewsFeed::default();
    assert_eq!(feed.title, "Legal News Feed");
}
#[test]
fn test_news_item_creation() {
    let item = NewsItem::new(
        "Test News",
        "Summary",
        "Source",
        "2024-01-01",
        NewsPriority::Medium,
    );
    assert_eq!(item.title, "Test News");
    assert_eq!(item.priority, NewsPriority::Medium);
}
#[test]
fn test_news_item_with_tag() {
    let item = NewsItem::new("Test", "Summary", "Source", "2024-01-01", NewsPriority::Low)
        .with_tag("Tag1")
        .with_tag("Tag2");
    assert_eq!(item.tags.len(), 2);
}
#[test]
fn test_news_priority_serialization() {
    let priority = NewsPriority::Urgent;
    let json = serde_json::to_string(&priority).unwrap();
    assert!(json.contains("Urgent"));
}
#[test]
fn test_regulatory_change_monitor_creation() {
    let monitor = RegulatoryChangeMonitor::new("Regulatory Monitor", "ws://localhost:8080");
    assert_eq!(monitor.title, "Regulatory Monitor");
    assert_eq!(monitor.ws_url, "ws://localhost:8080");
}
#[test]
fn test_regulatory_change_monitor_with_theme() {
    let monitor = RegulatoryChangeMonitor::new("Regulatory Monitor", "ws://localhost:8080")
        .with_theme(Theme::dark());
    assert_eq!(monitor.theme.background_color, "#1a1a1a");
}
#[test]
fn test_regulatory_change_monitor_html_generation() {
    let changes = vec![
        RegulatoryChange::new(
            "REG-2024-001",
            "New environmental standards",
            "EPA",
            "2024-06-01",
            RegulatoryStatus::Proposed,
        )
        .with_impact("Significant impact on manufacturing")
        .with_sector("Manufacturing")
        .with_sector("Energy"),
        RegulatoryChange::new(
            "REG-2024-002",
            "Financial reporting updates",
            "SEC",
            "2024-03-01",
            RegulatoryStatus::Enacted,
        )
        .with_sector("Finance"),
    ];
    let monitor = RegulatoryChangeMonitor::new("Regulatory Monitor", "ws://localhost:8080");
    let html = monitor.to_html(&changes);
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("Regulatory Monitor"));
    assert!(html.contains("REG-2024-001"));
    assert!(html.contains("REG-2024-002"));
    assert!(html.contains("EPA"));
    assert!(html.contains("SEC"));
    assert!(html.contains("Manufacturing"));
    assert!(html.contains("Finance"));
    assert!(html.contains("ws://localhost:8080"));
}
#[test]
fn test_regulatory_change_monitor_default() {
    let monitor = RegulatoryChangeMonitor::default();
    assert_eq!(monitor.title, "Regulatory Change Monitor");
}
#[test]
fn test_regulatory_change_creation() {
    let change = RegulatoryChange::new(
        "REG-001",
        "Description",
        "Agency",
        "2024-01-01",
        RegulatoryStatus::Proposed,
    );
    assert_eq!(change.regulation_id, "REG-001");
    assert_eq!(change.status, RegulatoryStatus::Proposed);
}
#[test]
fn test_regulatory_change_with_impact() {
    let change = RegulatoryChange::new(
        "REG-001",
        "Description",
        "Agency",
        "2024-01-01",
        RegulatoryStatus::Enacted,
    )
    .with_impact("High impact");
    assert_eq!(change.impact_assessment, Some("High impact".to_string()));
}
#[test]
fn test_regulatory_change_with_sector() {
    let change = RegulatoryChange::new(
        "REG-001",
        "Description",
        "Agency",
        "2024-01-01",
        RegulatoryStatus::Amended,
    )
    .with_sector("Healthcare")
    .with_sector("Technology");
    assert_eq!(change.affected_sectors.len(), 2);
}
#[test]
fn test_regulatory_status_serialization() {
    let status = RegulatoryStatus::Repealed;
    let json = serde_json::to_string(&status).unwrap();
    assert!(json.contains("Repealed"));
}
#[test]
fn test_enforcement_action_tracker_creation() {
    let tracker = EnforcementActionTracker::new("Enforcement Tracker", "ws://localhost:8080");
    assert_eq!(tracker.title, "Enforcement Tracker");
    assert_eq!(tracker.ws_url, "ws://localhost:8080");
}
#[test]
fn test_enforcement_action_tracker_with_theme() {
    let tracker = EnforcementActionTracker::new("Enforcement Tracker", "ws://localhost:8080")
        .with_theme(Theme::dark());
    assert_eq!(tracker.theme.background_color, "#1a1a1a");
}
#[test]
fn test_enforcement_action_tracker_html_generation() {
    let actions = vec![
        EnforcementAction::new(
            "Company A",
            "SEC",
            "2024-01-15",
            EnforcementActionType::Fine,
            EnforcementStatus::Active,
        )
        .with_fine(1000000.0)
        .with_violation("Insider trading")
        .with_violation("Misrepresentation"),
        EnforcementAction::new(
            "Company B",
            "FTC",
            "2024-02-10",
            EnforcementActionType::Warning,
            EnforcementStatus::Resolved,
        ),
    ];
    let tracker = EnforcementActionTracker::new("Enforcement Tracker", "ws://localhost:8080");
    let html = tracker.to_html(&actions);
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("Enforcement Tracker"));
    assert!(html.contains("Company A"));
    assert!(html.contains("Company B"));
    assert!(html.contains("SEC"));
    assert!(html.contains("FTC"));
    assert!(html.contains("1000000"));
    assert!(html.contains("Insider trading"));
    assert!(html.contains("ws://localhost:8080"));
}
#[test]
fn test_enforcement_action_tracker_default() {
    let tracker = EnforcementActionTracker::default();
    assert_eq!(tracker.title, "Enforcement Action Tracker");
}
#[test]
fn test_enforcement_action_creation() {
    let action = EnforcementAction::new(
        "Entity",
        "Agency",
        "2024-01-01",
        EnforcementActionType::Settlement,
        EnforcementStatus::Pending,
    );
    assert_eq!(action.entity, "Entity");
    assert_eq!(action.action_type, EnforcementActionType::Settlement);
    assert_eq!(action.status, EnforcementStatus::Pending);
}
#[test]
fn test_enforcement_action_with_fine() {
    let action = EnforcementAction::new(
        "Entity",
        "Agency",
        "2024-01-01",
        EnforcementActionType::Fine,
        EnforcementStatus::Active,
    )
    .with_fine(500000.0);
    assert_eq!(action.fine_amount, Some(500000.0));
}
#[test]
fn test_enforcement_action_with_violation() {
    let action = EnforcementAction::new(
        "Entity",
        "Agency",
        "2024-01-01",
        EnforcementActionType::Investigation,
        EnforcementStatus::Pending,
    )
    .with_violation("Violation 1")
    .with_violation("Violation 2");
    assert_eq!(action.violations.len(), 2);
}
#[test]
fn test_enforcement_action_type_serialization() {
    let action_type = EnforcementActionType::Suspension;
    let json = serde_json::to_string(&action_type).unwrap();
    assert!(json.contains("Suspension"));
}
#[test]
fn test_enforcement_status_serialization() {
    let status = EnforcementStatus::Appealed;
    let json = serde_json::to_string(&status).unwrap();
    assert!(json.contains("Appealed"));
}
#[test]
fn test_market_impact_visualizer_creation() {
    let visualizer = MarketImpactVisualizer::new("Market Impact", "ws://localhost:8080");
    assert_eq!(visualizer.title, "Market Impact");
    assert_eq!(visualizer.ws_url, "ws://localhost:8080");
}
#[test]
fn test_market_impact_visualizer_with_theme() {
    let visualizer = MarketImpactVisualizer::new("Market Impact", "ws://localhost:8080")
        .with_theme(Theme::dark());
    assert_eq!(visualizer.theme.background_color, "#1a1a1a");
}
#[test]
fn test_market_impact_visualizer_html_generation() {
    let impacts = vec![
        MarketImpact::new(
            "Supreme Court Ruling on Tech",
            "2024-01-15",
            ImpactSeverity::High,
        )
        .with_stock_change(-5.2)
        .with_company("Tech Corp")
        .with_company("Data Inc")
        .with_sector("Technology"),
        MarketImpact::new(
            "New Financial Regulation",
            "2024-02-10",
            ImpactSeverity::Medium,
        )
        .with_stock_change(2.1)
        .with_company("Bank A")
        .with_sector("Finance"),
    ];
    let visualizer = MarketImpactVisualizer::new("Market Impact", "ws://localhost:8080");
    let html = visualizer.to_html(&impacts);
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("Market Impact"));
    assert!(html.contains("Supreme Court Ruling on Tech"));
    assert!(html.contains("New Financial Regulation"));
    assert!(html.contains("Tech Corp"));
    assert!(html.contains("Bank A"));
    assert!(html.contains("Technology"));
    assert!(html.contains("Finance"));
    assert!(html.contains("chart.js"));
    assert!(html.contains("ws://localhost:8080"));
}
#[test]
fn test_market_impact_visualizer_default() {
    let visualizer = MarketImpactVisualizer::default();
    assert_eq!(visualizer.title, "Market Impact Analysis");
}
#[test]
fn test_market_impact_creation() {
    let impact = MarketImpact::new("Legal Event", "2024-01-01", ImpactSeverity::Low);
    assert_eq!(impact.legal_event, "Legal Event");
    assert_eq!(impact.event_date, "2024-01-01");
    assert_eq!(impact.severity, ImpactSeverity::Low);
}
#[test]
fn test_market_impact_with_stock_change() {
    let impact =
        MarketImpact::new("Event", "2024-01-01", ImpactSeverity::High).with_stock_change(-3.5);
    assert_eq!(impact.stock_price_change, Some(-3.5));
}
#[test]
fn test_market_impact_with_company() {
    let impact = MarketImpact::new("Event", "2024-01-01", ImpactSeverity::Medium)
        .with_company("Company A")
        .with_company("Company B");
    assert_eq!(impact.affected_companies.len(), 2);
}
#[test]
fn test_market_impact_with_sector() {
    let impact = MarketImpact::new("Event", "2024-01-01", ImpactSeverity::High)
        .with_sector("Healthcare")
        .with_sector("Pharma");
    assert_eq!(impact.sectors.len(), 2);
}
#[test]
fn test_impact_severity_serialization() {
    let severity = ImpactSeverity::Medium;
    let json = serde_json::to_string(&severity).unwrap();
    assert!(json.contains("Medium"));
}
#[test]
fn test_scrollytelling_config_creation() {
    let config = ScrollytellingConfig::new();
    assert!(config.enable_animations);
    assert_eq!(config.trigger_threshold, 0.5);
    assert!(config.show_progress);
    assert!(config.enable_navigation);
}
#[test]
fn test_scrollytelling_config_customization() {
    let config = ScrollytellingConfig::new()
        .without_animations()
        .with_trigger_threshold(0.7)
        .without_progress()
        .without_navigation();
    assert!(!config.enable_animations);
    assert_eq!(config.trigger_threshold, 0.7);
    assert!(!config.show_progress);
    assert!(!config.enable_navigation);
}
#[test]
fn test_legal_history_scrollytelling_creation() {
    let scrolly = LegalHistoryScrollytelling::new("Legal Evolution");
    assert_eq!(scrolly.title, "Legal Evolution");
}
#[test]
fn test_legal_history_scrollytelling_html() {
    let chapters = vec![
        ScrollChapter::new("Chapter 1")
            .with_paragraph("First paragraph")
            .with_paragraph("Second paragraph")
            .with_visual("Visual element"),
        ScrollChapter::new("Chapter 2").with_paragraph("Content"),
    ];
    let scrolly = LegalHistoryScrollytelling::new("Test History");
    let html = scrolly.to_html(&chapters);
    assert!(html.contains("Test History"));
    assert!(html.contains("Chapter 1"));
    assert!(html.contains("Chapter 2"));
    assert!(html.contains("First paragraph"));
    assert!(html.contains("Visual element"));
}
#[test]
fn test_scroll_chapter_creation() {
    let chapter = ScrollChapter::new("Test Chapter")
        .with_paragraph("Para 1")
        .with_visual("Visual");
    assert_eq!(chapter.title, "Test Chapter");
    assert_eq!(chapter.content.len(), 1);
    assert!(chapter.visual.is_some());
}
#[test]
fn test_case_story_generator_creation() {
    let generator = CaseStoryGenerator::new();
    assert!(generator.include_timeline);
    assert!(generator.include_players);
}
#[test]
fn test_case_story_generator_customization() {
    let generator = CaseStoryGenerator::new()
        .without_timeline()
        .without_players();
    assert!(!generator.include_timeline);
    assert!(!generator.include_players);
}
#[test]
fn test_case_story_creation() {
    let case = CaseStory::new("Test Case", "Landmark Decision")
        .with_intro("Introduction paragraph")
        .with_player("John Doe", "Plaintiff")
        .with_event("2024-01-01", "Case filed")
        .with_resolution("Resolution paragraph")
        .with_outcome("Favorable outcome");
    assert_eq!(case.title, "Test Case");
    assert_eq!(case.subtitle, "Landmark Decision");
    assert_eq!(case.introduction.len(), 1);
    assert_eq!(case.key_players.len(), 1);
    assert_eq!(case.timeline.len(), 1);
    assert_eq!(case.resolution.len(), 1);
    assert!(case.outcome.is_some());
}
#[test]
fn test_case_story_html_generation() {
    let case = CaseStory::new("Famous Case", "Legal Milestone")
        .with_intro("This was an important case")
        .with_player("Alice", "Defendant")
        .with_event("2024-06-15", "Trial begins")
        .with_resolution("The case was resolved")
        .with_outcome("Victory");
    let generator = CaseStoryGenerator::new();
    let html = generator.generate_story(&case);
    assert!(html.contains("Famous Case"));
    assert!(html.contains("Legal Milestone"));
    assert!(html.contains("Alice"));
    assert!(html.contains("Defendant"));
    assert!(html.contains("2024-06-15"));
    assert!(html.contains("Victory"));
}
