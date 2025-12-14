# legalis-verifier TODO

## Completed

### Core Features
- [x] StatuteVerifier with configurable principles
- [x] VerificationResult with errors, warnings, suggestions
- [x] Circular reference detection
- [x] Dead statute detection
- [x] Constitutional conflict checking
- [x] Logical contradiction detection
- [x] Ambiguity detection
- [x] Unreachable code detection (dead branches)
- [x] Default constitutional principles
- [x] Result merging for multiple checks

### Analysis & Verification
- [x] Improved circular reference detection with proper graph analysis
- [x] Redundant condition detection (using SMT solver)
- [x] Unreachable code/dead branch detection (using SMT solver)
- [x] Complexity metrics calculation
- [x] Severity classification (Info, Warning, Error, Critical)

### Reports
- [x] JSON report format (with serialization/deserialization)
- [x] HTML report generation (with CSS styling)
- [x] SARIF output for IDE integration
- [x] Severity filtering and counting

### Performance
- [x] Verification caching (with cache statistics)
- [x] Cache management (clear, size queries)

## SMT Solver Integration

- [x] Integrate Z3 solver via z3 crate
- [x] Implement condition-to-Z3 translation (all Condition types supported)
- [x] Add satisfiability checking for conditions
- [x] Create counterexample generation (get_model, get_multiple_models)
- [x] Add implication checking
- [x] Add equivalence checking
- [x] Add tautology verification
- [x] Add unsat core generation
- [x] Implement proof generation and export

## Verification Checks

### Static Analysis
- [x] Improve circular reference detection with proper graph analysis
- [x] Add unreachable code detection (dead branches with SMT solver)
- [x] Implement redundant condition detection (using SMT solver)
- [x] Add complexity metrics calculation
- [x] Create code coverage analysis for conditions

### Semantic Analysis
- [ ] Add semantic similarity detection
- [ ] Implement term disambiguation
- [ ] Add cross-reference validation
- [ ] Create terminology consistency checking

### Temporal Logic
- [ ] Add LTL (Linear Temporal Logic) support
- [ ] Implement CTL (Computation Tree Logic) checking
- [ ] Add deadline verification
- [ ] Implement sequence constraint checking

## Constitutional Principles

### Built-in Principles
- [ ] Add comprehensive equality checking
- [ ] Implement due process verification
- [ ] Add privacy impact assessment
- [ ] Implement proportionality checking
- [ ] Add accessibility verification

### Custom Principles
- [ ] Create principle definition DSL
- [ ] Add principle composition
- [ ] Implement jurisdictional rule sets
- [ ] Add principle priority/hierarchy

## Reports

### Output Formats
- [x] Add JSON report format (with serialization/deserialization)
- [x] Create HTML report generation (with CSS styling)
- [x] Implement SARIF output (IDE integration)
- [x] Add PDF report generation
- [ ] Create interactive report viewer

### Content
- [x] Add fix suggestions for errors (via suggestions field)
- [x] Implement severity classification (Info, Warning, Error, Critical)
- [x] Add severity filtering and counting
- [ ] Add related precedent references
- [ ] Create impact assessment reports

## Integration

- [ ] Add CI/CD integration guides
- [ ] Create pre-commit hooks
- [ ] Implement watch mode for continuous verification
- [ ] Add IDE plugin support (VSCode, IntelliJ)

## Performance

- [x] Add verification caching (with cache statistics)
- [x] Implement parallel verification (with rayon integration)
- [ ] Add incremental verification
- [ ] Create verification budget management
