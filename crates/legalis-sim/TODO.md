# legalis-sim TODO

## Completed

- [x] SimEngine with async parallel execution
- [x] Entity trait for simulation subjects
- [x] BasicEntity with typed attributes
- [x] Population creation and management
- [x] Statute application to populations
- [x] SimMetrics for result aggregation
- [x] Rayon-based parallel processing
- [x] Deterministic vs discretionary tracking
- [x] Add time-step based simulation
- [x] Implement statute effective dates
- [x] Add agent lifecycle (birth, death, status changes)
- [x] Implement agent decision models
- [x] Add compliance probability modeling
- [x] Create evasion behavior simulation
- [x] Add agent learning/adaptation
- [x] Add inter-agent relationships (family, employer)
- [x] Implement organization hierarchies
- [x] Add property/asset relationships
- [x] Add realistic demographic distributions
- [x] Support geographic distribution
- [x] Implement income distribution models
- [x] Add configurable attribute generators
- [x] Add realistic correlation enforcement
- [x] Add statute version comparison
- [x] Implement A/B testing framework
- [x] Create sensitivity analysis tools
- [x] Add large-scale simulation benchmarks
- [x] Support retroactive law application
- [x] Add what-if scenario modeling
- [x] Add distribution analysis (normal, power law)
- [x] Implement correlation detection
- [x] Add time-series analysis
- [x] Add future projection simulation
- [x] Implement agent communication
- [x] Support contract relationships
- [x] Create cohort analysis tools
- [x] Support CSV/JSON population import
- [x] Create reproducible random tests

## Engine Features

### Temporal Simulation
(All temporal simulation features completed)

### Agent Behavior
(All basic agent behavior features completed)

### Relationships
- [x] Create relationship-based conditions

## Performance

### Parallelization
- [x] Optimize work distribution across threads
- [x] Implement batch processing for large populations
- [x] Add SIMD optimizations where applicable
- [x] Support distributed simulation (multi-node)

### Memory
- [x] Add memory-efficient streaming for large populations
- [x] Implement entity pooling/recycling
- [x] Add lazy attribute evaluation
- [x] Support memory-mapped populations

### Incremental
- [x] Implement dirty tracking for re-simulation
- [x] Add delta-based updates
- [x] Support checkpoint/restore
- [x] Create simulation replay

## Metrics & Analysis

### Statistics
(All statistics features completed)

### Visualization
- [x] Export to GraphViz format
- [x] Create D3.js compatible output
- [x] Add geographic visualization support
- [x] Implement interactive dashboards

## Population

### Generation
(All generation features completed)

### Validation
- [x] Add population consistency checks
- [x] Implement constraint satisfaction

## Testing

- [x] Add stress testing for memory limits
- [x] Implement simulation verification tests

## Enhancements (2025)

### Error Handling
- [x] Add comprehensive error types (`SimulationError`)
- [x] Add `SimResult<T>` type alias for ergonomic error handling
- [x] Proper error propagation throughout the crate

### Builder Patterns
- [x] Add `SimEngineBuilder` for flexible simulation configuration
- [x] Support incremental builder methods (add_statute, add_entity, etc.)
- [x] Validation support with configurable on/off
- [x] Comprehensive builder tests

### Utility Functions
- [x] Add `aggregate_metrics()` for combining multiple simulation runs
- [x] Add `compare_metrics()` for A/B testing and analysis
- [x] Add `MetricsDiff` type for metric comparisons
- [x] Add `SummaryStats` for statistical summaries
- [x] Add `StatuteMetrics::merge()` for metric aggregation

### Code Quality
- [x] Fix all clippy warnings (NO warnings policy)
- [x] Comprehensive test coverage (95 tests)
- [x] All doc tests passing

## Advanced Features (2025-Q4)

### Monte Carlo Simulation
- [x] Add Monte Carlo runner for probabilistic analysis
- [x] Implement confidence interval calculation
- [x] Add convergence detection
- [x] Support parallel Monte Carlo runs
- [x] Add variance reduction techniques
- [x] Update tests to match legalis-core API

### Economic Modeling
- [x] Add tax revenue projection tools (data structures)
- [x] Implement compliance cost calculation (data structures)
- [x] Add economic impact assessment (data structures)
- [x] Support cost-benefit analysis (NPV, IRR, BCR)
- [x] Add budget impact modeling (data structures)
- [x] Implement distributional analysis framework
- [x] Complete implementations to work with current API
- [x] Add comprehensive tests

### Network Effects
- [x] Add influence model configuration
- [x] Implement diffusion model types
- [x] Add centrality metrics data structures
- [x] Add diffusion result structures
- [ ] Adapt to work with UUID-based RelationshipGraph API
- [ ] Implement full network algorithms

### Policy Optimization
- [x] Add parameter optimization framework
- [x] Implement gradient-free optimization (Nelder-Mead)
- [x] Support grid search optimization
- [x] Add Pareto frontier analysis
- [x] Implement constraint-based optimization
- [x] Add comprehensive tests

### Calibration & Validation
- [x] Add parameter calibration tools
- [x] Implement goodness-of-fit metrics (MSE, RMSE, MAE, R²)
- [x] Support empirical data fitting
- [x] Add cross-validation framework
- [x] Implement sensitivity testing
- [x] Add comprehensive tests

### Impact Assessment
- [x] Add structured impact report generation
- [x] Implement equity analysis tools
- [x] Support distributional impact analysis
- [x] Add compliance burden metrics
- [x] Create regulatory impact templates
- [ ] Fix API compatibility issues
- [ ] Add comprehensive tests

### Event-Driven Simulation
- [x] Add discrete event simulation support
- [x] Implement event queue and priority handling
- [x] Support hybrid time-step/event-driven
- [x] Add event logging and replay
- [x] Implement event-based triggers
- [x] Complete all event-driven tests

## Implementation Notes

### API Compatibility
Several new modules (economic, impact) have data structures in place but require
adaptation to work with the current legalis-core API:
- BasicEntity API changed (no-arg constructor, String-only attributes)
- Need to parse String attributes to f64/other types
- Full implementations pending API clarification

### Test Status
- Monte Carlo: Full tests passing (5 tests)
- Economic: Full tests passing (8 tests)
- Network Effects: Basic tests passing (data structures only)
- Optimization: Full tests passing (10 tests)
- Calibration: Full tests passing (11 tests)
- Impact: Tests disabled pending API fixes
- Event-Driven: Full tests passing

### Overall Test Statistics (as of 2025-12-26)
- Total tests: 142 passing, 1 ignored
- All clippy warnings resolved
- All doc tests passing (5 tests)
- NO WARNINGS policy maintained
