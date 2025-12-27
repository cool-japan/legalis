# legalis-diff TODO

## Completed

- [x] Structural diff between statutes
- [x] Change categorization (added/removed/modified)
- [x] Impact assessment with severity levels
- [x] Basic change reports

## Features

- [x] Semantic diff (understanding meaning changes)
- [x] Cross-statute impact analysis
- [x] Amendment chain visualization
- [x] Diff output in multiple formats (JSON, HTML, Markdown)
- [x] Side-by-side comparison view

## Advanced Analysis

- [x] Detect logically equivalent changes
- [x] Identify breaking vs non-breaking changes
- [x] Track condition relaxation/tightening
- [x] Analyze effect scope changes

## Merge Support

- [x] Add three-way merge for concurrent amendments
- [x] Implement conflict detection and resolution
- [x] Support merge strategies (ours, theirs, union)

## Visualization

- [x] Generate visual diff reports
- [x] Timeline visualization for amendments
- [x] Blame-style annotation for change tracking

## Integration

- [x] Git-style diff interface
- [x] Hook into version control systems
- [x] Create diff templates for common patterns

## Testing

- [x] Add comprehensive diff test cases
- [x] Test edge cases (empty statutes, identical statutes)
- [x] Benchmark diff performance on large statutes

## Performance & Optimization

- [x] Implement diff caching and memoization
- [x] Add incremental diff support
- [x] Create batch diff computation
- [x] Optimize for repeated diffs

## Advanced Algorithms

- [x] Implement Myers diff algorithm
- [x] Implement Patience diff algorithm
- [x] Add edit distance calculation
- [x] Support for advanced diff operations

## Statistical Analysis

- [x] Add statistical analysis of changes
- [x] Implement change pattern detection
- [x] Create aggregate statistics across multiple diffs
- [x] Generate statistical summaries and reports

## Enhanced Error Handling

- [x] Add specific error variants for different scenarios
- [x] Implement version conflict detection
- [x] Add merge conflict error types
- [x] Support serialization error handling

## Fuzzy Matching & Similarity

- [x] Implement Levenshtein distance calculation
- [x] Add similarity scoring between changes
- [x] Find similar changes across multiple diffs
- [x] Group similar changes by pattern
- [x] Support configurable similarity thresholds

## Change Recommendation System

- [x] Implement recommendation generation based on patterns
- [x] Add priority levels (Low, Medium, High, Critical)
- [x] Support multiple recommendation categories
- [x] Provide confidence scores for recommendations
- [x] Filter and sort recommendations
- [x] Detect common pitfalls in amendments
- [x] Analyze historical patterns for suggestions

## Enhanced Summarization

- [x] Add detailed summary with confidence scores
- [x] Provide change detection confidence metrics
- [x] Include impact assessment confidence
- [x] Generate analytical insights
- [x] Break down changes by type (added/removed/modified)

## Partial Comparison Support

- [x] Compare only preconditions between statutes
- [x] Compare only effects between statutes
- [x] Support targeted diff operations
- [x] Reduce computational overhead for partial comparisons

## Parallel Processing

- [x] Implement parallel diff computation using rayon
- [x] Add batch diff operations for multiple statute pairs
- [x] Support parallel sequence diffing
- [x] Add parallel processing for multiple sequences

## Rollback Analysis

- [x] Generate rollback diffs (reverse changes)
- [x] Analyze rollback feasibility and complexity
- [x] Identify rollback risks and issues
- [x] Provide rollback recommendations
- [x] Support rollback chain generation

## Change Validation

- [x] Validate diff completeness and consistency
- [x] Detect inconsistent change data
- [x] Verify impact assessment accuracy
- [x] Check for duplicate changes
- [x] Provide validation scores and warnings

## Export Formats

- [x] CSV export for spreadsheet analysis
- [x] Batch CSV export for multiple diffs
- [x] Proper CSV escaping for special characters

## Integrated Analysis & Batch Operations

- [x] Parallel batch validation using rayon
- [x] Batch validation summaries with aggregate statistics
- [x] Failed statute tracking in batch operations
- [x] Average validation score calculation
- [x] Integration of validation with parallel processing

## Performance Benchmarks

- [x] Parallel diff pair benchmarks
- [x] Batch validation benchmarks
- [x] Parallel validation benchmarks
- [x] Rollback generation benchmarks
- [x] Rollback analysis benchmarks
- [x] Parallel rollback generation benchmarks
- [x] Parallel rollback analysis benchmarks
- [x] Rollback statistics computation benchmarks
- [x] Parallel rollback validation benchmarks

## Parallel Rollback Operations

- [x] Parallel rollback diff generation
- [x] Parallel rollback feasibility analysis
- [x] Batch rollback operations with rayon
- [x] Performance optimization for large-scale rollback processing

## Rollback Statistics

- [x] Aggregate statistics across multiple rollback analyses
- [x] Complexity distribution tracking
- [x] Risk distribution analysis
- [x] Average recommendations calculation
- [x] Feasibility metrics

## Rollback Validation

- [x] Validate rollback diffs against forward diffs
- [x] Ensure proper value reversal
- [x] Target consistency checking
- [x] Parallel rollback validation
- [x] Integration with existing validation framework

## Parallel Export Operations

- [x] Parallel export to multiple formats
- [x] Batch export with format selection
- [x] Export to all formats simultaneously
- [x] Single diff multi-format export
- [x] ExportFormat enum for type-safe format selection

## Roadmap for 0.1.0 Series

### Semantic Diff Improvements (v0.1.1)
- [x] Add semantic equivalence detection (same meaning, different syntax)
- [x] Add intent-preserving refactoring detection
- [x] Add condition relaxation/tightening metrics
- [x] Add effect scope change quantification
- [x] Add breaking change classification

### Advanced Merge (v0.1.2)
- [x] Add semantic merge for compatible changes
- [x] Add conflict resolution suggestions
- [x] Add merge preview with impact assessment
- [x] Add interactive merge mode
- [x] Add merge history tracking

### Change Analysis (v0.1.3)
- [x] Add change impact scoring (0-100 scale)
- [x] Add stakeholder impact analysis
- [x] Add regulatory compliance impact
- [x] Add backward compatibility scoring
- [x] Add migration effort estimation

### Visualization Enhancements (v0.1.4)
- [ ] Add interactive HTML diff viewer
- [ ] Add syntax-highlighted diff output
- [ ] Add inline annotations for change explanations
- [ ] Add diff animation for presentations
- [ ] Add three-way diff visualization

### Pattern Recognition (v0.1.5)
- [ ] Add common amendment pattern library
- [ ] Add pattern-based change suggestions
- [ ] Add anti-pattern detection
- [ ] Add best practice recommendations
- [ ] Add historical pattern learning

### Audit Trail (v0.1.6)
- [ ] Add change attribution (who changed what)
- [ ] Add change justification tracking
- [ ] Add approval workflow integration
- [ ] Add change lifecycle tracking (proposed → approved → enacted)
- [ ] Add rollback planning from diffs

### Performance (v0.1.7)
- [ ] Add streaming diff for large statutes
- [ ] Add incremental diff updates
- [ ] Add diff result caching
- [ ] Add parallel diff computation with SIMD
- [ ] Add memory-efficient diff algorithms

### Export Formats (v0.1.8)
- [ ] Add Word track-changes format
- [ ] Add PDF with highlighted changes
- [ ] Add LaTeX redline format
- [ ] Add unified diff format (patch files)
- [ ] Add structured changelog (CHANGELOG.md)

### Integration (v0.1.9)
- [ ] Add Git integration for version control
- [ ] Add GitHub/GitLab PR diff integration
- [ ] Add notification webhooks for changes
- [ ] Add diff-based CI/CD triggers
- [ ] Add diff API for external tools
