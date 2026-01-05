#!/bin/bash

# Legalis-RS Dry-Run Publishing Script
# Tests publishing without actually uploading to crates.io
# Author: COOLJAPAN OU (Team Kitasan)
# License: MIT OR Apache-2.0

# Note: cargo publish builds with default features only.
# Z3 and other optional dependencies are not required.

CRATES=(
    # Level 1: Foundation
    "crates/legalis-core"

    # Level 2: Core Dependencies
    "crates/legalis-dsl"
    "crates/legalis-registry"

    # Level 3: Intelligence & Analysis
    "crates/legalis-verifier"
    "crates/legalis-llm"
    "crates/legalis-sim"
    "crates/legalis-diff"
    "crates/legalis-i18n"

    # Level 4: Output & Interoperability
    "crates/legalis-viz"
    "crates/legalis-chain"
    "crates/legalis-lod"
    "crates/legalis-interop"
    "crates/legalis-porting"
    "crates/legalis-audit"

    # Level 5: Infrastructure (API & CLI)
    "crates/legalis-api"
    "crates/legalis-cli"

    # Level 6: Jurisdictions
    "jurisdictions/jp"
    "jurisdictions/de"
    "jurisdictions/fr"
    "jurisdictions/us"
)

echo "==================================================="
echo "Legalis-RS Dry-Run Publishing Test"
echo "Version: 0.1.0"
echo "==================================================="
echo ""
echo "This will test all crates with --dry-run flag"
echo "No packages will be uploaded to crates.io"
echo ""
echo "Note: Testing with default features only"
echo "      (Z3 and other optional dependencies not required)"
echo ""
echo "Total crates to test: ${#CRATES[@]}"
echo "==================================================="
echo ""

failed_crates=()
success_count=0

for i in "${!CRATES[@]}"; do
    crate="${CRATES[$i]}"
    crate_name=$(basename "$crate")

    echo "[$((i+1))/${#CRATES[@]}] Testing: $crate_name"

    cd "/Users/kitasan/work/legalis/$crate" || {
        echo "  ❌ Directory not found: $crate"
        failed_crates+=("$crate_name (directory not found)")
        continue
    }

    # Run cargo publish --dry-run --allow-dirty
    if cargo publish --dry-run --allow-dirty 2>&1 | tee "/tmp/legalis_dryrun_${crate_name}.log"; then
        echo "  ✅ $crate_name passes dry-run"
        ((success_count++))
    else
        echo "  ❌ $crate_name failed dry-run"
        echo "     Check log: /tmp/legalis_dryrun_${crate_name}.log"
        failed_crates+=("$crate_name")
    fi

    echo ""
done

echo "==================================================="
echo "Dry-Run Test Summary"
echo "==================================================="
echo "Total crates tested: ${#CRATES[@]}"
echo "Passed: $success_count"
echo "Failed: ${#failed_crates[@]}"
echo ""

if [[ ${#failed_crates[@]} -eq 0 ]]; then
    echo "✅ All crates passed dry-run!"
    echo ""
    echo "You can now run the actual publish script:"
    echo "  /Users/kitasan/work/pub_legalis.sh"
else
    echo "❌ The following crates failed:"
    for crate in "${failed_crates[@]}"; do
        echo "  - $crate"
    done
    echo ""
    echo "Please fix the errors before publishing"
    echo "Check individual logs in /tmp/legalis_dryrun_*.log"
    exit 1
fi
echo ""
