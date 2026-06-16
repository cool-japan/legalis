# legalis-cli TODO

## Status Summary

Version: 0.2.6 | Status: Stable | Tests: Passing (324) | Warnings: 0

All v0.1.1 through v0.2.6 roadmap features are complete. v0.2.0 AI-powered CLI, v0.2.1 Interactive TUI, v0.2.2 Workflow Automation, v0.2.3 Cloud Integration, v0.2.4 Collaboration Features, v0.2.5 Performance Profiling, and v0.2.6 Offline Capabilities have been implemented and tested. A subset of v0.2.7–v0.3.4 (enterprise, UX, self-healing) has been implemented; see the dated section below.

## COMPLETED (2026-06-14 — enterprise/UX/self-healing subset)

Implemented an additive, backward-compatible enterprise / UX / self-healing
feature set (no warnings; `cargo clippy -p legalis --all-targets -- -D warnings`
clean; 324 tests passing). New modules and the commands wiring them in:

- `src/verbosity.rs` — ordered `Verbosity` levels (silent → trace) resolved from
  `--verbosity` / `-v` / `--quiet` / `LEGALIS_VERBOSITY`, a process-global
  accessor, and `tracing` directive mapping. Honored by new text output.
- `src/theme.rs` (extended) + `ColorTheme::HighContrast` — high-contrast theme
  and `configure_colors()` honoring `--theme none`, `NO_COLOR`, `CLICOLOR_FORCE`.
- `src/paths.rs` — single overridable state root (`LEGALIS_DATA_DIR`) for audit
  log, usage stats, and checkpoints.
- `src/audit_log.rs` — hash-chained JSONL audit trail over `legalis-audit`
  recording every CLI operation (actor, command, args, outcome) with integrity
  verification.
- `src/compliance.rs` — compliance mode (`--compliance` / `LEGALIS_COMPLIANCE`)
  that guards sensitive operations and forces audit logging.
- `src/policy.rs` — enterprise policy (TOML/JSON/YAML): allow/deny command lists,
  numeric limits, require-compliance/require-audit, discovery + validation.
- `src/central_config.rs` — layered config (defaults → central → file → env →
  flags) with provenance tracking and strict validation.
- `src/suggest.rs` — usage learning (counts + Markov transitions), contextual
  next-command suggestions, and proactive recommendations.
- `src/recovery.rs` — transient-failure classifier + exponential-backoff retry
  engine (injectable sleeper); wired around mandatory audit-log opens.
- `src/diagnostics.rs` — structured self-diagnostics, configuration repair, and
  crash-recovery checkpoints (resume after interruption; wired into offline sync).
- `src/plugin_security.rs` — pure-Rust SemVer + requirement language, plugin
  dependency validation/topological resolution (cycle + missing detection), and
  a plugin security scanner.
- `src/commands/enterprise.rs` — handlers for `audit-log`, `policy`,
  `central-config`, `assistant`, `diagnose`, `repair`, `recover`, plus the
  cross-cutting policy/compliance gate, usage recording, and audit recording
  invoked from `main.rs` around every command.
- New `PluginOperation` subcommands `scan` / `deps` / `check-version` handled in
  `src/commands/registry_plugin_config.rs`.

Cross-cutting integration in `main.rs`: every invocation is policy-gated,
compliance-guarded, usage-recorded, and audit-logged (mandatory under compliance
or `require_audit_log`, best-effort otherwise).

Deferred items (need speech/LLM, AR/VR hardware, cluster infra, an update/SSO/
marketplace service, or are unsafe to auto-run) are marked inline below with a
one-line reason.

### Completed 2026-06-14: Performance Profiling (v0.2.5) + Offline Capabilities (v0.2.6)

- `src/profiling.rs` — high-resolution, allocation-aware micro-profiler (distinct
  from the RSS-based `src/perf.rs`). Key types: `TrackingAllocator<A>` (a pure-Rust
  instrumented `GlobalAlloc` installed as the process `#[global_allocator]` in
  `main.rs`), `MemorySource` trait with `AllocatorSource`/`RssMemorySource`/
  `ManualMemorySource`, `Profiler` (phase-scoped `measure`/`try_measure`),
  `DurationStats` (mean/stddev + linear-interpolated p50/p90/p95/p99),
  `PhaseProfile`, `PhaseBottleneck`, `OptimizationHint`, and `ProfileReport`
  rendered through every `OutputFormat` (text/json/yaml/toml/csv/table/html).
  14 unit tests.
- `src/offline.rs` — offline-first subsystem. Key types: `CommandQueue`/
  `QueuedCommand` (file-backed JSON queue), `LocalCache`/`CacheRecord` (versioned
  TTL cache), `ConnectivityProbe` (`AlwaysOnline`/`AlwaysOffline`/`TcpProbe`),
  offline `validate_command`, `JournalApplier` (file-backed authoritative store),
  `OfflineStore::sync` reconciler with version-based conflict detection,
  `ConflictStrategy` (last-writer-wins / remote-wins / merge), a recursive
  three-way JSON `merge_three_way`, and explicit `ConflictRecord`s. 18 unit tests.
- `src/commands/profiling_offline.rs` — CLI handlers wiring the new `Profiling`
  and `Offline` subcommands (queue/list/validate/sync/conflicts/resolve/
  cache-stats/cache-prune/clear) into `main.rs`.
- 32 new tests (167 -> 199); `cargo clippy -p legalis --no-deps --all-targets
  -- -D warnings` clean.

---

## Completed

- [x] Parse command with JSON/YAML output
- [x] Verify command with strict mode
- [x] Visualization command (Mermaid, DOT, ASCII)
- [x] Export command (RDF, Solidity, WASM)
- [x] Serve command for API server
- [x] Init command for project scaffolding
- [x] Diff command for statute comparison
- [x] Simulate command with async execution
- [x] Audit command for complexity analysis
- [x] Complexity analysis command
- [x] Shell completion generation
- [x] Verbosity control with logging levels

## Commands

### New Commands
- [x] Add `lint` command for style checking
- [x] Create `fmt` command for formatting (implemented as `format` command)
- [x] Add `watch` command for file watching
- [x] Implement `repl` command for interactive mode
- [x] Add `test` command for statute testing
- [x] Create `publish` command for registry upload

### Enhancements
- [x] Add `--dry-run` flag for modify commands (init, format)
- [x] Implement `--interactive` mode for guided input
- [x] Add `--quiet` mode for scripting
- [x] Create `--json-output` for machine parsing (already exists via --format)
- [x] Add `--config` flag for config file

## Configuration

- [x] Add config file support (legalis.toml)
- [x] Implement project-level configuration
- [x] Create user-level global configuration
- [x] Add environment variable overrides (LEGALIS_*)
- [x] Support config inheritance

## Output

### Formatting
- [x] Add colored output with styles (for verify and lint commands)
- [x] Implement table output format (--format table)
- [x] Create progress bars for long operations
- [x] Add spinner for async operations
- [x] Implement diff highlighting

### Formats
- [x] Add YAML output support
- [x] Implement TOML output
- [x] Create CSV output for reports
- [x] Add HTML output option

## Project Management

- [x] Add `new` command for statute templates (with 5 template types)
- [x] Create `add` command for dependencies
- [x] Implement `update` command for updates
- [x] Add `clean` command for cache cleanup
- [x] Create `doctor` command for diagnostics

## Registry Integration

- [x] Add `search` command for registry
- [x] Implement `install` command for statutes
- [x] Create `list` command for installed statutes
- [x] Add `outdated` command for updates
- [x] Implement `uninstall` command

## Interop

- [x] Add `import` command for format conversion
- [x] Create `convert` command for bidirectional conversion
- [x] Implement `validate` command for format validation

## Developer Experience

- [x] Add man page generation
- [x] Create interactive tutorials
- [x] Implement error suggestions
- [x] Add command aliases
- [x] Create plugin system

## Performance

- [x] Add command caching
- [x] Implement lazy loading
- [x] Create parallel processing for batch ops
- [x] Add progress estimation

## Testing

- [x] Add integration tests for all commands
- [x] Create snapshot tests for output
- [x] Implement CLI contract tests
- [x] Add performance benchmarks

## Roadmap for 0.1.0 Series

### Enhanced Commands (v0.1.1)
- [x] Add `explain` command - explain statute in natural language
- [x] Add `trace` command - trace condition evaluation path
- [x] Add `benchmark` command - benchmark verification/simulation
- [x] Add `migrate` command - migrate statutes between versions
- [x] Add `graph` command - generate dependency graphs

### Interactive Features (v0.1.2)
- [x] Add interactive statute builder wizard
- [x] Add interactive conflict resolution UI
- [x] Add interactive diff viewer with accept/reject
- [x] Add interactive simulation parameter tuning
- [x] Add TUI dashboard for registry browsing

### Batch Operations (v0.1.3)
- [x] Add `batch` subcommand for bulk operations
- [x] Add parallel processing for batch verify
- [x] Add progress bars with ETA for long operations
- [x] Add resumable batch operations
- [x] Add batch operation journaling for recovery

### Profile & Debug (v0.1.4)
- [x] Add `profile` command - CPU/memory profiling
- [x] Add `debug` command - step-through evaluation
- [x] Add `flamegraph` output for performance analysis
- [x] Add memory usage reporting
- [x] Add timing breakdown for complex operations

### Registry Commands (v0.1.5)
- [x] Add `registry push` - push statute to registry
- [x] Add `registry pull` - pull statutes from registry
- [x] Add `registry diff` - diff local vs remote
- [x] Add `registry sync` - synchronize with registry
- [x] Add `registry login/logout` - authentication

### Plugin System (v0.1.6)
- [x] Add plugin discovery and loading
- [x] Add plugin manifest format
- [x] Add plugin sandboxing
- [x] Add built-in plugin manager commands
- [x] Add plugin hook points (pre-parse, post-verify, etc.)

### Output Enhancements (v0.1.7)
- [x] Add `--theme` flag for color schemes
- [x] Add `--no-emoji` flag for terminals without emoji
- [x] Add `--width` flag for output width control
- [x] Add pager integration for long outputs
- [x] Add structured logging output mode

### Configuration (v0.1.8)
- [x] Add profile support (dev, staging, prod)
- [x] Add remote configuration sources
- [x] Add configuration validation command
- [x] Add configuration diff between profiles
- [x] Add environment-specific overrides

### Scripting (v0.1.9)
- [x] Add Lua scripting for custom commands
- [x] Add script execution with sandbox
- [x] Add built-in script library
- [x] Add script debugging support
- [x] Add script package manager

## Roadmap for 0.2.0 Series

### AI-Powered CLI (v0.2.0)
- [x] Add natural language command parsing
- [x] Implement AI-suggested commands
- [x] Add intelligent autocomplete
- [x] Create AI-powered help system
- [x] Add command intent recognition

### Interactive TUI (v0.2.1)
- [x] Add full-featured TUI dashboard
- [x] Implement keyboard shortcuts customization
- [x] Add mouse support for navigation
- [x] Create split-pane views
- [x] Add resizable panels

### Workflow Automation (v0.2.2)
- [x] Add workflow definition files
- [x] Implement task pipelines
- [x] Add conditional execution
- [x] Create parallel task execution
- [x] Add workflow templates library

### Cloud Integration (v0.2.3)
- [x] Add AWS CLI integration
- [x] Implement Azure CLI integration
- [x] Add GCP CLI integration
- [x] Create multi-cloud management
- [x] Add cloud resource provisioning

### Collaboration Features (v0.2.4)
- [x] Add team workspace support
- [x] Implement shared command history
- [x] Add collaborative sessions
- [x] Create team notification system
- [x] Add role-based command access

### Performance Profiling (v0.2.5)
- [x] Add command execution profiling
- [x] Implement memory usage tracking
- [x] Add bottleneck detection
- [x] Create performance reports
- [x] Add optimization suggestions

### Offline Capabilities (v0.2.6)
- [x] Add offline command queue
- [x] Implement local caching
- [x] Add sync when online
- [x] Create offline validation
- [x] Add conflict resolution for offline changes

### Accessibility (v0.2.7)
- [ ] Add screen reader support — DEFERRED: needs a screen-reader/AT integration layer (no pure-Rust facility available here)
- [x] Implement high contrast mode
- [ ] Add keyboard-only navigation
- [ ] Create voice command support — DEFERRED: requires speech recognition
- [x] Add customizable output verbosity

### Plugin Ecosystem (v0.2.8)
- [ ] Add plugin marketplace — DEFERRED: requires an external marketplace service
- [x] Implement plugin versioning
- [x] Add plugin dependency management
- [ ] Create plugin development kit
- [x] Add plugin security scanning

### Enterprise Features (v0.2.9)
- [ ] Add SSO authentication — DEFERRED: requires an external identity provider (OIDC/SAML)
- [x] Implement audit logging
- [x] Add compliance mode
- [x] Create enterprise policy enforcement
- [x] Add centralized configuration management

## Roadmap for 0.3.0 Series (Next-Gen Features)

### Voice-First CLI (v0.3.0)
- [ ] Add voice command input — DEFERRED: requires speech recognition
- [ ] Implement voice feedback output — DEFERRED: requires text-to-speech
- [ ] Add multilingual voice support — DEFERRED: requires speech recognition
- [ ] Create hands-free operation mode — DEFERRED: requires speech/voice stack
- [ ] Add voice command training — DEFERRED: requires speech recognition

### Intelligent Assistant (v0.3.1)
- [x] Add contextual command suggestions
- [x] Implement learning from user patterns
- [x] Add proactive recommendations
- [ ] Create predictive command execution — DEFERRED: auto-running commands without explicit user intent is unsafe
- [ ] Add natural conversation mode — DEFERRED: requires an LLM/conversational backend

### AR/VR Integration (v0.3.2)
- [ ] Add AR command overlay — DEFERRED: requires AR/VR hardware
- [ ] Implement VR workspace — DEFERRED: requires AR/VR hardware
- [ ] Add spatial command organization — DEFERRED: requires AR/VR hardware
- [ ] Create gesture-based commands — DEFERRED: requires AR/VR hardware
- [ ] Add immersive documentation — DEFERRED: requires AR/VR hardware

### Distributed CLI (v0.3.3)
- [ ] Add multi-node command execution — DEFERRED: requires cluster infrastructure
- [ ] Implement distributed workflows — DEFERRED: requires cluster infrastructure
- [ ] Add cluster management — DEFERRED: requires cluster infrastructure
- [ ] Create edge computing support — DEFERRED: requires edge/cluster infrastructure
- [ ] Add federated command routing — DEFERRED: requires cluster infrastructure

### Self-Healing CLI (v0.3.4)
- [x] Add automatic error recovery
- [x] Implement self-diagnostic commands
- [ ] Add automatic updates — DEFERRED: requires an update server
- [x] Create configuration repair
- [x] Add crash recovery and resume
