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
- [ ] PowerPoint/Keynote integration
- [ ] Embedding in documents
- [ ] Animation support for presentations

## Integration

- [x] Real-time updates from simulation (LiveVisualization with WebSocket support)
- [x] Integration with web frontends (HTML export with embedded JavaScript)
- [x] Plugin system for custom renderers (Renderer trait + RendererRegistry)

## Testing

- [ ] Visual regression tests
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
