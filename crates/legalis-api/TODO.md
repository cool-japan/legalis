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
- [x] Implement OAuth2/OIDC support (Keycloak, Auth0, Okta, Google, GitHub, Generic OIDC)
- [x] Add API key authentication
- [x] Create role-based access control (RBAC + ReBAC)
- [x] Implement rate limiting (global, needs per-user enhancement)

## Documentation

- [x] Add OpenAPI/Swagger documentation (comprehensive with examples)
- [x] Create interactive API explorer (Swagger UI)
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
- [x] Add Redis caching layer (with trait-based backend abstraction)
- [x] Create cache invalidation strategy (pattern-based, TTL, write-through)

### Optimization
- [x] Add request compression (gzip, brotli)
- [x] Implement pagination (offset-based, see search endpoint)
- [x] Implement pagination with cursors
- [x] Add field selection (GraphQL-style) for REST API endpoints
- [x] Create connection pooling (Redis ConnectionManager provides pooling)

## Observability

- [x] Add structured logging
- [x] Implement request tracing (via logging middleware)
- [x] Create metrics endpoint (Prometheus)
- [x] Add health check endpoints
- [x] Implement distributed tracing (OpenTelemetry with OTLP support)

## Deployment

- [x] Add Docker configuration
- [x] Create docker-compose.yml with services (API, Redis, Prometheus, Grafana)
- [x] Create Kubernetes manifests (Deployment, Service, Ingress, HPA, ConfigMap, ServiceMonitor)
- [x] Implement graceful shutdown
- [x] Add configuration via environment variables
- [ ] Create deployment documentation

## Testing

- [x] Add integration tests (108 tests covering REST API, GraphQL, authentication, health checks, search, and batch operations)
- [ ] Create API contract tests (framework ready via security module)
- [ ] Implement load testing
- [x] Add security testing (OWASP vulnerability checks, input validation, security headers)

## Roadmap for 0.1.0 Series

### Advanced Endpoints (v0.1.1)
- [x] Add bulk verification endpoint with streaming results (POST /api/v1/verify/bulk/stream)
- [ ] Add statute suggestion endpoint (AI-powered)
- [x] Add compliance check endpoint for entity (POST /api/v1/simulate/compliance)
- [x] Add what-if analysis endpoint (POST /api/v1/simulate/whatif)
- [x] Add statute comparison matrix endpoint (POST /api/v1/statutes/compare/matrix)

### GraphQL Enhancements (v0.1.2)
- [x] Add subscription support for real-time updates
- [ ] Add DataLoader for N+1 optimization
- [x] Add relay-style pagination
- [x] Add field-level permissions
- [x] Add query complexity limiting

### Authentication & Authorization (v0.1.3)
- [x] Add OAuth2/OIDC support (Keycloak, Auth0, Okta, Google, GitHub, Generic OIDC)
- [x] Add fine-grained permissions per statute (via ReBAC with grant/revoke endpoints)
- [x] Add audit logging for all mutations (comprehensive audit trail with filtering)
- [x] Add API key scoping and rotation (create, list, revoke, rotate with custom scopes and expiration)
- [x] Add multi-tenant isolation (basic tenant context extraction via headers)

### Real-time Features (v0.1.4)
- [x] Add WebSocket support for live updates
- [x] Add pub/sub for statute changes (GraphQL mutations now broadcast WS notifications)
- [x] Add collaborative editing support (operational transformation, conflict resolution)
- [x] Add real-time conflict detection (concurrent edit detection with auto-resolution)
- [x] Add presence awareness (who's viewing what)

### Caching & Performance (v0.1.5)
- [x] Add Redis caching layer (with trait-based backend abstraction)
- [x] Add cache invalidation strategy (pattern-based, TTL, write-through)
- [x] Add edge caching (CDN-friendly with Cache-Control, Vary, and surrogate keys)
- [x] Add query result caching with TTL
- [x] Add cache warming strategies (warm() method with batch loading, stats tracking)

### Observability (v0.1.6)
- [x] Add OpenTelemetry distributed tracing (OTLP with configurable sampling)
- [x] Add custom metrics for business logic (statute operations, verification results, simulation outcomes, permissions, etc.)
- [x] Add request sampling for high-volume endpoints (adaptive, random, head-based strategies)
- [x] Add anomaly detection for API usage (time-series based with z-score analysis)
- [x] Add SLO/SLI tracking (availability, latency, error rate, throughput with error budgets)

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
- [x] Add URL-based versioning (v1, v2 with path-based routing)
- [x] Add header-based versioning (X-API-Version and Accept headers)
- [x] Add deprecation warnings (Sunset and Warning headers)
- [x] Add version migration tools (VersionMigration with breaking changes documentation)
- [x] Add backward compatibility testing (CompatibilityChecker for features and endpoints)
