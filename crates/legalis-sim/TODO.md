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
