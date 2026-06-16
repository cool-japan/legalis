# Deployment Guide

This guide covers deploying Legalis-RS in production: building release binaries,
running the `legalis-api` service, running the CLI in production, and choosing
feature flags. It focuses on running the **native binaries** directly.

For a container-first walkthrough (Docker / Docker Compose, PostgreSQL, Redis,
nginx), see the repository's existing [`DEPLOYMENT.md`](../DEPLOYMENT.md) at the
root, which this guide complements rather than repeats.

## Contents

1. [Building release binaries](#1-building-release-binaries)
2. [Deploying the API service](#2-deploying-the-api-service)
3. [Running the CLI in production](#3-running-the-cli-in-production)
4. [Feature flags](#4-feature-flags)
5. [Operational notes](#5-operational-notes)

---

## 1. Building release binaries

Legalis-RS builds with a pure-Rust toolchain — no system libraries are required
for the default feature set. Build optimized binaries with the `release` profile:

```bash
# The API server binary
cargo build --release -p legalis-api

# The CLI (the binary is named `legalis`)
cargo build --release -p legalis
```

The resulting binaries are in `target/release/`:

- `target/release/legalis-api-server` — the HTTP/API service.
- `target/release/legalis` — the command-line tool.

To build with optional capabilities enabled, add the relevant features (see
[section 4](#4-feature-flags)):

```bash
# API server without gRPC (gRPC is on by default for the API crate)
cargo build --release -p legalis-api --no-default-features

# Verifier-backed workloads with the pure-Rust SMT solver
cargo build --release --features smt-solver
```

Install the CLI onto a host directly:

```bash
cargo install --path crates/legalis-cli   # installs the `legalis` binary
```

---

## 2. Deploying the API service

The API crate (`legalis-api`) builds a server binary named
`legalis-api-server`. There are two ways to start the service.

### Option A: the dedicated server binary

```bash
./target/release/legalis-api-server
```

### Option B: via the CLI

The CLI can launch the same API surface through its `serve` subcommand, which
takes an explicit host and port:

```bash
legalis serve --host 0.0.0.0 --port 3000
```

`legalis serve` flags:

| Flag | Default | Meaning |
|------|---------|---------|
| `--host` | `127.0.0.1` | Address to bind to. Use `0.0.0.0` to accept external connections. |
| `-p`, `--port` | `3000` | TCP port to listen on. |

### Port and health check

The service listens on port **3000** by default. A health endpoint is available
for liveness/readiness probes:

```bash
curl http://localhost:3000/health
```

### Configuration and environment

- **CLI configuration** is read from `legalis.toml` (or `--config <file>`), and
  CLI behavior can be overridden through `LEGALIS_*` environment variables.
- **Logging** uses the `tracing` ecosystem with an env-filter, so verbosity is
  controlled via the standard `RUST_LOG` variable, e.g.:

  ```bash
  RUST_LOG=info ./target/release/legalis-api-server
  ```

- **Optional integrations** (Redis caching, OAuth2/OIDC, OpenTelemetry tracing,
  gRPC) are compiled in via feature flags — see [section 4](#4-feature-flags).
  Secrets and endpoints for those integrations should be provided through your
  process manager's environment, not committed to the repo.

### Running behind a reverse proxy

Terminate TLS and apply rate limiting at a reverse proxy in front of the service.
The repository ships an example `nginx.conf` and `docker-compose.yml` you can use
as a starting point. The service itself binds plain HTTP on its configured host
and port.

### Running as a managed process

A minimal systemd unit for the dedicated server binary:

```ini
[Unit]
Description=Legalis API server
After=network.target

[Service]
ExecStart=/opt/legalis/legalis-api-server
Environment=RUST_LOG=info
Restart=on-failure
User=legalis

[Install]
WantedBy=multi-user.target
```

Because library code follows a no-panic policy
([ADR-0008](adr/0008-no-panic-error-handling-policy.md)), the service is designed
to return structured errors rather than crash; `Restart=on-failure` is still a
sensible safety net.

---

## 3. Running the CLI in production

The `legalis` CLI is useful in batch/automation contexts (CI pipelines, scheduled
jobs, data processing). Useful patterns:

```bash
# Gate a pipeline on statute consistency (non-zero exit on failure)
legalis verify --input "statutes/**/*.legalis" --strict

# Validate format compliance
legalis validate --input "statutes/**/*.legalis" --strict

# Batch operations across many files with parallel workers
legalis batch verify --input "statutes/" --workers 8
legalis batch export --input "statutes/" --output build/ --export-format json

# Machine-readable output for downstream tooling
legalis verify --input "statutes/*.legalis" --format json
```

For air-gapped or intermittently-connected environments, the CLI provides an
offline command queue that records mutating operations locally and reconciles
them later:

```bash
legalis offline queue --command publish --resource my-statute --payload '{...}'
legalis offline sync  --strategy last-writer-wins
legalis offline conflicts
```

This reflects the project's offline-first stance
([ADR-0009](adr/0009-offline-first-no-mandatory-services.md)): the core workflows
do not require any external service.

---

## 4. Feature flags

Optional and heavier capabilities are behind Cargo features so the default build
stays light and dependency-free. Enable only what your deployment needs.

### `legalis-api`

| Feature | Default | Enables |
|---------|---------|---------|
| `grpc` | on | gRPC server (with reflection and health checks) |
| `redis-cache` | off | Redis-backed response caching |
| `oauth2-auth` | off | OAuth2/OIDC authentication |
| `otel-tracing` | off | OpenTelemetry (OTLP) tracing export |

```bash
# Build the API with Redis cache and OAuth2, without the default gRPC
cargo build --release -p legalis-api --no-default-features \
    --features "redis-cache oauth2-auth"
```

### `legalis-verifier`

| Feature | Default | Enables |
|---------|---------|---------|
| `smt-solver` | off | Satisfiability checking via the pure-Rust OxiZ SMT solver |
| `parallel` | off | Multi-core verification |
| `pdf` | off | PDF report output (pure-Rust `fop-render` backend) |
| `watch` | off | Filesystem watching for continuous verification |

### `legalis-sim`

| Feature | Default | Enables |
|---------|---------|---------|
| `cuda` | off | NVIDIA GPU-accelerated condition evaluation. Loads the CUDA driver at runtime via dynamic loading and falls back to CPU when no device is present, so builds still work on hosts without a CUDA toolkit. |

> Guidance: start from the defaults. Add `smt-solver` if you need rigorous
> verification, `redis-cache`/`otel-tracing` for a production API behind a cache
> and an observability stack, and `cuda` only on GPU hosts running large
> simulations.

---

## 5. Operational notes

- **Stateless core.** Parsing, verification, simulation, visualization, and
  export are stateless and need no database. Persistence (audit trails, the
  registry) is pluggable: in-memory and embedded pure-Rust SQLite-compatible
  backends exist, and S3-compatible object storage is supported but optional.
- **Resource sizing.** Verification and simulation are CPU-bound; the `parallel`
  feature and `legalis batch --workers N` scale across cores. Simulation memory
  scales with population size (`--population`).
- **Observability.** Use `RUST_LOG` for log verbosity; enable
  `legalis-api`'s `otel-tracing` feature to export traces to an OTLP collector.
  The audit layer integrates OpenTelemetry as well.
- **Security.** Run the service as a non-root user, terminate TLS at a proxy,
  enable rate limiting (the API has built-in rate limiting; the proxy can add a
  second layer), and supply auth/secrets via the environment. Enable
  `oauth2-auth` when integrating with an identity provider.
- **Upgrades.** The workspace versions all crates together
  ([ADR-0002](adr/0002-cargo-workspace-dependency-policy.md)); deploy matching
  binary versions of the API and CLI.
