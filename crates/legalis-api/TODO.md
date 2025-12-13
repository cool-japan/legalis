# legalis-api TODO

## Completed

- [x] Axum-based REST API server
- [x] CRUD endpoints for statutes
- [x] Error handling with proper HTTP status codes
- [x] CORS support
- [x] Basic response structure

## Endpoints

### Statute Operations
- [ ] Add batch statute operations
- [ ] Implement statute versioning endpoints
- [ ] Add search/filter endpoints
- [ ] Create statute comparison endpoint
- [ ] Add statute fork/clone endpoint

### Verification
- [ ] Add verification endpoints
- [ ] Implement async verification with polling
- [ ] Create verification report endpoints
- [ ] Add batch verification support

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
- [ ] Add request compression (gzip, brotli)
- [ ] Implement pagination with cursors
- [ ] Add field selection (GraphQL-style)
- [ ] Create connection pooling

## Observability

- [ ] Add structured logging
- [ ] Implement request tracing
- [ ] Create metrics endpoint (Prometheus)
- [ ] Add health check endpoints
- [ ] Implement distributed tracing

## Deployment

- [ ] Add Docker configuration
- [ ] Create Kubernetes manifests
- [ ] Implement graceful shutdown
- [ ] Add configuration via environment variables
- [ ] Create deployment documentation

## Testing

- [ ] Add integration tests
- [ ] Create API contract tests
- [ ] Implement load testing
- [ ] Add security testing (OWASP)
