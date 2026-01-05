#!/bin/bash
# Validation script for legalis project
# Can be run manually or in CI to validate the entire codebase

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}═══════════════════════════════════════${NC}"
echo -e "${BLUE}  Legalis Codebase Validation${NC}"
echo -e "${BLUE}═══════════════════════════════════════${NC}"
echo ""

# Track overall status
FAILED=0
TOTAL_CHECKS=0
PASSED_CHECKS=0

# Function to run a check
run_check() {
    local name="$1"
    local command="$2"

    TOTAL_CHECKS=$((TOTAL_CHECKS + 1))
    echo -e "${YELLOW}➜${NC} $name..."

    if eval "$command" > /tmp/check_output.txt 2>&1; then
        echo -e "${GREEN}✓${NC} $name passed"
        PASSED_CHECKS=$((PASSED_CHECKS + 1))
        return 0
    else
        echo -e "${RED}✗${NC} $name failed"
        echo -e "${YELLOW}Output:${NC}"
        cat /tmp/check_output.txt | head -20
        echo ""
        FAILED=1
        return 1
    fi
}

# 1. Format check
run_check "Code formatting" "cargo fmt --all -- --check"

# 2. Clippy
run_check "Clippy lints" "cargo clippy --all-targets --all-features -- -D warnings"

# 3. Build
run_check "Debug build" "cargo build --all-targets --all-features"

# 4. Release build
run_check "Release build" "cargo build --all-targets --all-features --release"

# 5. Tests
run_check "Unit tests" "cargo test --all-features --lib"

# 6. Doc tests
run_check "Doc tests" "cargo test --all-features --doc"

# 7. Integration tests
run_check "Integration tests" "cargo test --all-features --test '*'"

# 8. Documentation
run_check "Documentation build" "cargo doc --all-features --no-deps"

# 9. Benchmarks compile
run_check "Benchmark compilation" "cargo bench --all-features --no-run"

# 10. Check for unused dependencies
if command -v cargo-udeps > /dev/null 2>&1; then
    run_check "Unused dependencies" "cargo +nightly udeps --all-targets"
else
    echo -e "${YELLOW}⊘${NC} Skipping unused dependencies check (cargo-udeps not installed)"
    echo -e "${YELLOW}   Install with: cargo install cargo-udeps${NC}"
fi

# 11. Security audit
if command -v cargo-audit > /dev/null 2>&1; then
    run_check "Security audit" "cargo audit"
else
    echo -e "${YELLOW}⊘${NC} Skipping security audit (cargo-audit not installed)"
    echo -e "${YELLOW}   Install with: cargo install cargo-audit${NC}"
fi

# Clean up
rm -f /tmp/check_output.txt

# Print summary
echo ""
echo -e "${BLUE}═══════════════════════════════════════${NC}"
echo -e "${BLUE}  Validation Summary${NC}"
echo -e "${BLUE}═══════════════════════════════════════${NC}"
echo ""
echo -e "Total checks: $TOTAL_CHECKS"
echo -e "${GREEN}Passed: $PASSED_CHECKS${NC}"

if [ $FAILED -eq 0 ]; then
    echo -e "${RED}Failed: 0${NC}"
    echo ""
    echo -e "${GREEN}✓ All validation checks passed!${NC}"
    echo -e "${GREEN}  The codebase is ready for commit/push.${NC}"
    exit 0
else
    FAILED_COUNT=$((TOTAL_CHECKS - PASSED_CHECKS))
    echo -e "${RED}Failed: $FAILED_COUNT${NC}"
    echo ""
    echo -e "${RED}✗ Some validation checks failed${NC}"
    echo -e "${YELLOW}  Please fix the issues above before pushing.${NC}"
    exit 1
fi
