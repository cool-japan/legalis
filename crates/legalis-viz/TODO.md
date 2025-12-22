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
