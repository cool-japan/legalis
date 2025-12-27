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

## Roadmap for 0.1.0 Series

### Interactive Visualizations (v0.1.1)
- [ ] Add zoom and pan controls for large graphs
- [ ] Add node/edge hover tooltips
- [ ] Add click-to-expand for condition trees
- [ ] Add search and highlight functionality
- [ ] Add mini-map for navigation

### 3D Visualization (v0.1.2)
- [ ] Add 3D dependency graph using WebGL
- [ ] Add 3D timeline visualization
- [ ] Add VR/AR support for immersive exploration
- [ ] Add force-directed 3D layout
- [ ] Add depth-based coloring

### Accessibility (v0.1.3)
- [ ] Add WCAG 2.1 AA compliance
- [ ] Add screen reader descriptions
- [ ] Add keyboard navigation
- [ ] Add high contrast mode improvements
- [ ] Add reduced motion option

### Export Formats (v0.1.4)
- [ ] Add animated GIF export
- [ ] Add video export (MP4, WebM)
- [ ] Add print-optimized PDF
- [ ] Add vector PDF export
- [ ] Add poster-size output

### Real-Time Features (v0.1.5)
- [ ] Add live simulation visualization
- [ ] Add streaming data updates
- [ ] Add collaborative viewing
- [ ] Add annotation sharing
- [ ] Add cursor presence indicators

### Embedding (v0.1.6)
- [ ] Add React component wrapper
- [ ] Add Vue.js component wrapper
- [ ] Add Angular component wrapper
- [ ] Add Web Component standard
- [ ] Add WordPress plugin

### Theming (v0.1.7)
- [ ] Add custom theme builder
- [ ] Add organization branding support
- [ ] Add seasonal/event themes
- [ ] Add theme import/export
- [ ] Add CSS variable customization

### Performance (v0.1.8)
- [ ] Add virtualization for large datasets
- [ ] Add WebWorker rendering
- [ ] Add progressive loading
- [ ] Add level-of-detail rendering
- [ ] Add canvas fallback for performance

### Domain-Specific Visualizations (v0.1.9)
- [ ] Add court hierarchy visualization
- [ ] Add legislative process flowchart
- [ ] Add case citation network
- [ ] Add regulatory landscape map
- [ ] Add compliance status dashboard
