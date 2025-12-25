# legalis-api TODO

## Completed

- [x] Axum-based REST API server
- [x] CRUD endpoints for statutes
- [x] Error handling with proper HTTP status codes
- [x] CORS support
- [x] Basic response structure

## Endpoints

### Statute Operations
- [x] Add batch statute operations
- [x] Implement statute versioning endpoints
- [x] Add search/filter endpoints
- [x] Create statute comparison endpoint
- [x] Add statute fork/clone endpoint (via versioning)

### Verification
- [x] Add verification endpoints
- [x] Implement async verification with polling
- [x] Create verification report endpoints (via detailed endpoint)
- [x] Add batch verification support

### Simulation
- [x] Add simulation trigger endpoints
- [x] Implement streaming simulation results (SSE)
- [x] Create simulation comparison endpoints
- [x] Add saved simulation management

### Visualization
- [x] Add visualization endpoints
- [x] Implement dynamic graph generation (SVG, Mermaid, PlantUML, etc.)
- [x] Create interactive visualization support (HTML output with themes)

## GraphQL

- [x] Add GraphQL schema
- [x] Implement query resolvers
- [x] Add mutation resolvers
- [ ] Create subscription support for real-time updates
- [ ] Add DataLoader for N+1 optimization

## Authentication

- [x] Add JWT authentication (basic extraction)
- [ ] Implement OAuth2/OIDC support
- [x] Add API key authentication
- [x] Create role-based access control (RBAC + ReBAC)
- [x] Implement rate limiting (global, needs per-user enhancement)

## Documentation

- [x] Add OpenAPI/Swagger documentation (comprehensive with examples)
- [ ] Create interactive API explorer (Swagger UI)
- [x] Add request/response examples
- [ ] Create SDK generation from OpenAPI

## Real-time

- [ ] Add WebSocket support
- [x] Implement Server-Sent Events
- [x] Create real-time simulation streaming
- [ ] Add notification push support

## Performance

### Caching
- [x] Add response caching (in-memory cache store)
- [x] Implement ETag support
- [ ] Add Redis caching layer
- [ ] Create cache invalidation strategy

### Optimization
- [x] Add request compression (gzip, brotli)
- [x] Implement pagination (offset-based, see search endpoint)
- [x] Implement pagination with cursors
- [ ] Add field selection (GraphQL-style)
- [ ] Create connection pooling

## Observability

- [x] Add structured logging
- [x] Implement request tracing (via logging middleware)
- [x] Create metrics endpoint (Prometheus)
- [x] Add health check endpoints
- [ ] Implement distributed tracing (OpenTelemetry)

## Deployment

- [x] Add Docker configuration
- [x] Create docker-compose.yml with services (API, Redis, Prometheus, Grafana)
- [x] Create Kubernetes manifests (Deployment, Service, Ingress, HPA, ConfigMap, ServiceMonitor)
- [x] Implement graceful shutdown
- [x] Add configuration via environment variables
- [ ] Create deployment documentation

## Testing

- [ ] Add integration tests
- [ ] Create API contract tests
- [ ] Implement load testing
- [ ] Add security testing (OWASP)
