#!/usr/bin/env bash
# Pre-commit hook for Legalis verifier
# This script runs verification checks before allowing a commit

set -e

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Running Legalis pre-commit checks...${NC}"

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}Error: Not in a Rust project directory${NC}"
    exit 1
fi

# Run cargo check
echo -e "${YELLOW}Running cargo check...${NC}"
if ! cargo check --quiet 2>&1; then
    echo -e "${RED}Cargo check failed! Please fix errors before committing.${NC}"
    exit 1
fi

# Run cargo clippy
echo -e "${YELLOW}Running cargo clippy...${NC}"
if ! cargo clippy --all-targets --all-features -- -D warnings 2>&1; then
    echo -e "${RED}Clippy warnings found! Please fix warnings before committing.${NC}"
    exit 1
fi

# Run cargo test
echo -e "${YELLOW}Running cargo test...${NC}"
if ! cargo test --quiet 2>&1; then
    echo -e "${RED}Tests failed! Please fix failing tests before committing.${NC}"
    exit 1
fi

# Run cargo fmt check
echo -e "${YELLOW}Checking code formatting...${NC}"
if ! cargo fmt --all -- --check 2>&1; then
    echo -e "${YELLOW}Code formatting issues found. Running cargo fmt...${NC}"
    cargo fmt --all
    echo -e "${YELLOW}Code has been formatted. Please review and stage the changes.${NC}"
    exit 1
fi

echo -e "${GREEN}All pre-commit checks passed!${NC}"
exit 0
