# IDE Integration Guide for Legalis Verifier

This guide explains how to integrate the Legalis verifier with popular IDEs for a better development experience.

## Table of Contents

- [Visual Studio Code](#visual-studio-code)
- [IntelliJ IDEA / CLion](#intellij-idea--clion)
- [Language Server Protocol (LSP)](#language-server-protocol-lsp)
- [Diagnostic Formats](#diagnostic-formats)
- [Quick Fixes](#quick-fixes)
- [Troubleshooting](#troubleshooting)

## Visual Studio Code

### Setup

1. **Install Required Extensions**
   - [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
   - [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) (for debugging)
   - [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)

2. **Copy Configuration Files**
   ```bash
   cp -r .vscode /path/to/your/project/
   ```

3. **Configure Tasks**
   The included `.vscode/tasks.json` provides:
   - `Legalis: Verify Statutes` - Verify the current file
   - `Legalis: Run All Tests` - Run all tests
   - `Legalis: Generate HTML Report` - Generate interactive report
   - `Legalis: Watch Mode` - Continuous verification

### Usage

#### Verify Current File
- Press `Ctrl+Shift+B` (or `Cmd+Shift+B` on Mac)
- Select "Legalis: Verify Statutes"

#### Run in Watch Mode
- Press `Ctrl+Shift+P` (or `Cmd+Shift+P` on Mac)
- Type "Tasks: Run Task"
- Select "Legalis: Watch Mode"

#### View Diagnostics
- Errors and warnings appear in the Problems panel (`Ctrl+Shift+M`)
- Hover over underlined code for details
- Click on problems to navigate to source

### Custom Keybindings

Add to your `keybindings.json`:

```json
[
  {
    "key": "ctrl+alt+v",
    "command": "workbench.action.tasks.runTask",
    "args": "Legalis: Verify Statutes"
  },
  {
    "key": "ctrl+alt+w",
    "command": "workbench.action.tasks.runTask",
    "args": "Legalis: Watch Mode"
  }
]
```

### Snippets

Create a `legalis.code-snippets` file in `.vscode`:

```json
{
  "Statute Definition": {
    "prefix": "statute",
    "body": [
      "{",
      "  \"id\": \"${1:statute-id}\",",
      "  \"title\": \"${2:Statute Title}\",",
      "  \"effect\": {",
      "    \"type\": \"${3|Grant,Deny,Require|}\",",
      "    \"description\": \"${4:Effect description}\"",
      "  },",
      "  \"preconditions\": [",
      "    ${5}",
      "  ]",
      "}"
    ],
    "description": "Create a new statute definition"
  }
}
```

## IntelliJ IDEA / CLion

### Setup

1. **Install Required Plugins**
   - [Rust Plugin](https://plugins.jetbrains.com/plugin/8182-rust)
   - [TOML Plugin](https://plugins.jetbrains.com/plugin/8195-toml)

2. **Import Configuration**
   - The `.idea` folder contains pre-configured run configurations
   - Import inspection profiles from `.idea/inspectionProfiles/Legalis.xml`

3. **Configure Cargo**
   - Go to Settings → Languages & Frameworks → Rust → Cargo
   - Enable "Use all features"
   - Enable "Expand declarative macros"

### Run Configurations

Available configurations:
- **Verify Statutes** - Verify the current file
- **Run Tests** - Run all tests
- **Clippy** - Run clippy with strict warnings

### File Watchers

The included watcher tasks automatically:
- Format code on save (`cargo fmt`)
- Run clippy on file changes

To enable:
1. Go to Settings → Tools → File Watchers
2. Import from `.idea/watcherTasks.xml`

### External Tools

Add custom external tools for common operations:

1. Go to Settings → Tools → External Tools
2. Add new tool:
   - Name: "Generate Report"
   - Program: `cargo`
   - Arguments: `run --package legalis-verifier -- report --format html --output report.html $FilePath$`
   - Working directory: `$ProjectFileDir$`

## Language Server Protocol (LSP)

### Overview

The verifier provides LSP-compatible diagnostic output for seamless IDE integration.

### Diagnostic Format

```rust
use legalis_verifier::{generate_lsp_diagnostics, StatuteVerifier};

let verifier = StatuteVerifier::new();
let result = verifier.verify(&statutes);

// Generate LSP diagnostics
let lsp_json = generate_lsp_diagnostics(&result)?;
```

Output format:
```json
[
  {
    "severity": "error",
    "message": "Circular reference detected: statute A depends on B which depends on A",
    "location": {
      "file": "statutes/example.json",
      "line": 42,
      "column": 10
    },
    "code": "L001",
    "source": "legalis-verifier",
    "related": [],
    "fixes": []
  }
]
```

### Severity Levels

- `error` - Critical and Error severity
- `warning` - Warning severity
- `information` - Info severity
- `hint` - Suggestions

### Diagnostic Codes

| Code | Error Type | Description |
|------|-----------|-------------|
| L001 | Circular Reference | Circular dependency detected |
| L002 | Dead Statute | Unsatisfiable preconditions |
| L003 | Constitutional Conflict | Violates constitutional principle |
| L004 | Logical Contradiction | Contradictory effects |
| L005 | Ambiguity | Ambiguous language or conditions |
| L006 | Unreachable Code | Unreachable statute branches |

## Diagnostic Formats

### SARIF (Static Analysis Results Interchange Format)

```rust
use legalis_verifier::generate_sarif_report;

let sarif = generate_sarif_report(&result, "legalis-verifier", "0.2.0")?;
```

Compatible with:
- GitHub Code Scanning
- Visual Studio
- Azure DevOps

### Custom IDE Diagnostics

```rust
use legalis_verifier::{to_ide_diagnostics, IdeDiagnostic, DiagnosticLocation};

let diagnostics = to_ide_diagnostics(&result);

for diagnostic in diagnostics {
    println!("{}: {}", diagnostic.severity, diagnostic.message);

    if let Some(location) = diagnostic.location {
        println!("  at {}:{}:{}", location.file, location.line, location.column);
    }
}
```

## Quick Fixes

### Code Actions

The verifier provides quick fix suggestions for common errors:

```rust
use legalis_verifier::generate_quick_fixes;

for error in &result.errors {
    let fixes = generate_quick_fixes(error);

    for fix in fixes {
        println!("Fix: {}", fix.title);
        println!("  {}", fix.description);

        for edit in &fix.edits {
            println!("  Edit {}: {}:{} -> {}",
                edit.file, edit.start_line, edit.start_column, edit.new_text);
        }
    }
}
```

### Available Quick Fixes

1. **Circular Reference** - Break circular dependency
2. **Dead Statute** - Fix unsatisfiable conditions
3. **Constitutional Conflict** - Update to comply with principles
4. **Logical Contradiction** - Resolve contradictory logic
5. **Ambiguity** - Clarify ambiguous language
6. **Unreachable Code** - Remove or refactor dead code

## Troubleshooting

### VSCode: Tasks Not Showing

1. Reload window: `Ctrl+Shift+P` → "Reload Window"
2. Check `.vscode/tasks.json` exists
3. Verify cargo is in PATH: `cargo --version`

### IntelliJ: Run Configurations Missing

1. File → Invalidate Caches / Restart
2. Re-import the project
3. Check `.idea/runConfigurations/` exists

### Diagnostics Not Appearing

1. Check rust-analyzer is running
2. Verify `cargo check` works: `cargo check`
3. Check cargo features are enabled
4. Review IDE settings for Rust diagnostics

### File Watchers Not Working

1. Enable file watchers in IDE settings
2. Check file watcher configuration is correct
3. Verify cargo fmt/clippy are installed:
   ```bash
   rustup component add rustfmt clippy
   ```

## Advanced Configuration

### Custom Diagnostic Mapping

Map verifier errors to your IDE's format:

```rust
use legalis_verifier::{IdeDiagnostic, DiagnosticLocation};

fn to_custom_format(diagnostic: &IdeDiagnostic) -> CustomDiagnostic {
    CustomDiagnostic {
        level: diagnostic.severity.clone(),
        msg: diagnostic.message.clone(),
        span: diagnostic.location.as_ref().map(|loc| CustomSpan {
            file: loc.file.clone(),
            start: (loc.line, loc.column),
            end: (loc.end_line.unwrap_or(loc.line),
                  loc.end_column.unwrap_or(loc.column)),
        }),
    }
}
```

### Real-time Verification

Enable real-time verification in your IDE:

```rust
use legalis_verifier::watch::{WatchConfig, StatuteWatcher};

let config = WatchConfig::new()
    .with_path("./statutes")
    .with_extensions(vec!["json".to_string()])
    .with_debounce(500);

let watcher = StatuteWatcher::new(config, verifier);

watcher.watch(|path, result| {
    if !result.passed {
        // Send diagnostics to IDE
        send_to_ide(path, result);
    }
})?;
```

## Contributing

To add support for a new IDE:

1. Create configuration files in the appropriate format
2. Document the setup process
3. Add example configurations
4. Test with actual IDE
5. Submit a pull request

## Resources

- [Legalis Documentation](https://docs.legalis-rs.org)
- [LSP Specification](https://microsoft.github.io/language-server-protocol/)
- [SARIF Specification](https://docs.oasis-open.org/sarif/sarif/v2.1.0/sarif-v2.1.0.html)
- [Rust Analyzer Manual](https://rust-analyzer.github.io/manual.html)
