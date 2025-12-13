# legalis-api

REST API server for Legalis-RS.

## Overview

`legalis-api` provides a production-ready REST API server built with Axum for managing and evaluating legal statutes programmatically.

## Features

- **REST API**: Full CRUD operations for statutes
- **Verification Endpoints**: Validate statute consistency
- **Evaluation Endpoints**: Check eligibility for entities
- **Health Checks**: Kubernetes-ready health endpoints
- **OpenAPI Compatible**: Standard REST conventions

## Endpoints

### Statutes

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/v1/statutes` | List all statutes |
| `GET` | `/api/v1/statutes/:id` | Get statute by ID |
| `POST` | `/api/v1/statutes` | Create new statute |
| `PUT` | `/api/v1/statutes/:id` | Update statute |
| `DELETE` | `/api/v1/statutes/:id` | Delete statute |

### Verification

| Method | Path | Description |
|--------|------|-------------|
| `POST` | `/api/v1/verify` | Verify statute(s) |
| `POST` | `/api/v1/verify/:id` | Verify specific statute |

### Evaluation

| Method | Path | Description |
|--------|------|-------------|
| `POST` | `/api/v1/evaluate` | Evaluate entity against statutes |
| `POST` | `/api/v1/evaluate/:id` | Evaluate against specific statute |

### Health

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/health` | Health check |
| `GET` | `/ready` | Readiness check |

## Usage

### As Library

```rust
use legalis_api::{create_router, AppState};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState::new());
    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### Via CLI

```bash
# Start API server
legalis serve --host 0.0.0.0 --port 3000
```

### API Examples

```bash
# Create a statute
curl -X POST http://localhost:3000/api/v1/statutes \
  -H "Content-Type: application/json" \
  -d '{"id": "adult-rights", "title": "Adult Rights", ...}'

# Verify all statutes
curl -X POST http://localhost:3000/api/v1/verify

# Evaluate an entity
curl -X POST http://localhost:3000/api/v1/evaluate \
  -H "Content-Type: application/json" \
  -d '{"entity": {"age": 25, "income": 50000}}'
```

## Configuration

| Environment Variable | Default | Description |
|---------------------|---------|-------------|
| `HOST` | `127.0.0.1` | Server bind address |
| `PORT` | `3000` | Server port |
| `LOG_LEVEL` | `info` | Logging level |

## License

MIT OR Apache-2.0
