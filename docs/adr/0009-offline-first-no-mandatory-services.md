# ADR-0009: Offline-first design with no mandatory external services

## Status

Accepted

## Context

The target environments for Legalis-RS — courts, ministries, regulated
enterprises — are frequently air-gapped or operate under strict data-egress rules.
A legal-reasoning tool that *requires* a cloud LLM, a hosted database, or an
external SMT/PDF service to do its core work would be unusable in exactly the
places it is meant to serve. At the same time, the project does integrate with
LLMs, blockchains, object storage, and cloud providers when they are available
and wanted.

The design needs to make the *core* workflows run with nothing but the binary,
while making external integrations *optional* enhancements.

## Decision

**The core workflows — authoring (DSL), parsing, verification, simulation,
visualization, diffing, and export — run fully offline with no mandatory external
service.** External integrations are opt-in:

- **LLMs** are accessed through the vendor-agnostic `LLMProvider` trait in
  `legalis-llm` (OpenAI, Anthropic, Gemini, Ollama, …). Using an LLM is a choice;
  the platform's deterministic core does not depend on one. Local models (Ollama)
  are supported precisely so inference can stay on-prem.
- **Heavy/native-ish capabilities are feature-gated** (per ADR-0001): the SMT
  solver (`legalis-verifier` `smt-solver`), PDF export, GPU acceleration
  (`legalis-sim` `cuda`), gRPC, Redis caching, and OAuth2 are all optional.
- **Storage is pluggable.** Audit and registry persistence can use in-memory or
  embedded (pure-Rust SQLite-compatible) backends; S3-compatible object storage is
  supported but not required, and can itself be a pure-Rust gateway.
- **The CLI has explicit offline support.** The `legalis offline` command queues
  mutating operations, caches data locally, validates, and later syncs with
  conflict resolution when connectivity returns. Asynchronicity uses Rust's native
  Tokio runtime rather than an external task queue or broker.

## Consequences

**Benefits**

- The binary is useful from a clean install with no network: parse, verify,
  simulate, visualize, and export all work offline.
- Deployments in restricted environments are first-class, not an afterthought; the
  default feature set avoids pulling in services that would be blocked.
- Optional integrations can be enabled exactly where allowed, without changing the
  core code paths (the trait/feature-flag boundary keeps them separable).

**Trade-offs / risks accepted**

- Functionality is spread across feature flags and optional dependencies, which
  adds configuration surface; the deployment guide documents which flags enable
  what.
- The offline command queue introduces real distributed-systems concerns
  (conflict detection and resolution strategies such as last-writer-wins, merge);
  these are implemented deliberately rather than avoided.
- Keeping "the core never requires a service" true is an ongoing constraint on new
  features: anything that would introduce a mandatory external dependency must
  instead be made optional or provided in a pure-Rust, embeddable form.
