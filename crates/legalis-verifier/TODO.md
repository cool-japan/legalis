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
- [x] Add semantic similarity detection
- [x] Implement term disambiguation
- [x] Add cross-reference validation
- [x] Create terminology consistency checking

### Temporal Logic
- [x] Add LTL (Linear Temporal Logic) support
- [x] Implement CTL (Computation Tree Logic) checking
- [x] Add deadline verification
- [x] Implement sequence constraint checking

## Constitutional Principles

### Built-in Principles
- [x] Add comprehensive equality checking
- [x] Implement due process verification
- [x] Add privacy impact assessment
- [x] Implement proportionality checking
- [x] Add accessibility verification
- [x] Implement non-retroactivity (ex post facto) checking

### Custom Principles
- [x] Create principle definition DSL
- [x] Add principle composition
- [x] Implement jurisdictional rule sets
- [x] Add principle priority/hierarchy

## Reports

### Output Formats
- [x] Add JSON report format (with serialization/deserialization)
- [x] Create HTML report generation (with CSS styling)
- [x] Implement SARIF output (IDE integration)
- [x] Add PDF report generation
- [x] Create interactive report viewer

### Content
- [x] Add fix suggestions for errors (via suggestions field)
- [x] Implement severity classification (Info, Warning, Error, Critical)
- [x] Add severity filtering and counting
- [x] Add related precedent references
- [x] Create impact assessment reports

## Integration

- [x] Add CI/CD integration guides
- [x] Create pre-commit hooks
- [x] Implement watch mode for continuous verification
- [x] Add IDE plugin support (VSCode, IntelliJ)

## Performance

- [x] Add verification caching (with cache statistics)
- [x] Implement parallel verification (with rayon integration)
- [x] Add incremental verification
- [x] Create verification budget management

## Recent Enhancements (December 2025)

### Non-Retroactivity Principle Check
- [x] Implemented comprehensive `check_retroactivity()` function
- [x] Detects retroactive application of prohibitions, obligations, and revocations
- [x] Checks for explicit retroactive language in effect descriptions
- [x] Validates effect parameters for retroactive flags
- [x] Compares application dates with effective dates
- [x] Ensures effective dates don't precede enactment dates
- [x] Special handling for monetary penalties and fines
- [x] Suggests grace periods for compliance
- [x] Added 8 comprehensive unit tests covering various retroactivity scenarios
- [x] Integrated with `PrincipleCheck::NoRetroactivity` in jurisdiction verification

### Statute Conflict Detection System
- [x] Implemented `ConflictType` enum with 5 conflict categories
  - Effect Conflicts (contradictory effects with overlapping conditions)
  - Jurisdictional Overlaps (multiple statutes in same jurisdiction)
  - Temporal Conflicts (overlapping validity periods with different versions)
  - Hierarchy Violations (lower-level contradicting higher-level)
  - ID Collisions (duplicate statute IDs)
- [x] Created `StatuteConflict` struct with severity classification and resolution suggestions
- [x] Implemented `detect_statute_conflicts()` main detection function
- [x] Added `detect_id_collisions()` for duplicate ID detection
- [x] Added `detect_effect_conflicts()` for contradictory effect detection
- [x] Added `detect_jurisdictional_overlaps()` for jurisdiction analysis
- [x] Added `detect_temporal_conflicts()` for temporal overlap detection
- [x] Implemented helper functions:
  - `temporal_validity_overlaps()` - checks temporal period overlap
  - `conditions_overlap()` - detects overlapping preconditions
  - `effects_contradict()` - identifies contradictory effects
  - `title_similarity()` - Jaccard similarity for statute titles
- [x] Created `conflict_detection_report()` for generating detailed conflict reports
- [x] Added 10 comprehensive unit tests for conflict detection
- [x] All tests passing (83 total)
- [x] No compiler or clippy warnings
