# CI/CD Integration Guide for Legalis Verifier

This guide provides instructions for integrating Legalis verifier into various CI/CD pipelines.

## GitHub Actions

Create `.github/workflows/verify.yml` in your repository:

```yaml
name: Legalis Verification

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always

jobs:
  verify:
    name: Verify Statutes
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v3

    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
        components: rustfmt, clippy

    - name: Cache cargo registry
      uses: actions/cache@v3
      with:
        path: ~/.cargo/registry
        key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}

    - name: Cache cargo index
      uses: actions/cache@v3
      with:
        path: ~/.cargo/git
        key: ${{ runner.os }}-cargo-index-${{ hashFiles('**/Cargo.lock') }}

    - name: Cache cargo build
      uses: actions/cache@v3
      with:
        path: target
        key: ${{ runner.os }}-cargo-build-target-${{ hashFiles('**/Cargo.lock') }}

    - name: Check formatting
      run: cargo fmt --all -- --check

    - name: Run clippy
      run: cargo clippy --all-targets --all-features -- -D warnings

    - name: Build
      run: cargo build --verbose --all-features

    - name: Run tests
      run: cargo test --verbose --all-features

    - name: Run verifier on statutes
      run: |
        # Add your statute verification commands here
        # Example: cargo run --bin verify-statutes -- ./statutes/*.json
        echo "Running statute verification..."
```

## GitLab CI

Create `.gitlab-ci.yml`:

```yaml
image: rust:latest

variables:
  CARGO_HOME: $CI_PROJECT_DIR/.cargo

stages:
  - check
  - test
  - verify

cache:
  paths:
    - .cargo/
    - target/

before_script:
  - rustc --version
  - cargo --version

check:formatting:
  stage: check
  script:
    - rustup component add rustfmt
    - cargo fmt --all -- --check

check:clippy:
  stage: check
  script:
    - rustup component add clippy
    - cargo clippy --all-targets --all-features -- -D warnings

test:
  stage: test
  script:
    - cargo test --all-features --verbose

verify:statutes:
  stage: verify
  script:
    - cargo build --release --all-features
    # Add your statute verification commands here
    - echo "Running statute verification..."
  artifacts:
    reports:
      # Export verification results
    paths:
      - verification-report.json
      - verification-report.html
    expire_in: 1 week
```

## Jenkins

Create a `Jenkinsfile`:

```groovy
pipeline {
    agent any

    environment {
        CARGO_HOME = "${WORKSPACE}/.cargo"
    }

    stages {
        stage('Setup') {
            steps {
                sh 'rustc --version'
                sh 'cargo --version'
            }
        }

        stage('Format Check') {
            steps {
                sh 'cargo fmt --all -- --check'
            }
        }

        stage('Lint') {
            steps {
                sh 'cargo clippy --all-targets --all-features -- -D warnings'
            }
        }

        stage('Build') {
            steps {
                sh 'cargo build --verbose --all-features'
            }
        }

        stage('Test') {
            steps {
                sh 'cargo test --verbose --all-features'
            }
        }

        stage('Verify Statutes') {
            steps {
                script {
                    // Run statute verification
                    sh '''
                        cargo build --release --all-features
                        # Add verification commands here
                    '''
                }
            }
        }
    }

    post {
        always {
            // Archive verification reports
            archiveArtifacts artifacts: 'verification-*.json,verification-*.html', allowEmptyArchive: true
        }
    }
}
```

## Circle CI

Create `.circleci/config.yml`:

```yaml
version: 2.1

jobs:
  verify:
    docker:
      - image: rust:latest
    steps:
      - checkout
      - restore_cache:
          keys:
            - v1-cargo-cache-{{ checksum "Cargo.lock" }}
            - v1-cargo-cache-

      - run:
          name: Install components
          command: |
            rustup component add rustfmt clippy

      - run:
          name: Check formatting
          command: cargo fmt --all -- --check

      - run:
          name: Run clippy
          command: cargo clippy --all-targets --all-features -- -D warnings

      - run:
          name: Build
          command: cargo build --verbose --all-features

      - run:
          name: Run tests
          command: cargo test --verbose --all-features

      - run:
          name: Verify statutes
          command: |
            # Add verification commands here
            echo "Running statute verification..."

      - save_cache:
          key: v1-cargo-cache-{{ checksum "Cargo.lock" }}
          paths:
            - ~/.cargo
            - target

      - store_artifacts:
          path: verification-report.json
          destination: reports/

      - store_artifacts:
          path: verification-report.html
          destination: reports/

workflows:
  version: 2
  verify:
    jobs:
      - verify
```

## Travis CI

Create `.travis.yml`:

```yaml
language: rust
rust:
  - stable
  - beta

cache: cargo

before_script:
  - rustup component add rustfmt clippy

script:
  - cargo fmt --all -- --check
  - cargo clippy --all-targets --all-features -- -D warnings
  - cargo build --verbose --all-features
  - cargo test --verbose --all-features
  # Add verification commands here

after_success:
  # Upload verification reports
  - echo "Verification complete"
```

## Docker Integration

Create a `Dockerfile` for containerized verification:

```dockerfile
FROM rust:latest as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

# Build the project
RUN cargo build --release --all-features

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/legalis-verifier /usr/local/bin/

WORKDIR /statutes

ENTRYPOINT ["legalis-verifier"]
```

## Best Practices

1. **Run all checks**: Always run formatting, linting, and tests
2. **Cache dependencies**: Use caching to speed up builds
3. **Fail fast**: Configure pipelines to fail on warnings
4. **Artifact reports**: Store verification reports as artifacts
5. **Parallel execution**: Run independent checks in parallel when possible
6. **Status badges**: Add CI status badges to your README

## Environment Variables

Useful environment variables for CI/CD:

- `CARGO_TERM_COLOR=always` - Enable colored output
- `RUST_BACKTRACE=1` - Enable backtraces for debugging
- `RUSTFLAGS="-D warnings"` - Treat warnings as errors

## Troubleshooting

### Cache Issues
```bash
# Clear cargo cache
rm -rf ~/.cargo/registry
rm -rf target
```

### Dependency Problems
```bash
# Update dependencies
cargo update
```

### Test Failures
```bash
# Run tests with output
cargo test -- --nocapture --test-threads=1
```
