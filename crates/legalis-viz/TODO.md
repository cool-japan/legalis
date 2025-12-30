# legalis-viz TODO

## Completed

- [x] Mermaid flowchart generation
- [x] GraphViz DOT format
- [x] ASCII art for terminal output (tree and box formats)
- [x] Decision tree visualization
- [x] Discretion zone highlighting
- [x] Dependency graph between statutes
- [x] PlantUML sequence diagrams
- [x] HTML with embedded JavaScript (D3.js)
- [x] D3.js interactive visualization
- [x] Timeline visualization for temporal statutes
- [x] Customizable styling/theming (light, dark, high-contrast, colorblind-friendly)
- [x] Annotation support for judicial notes
- [x] Support for large graph layouts

## Formats

- [x] SVG direct rendering
- [x] PNG direct rendering (via `png-export` feature)

## Features

- [x] Interactive web-based visualization
- [x] Statute dependency graphs
- [x] Timeline visualization for temporal statutes
- [x] Population distribution charts from simulations
- [x] Drill-down navigation (interactive HTML with detail panels)

## Improvements

- [x] Add customizable styling/theming
- [x] Create accessibility-compliant output
- [x] Add annotation support for judicial notes
- [x] Support for large graph layouts

## Export

- [x] PDF export (framework in place via `pdf-export` feature)
- [x] PowerPoint/Keynote integration (PresentationExporter with PPTX and Keynote formats)
- [x] Embedding in documents (DocumentEmbedder for Markdown, LaTeX, reStructuredText, AsciiDoc, HTML iframe)
- [x] Animation support for presentations (AnimationType with fade, slide, zoom effects)

## Integration

- [x] Real-time updates from simulation (LiveVisualization with WebSocket support)
- [x] Integration with web frontends (HTML export with embedded JavaScript)
- [x] Plugin system for custom renderers (Renderer trait + RendererRegistry)

## Testing

- [x] Visual regression tests (VisualRegressionTest and VisualRegressionSuite implemented)
- [x] Test all output formats (comprehensive tests added)
- [x] Benchmark rendering performance (benchmarks/rendering.rs added)

## Advanced Visualizations

- [x] Statute diff visualization (StatuteDiffVisualizer with HTML, Mermaid, ASCII formats)
- [x] Legal reasoning chain visualization (ReasoningChainVisualizer with HTML, Mermaid, ASCII formats)
- [x] Evaluation audit trail visualization (AuditTrailVisualizer with HTML, ASCII formats)
- [x] Comprehensive test coverage for all new visualizers (20 new tests added)

## Notes

### PNG Export
PNG export is available via the `png-export` feature flag. Enable with:
```toml
legalis-viz = { version = "0.2.0", features = ["png-export"] }
```

### Real-time Updates
The `LiveVisualization` struct supports real-time updates via WebSocket connections:
- Process `UpdateEvent`s for population, dependencies, and timelines
- Generate live HTML dashboards with automatic reconnection
- Track update history

### Plugin System
Custom renderers can be created by implementing the `Renderer` trait and registering them with `RendererRegistry`:
```rust
struct MyRenderer;
impl Renderer for MyRenderer {
    type Output = String;
    // ... implement methods
}

let mut registry = RendererRegistry::new();
registry.register("my-renderer", MyRenderer);
```

### Benchmarks
Run performance benchmarks with:
```bash
cargo bench --bench rendering
```

Enable PNG benchmarks with:
```bash
cargo bench --bench rendering --features png-export
```

### PowerPoint/Keynote Export
Create presentations with the `PresentationExporter`:
```rust
let mut exporter = PresentationExporter::new()
    .with_theme(Theme::dark());

// Add slides from decision trees or dependency graphs
exporter.add_decision_tree_slide("My Decision Tree", &tree);
exporter.add_dependency_graph_slide("Dependencies", &graph);

// Export to PowerPoint PPTX format
let pptx = exporter.to_pptx()?;

// Export to Keynote format
let keynote = exporter.to_keynote()?;

// Export to animated HTML presentation
let html = exporter.to_animated_html();
```

### Document Embedding
Embed visualizations in various document formats with `DocumentEmbedder`:
```rust
let embedder = DocumentEmbedder::new()
    .with_theme(Theme::colorblind_friendly());

// Embed in Markdown (with base64-encoded SVG)
let markdown = embedder.embed_in_markdown(&tree);

// Embed in LaTeX (TikZ format)
let latex = embedder.embed_in_latex(&tree);

// Embed in reStructuredText
let rst = embedder.embed_in_rst(&tree);

// Embed in AsciiDoc
let asciidoc = embedder.embed_in_asciidoc(&tree);

// Embed as HTML iframe
let iframe = embedder.embed_as_iframe(&tree, 800, 600);
```

### Animation Support
Create animated presentations with various transition effects:
```rust
let animation = Animation {
    target: "element-id".to_string(),
    animation_type: AnimationType::FadeIn,
    duration_ms: 500,
    delay_ms: 100,
};

// Available animation types:
// - FadeIn, FadeOut
// - SlideInLeft, SlideInRight, SlideInTop, SlideInBottom
// - ZoomIn, ZoomOut
// - Highlight
// - ProgressiveReveal
```

### Visual Regression Testing
Test visual output for regressions:
```rust
let baseline = tree.to_svg();
// ... make changes ...
let actual = tree.to_svg();

let test = VisualRegressionTest::new("svg_output", &baseline, &actual);
assert!(test.passed);

// Create a test suite
let mut suite = VisualRegressionSuite::new();
suite.add_test(test);
let summary = suite.run();
assert!(suite.all_passed());
```

### Statute Diff Visualization
Visualize changes between statute versions:
```rust
use legalis_viz::StatuteDiffVisualizer;

let visualizer = StatuteDiffVisualizer::new()
    .with_theme(Theme::dark());

// Visualize the diff between two statute versions
let html = visualizer.to_html(&statute_diff);
let mermaid = visualizer.to_mermaid(&statute_diff);
let ascii = visualizer.to_ascii(&statute_diff);
```

### Legal Reasoning Chain Visualization
Visualize legal reasoning and explanations:
```rust
use legalis_viz::ReasoningChainVisualizer;

let visualizer = ReasoningChainVisualizer::new()
    .with_theme(Theme::colorblind_friendly());

// Visualize a legal explanation with reasoning steps
let html = visualizer.to_html(&legal_explanation);
let mermaid = visualizer.to_mermaid(&legal_explanation);
let ascii = visualizer.to_ascii(&legal_explanation);
```

### Evaluation Audit Trail Visualization
Visualize evaluation audit trails with performance metrics:
```rust
use legalis_viz::AuditTrailVisualizer;

let visualizer = AuditTrailVisualizer::new()
    .with_theme(Theme::high_contrast());

// Visualize the audit trail with statistics
let html = visualizer.to_html(&audit_trail);
let ascii = visualizer.to_ascii(&audit_trail);
```

### Interactive Visualization
Create interactive HTML visualizations with zoom, pan, tooltips, search, and mini-map:
```rust
use legalis_viz::{InteractiveVisualizer, InteractiveConfig};

// Create an interactive visualizer with default settings
let visualizer = InteractiveVisualizer::new();

// Generate interactive HTML for a decision tree
let html = visualizer.to_interactive_html(&decision_tree);

// Generate interactive HTML for a dependency graph
let html = visualizer.to_interactive_html_graph(&dependency_graph);

// Customize the interactive features
let config = InteractiveConfig {
    enable_zoom_pan: true,
    enable_tooltips: true,
    enable_click_expand: true,
    enable_search: true,
    enable_minimap: true,
    initial_zoom: 1.0,
    min_zoom: 0.1,
    max_zoom: 5.0,
    minimap_size: (200, 150),
};

let visualizer = InteractiveVisualizer::new()
    .with_theme(Theme::dark())
    .with_config(config);

let html = visualizer.to_interactive_html(&decision_tree);
```

Features:
- **Zoom and Pan**: Mouse wheel zoom, drag to pan, zoom buttons, fit-to-screen
- **Tooltips**: Hover over nodes and edges to see details
- **Click-to-Expand**: Click nodes to collapse/expand subtrees
- **Search**: Search and highlight nodes by text content
- **Mini-map**: Overview navigation panel in the bottom-right corner

### 3D Visualization
Create immersive 3D visualizations using WebGL (Three.js) with VR/AR support:
```rust
use legalis_viz::{ThreeDVisualizer, ThreeDConfig};

// Create a 3D visualizer with default settings
let visualizer = ThreeDVisualizer::new();

// Generate 3D HTML for a dependency graph
let html = visualizer.to_3d_html_graph(&dependency_graph);

// Generate 3D HTML for a timeline
let html = visualizer.to_3d_html_timeline(&timeline);

// Customize the 3D features
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

let visualizer = ThreeDVisualizer::new()
    .with_theme(Theme::dark())
    .with_config(config);

let html = visualizer.to_3d_html_graph(&dependency_graph);
```

Features:
- **WebGL Rendering**: Hardware-accelerated 3D graphics using Three.js
- **Force-Directed Layout**: Organic graph layouts with physics-based positioning
- **Depth-Based Coloring**: Nodes colored by their depth in the graph hierarchy
- **VR/AR Support**: Enter VR or AR mode for immersive exploration (requires compatible device)
- **Auto-Rotation**: Automatic camera rotation for presentations
- **Interactive Controls**: Reset camera, toggle rotation, reset force simulation
- **Timeline Layout**: Specialized 3D layout for temporal visualizations
- **Mouse Interaction**: Hover over nodes to see details

### Accessibility Features
Create fully accessible visualizations with WCAG 2.1 AA compliance:
```rust
use legalis_viz::{AccessibilityEnhancer, AccessibilityConfig};

// Create an accessibility enhancer with default settings
let enhancer = AccessibilityEnhancer::new();

// Generate accessible HTML for a decision tree
let html = enhancer.to_accessible_html(&decision_tree);

// Generate accessible HTML for a dependency graph
let html = enhancer.to_accessible_html_graph(&dependency_graph);

// Customize accessibility features
let config = AccessibilityConfig::screen_reader_optimized();
let enhancer = AccessibilityEnhancer::new().with_config(config);

// Use specific accessibility configurations
let config = AccessibilityConfig::high_contrast();  // High contrast mode
let config = AccessibilityConfig::reduced_motion(); // Reduced motion for sensitive users
```

Features:
- **WCAG 2.1 AA Compliance**: Ensures minimum 4.5:1 contrast ratio for all text
- **Screen Reader Support**: Full ARIA labels and descriptions for all interactive elements
- **Keyboard Navigation**: Complete keyboard control with Tab, Arrow keys, Home/End navigation
- **High Contrast Mode**: Improved visibility with bold text and increased contrast
- **Reduced Motion**: Disables animations for users sensitive to motion
- **Focus Indicators**: Clear visual focus indicators for keyboard navigation
- **Responsive Design**: Viewport meta tags and responsive layouts
- **Semantic HTML**: Proper HTML5 semantic structure with lang attributes

### Advanced Export Formats
Export visualizations to professional formats for presentations and publications:
```rust
use legalis_viz::{AdvancedExporter, AnimatedGifConfig, VideoConfig, PdfConfig, PosterConfig};

// Create an advanced exporter
let exporter = AdvancedExporter::new();

// Export to animated GIF
let config = AnimatedGifConfig::new()
    .with_fps(30)
    .with_duration(10)
    .with_loop_count(0);  // Infinite loop
let frames = exporter.to_animated_gif(&tree, config);

// Export to video frames (MP4, WebM)
let config = VideoConfig::hd_1080p()
    .with_codec("h264")
    .with_bitrate(8000);
let frames = exporter.to_video_frames(&tree, config);

// Export to print-optimized PDF
let config = PdfConfig::a4()
    .print_optimized()
    .with_dpi(300);
let svg = exporter.to_print_pdf(&tree, config);

// Export to vector PDF
let config = PdfConfig::letter()
    .vector()
    .with_margin(20.0);
let svg = exporter.to_vector_pdf(&tree, config);

// Export to poster size
let config = PosterConfig::a0()
    .landscape()
    .with_dpi(300);
let svg = exporter.to_poster(&tree, config);
```

Features:
- **Animated GIF**: Generate frame-by-frame SVGs for GIF encoding with configurable FPS, duration, and quality
- **Video Export**: Create video frames (MP4/WebM) with HD, Full HD, and 4K presets
- **Print PDF**: Optimized for high-quality printing with serif fonts and print-specific CSS
- **Vector PDF**: Scalable vector graphics with PDF-specific metadata
- **Poster Output**: Large-format printing support (A0, A1, A2, 24x36") with high DPI options
- **Flexible Configuration**: Builder pattern for all export configurations
- **Multiple Formats**: Support for both decision trees and dependency graphs
- **Standard Sizes**: Predefined paper sizes (A3, A4, Letter, Tabloid) and video resolutions (720p, 1080p, 4K)

## Roadmap for 0.1.0 Series

### Interactive Visualizations (v0.1.1)
- [x] Add zoom and pan controls for large graphs
- [x] Add node/edge hover tooltips
- [x] Add click-to-expand for condition trees
- [x] Add search and highlight functionality
- [x] Add mini-map for navigation

### 3D Visualization (v0.1.2)
- [x] Add 3D dependency graph using WebGL
- [x] Add 3D timeline visualization
- [x] Add VR/AR support for immersive exploration
- [x] Add force-directed 3D layout
- [x] Add depth-based coloring

### Accessibility (v0.1.3)
- [x] Add WCAG 2.1 AA compliance
- [x] Add screen reader descriptions
- [x] Add keyboard navigation
- [x] Add high contrast mode improvements
- [x] Add reduced motion option

### Export Formats (v0.1.4)
- [x] Add animated GIF export
- [x] Add video export (MP4, WebM)
- [x] Add print-optimized PDF
- [x] Add vector PDF export
- [x] Add poster-size output

### Real-Time Features (v0.1.5)
- [x] Add live simulation visualization
- [x] Add streaming data updates
- [x] Add collaborative viewing
- [x] Add annotation sharing
- [x] Add cursor presence indicators

### Embedding (v0.1.6)
- [x] Add React component wrapper
- [x] Add Vue.js component wrapper
- [x] Add Angular component wrapper
- [x] Add Web Component standard
- [x] Add WordPress plugin

### Theming (v0.1.7)
- [x] Add custom theme builder
- [x] Add organization branding support
- [x] Add theme import/export
- [x] Add seasonal/event themes
- [x] Add CSS variable customization

### Performance (v0.1.8)
- [x] Add virtualization for large datasets
- [x] Add WebWorker rendering
- [x] Add progressive loading
- [x] Add level-of-detail rendering
- [x] Add canvas fallback for performance

### Domain-Specific Visualizations (v0.1.9)
- [x] Add court hierarchy visualization
- [x] Add legislative process flowchart
- [x] Add case citation network
- [x] Add regulatory landscape map
- [x] Add compliance status dashboard

## New Features Documentation

### Real-Time Collaboration (v0.1.5)

#### Streaming Data Source
Continuous data updates from external sources:
```rust
use legalis_viz::StreamingDataSource;

let source = StreamingDataSource::new("my-stream", "ws://localhost:8080", 1000)
    .with_buffer_size(500);

// JavaScript client code is auto-generated
let js_code = source.to_javascript();
```

#### Collaborative Sessions
Multi-user viewing with real-time updates:
```rust
use legalis_viz::{CollaborativeSession, CollaborativeUser, CursorPosition, SharedAnnotation};

// Create a collaborative session
let mut session = CollaborativeSession::new("session-123", "ws://localhost:8080");

// Add users
let alice = CollaborativeUser::new("user-1", "Alice", "#ff0000");
let bob = CollaborativeUser::new("user-2", "Bob", "#00ff00");
session.add_user(alice.clone());
session.add_user(bob.clone());

// Update cursor positions
let cursor = CursorPosition::new(alice.clone(), 50.0, 75.0, 1234567890);
session.update_cursor(cursor);

// Add shared annotations
let annotation = SharedAnnotation::new(
    "annot-1",
    alice,
    "node-123",
    "This needs review",
    1234567890,
);
session.add_annotation(annotation);

// Generate collaborative HTML
let html = session.to_collaborative_html(&decision_tree);
```

Features:
- **Real-time cursor presence**: See where other users are looking
- **Shared annotations**: Collaborative commenting with resolve functionality
- **Active user list**: Panel showing all active participants
- **Auto-reconnection**: WebSocket connection with automatic retry
- **User color coding**: Each user has a distinct color for cursors and annotations

### Custom Theme Builder (v0.1.7)

Create custom branded themes:
```rust
use legalis_viz::CustomThemeBuilder;

// Build a custom theme from scratch
let theme = CustomThemeBuilder::new()
    .with_background_color("#1a1a1a")
    .with_text_color("#ffffff")
    .with_condition_color("#3498db")
    .with_outcome_color("#2ecc71")
    .with_discretion_color("#e74c3c")
    .build();

// Use organization branding
let branded_theme = CustomThemeBuilder::new()
    .with_branding("#ff6600", "#0066cc")  // primary, secondary colors
    .build();

// Use a complete color palette
let palette_theme = CustomThemeBuilder::new()
    .with_palette(
        "#ffffff",  // background
        "#000000",  // foreground
        "#ff0000",  // accent 1
        "#00ff00",  // accent 2
        "#0000ff",  // accent 3
    )
    .build();

// Export and import themes
let json = CustomThemeBuilder::new()
    .with_background_color("#ffffff")
    .to_json()?;

let imported = CustomThemeBuilder::from_json(&json)?;
let theme = imported.build();
```

Features:
- **Builder pattern**: Fluent API for easy theme creation
- **Organization branding**: Quick setup with brand colors
- **Color palettes**: Complete color scheme setup
- **JSON import/export**: Save and share themes
- **Theme inheritance**: Start from existing themes (light, dark, etc.)

### Usage Examples

#### Complete Collaborative Visualization Pipeline
```rust
use legalis_viz::{
    CollaborativeSession,
    CollaborativeUser,
    CustomThemeBuilder,
    DecisionTree,
};
use legalis_core::{Statute, Effect, EffectType};

// Create a statute and decision tree
let statute = Statute::new(
    "statute-1",
    "Example Statute",
    Effect::new(EffectType::Grant, "Grants permission"),
);
let tree = DecisionTree::from_statute(&statute)?;

// Create a custom branded theme
let theme = CustomThemeBuilder::new()
    .with_branding("#3498db", "#2ecc71")
    .build();

// Set up collaborative session
let mut session = CollaborativeSession::new("collab-1", "ws://localhost:9000");
let user1 = CollaborativeUser::new("user-1", "Alice", "#e74c3c");
let user2 = CollaborativeUser::new("user-2", "Bob", "#f39c12");
session.add_user(user1);
session.add_user(user2);

// Generate the collaborative HTML
let html = session.to_collaborative_html(&tree);

// Write to file
std::fs::write("collaborative.html", html)?;
```

#### Streaming Data Visualization
```rust
use legalis_viz::{StreamingDataSource, LiveVisualization, UpdateEvent};

// Create a live visualization
let mut live_viz = LiveVisualization::new("Live Dashboard");

// Set up streaming source
let stream = StreamingDataSource::new("data-stream", "ws://data.example.com", 1000);

// Process updates
let update = UpdateEvent::PopulationUpdate {
    category: "Eligible".to_string(),
    count: 150,
    timestamp: "2025-12-29T12:00:00Z".to_string(),
};
live_viz.process_update(update);

// Generate live dashboard HTML
let dashboard_html = live_viz.to_live_html("ws://localhost:9000");
std::fs::write("live_dashboard.html", dashboard_html)?;
```

### Seasonal and Event Themes (v0.1.7)

Pre-configured seasonal and event-specific themes:
```rust
use legalis_viz::SeasonalThemes;

// Use a seasonal theme
let winter_theme = SeasonalThemes::winter();
let spring_theme = SeasonalThemes::spring();
let summer_theme = SeasonalThemes::summer();
let autumn_theme = SeasonalThemes::autumn();

// Use event/professional themes
let holiday_theme = SeasonalThemes::holiday();
let corporate_theme = SeasonalThemes::corporate();
let academic_theme = SeasonalThemes::academic();
let legal_theme = SeasonalThemes::legal();

// Apply to visualization
let tree = DecisionTree::from_statute(&statute)?;
let html = tree.to_html_with_theme(&winter_theme);
```

Available themes:
- **Winter**: Cool blues and whites for winter/cold weather
- **Spring**: Fresh greens and pastels for spring/renewal
- **Summer**: Warm, vibrant colors for summer/energy
- **Autumn**: Warm earth tones for fall/harvest
- **Holiday**: Festive reds and greens for holidays
- **Corporate**: Professional navy and gray for business
- **Academic**: Scholarly blues for education
- **Legal**: Traditional colors for legal/government use

### CSS Variable Customization (v0.1.7)

Dynamic theming with CSS variables:
```rust
use legalis_viz::{CssVariableTheme, Theme};

// Create from existing theme
let theme = Theme::dark();
let css_vars = CssVariableTheme::from_theme(&theme);

// Generate CSS with :root selector
let css = css_vars.to_css();
// Output:
// :root {
//   --viz-root-color: #2c2c2c;
//   --viz-condition-color: #1e3a5f;
//   ...
// }

// Or use custom selector
let css = css_vars.to_css_with_selector(".my-theme");

// Create custom variables
let custom_css = CssVariableTheme::new()
    .add_variable("--primary", "#3498db")
    .add_variable("--secondary", "#2ecc71")
    .add_variable("--accent", "#e74c3c")
    .to_css();
```

### Performance Optimizations (v0.1.8)

#### Virtualization for Large Datasets
Efficiently render large lists by only rendering visible items:
```rust
use legalis_viz::VirtualizationConfig;

let config = VirtualizationConfig::new()
    .with_batch_size(100)       // Render 100 items at a time
    .with_buffer_size(20)        // Keep 20 items buffer
    .with_dynamic_height();      // Calculate heights dynamically

// Generate JavaScript code
let js_code = config.to_javascript();

// Use in HTML
let html = format!(r#"
<!DOCTYPE html>
<html>
<head>
    <script>{}</script>
</head>
<body>
    <div id="container"></div>
    <script>
        const items = [...]; // Large array of items
        const scroller = new VirtualScroller(
            document.getElementById('container'),
            items,
            {{}}
        );
    </script>
</body>
</html>
"#, js_code);
```

Features:
- Only renders visible items plus buffer
- Smooth scrolling with minimal reflows
- Configurable batch and buffer sizes
- Optional dynamic height calculation
- Handles thousands of items efficiently

#### Progressive Loading
Load data incrementally as the user scrolls:
```rust
use legalis_viz::ProgressiveLoadingConfig;

let config = ProgressiveLoadingConfig::new()
    .with_initial_load(50)          // Load 50 items initially
    .with_load_increment(25)         // Load 25 more on scroll
    .without_loading_indicator();    // Hide loading spinner

// Generate JavaScript code
let js_code = config.to_javascript();

// Integrate with async data provider
// The data provider should return a Promise<Array>
```

Features:
- Lazy loading on scroll
- Configurable initial and incremental loads
- Optional loading indicators
- Debounced scroll detection
- Prevents over-fetching

#### Level-of-Detail Rendering
Adjust visualization detail based on zoom level:
```rust
use legalis_viz::LevelOfDetailConfig;

let config = LevelOfDetailConfig::new()
    .with_zoom_thresholds(vec![0.25, 0.5, 0.75, 1.0]);

// Generate JavaScript code
let js_code = config.to_javascript();
```

Features:
- Hide/show labels based on zoom
- Simplify edges at low zoom levels
- Aggregate nodes into clusters
- Custom zoom thresholds
- Automatic detail level switching

### Performance Best Practices

For optimal performance with large datasets:
1. Use **virtualization** for lists with 1000+ items
2. Use **progressive loading** for dynamic data
3. Use **level-of-detail** for complex graphs
4. Combine all three for maximum performance

Example combining all performance features:
```rust
use legalis_viz::{VirtualizationConfig, ProgressiveLoadingConfig, LevelOfDetailConfig};

// Set up all performance optimizations
let virt_config = VirtualizationConfig::new().with_batch_size(100);
let prog_config = ProgressiveLoadingConfig::new().with_initial_load(50);
let lod_config = LevelOfDetailConfig::new();

// Generate combined JavaScript
let html = format!(r#"
<!DOCTYPE html>
<html>
<head>
    <script>{}</script>
    <script>{}</script>
    <script>{}</script>
</head>
<body>
    <!-- High-performance visualization -->
</body>
</html>
"#, virt_config.to_javascript(), prog_config.to_javascript(), lod_config.to_javascript());
```

## Domain-Specific Visualizations (v0.1.9)

### Court Hierarchy Visualization

Visualize court systems with hierarchical structure:

```rust
use legalis_viz::{CourtHierarchyVisualizer, CourtNode};

let courts = vec![
    CourtNode {
        id: "supreme-1".to_string(),
        name: "Supreme Court".to_string(),
        level: "Supreme".to_string(),
        jurisdiction: "Federal".to_string(),
        judge_count: 9,
    },
    CourtNode {
        id: "appellate-1".to_string(),
        name: "First Circuit Court of Appeals".to_string(),
        level: "Appellate".to_string(),
        jurisdiction: "Regional".to_string(),
        judge_count: 15,
    },
];

let visualizer = CourtHierarchyVisualizer::new();
let html = visualizer.to_html(&courts);
let mermaid = visualizer.to_mermaid(&courts);
```

Features:
- **Hierarchical Levels**: Supreme, Appellate, Trial, District, Municipal
- **Court Information**: Name, jurisdiction, judge count
- **Multiple Formats**: HTML and Mermaid diagram support
- **Theme Support**: Customizable color themes

### Legislative Process Flowchart

Visualize the legislative process with sequential steps:

```rust
use legalis_viz::{LegislativeProcessVisualizer, LegislativeStep};

let steps = vec![
    LegislativeStep {
        id: "step-1".to_string(),
        name: "Bill Introduction".to_string(),
        description: "A member introduces the bill in the chamber".to_string(),
        actors: vec!["Senator".to_string(), "Representative".to_string()],
        duration_days: Some(1),
    },
    LegislativeStep {
        id: "step-2".to_string(),
        name: "Committee Review".to_string(),
        description: "The bill is assigned to a committee for review".to_string(),
        actors: vec!["Committee Chair".to_string(), "Committee Members".to_string()],
        duration_days: Some(30),
    },
];

let visualizer = LegislativeProcessVisualizer::new();
let html = visualizer.to_html(&steps);
let mermaid = visualizer.to_mermaid(&steps);
```

Features:
- **Sequential Steps**: Ordered visualization of legislative stages
- **Actor Information**: Shows who is involved in each step
- **Duration Estimates**: Optional time estimates for each step
- **Multiple Formats**: HTML and Mermaid flowchart support

### Case Citation Network

Visualize relationships between legal cases through citations:

```rust
use legalis_viz::{CaseCitationNetworkVisualizer, CaseCitation};

let cases = vec![
    CaseCitation {
        id: "case-1".to_string(),
        name: "Brown v. Board of Education".to_string(),
        year: 1954,
        court: "Supreme Court".to_string(),
        citations: vec!["case-2".to_string()],
    },
    CaseCitation {
        id: "case-2".to_string(),
        name: "Plessy v. Ferguson".to_string(),
        year: 1896,
        court: "Supreme Court".to_string(),
        citations: vec![],
    },
];

let visualizer = CaseCitationNetworkVisualizer::new();
let html = visualizer.to_html(&cases);  // Interactive D3.js network
let mermaid = visualizer.to_mermaid(&cases);
```

Features:
- **Interactive Network**: D3.js force-directed graph with drag support
- **Citation Links**: Visualize how cases reference each other
- **Case Metadata**: Year, court, and case name displayed
- **Graph Formats**: HTML with D3.js and Mermaid diagram support

### Regulatory Landscape Map

Visualize regulatory entities and their jurisdictions:

```rust
use legalis_viz::{RegulatoryLandscapeVisualizer, RegulatoryEntity};

let entities = vec![
    RegulatoryEntity {
        id: "entity-1".to_string(),
        name: "Securities and Exchange Commission".to_string(),
        entity_type: "Agency".to_string(),
        jurisdiction: "Federal".to_string(),
        sectors: vec!["Finance".to_string(), "Securities".to_string()],
    },
    RegulatoryEntity {
        id: "entity-2".to_string(),
        name: "Federal Trade Commission".to_string(),
        entity_type: "Commission".to_string(),
        jurisdiction: "Federal".to_string(),
        sectors: vec!["Consumer Protection".to_string(), "Antitrust".to_string()],
    },
];

let visualizer = RegulatoryLandscapeVisualizer::new();
let html = visualizer.to_html(&entities);
```

Features:
- **Entity Types**: Agency, Authority, Commission, etc.
- **Jurisdiction Information**: Federal, state, or local
- **Sector Tags**: Visual indicators for regulated sectors
- **Responsive Grid**: Automatically adjusts layout based on screen size

### Compliance Status Dashboard

Create comprehensive compliance dashboards:

```rust
use legalis_viz::{ComplianceDashboardVisualizer, ComplianceItem, ComplianceStatus};

let items = vec![
    ComplianceItem {
        id: "req-1".to_string(),
        requirement: "Data Protection Policy".to_string(),
        status: ComplianceStatus::Compliant,
        category: "Privacy".to_string(),
        notes: "Policy updated and approved".to_string(),
    },
    ComplianceItem {
        id: "req-2".to_string(),
        requirement: "Security Audit".to_string(),
        status: ComplianceStatus::PartiallyCompliant,
        category: "Security".to_string(),
        notes: "Audit in progress, 70% complete".to_string(),
    },
];

let visualizer = ComplianceDashboardVisualizer::new();
let html = visualizer.to_html(&items);
```

Features:
- **Status Tracking**: Compliant, Partially Compliant, Non-Compliant, Not Applicable
- **Summary Statistics**: Overall compliance rate and counts
- **Category Organization**: Group requirements by category
- **Visual Indicators**: Color-coded status badges
- **Detailed Notes**: Additional context for each requirement

## Advanced Performance Features (v0.1.8)

### WebWorker Rendering

Offload rendering to background threads for better performance:

```rust
use legalis_viz::WebWorkerConfig;

let config = WebWorkerConfig::new()
    .with_worker_count(4)
    .with_chunk_size(100);

let js_code = config.to_javascript();
```

Features:
- **Parallel Processing**: Utilize multiple web workers
- **Configurable Chunks**: Control data batch size
- **Automatic Merging**: Results automatically combined
- **Non-Blocking**: Main thread remains responsive

### Canvas Fallback

Automatically switch to Canvas rendering for large visualizations:

```rust
use legalis_viz::CanvasFallbackConfig;

let config = CanvasFallbackConfig::new()
    .with_threshold(1000);  // Use canvas for 1000+ nodes

let js_code = config.to_javascript();
```

Features:
- **Automatic Switching**: Based on node count threshold
- **Offscreen Canvas**: Optional offscreen rendering
- **Performance Optimized**: Hardware-accelerated when available
- **SVG Fallback**: Small graphs still use SVG

## Web Components (v0.1.6)

### Web Component Standard

Create reusable Web Components for visualizations:

```rust
use legalis_viz::WebComponentConfig;

let config = WebComponentConfig::new("legalis-viz-component")
    .without_shadow_dom();

let js_code = config.to_javascript("<div>My Visualization</div>");
```

Features:
- **Custom Elements**: Standard Web Component API
- **Shadow DOM**: Optional encapsulation
- **Auto-Registration**: Automatic custom element registration
- **Reactive Attributes**: Observable data and theme attributes
- **Framework Agnostic**: Works with any framework or vanilla JS

Usage:
```html
<legalis-viz-component data='{"nodes": [...]}' theme='dark'></legalis-viz-component>
```

## Framework Integration (v0.1.6)

### React Component Wrapper

Generate React components with TypeScript or JavaScript:

```rust
use legalis_viz::ReactComponentConfig;

// TypeScript React component
let config = ReactComponentConfig::new("LegalisViz");
let component_code = config.to_react_component();

// JavaScript React component with PropTypes
let config = ReactComponentConfig::new("LegalisViz")
    .without_typescript()
    .with_prop_types();
let component_code = config.to_react_component();
```

Features:
- **TypeScript Support**: Full TypeScript definitions with type-safe props
- **PropTypes Validation**: Optional PropTypes for JavaScript components
- **React Hooks**: Uses modern hooks (useEffect, useRef, useState)
- **Event Handling**: Supports onNodeClick callback
- **Error Boundaries**: Built-in error handling

Usage in React:
```tsx
import { LegalisViz } from './LegalisViz';

function App() {
  const data = { nodes: [...], edges: [...] };

  return (
    <LegalisViz
      data={data}
      theme="dark"
      width={1000}
      height={700}
      onNodeClick={(node) => console.log('Clicked:', node)}
    />
  );
}
```

### Vue.js Component Wrapper

Generate Vue components with Composition API or Options API:

```rust
use legalis_viz::VueComponentConfig;

// Vue 3 with Composition API and TypeScript
let config = VueComponentConfig::new("LegalisViz");
let component_code = config.to_vue_component();

// Vue 3 with Composition API (JavaScript)
let config = VueComponentConfig::new("LegalisViz")
    .without_typescript();
let component_code = config.to_vue_component();

// Vue 2 with Options API
let config = VueComponentConfig::new("LegalisViz")
    .with_options_api()
    .without_typescript();
let component_code = config.to_vue_component();
```

Features:
- **Composition API**: Modern Vue 3 composition API
- **Options API**: Classic Vue 2/3 options API
- **TypeScript Support**: Full TypeScript support with type definitions
- **Reactive Props**: Automatic re-rendering on prop changes
- **Event Emission**: Custom nodeClick event
- **Scoped Styles**: Component-scoped CSS

Usage in Vue:
```vue
<template>
  <LegalisViz
    :data="vizData"
    theme="light"
    :width="800"
    :height="600"
    @nodeClick="handleClick"
  />
</template>

<script setup>
import { ref } from 'vue';
import LegalisViz from './LegalisViz.vue';

const vizData = ref({ nodes: [...], edges: [...] });

function handleClick(node) {
  console.log('Clicked:', node);
}
</script>
```

### Angular Component Wrapper

Generate Angular components with TypeScript:

```rust
use legalis_viz::AngularComponentConfig;

let config = AngularComponentConfig::new(
    "LegalisVizComponent",
    "app-legalis-viz"
);

let (ts_code, html_code, css_code) = config.to_angular_component();

// Write to files:
// - legalis-viz.component.ts
// - legalis-viz.component.html
// - legalis-viz.component.css
```

Features:
- **Angular Lifecycle Hooks**: Implements OnInit and OnChanges
- **Input Properties**: Reactive @Input() decorators
- **Output Events**: @Output() event emitters
- **ViewChild**: Direct DOM access with ElementRef
- **TypeScript**: Full TypeScript support
- **Template Files**: Separate HTML and CSS files

Usage in Angular:
```typescript
import { Component } from '@angular/core';

@Component({
  selector: 'app-root',
  template: `
    <app-legalis-viz
      [data]="vizData"
      [theme]="'dark'"
      [width]="1000"
      [height]="700"
      (nodeClick)="onNodeClick($event)"
    ></app-legalis-viz>
  `
})
export class AppComponent {
  vizData = { nodes: [...], edges: [...] };

  onNodeClick(node: any) {
    console.log('Clicked:', node);
  }
}
```

### WordPress Plugin Integration

Generate WordPress plugin with shortcode support:

```rust
use legalis_viz::WordPressPluginConfig;

let config = WordPressPluginConfig::new("Legalis Visualization");
let plugin_php = config.to_wordpress_plugin();

// Custom shortcode
let config = WordPressPluginConfig::new("Legalis Visualization")
    .with_shortcode("legal_viz");
let plugin_php = config.to_wordpress_plugin();
```

Features:
- **Shortcode API**: WordPress shortcode for embedding visualizations
- **Script Enqueuing**: Proper WordPress script and style loading
- **Sanitization**: Built-in data sanitization for security
- **Customizable**: Configurable plugin name and shortcode
- **PHP Best Practices**: Follows WordPress coding standards

Usage in WordPress:
```php
// In WordPress editor or post:
[legalis_visualization_viz data='{"nodes":[...]}' theme='light' width='800' height='600']

// Custom shortcode example:
[legal_viz data='{"nodes":[...]}' theme='dark' width='1200' height='800']
```

Plugin structure:
```
legalis-visualization/
├── legalis-visualization.php  (Main plugin file)
├── js/
│   └── visualization.js       (JavaScript rendering logic)
└── css/
    └── visualization.css      (Visualization styles)
```

### Framework Comparison

| Feature | React | Vue | Angular | WordPress | Web Components |
|---------|-------|-----|---------|-----------|----------------|
| TypeScript | ✓ | ✓ | ✓ | - | - |
| Event Handling | ✓ | ✓ | ✓ | - | ✓ |
| Reactive Updates | ✓ | ✓ | ✓ | - | ✓ |
| SSR Compatible | ✓ | ✓ | ✓ | N/A | ✓ |
| Scoped Styles | CSS-in-JS | ✓ | ✓ | ✓ | Shadow DOM |
| Build Required | Yes | Yes | Yes | No | No |
| Framework Version | 16.8+ | 3.0+ | 12+ | 5.0+ | Native |
