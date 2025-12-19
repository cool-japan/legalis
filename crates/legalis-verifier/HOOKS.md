# Pre-Commit Hooks for Legalis Verifier

This guide explains how to set up and use pre-commit hooks for the Legalis verifier project.

## What are Pre-Commit Hooks?

Pre-commit hooks are scripts that run automatically before each Git commit. They help ensure code quality by running checks like:
- Code compilation
- Linting (clippy)
- Testing
- Formatting

## Installation

### Option 1: Manual Installation

1. Copy the pre-commit script to your Git hooks directory:
   ```bash
   cp pre-commit.sh ../.git/hooks/pre-commit
   chmod +x ../.git/hooks/pre-commit
   ```

2. The hook will now run automatically before each commit.

### Option 2: Using a Pre-Commit Framework

If you're using a tool like [pre-commit](https://pre-commit.com/), create a `.pre-commit-config.yaml` file:

```yaml
repos:
  - repo: local
    hooks:
      - id: cargo-check
        name: Cargo Check
        entry: cargo check
        language: system
        pass_filenames: false

      - id: cargo-clippy
        name: Cargo Clippy
        entry: cargo clippy --all-targets --all-features -- -D warnings
        language: system
        pass_filenames: false

      - id: cargo-test
        name: Cargo Test
        entry: cargo test
        language: system
        pass_filenames: false

      - id: cargo-fmt
        name: Cargo Format
        entry: cargo fmt --all -- --check
        language: system
        pass_filenames: false
```

Then install the hooks:
```bash
pre-commit install
```

## What the Hook Checks

The pre-commit hook performs the following checks:

1. **Cargo Check**: Ensures the code compiles without errors
2. **Cargo Clippy**: Checks for common mistakes and style issues
3. **Cargo Test**: Runs all tests to ensure they pass
4. **Cargo Fmt**: Checks code formatting (auto-formats if needed)

## Bypassing the Hook

In rare cases where you need to bypass the hook (not recommended):
```bash
git commit --no-verify -m "Your commit message"
```

## Customization

You can customize the hook by editing `pre-commit.sh`:
- Add additional checks
- Modify warning levels
- Add custom verification steps

## Troubleshooting

### Hook Not Running

If the hook isn't running:
1. Check that the file exists: `ls -la .git/hooks/pre-commit`
2. Ensure it's executable: `chmod +x .git/hooks/pre-commit`
3. Check for syntax errors: `bash -n .git/hooks/pre-commit`

### Hook Failing

If the hook fails:
1. Read the error message carefully
2. Fix the issues reported
3. Run the checks manually to verify: `cargo check && cargo clippy && cargo test && cargo fmt --check`
4. Try committing again

## Best Practices

- Always run `cargo test` locally before committing
- Keep commits focused and atomic
- Fix clippy warnings promptly
- Use `cargo fmt` regularly to maintain consistent formatting
