# Contributing to Legalis-RS

Thank you for your interest in Legalis-RS, a pure-Rust framework for parsing,
analyzing, verifying, and simulating legal statutes across jurisdictions. This
guide explains the repository layout, the build/test commands, the coding
policies we enforce, and the common contribution workflows.

By participating you agree to uphold a respectful, collaborative environment.
Legalis-RS is licensed under Apache-2.0; contributions are accepted under the same
license.

## Table of contents

- [Repository layout](#repository-layout)
- [Building and testing](#building-and-testing)
- [Coding policies (please read)](#coding-policies-please-read)
- [The TODO.md-driven workflow](#the-todomd-driven-workflow)
- [Adding a new jurisdiction crate](#adding-a-new-jurisdiction-crate)
- [Submitting changes](#submitting-changes)

## Repository layout

This is a single Cargo workspace (see
[ADR-0002](docs/adr/0002-cargo-workspace-dependency-policy.md)). The members are
declared in the root `Cargo.toml`.

```
legalis/
├── Cargo.toml            # workspace manifest + [workspace.dependencies]
├── crates/               # the engine
│   ├── legalis-core/       # core types: Statute, Condition, Effect, LegalResult
│   ├── legalis-dsl/        # the Legal DSL: tokenizer, parser, printer, LSP, REPL
│   ├── legalis-registry/   # statute registry with version control
│   ├── legalis-llm/        # vendor-agnostic LLM provider abstraction
│   ├── legalis-verifier/   # static verification + (optional) SMT solving
│   ├── legalis-sim/        # population simulation engine (async)
│   ├── legalis-diff/       # statute diffing and change detection
│   ├── legalis-i18n/       # multi-language / multi-jurisdiction support
│   ├── legalis-porting/    # cross-jurisdiction law transfer
│   ├── legalis-interop/    # import/export: Catala, Stipula, L4, Akoma Ntoso, …
│   ├── legalis-viz/        # visualization (decision trees, flowcharts)
│   ├── legalis-chain/      # smart-contract export (Solidity, WASM, Ink!, …)
│   ├── legalis-lod/        # Linked Open Data (RDF/TTL/JSON-LD) export
│   ├── legalis-audit/      # audit trail and decision logging
│   ├── legalis-api/        # REST/gRPC/GraphQL API server (binary: legalis-api-server)
│   └── legalis-cli/        # the `legalis` command-line tool
├── jurisdictions/        # one crate per country (jp, de, fr, us, eu, …)
├── examples/             # 35+ runnable example crates
└── docs/                 # this documentation set (ADRs, guides, tutorials)
```

A quick mental model: **`legalis-core` defines the model, `legalis-dsl` is the
authoring front-end, the other crates are layers over the core, jurisdiction
crates consume the engine, and the CLI/API expose it.**

## Building and testing

The default build is pure Rust — no C/C++ toolchain, no system libraries, no
environment variables required.

```bash
# Build everything
cargo build

# Run the full test suite. We use cargo-nextest (install: cargo install cargo-nextest)
cargo nextest run

# With all optional features enabled (SMT solver, etc.)
cargo nextest run --all-features

# Lint — this MUST pass with zero warnings (see policies below)
cargo clippy --all-targets -- -D warnings

# Format
cargo fmt --all
```

Run a single example to see the pieces working together:

```bash
cargo run -p welfare-benefits
```

> Do not commit changes that fail `cargo clippy --all-targets -- -D warnings` or
> that break `cargo nextest run`.

## Coding policies (please read)

These policies are enforced in review and, where possible, by tooling. They are
not optional.

1. **No warnings.** The entire workspace compiles cleanly, including under
   `--all-features --all-targets`. New warnings are treated as errors. Run
   `cargo clippy --all-targets -- -D warnings` before pushing.

2. **No `unwrap()` / `expect()` / `panic!` in non-test code.** Library and
   binary code returns `Result`/`Option` (or domain types like `LegalResult<T>`)
   and propagates errors with `?`. Prefer validation that returns errors over
   aborting. `unwrap()` in `#[test]`/`#[cfg(test)]` code is fine. See
   [ADR-0008](docs/adr/0008-no-panic-error-handling-policy.md).

3. **Files under 2000 lines.** Keep source files below ~2000 lines; split larger
   modules. (The workspace was refactored to honor this; please keep it that
   way.)

4. **Workspace dependency policy.** Add shared dependencies to the root
   `[workspace.dependencies]` and reference them from member crates with
   `<crate>.workspace = true`. **Never** put a version number on a dependency
   line in a member crate's `Cargo.toml`. Use the latest version available on
   crates.io. See [ADR-0002](docs/adr/0002-cargo-workspace-dependency-policy.md).

5. **Pure Rust, no mandatory C/C++ dependencies.** Capabilities that would
   normally need a native library use pure-Rust components (e.g. `oxiz-*` for
   SMT, `oxisql-*` for SQLite-compatible storage, `fop-render` for PDF). Anything
   heavier is behind a feature flag. See
   [ADR-0001](docs/adr/0001-pure-rust-no-mandatory-c-dependencies.md).

6. **Naming conventions.** Follow standard Rust style: `snake_case` for
   variables/functions/modules, `CamelCase` for types/traits, `SCREAMING_SNAKE`
   for constants.

7. **Tests use temporary directories.** Tests that touch the filesystem must use
   `std::env::temp_dir()` (or the `tempfile` crate) — never write into the source
   tree or hard-coded paths. Avoid time-bomb tests that hard-code future dates;
   derive dates relative to "now" instead.

8. **Document public APIs.** Public items get `///` doc comments; modules get
   `//!` docs. Doc examples should compile.

## The TODO.md-driven workflow

Work in this repository is tracked in `TODO.md` files rather than (only) an issue
tracker. There is a root `TODO.md` for project-wide items and a `TODO.md` inside
each crate for crate-specific work.

The convention is:

- Open items are unchecked checkboxes: `- [ ] Do the thing`.
- Completed items are checked: `- [x] Do the thing`.
- When you finish a body of work, add a dated note describing what landed, e.g.
  a `## COMPLETED (YYYY-MM-DD — short summary)` section, and check off the
  relevant boxes.

When you pick up a task, find it (or add it) in the appropriate `TODO.md`, do the
work, then update the checkbox and add the dated note in the same change. This
keeps the roadmap and the code in sync.

## Adding a new jurisdiction crate

Each country lives in its own crate under `jurisdictions/` and *consumes* the
shared engine rather than reimplementing it (see
[ADR-0007](docs/adr/0007-jurisdiction-crate-per-country.md)). To add one:

1. **Create the crate.** Add `jurisdictions/<cc>/` (ISO country code), with a
   `Cargo.toml` that uses workspace inheritance:

   ```toml
   [package]
   name = "legalis-<cc>"
   version.workspace = true
   edition.workspace = true
   license.workspace = true

   [dependencies]
   legalis-core.workspace = true
   legalis-verifier.workspace = true
   legalis-i18n.workspace = true
   # add other engine crates as needed
   ```

2. **Register it in the workspace.** Add `"jurisdictions/<cc>"` to `members` and
   a `legalis-<cc> = { version = "x.y.z", path = "jurisdictions/<cc>" }` line to
   `[workspace.dependencies]` in the root `Cargo.toml`.

3. **Model real law.** Build `legalis_core::Statute` values grounded in the
   jurisdiction's actual statutes — accurate IDs, citations, effects, and
   preconditions, sourced from real laws. Existing crates (e.g.
   `jurisdictions/za`, `jurisdictions/br`, `jurisdictions/in`) are good templates.

4. **Add jurisdiction-specific concerns** as needed: calendar/holiday handling,
   citation formatting, constitutional/hierarchy checks (a `verifier.rs`), and
   simulation/i18n integration.

5. **Expose statutes as DSL.** Provide `dsl::statutes_as_dsl()` (or
   `reasoning::dsl::statutes_as_dsl()`) that renders the crate's statutes via
   `legalis_dsl::format_statutes`, with a render test.

6. **Test it.** Add unit tests (using `temp_dir()` for any filesystem work) and
   make sure the crate is warning-free.

7. **Update the docs/TODO.** Add the jurisdiction to the relevant tables in
   `README.md` and check off / note the work in the appropriate `TODO.md`.

## Submitting changes

1. Branch off `master` (do not commit directly to `master`).
2. Make your change, keeping commits focused.
3. Ensure the gates pass locally:
   - `cargo clippy --all-targets -- -D warnings`
   - `cargo nextest run` (and `--all-features` if you touched feature-gated code)
   - `cargo fmt --all`
4. Update the relevant `TODO.md` (check boxes + dated note) and any affected docs.
5. Open a pull request describing *what* changed and *why*, and confirm the
   coding policies above are met.

Questions or larger proposals are welcome as issues or draft PRs before you invest
heavily in implementation.
