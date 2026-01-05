# legalis

Command-line interface for Legalis-RS.

## Overview

`legalis` provides a comprehensive command-line tool for working with legal statutes, including parsing, verification, visualization, simulation, and more.

## Installation

```bash
# From crates.io (when published)
cargo install legalis

# From source
cargo install --path crates/legalis-cli
```

## Commands

### Core Commands

| Command | Description |
|---------|-------------|
| `parse` | Parse and validate DSL files |
| `verify` | Verify statute consistency |
| `viz` | Generate visualizations |
| `format` | Format DSL files |

### Export Commands

| Command | Description |
|---------|-------------|
| `export` | Export to smart contracts |
| `lod` | Export to RDF/TTL/JSON-LD |

### Analysis Commands

| Command | Description |
|---------|-------------|
| `diff` | Compare statute versions |
| `simulate` | Run population simulations |
| `audit` | Generate audit reports |
| `complexity` | Analyze statute complexity |

### Interoperability Commands

| Command | Description |
|---------|-------------|
| `import` | Import from external formats |
| `convert` | Convert between formats |
| `port` | Port to different jurisdictions |

### Infrastructure Commands

| Command | Description |
|---------|-------------|
| `serve` | Start API server |
| `init` | Initialize new project |
| `completions` | Generate shell completions |

## Usage Examples

```bash
# Parse a DSL file
legalis parse laws/adult-rights.legalis

# Verify all statutes
legalis verify laws/*.legalis --strict

# Generate Mermaid diagram
legalis viz laws/adult-rights.legalis --format mermaid

# Export to Solidity
legalis export laws/adult-rights.legalis --format solidity

# Run simulation with 1000 entities
legalis simulate laws/*.legalis --population 1000

# Compare two versions
legalis diff laws/v1.legalis laws/v2.legalis

# Import from Catala
legalis import --from catala input.catala_en

# Start API server
legalis serve --host 0.0.0.0 --port 3000

# Generate shell completions
legalis completions bash > ~/.bash_completion.d/legalis
```

## Global Options

| Option | Description |
|--------|-------------|
| `-v, --verbose` | Increase verbosity (can be repeated) |
| `-f, --format` | Output format (json, yaml, text) |
| `-h, --help` | Show help |
| `-V, --version` | Show version |

## Shell Completions

Generate completions for your shell:

```bash
# Bash
legalis completions bash > ~/.bash_completion.d/legalis

# Zsh
legalis completions zsh > ~/.zsh/completions/_legalis

# Fish
legalis completions fish > ~/.config/fish/completions/legalis.fish

# PowerShell
legalis completions powershell > legalis.ps1
```

## Exit Codes

| Code | Description |
|------|-------------|
| `0` | Success |
| `1` | General error |
| `2` | Parse error |
| `3` | Verification failed |
| `4` | I/O error |

## License

MIT OR Apache-2.0
