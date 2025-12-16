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
- [ ] Add simulation trigger endpoints
- [ ] Implement streaming simulation results
- [ ] Create simulation comparison endpoints
- [ ] Add saved simulation management

### Visualization
- [ ] Add visualization endpoints
- [ ] Implement dynamic graph generation
- [ ] Create interactive visualization support

## GraphQL

- [ ] Add GraphQL schema
- [ ] Implement query resolvers
- [ ] Add mutation resolvers
- [ ] Create subscription support for real-time updates
- [ ] Add DataLoader for N+1 optimization

## Authentication

- [ ] Add JWT authentication
- [ ] Implement OAuth2/OIDC support
- [ ] Add API key authentication
- [ ] Create role-based access control
- [ ] Implement rate limiting per user

## Documentation

- [ ] Add OpenAPI/Swagger documentation
- [ ] Create interactive API explorer
- [ ] Add request/response examples
- [ ] Create SDK generation from OpenAPI

## Real-time

- [ ] Add WebSocket support
- [ ] Implement Server-Sent Events
- [ ] Create real-time simulation streaming
- [ ] Add notification push support

## Performance

### Caching
- [ ] Add response caching
- [ ] Implement ETag support
- [ ] Add Redis caching layer
- [ ] Create cache invalidation strategy

### Optimization
- [x] Add request compression (gzip, brotli)
- [x] Implement pagination (offset-based, see search endpoint)
- [ ] Implement pagination with cursors
- [ ] Add field selection (GraphQL-style)
- [ ] Create connection pooling

## Observability

- [x] Add structured logging
- [x] Implement request tracing (via logging middleware)
- [x] Create metrics endpoint (Prometheus)
- [x] Add health check endpoints
- [ ] Implement distributed tracing (OpenTelemetry)

## Deployment

- [ ] Add Docker configuration
- [ ] Create Kubernetes manifests
- [x] Implement graceful shutdown
- [x] Add configuration via environment variables
- [ ] Create deployment documentation

## Testing

- [ ] Add integration tests
- [ ] Create API contract tests
- [ ] Implement load testing
- [ ] Add security testing (OWASP)
