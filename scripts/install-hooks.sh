#!/bin/bash
# Install git hooks for the legalis project

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Installing git hooks for legalis...${NC}"

# Check if we're in a git repository
if [ ! -d ".git" ]; then
    echo -e "${RED}Error: Not in a git repository${NC}"
    exit 1
fi

# Check if we're in the project root
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}Error: Not in project root directory${NC}"
    exit 1
fi

# Create hooks directory if it doesn't exist
mkdir -p .git/hooks

# Install pre-commit hook
if [ -f "scripts/pre-commit" ]; then
    cp scripts/pre-commit .git/hooks/pre-commit
    chmod +x .git/hooks/pre-commit
    echo -e "${GREEN}✓${NC} Installed pre-commit hook"
else
    echo -e "${RED}✗${NC} scripts/pre-commit not found"
    exit 1
fi

echo ""
echo -e "${GREEN}Git hooks installed successfully!${NC}"
echo ""
echo "The pre-commit hook will now run automatically before each commit."
echo ""
echo -e "${YELLOW}Available environment variables:${NC}"
echo "  SKIP_TESTS=1  Skip running tests in pre-commit hook"
echo ""
echo -e "${YELLOW}Example usage:${NC}"
echo "  git commit -m 'Your message'           # Run all checks"
echo "  SKIP_TESTS=1 git commit -m 'Fast'     # Skip tests"
echo "  git commit --no-verify -m 'Emergency' # Skip all checks (not recommended)"
echo ""
