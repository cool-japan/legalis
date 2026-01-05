# Legalis-RS Deployment Guide

This guide provides comprehensive instructions for deploying Legalis-RS in various environments using Docker.

## Table of Contents

- [Quick Start](#quick-start)
- [Development Deployment](#development-deployment)
- [Production Deployment](#production-deployment)
- [Environment Configuration](#environment-configuration)
- [Scaling & Performance](#scaling--performance)
- [Monitoring & Logging](#monitoring--logging)
- [Security Best Practices](#security-best-practices)
- [Troubleshooting](#troubleshooting)

---

## Quick Start

### Prerequisites

- Docker 24.0+ and Docker Compose 2.0+
- At least 2GB RAM available
- 5GB free disk space

### Run API Server (Production)

```bash
# Build and start the API server
docker-compose up -d api

# Check status
docker-compose ps

# View logs
docker-compose logs -f api

# Test the API
curl http://localhost:3000/health
```

The API will be available at `http://localhost:3000`.

---

## Development Deployment

### Setup Development Environment

1. **Start all services in development mode:**

```bash
docker-compose -f docker-compose.yml -f docker-compose.dev.yml up
```

This will:
- Mount your source code as a volume
- Enable auto-reload with `cargo-watch`
- Start PostgreSQL and Redis for testing
- Enable debug logging

2. **Access the services:**

- API Server: `http://localhost:3000`
- PostgreSQL: `localhost:5432` (user: legalis, password: dev_password_change_in_production)
- Redis: `localhost:6379`

3. **Development workflow:**

```bash
# Make changes to your code
# Docker will automatically detect changes and rebuild

# Run tests inside container
docker-compose exec api cargo test

# Check logs
docker-compose logs -f api

# Restart a specific service
docker-compose restart api
```

### Build Development Image

```bash
docker build -f Dockerfile.dev -t legalis-rs:dev .
```

---

## Production Deployment

### 1. Basic Production Setup

```bash
# Build production image
docker build -t legalis-rs:latest .

# Start services
docker-compose up -d api redis postgres

# Verify health
curl http://localhost:3000/health
```

### 2. Production with Nginx Reverse Proxy

#### Step 1: Generate SSL Certificates

```bash
# Using Let's Encrypt with certbot
mkdir -p ssl
docker run -it --rm -v $(pwd)/ssl:/etc/letsencrypt certbot/certbot certonly \
  --standalone \
  -d your-domain.com \
  -m your-email@example.com \
  --agree-tos

# Copy certificates
cp ssl/live/your-domain.com/fullchain.pem ssl/cert.pem
cp ssl/live/your-domain.com/privkey.pem ssl/key.pem
```

#### Step 2: Start with Nginx

```bash
# Start all services including nginx
docker-compose --profile production up -d

# Services will be available:
# - HTTPS: https://your-domain.com
# - HTTP: http://your-domain.com (redirects to HTTPS)
```

### 3. Environment Variables

Create a `.env` file:

```bash
# API Configuration
LEGALIS_HOST=0.0.0.0
LEGALIS_PORT=3000
RUST_LOG=info

# Database Configuration
POSTGRES_DB=legalis
POSTGRES_USER=legalis
POSTGRES_PASSWORD=<strong-password-here>

# Redis Configuration
REDIS_URL=redis://redis:6379

# Authentication (when implemented)
JWT_SECRET=<generate-strong-secret>
API_KEY_SALT=<generate-strong-salt>
```

**Generate secure secrets:**

```bash
# Generate JWT secret
openssl rand -base64 32

# Generate API key salt
openssl rand -base64 32
```

### 4. Docker Compose Production Override

Create `docker-compose.prod.yml`:

```yaml
version: '3.8'

services:
  api:
    restart: always
    environment:
      - RUST_LOG=warn
    deploy:
      resources:
        limits:
          cpus: '2'
          memory: 2G
        reservations:
          cpus: '1'
          memory: 1G

  postgres:
    restart: always
    environment:
      - POSTGRES_PASSWORD=${POSTGRES_PASSWORD}
    deploy:
      resources:
        limits:
          cpus: '1'
          memory: 1G

  redis:
    restart: always
    deploy:
      resources:
        limits:
          cpus: '0.5'
          memory: 512M
```

Usage:

```bash
docker-compose -f docker-compose.yml -f docker-compose.prod.yml up -d
```

---

## Environment Configuration

### Available Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `RUST_LOG` | `info` | Log level (trace, debug, info, warn, error) |
| `LEGALIS_HOST` | `0.0.0.0` | API server bind address |
| `LEGALIS_PORT` | `3000` | API server port |
| `RUST_BACKTRACE` | `0` | Enable backtrace (1 for full, 0 for disabled) |
| `POSTGRES_DB` | `legalis` | PostgreSQL database name |
| `POSTGRES_USER` | `legalis` | PostgreSQL username |
| `POSTGRES_PASSWORD` | - | PostgreSQL password (required) |
| `REDIS_URL` | `redis://redis:6379` | Redis connection string |

### Configuration Files

- `Cargo.toml` - Rust workspace configuration
- `.cargo/config.toml` - Cargo build configuration
- `nginx.conf` - Nginx reverse proxy configuration
- `docker-compose.yml` - Main service configuration
- `docker-compose.dev.yml` - Development overrides
- `docker-compose.prod.yml` - Production overrides (create this)

---

## Scaling & Performance

### Horizontal Scaling

Deploy multiple API instances behind a load balancer:

```yaml
# docker-compose.scale.yml
version: '3.8'

services:
  api:
    deploy:
      replicas: 3

  nginx:
    depends_on:
      - api
```

```bash
docker-compose -f docker-compose.yml -f docker-compose.scale.yml up -d --scale api=3
```

### Performance Tuning

#### API Server

```bash
# Increase worker threads
TOKIO_WORKER_THREADS=8 docker-compose up api
```

#### PostgreSQL

Add to `docker-compose.yml`:

```yaml
postgres:
  command: >
    postgres
    -c shared_buffers=256MB
    -c effective_cache_size=1GB
    -c max_connections=200
```

#### Redis

```yaml
redis:
  command: >
    redis-server
    --maxmemory 512mb
    --maxmemory-policy allkeys-lru
    --appendonly yes
```

### Resource Limits

Monitor resource usage:

```bash
docker stats

# Output:
# CONTAINER ID   NAME              CPU %     MEM USAGE / LIMIT
# abc123         legalis-api       45.2%     512MiB / 2GiB
# def456         legalis-postgres  12.3%     256MiB / 1GiB
# ghi789         legalis-redis     3.5%      64MiB / 512MiB
```

---

## Monitoring & Logging

### View Logs

```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f api

# Last 100 lines
docker-compose logs --tail=100 api

# Since specific time
docker-compose logs --since 2024-01-01T00:00:00 api
```

### Log Configuration

Centralized logging with fluentd (optional):

```yaml
# docker-compose.logging.yml
version: '3.8'

services:
  api:
    logging:
      driver: "fluentd"
      options:
        fluentd-address: localhost:24224
        tag: legalis.api
```

### Health Checks

Built-in health check endpoint:

```bash
# Manual health check
curl http://localhost:3000/health

# Expected response:
# {"status":"healthy","service":"legalis-api","version":"0.2.0"}

# Docker health status
docker inspect --format='{{.State.Health.Status}}' legalis-api
```

### Metrics (Future)

When Prometheus integration is added:

```bash
# Metrics endpoint
curl http://localhost:3000/metrics
```

---

## Security Best Practices

### 1. Use Strong Passwords

```bash
# Generate secure password
openssl rand -base64 32

# Update .env file
echo "POSTGRES_PASSWORD=$(openssl rand -base64 32)" >> .env
```

### 2. Run as Non-Root

The Dockerfile already configures a non-root user:

```dockerfile
RUN useradd -m -u 1000 -s /bin/bash legalis
USER legalis
```

### 3. Enable HTTPS

Always use HTTPS in production:

- Use Let's Encrypt for free SSL certificates
- Configure HSTS headers (see nginx.conf)
- Redirect HTTP to HTTPS

### 4. Network Security

```yaml
# Restrict external access
services:
  postgres:
    ports: []  # Don't expose to host
  redis:
    ports: []  # Don't expose to host
```

### 5. Secrets Management

Use Docker secrets for sensitive data:

```bash
# Create secret
echo "my-secret-password" | docker secret create postgres_password -

# Use in compose
services:
  postgres:
    secrets:
      - postgres_password
    environment:
      POSTGRES_PASSWORD_FILE: /run/secrets/postgres_password

secrets:
  postgres_password:
    external: true
```

### 6. Regular Updates

```bash
# Update base images
docker-compose pull

# Rebuild with latest dependencies
docker-compose build --no-cache

# Restart services
docker-compose up -d
```

---

## Troubleshooting

### Container Won't Start

```bash
# Check logs
docker-compose logs api

# Check container status
docker ps -a

# Inspect container
docker inspect legalis-api

# Common issues:
# 1. Port already in use
sudo lsof -i :3000

# 2. Permission issues
docker-compose down -v  # Remove volumes
docker-compose up -d
```

### Database Connection Issues

```bash
# Test PostgreSQL connection
docker-compose exec postgres psql -U legalis -d legalis

# Check if database is ready
docker-compose exec api sh -c 'until pg_isready -h postgres; do sleep 1; done'

# Reset database
docker-compose down -v
docker-compose up -d postgres
```

### Memory Issues

```bash
# Check memory usage
docker stats

# Increase Docker memory limit
# Docker Desktop: Settings → Resources → Memory

# Limit individual containers
services:
  api:
    deploy:
      resources:
        limits:
          memory: 2G
```

### Build Failures

```bash
# Clean build
docker system prune -a
docker volume prune

# Build with verbose output
docker-compose build --progress=plain api

# Check disk space
df -h
```

### Networking Issues

```bash
# Inspect network
docker network inspect legalis_legalis-network

# Test connectivity between containers
docker-compose exec api ping postgres
docker-compose exec api curl http://redis:6379

# Recreate network
docker-compose down
docker network prune
docker-compose up -d
```

---

## Backup & Recovery

### Database Backup

```bash
# Backup PostgreSQL
docker-compose exec -T postgres pg_dump -U legalis legalis > backup.sql

# Backup with timestamp
docker-compose exec -T postgres pg_dump -U legalis legalis > backup_$(date +%Y%m%d_%H%M%S).sql

# Restore
cat backup.sql | docker-compose exec -T postgres psql -U legalis legalis
```

### Volume Backup

```bash
# Backup all volumes
docker run --rm \
  -v legalis_postgres-data:/data \
  -v $(pwd):/backup \
  alpine tar czf /backup/postgres-data.tar.gz -C /data .

# Restore
docker run --rm \
  -v legalis_postgres-data:/data \
  -v $(pwd):/backup \
  alpine tar xzf /backup/postgres-data.tar.gz -C /data
```

---

## Cloud Deployment

### AWS ECS

```bash
# Build for multi-platform
docker buildx build --platform linux/amd64,linux/arm64 -t your-registry/legalis-rs:latest .

# Push to ECR
aws ecr get-login-password | docker login --username AWS --password-stdin your-account.dkr.ecr.region.amazonaws.com
docker push your-registry/legalis-rs:latest
```

### Google Cloud Run

```bash
# Build and push
gcloud builds submit --tag gcr.io/your-project/legalis-rs

# Deploy
gcloud run deploy legalis-api \
  --image gcr.io/your-project/legalis-rs \
  --platform managed \
  --region us-central1 \
  --allow-unauthenticated
```

### Digital Ocean App Platform

Use the `Dockerfile` directly with App Platform's Docker deployment.

---

## Maintenance

### Update Rust Version

Edit `Dockerfile`:

```dockerfile
FROM rust:1.84-slim-bookworm AS builder
```

Rebuild:

```bash
docker-compose build --no-cache api
```

### Clean Up

```bash
# Stop all services
docker-compose down

# Remove volumes (WARNING: deletes data)
docker-compose down -v

# Remove all unused Docker resources
docker system prune -a --volumes
```

---

## Support

For issues or questions:
- GitHub Issues: https://github.com/yourusername/legalis-rs/issues
- Documentation: https://github.com/yourusername/legalis-rs

---

**Last Updated:** December 2024
