# legalis-cli TODO

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
- [ ] Add `lint` command for style checking
- [ ] Create `fmt` command for formatting
- [ ] Add `watch` command for file watching
- [ ] Implement `repl` command for interactive mode
- [ ] Add `test` command for statute testing
- [ ] Create `publish` command for registry upload

### Enhancements
- [ ] Add `--dry-run` flag for modify commands
- [ ] Implement `--interactive` mode for guided input
- [ ] Add `--quiet` mode for scripting
- [ ] Create `--json-output` for machine parsing
- [ ] Add `--config` flag for config file

## Configuration

- [ ] Add config file support (legalis.toml)
- [ ] Implement project-level configuration
- [ ] Create user-level global configuration
- [ ] Add environment variable overrides
- [ ] Support config inheritance

## Output

### Formatting
- [ ] Add colored output with styles
- [ ] Implement table output format
- [ ] Create progress bars for long operations
- [ ] Add spinner for async operations
- [ ] Implement diff highlighting

### Formats
- [ ] Add YAML output support
- [ ] Implement TOML output
- [ ] Create CSV output for reports
- [ ] Add HTML output option

## Project Management

- [ ] Add `new` command for statute templates
- [ ] Create `add` command for dependencies
- [ ] Implement `update` command for updates
- [ ] Add `clean` command for cache cleanup
- [ ] Create `doctor` command for diagnostics

## Registry Integration

- [ ] Add `search` command for registry
- [ ] Implement `install` command for statutes
- [ ] Create `list` command for installed statutes
- [ ] Add `outdated` command for updates
- [ ] Implement `uninstall` command

## Interop

- [ ] Add `import` command for format conversion
- [ ] Create `convert` command for bidirectional conversion
- [ ] Implement `validate` command for format validation

## Developer Experience

- [ ] Add man page generation
- [ ] Create interactive tutorials
- [ ] Implement error suggestions
- [ ] Add command aliases
- [ ] Create plugin system

## Performance

- [ ] Add command caching
- [ ] Implement lazy loading
- [ ] Create parallel processing for batch ops
- [ ] Add progress estimation

## Testing

- [ ] Add integration tests for all commands
- [ ] Create snapshot tests for output
- [ ] Implement CLI contract tests
- [ ] Add performance benchmarks
