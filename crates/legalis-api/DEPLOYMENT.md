# legalis-api Deployment Guide

This document covers building, configuring, and running the `legalis-api` crate in both Docker Compose and Kubernetes environments.

For workspace-level deployment context see [/DEPLOYMENT.md](../../DEPLOYMENT.md) and [/docs/deployment.md](../../docs/deployment.md).

## Table of Contents

- [Overview](#overview)
- [Quick Start (Docker Compose)](#quick-start-docker-compose)
- [Docker Image](#docker-image)
- [Docker Compose](#docker-compose)
- [Kubernetes Deployment](#kubernetes-deployment)
- [Feature Flags](#feature-flags)
- [Binaries](#binaries)
- [Monitoring](#monitoring)
- [Health Checks](#health-checks)
- [Related Resources](#related-resources)

---

## Overview

`legalis-api` is the REST/GraphQL/gRPC API server for Legalis-RS. It exposes:

- REST endpoints via [axum](https://github.com/tokio-rs/axum)
- GraphQL via [async-graphql](https://github.com/async-graphql/async-graphql)
- gRPC via [tonic](https://github.com/hyperium/tonic) (enabled by the `grpc` default feature)
- Prometheus metrics at `/metrics`
- Health endpoints at `/health` (liveness) and `/health/ready` (readiness)

The server listens on port **3000** by default.

---

## Quick Start (Docker Compose)

Requirements: Docker 24.0+ and Docker Compose 2.0+.

```bash
# Build and start only the API server
docker compose -f crates/legalis-api/docker-compose.yml up -d api

# Confirm the server is healthy
curl http://localhost:3000/health

# Start the full observability stack (API + Redis + Prometheus + Grafana)
docker compose -f crates/legalis-api/docker-compose.yml up -d

# Stream logs
docker compose -f crates/legalis-api/docker-compose.yml logs -f api

# Tear down
docker compose -f crates/legalis-api/docker-compose.yml down
```

---

## Docker Image

### Building

The `Dockerfile` (at `crates/legalis-api/Dockerfile`) uses a two-stage build. The build context must be the workspace root so that the full `Cargo.toml` / `Cargo.lock` and all crates are available:

```bash
# From the workspace root
docker build \
  -f crates/legalis-api/Dockerfile \
  -t legalis/api:latest \
  .
```

### Multi-stage build details

| Stage | Base image | Purpose |
|---|---|---|
| `builder` | `rust:1.75-slim` | Compile `legalis-api-server` in release mode |
| runtime | `debian:bookworm-slim` | Minimal runtime with `ca-certificates` and `libssl3` |

The compiled binary is copied from `/build/target/release/legalis-api-server` into `/app/legalis-api-server` in the runtime stage. A non-root user `legalis` (UID 1000) is created and owns `/app`; the container runs as that user.

### Environment variables

These are the variables recognised by the server. Values shown are the defaults applied in the Kubernetes ConfigMap (`k8s/configmap.yaml`) and Deployment manifest (`k8s/deployment.yaml`):

| Variable | Default (K8s) | Description |
|---|---|---|
| `RUST_LOG` | `info,legalis_api=debug` (ConfigMap) / `info` (Deployment) | Tracing/log filter |
| `LEGALIS_HOST` | `0.0.0.0` | Address the HTTP server binds to |
| `LEGALIS_PORT` | `3000` | TCP port the HTTP server listens on |

In Docker Compose (`docker-compose.yml`) the defaults are:

| Variable | Value |
|---|---|
| `RUST_LOG` | `info` |
| `LEGALIS_HOST` | `0.0.0.0` |
| `LEGALIS_PORT` | `3000` |

---

## Docker Compose

The `docker-compose.yml` defines four services:

| Service | Image | Host port | Notes |
|---|---|---|---|
| `api` | built from workspace `Dockerfile` | `3000` | API server; restarts unless stopped |
| `redis` | `redis:7-alpine` | `6379` | Optional — enables `redis-cache` feature |
| `prometheus` | `prom/prometheus:latest` | `9090` | Mounts `prometheus.yml` (read-only) |
| `grafana` | `grafana/grafana:latest` | `3001` | Visualises Prometheus data; `GF_SECURITY_ADMIN_PASSWORD=admin` |

All services share the `legalis-network` bridge network. Named volumes `redis-data`, `prometheus-data`, and `grafana-data` provide persistence.

The Prometheus configuration (`prometheus.yml`) scrapes `api:3000/metrics` every 10 s under the global 15 s evaluation interval.

---

## Kubernetes Deployment

### Prerequisites

- Kubernetes 1.25+
- `kubectl` configured for the target cluster
- NGINX Ingress Controller (`ingressClassName: nginx`)
- [cert-manager](https://cert-manager.io/) with a `ClusterIssuer` named `letsencrypt-prod`
- Prometheus Operator (for the `ServiceMonitor` resource)

### Apply manifests (in correct order)

```bash
# 1. ConfigMap — must exist before the Deployment reads env vars
kubectl apply -f crates/legalis-api/k8s/configmap.yaml

# 2. Deployment + Service + PodDisruptionBudget
kubectl apply -f crates/legalis-api/k8s/deployment.yaml

# 3. Ingress
kubectl apply -f crates/legalis-api/k8s/ingress.yaml

# 4. HorizontalPodAutoscaler
kubectl apply -f crates/legalis-api/k8s/hpa.yaml

# 5. Prometheus ServiceMonitor
kubectl apply -f crates/legalis-api/k8s/servicemonitor.yaml
```

### ConfigMap

`legalis-api-config` (`k8s/configmap.yaml`) supplies three keys:

| Key | Value |
|---|---|
| `RUST_LOG` | `info,legalis_api=debug` |
| `LEGALIS_HOST` | `0.0.0.0` |
| `LEGALIS_PORT` | `3000` |

### Deployment

Key parameters from `k8s/deployment.yaml`:

| Parameter | Value |
|---|---|
| Initial replicas | `3` |
| Container image | `legalis/api:0.2.0` |
| Container port | `3000` (named `http`) |
| CPU request / limit | `250m` / `500m` |
| Memory request / limit | `256Mi` / `512Mi` |
| Run as UID | `1000` (non-root) |

**PodDisruptionBudget** (`legalis-api-pdb`): `minAvailable: 1` — at least one pod remains available during voluntary disruptions.

### HPA configuration

`k8s/hpa.yaml` configures `autoscaling/v2`:

| Parameter | Value |
|---|---|
| Min replicas | `3` |
| Max replicas | `10` |
| CPU scale trigger | 70% average utilisation |
| Memory scale trigger | 80% average utilisation |
| Scale-down stabilisation | 300 s; max 50% of pods per 60 s |
| Scale-up stabilisation | 0 s; max 100% of pods per 30 s or 2 pods per 60 s (whichever is greater) |

### Ingress configuration

`k8s/ingress.yaml` routes HTTPS traffic:

| Parameter | Value |
|---|---|
| `ingressClassName` | `nginx` |
| Host | `api.legalis.example.com` |
| TLS secret | `legalis-api-tls` (managed by cert-manager) |
| ClusterIssuer | `letsencrypt-prod` |
| Rate limit (requests) | `100` |
| Rate limit (rps) | `10` |

All HTTP traffic is redirected to HTTPS (`ssl-redirect: "true"`).

### Prometheus ServiceMonitor

`k8s/servicemonitor.yaml` instructs the Prometheus Operator to scrape the `legalis-api` Service:

| Parameter | Value |
|---|---|
| Port | `http` (maps to container port 3000) |
| Path | `/metrics` |
| Scrape interval | `30s` |
| Scrape timeout | `10s` |
| Prometheus label | `prometheus: kube-prometheus` |

---

## Feature Flags

Defined in `Cargo.toml` under `[features]`:

### `grpc` (default)

Enabled by default. Pulls in the full gRPC stack:

- `tonic` — async gRPC framework
- `tonic-prost` — protobuf codec
- `prost` — protobuf runtime
- `tonic-reflection` — server reflection
- `tonic-health` — standard health protocol
- `tonic-web` — gRPC-Web transcoding

Disable with `--no-default-features` if you only need the REST/GraphQL surface.

### `redis-cache`

Enables the `redis` dependency (`redis = { version = "1.2", features = ["tokio-comp", "connection-manager"] }`). Activates in-memory caching backed by a Redis instance (see the `redis` service in `docker-compose.yml`).

### `oauth2-auth`

Enables OAuth 2.0 authentication middleware via the `oauth2` and `reqwest` crates.

### `otel-tracing`

Enables OpenTelemetry distributed tracing:

- `opentelemetry` 0.32
- `opentelemetry_sdk` 0.32 (with `rt-tokio`)
- `opentelemetry-otlp` 0.32 (exported over gRPC/tonic)
- `tracing-opentelemetry` 0.33

Requires an OTLP-compatible collector (Jaeger, Grafana Tempo, etc.) reachable from the pod.

---

## Binaries

### `legalis-api-server`

The primary binary. Starts the HTTP/gRPC server. Build and run locally:

```bash
cargo build --release --bin legalis-api-server
LEGALIS_HOST=127.0.0.1 LEGALIS_PORT=3000 ./target/release/legalis-api-server
```

### `export-openapi`

Generates the OpenAPI specification for the REST API:

```bash
cargo run --bin export-openapi > openapi.json
```

---

## Monitoring

### Prometheus (Docker Compose)

`prometheus.yml` is mounted at `/etc/prometheus/prometheus.yml` in the `prometheus` service. Configuration:

```yaml
global:
  scrape_interval: 15s
  evaluation_interval: 15s
  external_labels:
    monitor: 'legalis-monitor'

scrape_configs:
  - job_name: 'legalis-api'
    static_configs:
      - targets: ['api:3000']
    metrics_path: '/metrics'
    scrape_interval: 10s
```

Access Prometheus at `http://localhost:9090`.  
Access Grafana at `http://localhost:3001` (default credentials: `admin` / `admin`).

### Kubernetes

Apply `k8s/servicemonitor.yaml` after the Prometheus Operator is installed. The ServiceMonitor targets pods with label `app: legalis-api` and scrapes `/metrics` on the `http` port every 30 s.

---

## Health Checks

| Endpoint | Purpose | Used by |
|---|---|---|
| `GET /health` | Liveness — server process alive | Dockerfile `HEALTHCHECK`, K8s `livenessProbe`, Docker Compose healthcheck |
| `GET /health/ready` | Readiness — server ready to serve traffic | K8s `readinessProbe` |

Kubernetes probe configuration (from `k8s/deployment.yaml`):

| Probe | Initial delay | Period | Timeout | Failure threshold |
|---|---|---|---|---|
| Liveness (`/health`) | 10 s | 30 s | 5 s | 3 |
| Readiness (`/health/ready`) | 5 s | 10 s | 3 s | 3 |

---

## Related Resources

- [Workspace Deployment Guide](../../DEPLOYMENT.md) — Docker-based deployment across the full Legalis-RS workspace
- [docs/deployment.md](../../docs/deployment.md) — Additional deployment architecture documentation
- [legalis-verifier IDE Integration](../legalis-verifier/IDE_INTEGRATION.md) — Example topic-doc convention used across crates
