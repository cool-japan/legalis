# legalis-sim TODO

## Engine Features

### Temporal Simulation
- [ ] Add time-step based simulation
- [ ] Implement statute effective dates
- [ ] Add agent lifecycle (birth, death, status changes)
- [ ] Support retroactive law application
- [ ] Add future projection simulation

### Agent Behavior
- [ ] Implement agent decision models
- [ ] Add compliance probability modeling
- [ ] Create evasion behavior simulation
- [ ] Add agent learning/adaptation
- [ ] Implement agent communication

### Relationships
- [ ] Add inter-agent relationships (family, employer)
- [ ] Implement organization hierarchies
- [ ] Add property/asset relationships
- [ ] Support contract relationships
- [ ] Create relationship-based conditions

## Performance

### Parallelization
- [ ] Optimize work distribution across threads
- [ ] Implement batch processing for large populations
- [ ] Add SIMD optimizations where applicable
- [ ] Support distributed simulation (multi-node)

### Memory
- [ ] Add memory-efficient streaming for large populations
- [ ] Implement entity pooling/recycling
- [ ] Add lazy attribute evaluation
- [ ] Support memory-mapped populations

### Incremental
- [ ] Implement dirty tracking for re-simulation
- [ ] Add delta-based updates
- [ ] Support checkpoint/restore
- [ ] Create simulation replay

## Metrics & Analysis

### Statistics
- [ ] Add distribution analysis (normal, power law)
- [ ] Implement correlation detection
- [ ] Add time-series analysis
- [ ] Create cohort analysis tools

### Visualization
- [ ] Export to GraphViz format
- [ ] Create D3.js compatible output
- [ ] Add geographic visualization support
- [ ] Implement interactive dashboards

### Comparison
- [ ] Add statute version comparison
- [ ] Implement A/B testing framework
- [ ] Create sensitivity analysis tools
- [ ] Add what-if scenario modeling

## Population

### Generation
- [ ] Add realistic demographic distributions
- [ ] Support geographic distribution
- [ ] Implement income distribution models
- [ ] Add configurable attribute generators
- [ ] Support CSV/JSON population import

### Validation
- [ ] Add population consistency checks
- [ ] Implement constraint satisfaction
- [ ] Add realistic correlation enforcement

## Testing

- [ ] Add large-scale simulation benchmarks
- [ ] Create reproducible random tests
- [ ] Add stress testing for memory limits
- [ ] Implement simulation verification tests
