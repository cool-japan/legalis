# ADR-0007: One crate per jurisdiction

## Status

Accepted

## Context

Legalis-RS models real law from many countries — 23 operational jurisdictions
spanning civil-law, common-law, socialist, Islamic-law, and supranational
traditions. Each jurisdiction has its own statutes, citations, calendars (e.g.
Japanese 和暦, Thai Buddhist calendar, Hijri dates), languages, and legal
hierarchy rules. The question is how to organize this body of
jurisdiction-specific code relative to the generic engine.

Two anti-patterns to avoid:

- **A monolith** where all jurisdictions live in one crate: it would be enormous,
  slow to compile, and would force every consumer to pull in every country's law.
- **Country-specific engines** where each jurisdiction reimplements parsing,
  conditions, and verification: this is the traditional systems-integrator
  approach the project explicitly rejects (the "universal engine" thesis).

## Decision

**Each jurisdiction is its own crate under `jurisdictions/`, depending on the
shared engine crates rather than reimplementing them.** The workspace contains
`jurisdictions/jp`, `jurisdictions/de`, `jurisdictions/fr`, `jurisdictions/us`,
`jurisdictions/eu`, and so on (published as `legalis-jp`, `legalis-de`, …).

A jurisdiction crate is *data plus jurisdiction-specific logic* layered on the
generic core: it builds `legalis_core::Statute` values grounded in that country's
real laws, adds jurisdiction-specific concerns (calendars, citation formats,
constitutional/hierarchy checks), and integrates with the shared crates
(`legalis-verifier`, `legalis-i18n`, `legalis-audit`, `legalis-sim`,
`legalis-interop`, `legalis-lod`). Every jurisdiction crate exposes its statutes
as DSL via `statutes_as_dsl()` (or `reasoning::dsl::statutes_as_dsl()`).

## Consequences

**Benefits**

- Consumers depend only on the jurisdictions they need; adding a country does not
  bloat unrelated builds.
- The "one engine, many jurisdictions" property is structurally enforced:
  jurisdiction crates *consume* the engine, so adding jurisdiction N+1 is
  incremental data/logic rather than a new codebase.
- Country-specific concerns (calendars, citations, hierarchy rules) have a clear
  home and can evolve independently.

**Trade-offs / risks accepted**

- A large number of member crates increases workspace size and CI surface (each
  must stay warning-free and tested). This is accepted and managed by the
  workspace policy (ADR-0002) and the no-warnings policy.
- Cross-jurisdiction features (porting, comparative analysis) must be designed to
  span crates; this is handled by dedicated crates (`legalis-porting`) and
  examples (`comparative-tort-law`, `cross-jurisdiction-demo`) rather than by
  coupling jurisdiction crates to each other.
- Keeping 23 jurisdictions accurate is an ongoing scholarship burden; the crates
  document their sources, but they are models of the law, not legal advice.
