# ADR-0005: A dedicated DSL as a first-class legal authoring language

## Status

Accepted

## Context

The core model (ADR-0003) is precise but verbose to write by hand in Rust, and
its audience is not only Rust programmers. Legal experts, policy analysts, and
reviewers need a way to author and read rules that is:

- closer to legal prose than to Rust syntax;
- diffable and version-controllable as plain text;
- unambiguous enough to compile to the core model and be verified.

Existing computational-law DSLs (Catala, L4, Stipula) demonstrate the value of a
purpose-built syntax, but each is tied to its own toolchain and semantics.

## Decision

Legalis-RS ships its own **Legal DSL** as a first-class crate, `legalis-dsl`,
rather than treating Rust as the only authoring surface. The DSL has a real
tokenizer, parser, pretty-printer, type checker, LSP server, and REPL.

The canonical syntax is statute-oriented and reads close to the domain:

```legalis
STATUTE senior-pension: "Senior Citizens Pension Supplement" {
    JURISDICTION "US"
    VERSION 2
    EFFECTIVE_DATE 2024-01-01

    WHEN AGE >= 65 AND INCOME <= 50000
    THEN GRANT "Monthly pension supplement of $300"
}
```

Keywords include `STATUTE`, `WHEN`/`UNLESS`, `THEN`, the effect verbs
`GRANT`/`REVOKE`/`OBLIGATION`/`PROHIBITION`, `DISCRETION`, the metadata clauses
`JURISDICTION`/`VERSION`/`EFFECTIVE_DATE`/`EXPIRY_DATE`, the logical connectives
`AND`/`OR`/`NOT`, the condition operators (`>=`, `<=`, `>`, `<`, `==`/`=`, `!=`,
`BETWEEN ... AND ...`, `IN [...]`, `LIKE`, `MATCHES`), `REQUIRES`, `EXCEPTION`,
`AMENDMENT`, `SUPERSEDES`, and an `IMPORT ... [AS ...]` module system.

DSL source files use the `.legalis` extension (the `.ldsl` extension is also
recognized). The entry point is `LegalDslParser` (`new`, `parse_statute`,
`parse_statutes`, `parse_document`), and the inverse is the `DslPrinter` /
`format_statutes` pretty-printer, configurable via `PrinterConfig`
(`default`, `compact`, `verbose`).

The DSL compiles **down to** the `legalis-core` model — it is a front-end, not a
parallel semantics. Every jurisdiction crate can render its modelled statutes
back to DSL via `statutes_as_dsl()` (or `reasoning::dsl::statutes_as_dsl()`).

## Consequences

**Benefits**

- Non-Rust authors can read, write, and review rules; the syntax tracks legal
  structure (preconditions, effects, exceptions, amendments).
- Because parser and printer round-trip through the core model, rules can be
  authored in DSL, programmatically transformed, and printed back out — and the
  project tracks round-trip fidelity for a documented set of condition kinds.
- Tooling (LSP, REPL, linting, formatting via `legalis lint` / `legalis format`)
  makes the DSL a genuine authoring environment, not just a file format.

**Trade-offs / risks accepted**

- Maintaining a full language toolchain (lexer, parser, type checker, LSP,
  formatter) is substantial ongoing work compared with reusing an existing DSL.
- The most complex condition forms (e.g. `Composite`, `Temporal`, `Threshold`,
  `Probabilistic`, `Fuzzy`) do not yet round-trip losslessly through the primary
  parser; these are tracked as known limitations rather than silently dropped.
- A bespoke DSL is one more thing for newcomers to learn; this is mitigated by the
  interop layer (ADR-0006 / `legalis-interop`) that imports from and exports to
  Catala, L4, Stipula, and Akoma Ntoso.
