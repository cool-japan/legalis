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
- [x] Create subscription support for real-time updates (notifications, statute_events, verification_events, simulation_events)
- [x] Add DataLoader for N+1 optimization (TODO: needs refinement for trait signature)

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

- [x] Add WebSocket support (with pub/sub notifications)
- [x] Implement Server-Sent Events
- [x] Create real-time simulation streaming
- [x] Add notification push support (via WebSocket)

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
- [x] Add field selection (GraphQL-style) for REST API endpoints
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

- [x] Add integration tests (42 tests covering REST API, GraphQL, authentication, health checks, search, and batch operations)
- [ ] Create API contract tests
- [ ] Implement load testing
- [ ] Add security testing (OWASP)

## Roadmap for 0.1.0 Series

### Advanced Endpoints (v0.1.1)
- [ ] Add bulk verification endpoint with streaming results
- [ ] Add statute suggestion endpoint (AI-powered)
- [ ] Add compliance check endpoint for entity
- [ ] Add what-if analysis endpoint
- [ ] Add statute comparison matrix endpoint

### GraphQL Enhancements (v0.1.2)
- [ ] Add subscription support for real-time updates
- [ ] Add DataLoader for N+1 optimization
- [ ] Add relay-style pagination
- [ ] Add field-level permissions
- [ ] Add query complexity limiting

### Authentication & Authorization (v0.1.3)
- [ ] Add OAuth2/OIDC support (Keycloak, Auth0, Okta)
- [ ] Add fine-grained permissions per statute
- [ ] Add audit logging for all mutations
- [ ] Add API key scoping and rotation
- [ ] Add multi-tenant isolation

### Real-time Features (v0.1.4)
- [ ] Add WebSocket support for live updates
- [ ] Add pub/sub for statute changes
- [ ] Add collaborative editing support
- [ ] Add real-time conflict detection
- [ ] Add presence awareness (who's viewing what)

### Caching & Performance (v0.1.5)
- [ ] Add Redis caching layer
- [ ] Add cache invalidation strategy
- [ ] Add edge caching (CDN-friendly)
- [ ] Add query result caching with TTL
- [ ] Add cache warming strategies

### Observability (v0.1.6)
- [ ] Add OpenTelemetry distributed tracing
- [ ] Add custom metrics for business logic
- [ ] Add request sampling for high-volume endpoints
- [ ] Add anomaly detection for API usage
- [ ] Add SLO/SLI tracking

### SDK Generation (v0.1.7)
- [ ] Generate TypeScript SDK from OpenAPI
- [ ] Generate Python SDK from OpenAPI
- [ ] Generate Go SDK from OpenAPI
- [ ] Generate Rust SDK from OpenAPI
- [ ] Add SDK versioning and compatibility

### Federation (v0.1.8)
- [ ] Add GraphQL federation support
- [ ] Add cross-service registry queries
- [ ] Add federated verification
- [ ] Add distributed simulation coordination
- [ ] Add cross-region replication endpoints

### API Versioning (v0.1.9)
- [ ] Add URL-based versioning (v1, v2)
- [ ] Add header-based versioning
- [ ] Add deprecation warnings
- [ ] Add version migration tools
- [ ] Add backward compatibility testing
