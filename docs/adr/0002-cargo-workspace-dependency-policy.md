# ADR-0002: Single Cargo workspace with centralized dependency versions

## Status

Accepted

## Context

Legalis-RS is large: the root `Cargo.toml` lists on the order of 40 internal
crates (a core layer, an intelligence layer, simulation/analysis, i18n/porting,
interop, output, infrastructure) plus 23 jurisdiction crates and dozens of
example crates. Many of these depend on the same third-party libraries
(`serde`, `tokio`, `chrono`, `thiserror`, `uuid`, `clap`, …) and on each other.

Without a coordination mechanism, every crate would pin its own version of each
dependency. At this scale that produces version skew (two crates compiling two
incompatible `serde` majors), slow builds (duplicate compilations), and a release
nightmare (bumping a shared dependency means editing dozens of manifests).

The project also follows a "latest crates" policy: dependencies are kept current
with what is published on crates.io.

## Decision

**Use one Cargo workspace and declare every shared dependency once, in the root
`[workspace.dependencies]` table. Member crates inherit versions with
`<crate>.workspace = true` and do not pin their own versions.**

This applies to both third-party crates and the internal crates. The root
manifest declares each internal crate with a path and a single workspace version,
for example:

```toml
[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.52", features = ["full"] }
legalis-core = { version = "0.1.6", path = "crates/legalis-core" }
legalis-dsl  = { version = "0.1.6", path = "crates/legalis-dsl" }
```

A member crate's manifest then reads simply:

```toml
[dependencies]
legalis-core.workspace = true
serde.workspace = true
```

Workspace-wide package metadata (`version`, `edition`, `license`, `repository`)
is similarly centralized under `[workspace.package]`, and member crates use
`version.workspace = true`, `edition.workspace = true`, and so on.

## Consequences

**Benefits**

- A dependency upgrade is a one-line change in the root manifest that propagates
  to every crate — directly supporting the "latest crates" policy.
- Version unification: there is exactly one resolved version of each shared
  dependency, eliminating accidental incompatibilities and reducing build times.
- A coherent release: bumping the workspace version updates all crates together,
  which matches how the project versions releases (e.g. the `0.1.x` line).

**Trade-offs / risks accepted**

- Less per-crate independence: a crate cannot quietly hold back on an older
  version of a shared dependency. Divergent needs must be resolved at the
  workspace level (or by not sharing that dependency).
- Feature unification across the workspace means a feature enabled for one crate's
  use of a dependency is enabled everywhere that dependency is built; feature
  choices must be made with the whole workspace in mind.
- Contributors must know the rule: **never** add a version number to a member
  crate's dependency line. This is documented in `CONTRIBUTING.md` and enforced
  by review.
