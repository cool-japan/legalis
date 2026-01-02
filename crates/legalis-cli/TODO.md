# legalis-cli TODO

## Status Summary

Version: 0.2.0 | Status: Stable | Tests: Passing | Warnings: 0

All v0.1.1 through v0.1.5 roadmap features are complete. Plugin ecosystem (v0.1.6+) and AI-powered features (v0.2.0+) are planned for future releases.

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
- [ ] Add plugin discovery and loading
- [ ] Add plugin manifest format
- [ ] Add plugin sandboxing
- [ ] Add built-in plugin manager commands
- [ ] Add plugin hook points (pre-parse, post-verify, etc.)

### Output Enhancements (v0.1.7)
- [ ] Add `--theme` flag for color schemes
- [ ] Add `--no-emoji` flag for terminals without emoji
- [ ] Add `--width` flag for output width control
- [ ] Add pager integration for long outputs
- [ ] Add structured logging output mode

### Configuration (v0.1.8)
- [ ] Add profile support (dev, staging, prod)
- [ ] Add remote configuration sources
- [ ] Add configuration validation command
- [ ] Add configuration diff between profiles
- [ ] Add environment-specific overrides

### Scripting (v0.1.9)
- [ ] Add Lua scripting for custom commands
- [ ] Add script execution with sandbox
- [ ] Add built-in script library
- [ ] Add script debugging support
- [ ] Add script package manager

## Roadmap for 0.2.0 Series

### AI-Powered CLI (v0.2.0)
- [ ] Add natural language command parsing
- [ ] Implement AI-suggested commands
- [ ] Add intelligent autocomplete
- [ ] Create AI-powered help system
- [ ] Add command intent recognition

### Interactive TUI (v0.2.1)
- [ ] Add full-featured TUI dashboard
- [ ] Implement keyboard shortcuts customization
- [ ] Add mouse support for navigation
- [ ] Create split-pane views
- [ ] Add resizable panels

### Workflow Automation (v0.2.2)
- [ ] Add workflow definition files
- [ ] Implement task pipelines
- [ ] Add conditional execution
- [ ] Create parallel task execution
- [ ] Add workflow templates library

### Cloud Integration (v0.2.3)
- [ ] Add AWS CLI integration
- [ ] Implement Azure CLI integration
- [ ] Add GCP CLI integration
- [ ] Create multi-cloud management
- [ ] Add cloud resource provisioning

### Collaboration Features (v0.2.4)
- [ ] Add team workspace support
- [ ] Implement shared command history
- [ ] Add collaborative sessions
- [ ] Create team notification system
- [ ] Add role-based command access

### Performance Profiling (v0.2.5)
- [ ] Add command execution profiling
- [ ] Implement memory usage tracking
- [ ] Add bottleneck detection
- [ ] Create performance reports
- [ ] Add optimization suggestions

### Offline Capabilities (v0.2.6)
- [ ] Add offline command queue
- [ ] Implement local caching
- [ ] Add sync when online
- [ ] Create offline validation
- [ ] Add conflict resolution for offline changes

### Accessibility (v0.2.7)
- [ ] Add screen reader support
- [ ] Implement high contrast mode
- [ ] Add keyboard-only navigation
- [ ] Create voice command support
- [ ] Add customizable output verbosity

### Plugin Ecosystem (v0.2.8)
- [ ] Add plugin marketplace
- [ ] Implement plugin versioning
- [ ] Add plugin dependency management
- [ ] Create plugin development kit
- [ ] Add plugin security scanning

### Enterprise Features (v0.2.9)
- [ ] Add SSO authentication
- [ ] Implement audit logging
- [ ] Add compliance mode
- [ ] Create enterprise policy enforcement
- [ ] Add centralized configuration management

## Roadmap for 0.3.0 Series (Next-Gen Features)

### Voice-First CLI (v0.3.0)
- [ ] Add voice command input
- [ ] Implement voice feedback output
- [ ] Add multilingual voice support
- [ ] Create hands-free operation mode
- [ ] Add voice command training

### Intelligent Assistant (v0.3.1)
- [ ] Add contextual command suggestions
- [ ] Implement learning from user patterns
- [ ] Add proactive recommendations
- [ ] Create predictive command execution
- [ ] Add natural conversation mode

### AR/VR Integration (v0.3.2)
- [ ] Add AR command overlay
- [ ] Implement VR workspace
- [ ] Add spatial command organization
- [ ] Create gesture-based commands
- [ ] Add immersive documentation

### Distributed CLI (v0.3.3)
- [ ] Add multi-node command execution
- [ ] Implement distributed workflows
- [ ] Add cluster management
- [ ] Create edge computing support
- [ ] Add federated command routing

### Self-Healing CLI (v0.3.4)
- [ ] Add automatic error recovery
- [ ] Implement self-diagnostic commands
- [ ] Add automatic updates
- [ ] Create configuration repair
- [ ] Add crash recovery and resume
