#!/usr/bin/env bash
# Setup Z3 environment variables for building with z3-solver feature
# Source this file before running cargo commands: source setup-z3-env.sh

set -e

echo "Setting up Z3 environment variables..."

# Detect platform and set Z3 paths accordingly
if [[ "$OSTYPE" == "darwin"* ]]; then
  # macOS
  if [[ -d "/opt/homebrew/opt/z3" ]]; then
    # Apple Silicon (M1/M2/M3)
    echo "Detected: macOS Apple Silicon"
    export Z3_SYS_Z3_HEADER=/opt/homebrew/opt/z3/include/z3.h
    export LIBRARY_PATH=/opt/homebrew/opt/z3/lib:${LIBRARY_PATH:-}
    export DYLD_LIBRARY_PATH=/opt/homebrew/opt/z3/lib:${DYLD_LIBRARY_PATH:-}
    echo "✓ Z3 paths configured for Apple Silicon"
  elif [[ -d "/usr/local/opt/z3" ]]; then
    # Intel Mac
    echo "Detected: macOS Intel"
    export Z3_SYS_Z3_HEADER=/usr/local/opt/z3/include/z3.h
    export LIBRARY_PATH=/usr/local/opt/z3/lib:${LIBRARY_PATH:-}
    export DYLD_LIBRARY_PATH=/usr/local/opt/z3/lib:${DYLD_LIBRARY_PATH:-}
    echo "✓ Z3 paths configured for Intel Mac"
  else
    echo "⚠ Warning: Z3 not found. Install with: brew install z3"
    exit 1
  fi
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
  # Linux
  if [[ -f "/usr/include/z3.h" ]]; then
    echo "Detected: Linux (system package)"
    export Z3_SYS_Z3_HEADER=/usr/include/z3.h
    export LD_LIBRARY_PATH=/usr/lib:${LD_LIBRARY_PATH:-}
    echo "✓ Z3 paths configured"
  elif [[ -f "/usr/local/include/z3.h" ]]; then
    echo "Detected: Linux (local install)"
    export Z3_SYS_Z3_HEADER=/usr/local/include/z3.h
    export LD_LIBRARY_PATH=/usr/local/lib:${LD_LIBRARY_PATH:-}
    echo "✓ Z3 paths configured"
  else
    echo "⚠ Warning: Z3 not found. Install with your package manager:"
    echo "  Ubuntu/Debian: sudo apt install libz3-dev"
    echo "  Fedora/RHEL: sudo dnf install z3-devel"
    echo "  Arch Linux: sudo pacman -S z3"
    exit 1
  fi
else
  echo "⚠ Warning: Unsupported OS: $OSTYPE"
  exit 1
fi

echo ""
echo "Environment variables set:"
echo "  Z3_SYS_Z3_HEADER=$Z3_SYS_Z3_HEADER"
if [[ "$OSTYPE" == "darwin"* ]]; then
  echo "  LIBRARY_PATH=$LIBRARY_PATH"
  echo "  DYLD_LIBRARY_PATH=$DYLD_LIBRARY_PATH"
else
  echo "  LD_LIBRARY_PATH=$LD_LIBRARY_PATH"
fi
echo ""
echo "You can now run: cargo build --all-features"
echo "             or: cargo nextest run --all-features"
