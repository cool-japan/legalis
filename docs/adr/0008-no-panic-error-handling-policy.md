# ADR-0008: No-panic / no-unwrap error-handling policy in library code

## Status

Accepted

## Context

Legalis-RS is meant to run in settings where a crash is unacceptable: an API
service deciding eligibility, a batch verifier in CI, a long-running simulation.
A `panic!` (including an `unwrap()` or `expect()` on a `None`/`Err`) aborts the
current thread and, in many deployments, takes down the request or the job. For a
system whose outputs may inform legal or administrative decisions, "fail loudly by
crashing" is the wrong default; "return a structured error the caller can handle"
is the right one.

The project also enforces a strict **no-warnings** policy, and clean error
handling is part of keeping the codebase disciplined.

## Decision

**Non-test library code does not use `unwrap()`, `expect()`, or `panic!`.**
Fallible operations return `Result`/`Option` (or domain enums like
`LegalResult<T>`), and validation is expressed as functions that return errors
rather than aborting:

- `legalis-core` prefers *validation over panics*: `Statute::validate()` returns a
  `Vec<ValidationError>` and `Statute::validated()` returns
  `Result<Statute, Vec<ValidationError>>`, instead of panicking on a malformed
  statute. Typed-attribute access returns `Result` on type mismatch.
- Errors are modeled with `thiserror` and propagated with `?`; structured error
  types carry codes, severities, and suggestions where useful.

The `TODO.md` records this as completed work: "All `unwrap()` calls removed from
production source code across all core crates." Tests are exempt — `unwrap()` in
a `#[test]` (or `#[cfg(test)]`) is acceptable because a panicking test is exactly
how a test reports failure.

## Consequences

**Benefits**

- Library code degrades gracefully: callers (the API server, the CLI, the
  simulator) decide how to surface or recover from failures instead of crashing.
- Error paths are explicit and typed, which makes them visible in signatures,
  testable, and serializable for reporting.
- The policy reinforces the broader quality bar (no warnings, clean clippy).

**Trade-offs / risks accepted**

- More boilerplate: even "this cannot fail" spots must thread a `Result` or make a
  deliberate, justified choice. The payoff is no hidden abort points.
- The rule must be enforced continuously. Contributors run
  `cargo clippy --all-targets -- -D warnings`, and reviewers reject new
  `unwrap()`/`expect()`/`panic!` in non-test code. (The repo provides tooling to
  hunt down stray `unwrap()`s.)
- It is a *policy*, not a compiler guarantee for `panic!` in general; vigilance in
  review is part of the cost.
