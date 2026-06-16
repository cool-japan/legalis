# ADR-0001: Pure-Rust stack with no mandatory C/C++ dependencies

## Status

Accepted

## Context

Legalis-RS is a legal-technology platform intended to be auditable, reproducible,
and easy to build in restricted or air-gapped environments (courts, ministries,
regulated enterprises). Two forces push hard against pulling in C/C++ libraries:

- **Build reproducibility and portability.** A native build chain that links
  against system libraries (`libsqlite3`, an SMT solver such as Z3, an RSA-based
  PDF backend, a BLAS) makes the project hostage to the host's toolchain and
  package versions. This is the opposite of what a legal-record system needs.
- **Supply-chain and security posture.** Transitive C dependencies have been a
  recurring source of CVEs in the ecosystem. The project's `TODO.md` explicitly
  records remediation work here: an RSA-related CVE was eliminated by removing
  the `printpdf` dependency, and the SMT integration was moved to a pure-Rust
  solver.

The project is also part of the broader COOLJAPAN "Pure Rust" ecosystem, which
maintains pure-Rust replacements for common native libraries.

## Decision

**The workspace is built entirely in Rust with no mandatory C/C++ dependencies.**
Where a capability would normally require a native library, a pure-Rust component
is used instead. Concretely, as declared in the root `Cargo.toml`
`[workspace.dependencies]`:

- **SMT solving** uses `oxiz-solver` / `oxiz-core` (a pure-Rust SMT solver),
  gated behind the `legalis-verifier` `smt-solver` feature, rather than linking
  Z3.
- **SQLite-compatible storage** uses `oxisql-core` / `oxisql-sqlite-compat`
  rather than `rusqlite` / `libsqlite3-sys`.
- **PDF rendering** uses `fop-render` (a pure-Rust PDF backend) rather than
  `printpdf`.
- **Compression** uses `oxiarc-deflate`.

The one explicitly optional exception is GPU acceleration: `legalis-sim`'s
`cuda` feature pulls in `cudarc`, which `dlopen`s the CUDA driver at runtime via
`dynamic-loading`. This is **off by default**, and crates still build on hosts
without a CUDA toolkit.

## Consequences

**Benefits**

- `cargo build` works from a clean checkout with no `apt install`, no
  environment variables, and no vendored C sources. The README's quick-start is
  literally `git clone` then `cargo build`.
- A single, Cargo-auditable dependency graph (`cargo audit`, `cargo deny`) covers
  essentially the whole stack.
- Cross-compilation and WASM targets become realistic because there is no native
  linkage to reconcile.

**Trade-offs / risks accepted**

- Pure-Rust replacements for mature C libraries (an SMT solver, a SQLite engine)
  carry their own maturity and performance risk. The project mitigates this by
  keeping the heaviest of these (`smt-solver`) behind a feature flag so the core
  experience does not depend on it.
- The project takes on a dependency on the COOLJAPAN ecosystem crates
  (`oxiz-*`, `oxisql-*`, `fop-render`, `oxiarc-deflate`). This is an accepted,
  deliberate coupling.
- GPU acceleration is the sole place where an external runtime (the CUDA driver)
  may be involved, and only when a user opts in.
