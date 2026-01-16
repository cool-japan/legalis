# Legalis DSL for Visual Studio Code

Syntax highlighting and language support for the Legalis Legal Domain-Specific Language.

## Features

- **Syntax Highlighting**: Full syntax highlighting for Legalis DSL files (`.legalis`, `.legal`)
- **Bracket Matching**: Auto-closing and matching brackets
- **Comment Support**: Line (`//`) and block (`/* */`) comments
- **Keyword Recognition**: Recognizes all Legal DSL keywords:
  - Control: `STATUTE`, `WHEN`, `THEN`, `AND`, `OR`, `NOT`, `IF`, `ELSE`
  - Metadata: `EFFECTIVE_DATE`, `EXPIRY_DATE`, `JURISDICTION`, `VERSION`
  - Effects: `GRANT`, `DENY`, `REQUIRE`, `OBLIGATE`, `PERMIT`, `PROHIBIT`
  - Conditions: `AGE`, `INCOME`, `ATTRIBUTE`, `DATE`, `HAS`, `IN`, `BETWEEN`

## Usage

Create a file with `.legalis` or `.legal` extension and start writing your legal rules:

```legalis
// Example: Age restriction statute
STATUTE age-restriction: "Age must be 18 or older" {
    WHEN AGE >= 18
    THEN GRANT "access"
    ELSE DENY "access"
}

EFFECTIVE_DATE 2024-01-01
JURISDICTION US-CA
```

## Installation

### From VSIX (Recommended)

1. Download the `.vsix` file from the releases
2. Open VS Code
3. Press `Ctrl+Shift+P` (or `Cmd+Shift+P` on Mac)
4. Type "Install from VSIX" and select it
5. Choose the downloaded `.vsix` file

### From Source

```bash
cd vscode-extension
npm install -g vsce
vsce package
code --install-extension legalis-dsl-0.1.0.vsix
```

## Requirements

- Visual Studio Code 1.85.0 or higher

## Extension Settings

This extension does not currently contribute any settings.

## Known Issues

None at this time. Please report issues at: https://github.com/cool-japan/legalis/issues

## Release Notes

### 0.1.0

Initial release of Legalis DSL extension:
- Syntax highlighting for Legal DSL
- Basic language configuration
- Support for `.legalis` and `.legal` files

## Contributing

Contributions are welcome! Please see the [Legalis-RS repository](https://github.com/cool-japan/legalis) for contribution guidelines.

## License

MIT OR Apache-2.0

---

**Enjoy writing legal rules with Legalis DSL!**
