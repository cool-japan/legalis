# Legalis-RS Publishing Guide

This document explains how to publish Legalis-RS crates to crates.io.

## Prerequisites

1. **crates.io account** with publish permissions
2. **Rust toolchain** (1.86+ recommended)
3. **Logged in to crates.io**:
   ```bash
   cargo login <your-api-token>
   ```

**Note:** Z3 SMT Solver is **optional**. It's only required if you want to use the `z3-solver` feature in `legalis-verifier`. Default features do not require Z3.

### Platform Support

The publishing scripts support multiple platforms:
- ✅ **macOS** (Apple Silicon & Intel, via Homebrew)
- ✅ **Linux** (Ubuntu/Debian/Fedora/Arch via apt/yum/pacman)
- ✅ **Windows** (WSL, MSYS2, Git Bash)

The scripts automatically detect the platform and configure Z3 paths if Z3 is installed.

For detailed platform-specific instructions, see `PLATFORM-NOTES.md`.

## Publishing Scripts

We provide three scripts to help with the publishing process:

### 1. Dry-Run Test Script

**Location:** `./publish_dryrun.sh`

Tests all crates with `--dry-run` to verify they're ready for publishing without actually uploading them.

```bash
cd /Users/kitasan/work/legalis
./publish_dryrun.sh
```

**What it does:**
- Tests all 23 crates in dependency order
- Runs `cargo publish --dry-run` for each
- Saves logs to `/tmp/legalis_dryrun_*.log`
- Reports success/failure for each crate

**When to use:** Always run this first before actual publishing!

### 2. Main Publishing Script

**Location:** `/Users/kitasan/work/pub_legalis.sh`

Publishes all Legalis-RS crates to crates.io in the correct dependency order.

```bash
/Users/kitasan/work/pub_legalis.sh
```

**What it does:**
- Sets required Z3 environment variables
- Publishes all 23 crates in dependency order
- Waits 20 seconds between each publish (crates.io requirement)
- Asks for confirmation before starting
- Stops on first error

**Estimated time:** ~7 minutes (23 crates × 20 seconds)

### 3. Single Crate Publishing Script

**Location:** `./publish_one.sh`

Publishes a single crate (used internally by the main script, but can be used standalone).

```bash
./publish_one.sh crates/legalis-core
```

## Publishing Order

Crates are published in dependency order to ensure dependencies are available before dependent crates:

### Level 1: Foundation
1. `legalis-core` - Core types and traits

### Level 2: Core Dependencies
2. `legalis-dsl` - Domain Specific Language parser
3. `legalis-registry` - Statute registry

### Level 3: Intelligence & Analysis
4. `legalis-verifier` - Formal verification (Z3)
5. `legalis-llm` - LLM integration
6. `legalis-sim` - Simulation engine
7. `legalis-diff` - Statute diffing
8. `legalis-i18n` - Internationalization

### Level 4: Output & Interoperability
9. `legalis-viz` - Visualization
10. `legalis-chain` - Smart contract export
11. `legalis-lod` - Linked Open Data
12. `legalis-interop` - Format interop
13. `legalis-porting` - Cross-jurisdiction porting
14. `legalis-audit` - Audit trail

### Level 5: Infrastructure
15. `legalis-api` - REST API server
16. `legalis` - Command-line interface

### Level 6: Jurisdictions
17. `legalis-jp` - Japanese legal system
18. `legalis-de` - German legal system
19. `legalis-fr` - French legal system
20. `legalis-us` - US legal system
21. `legalis-eu` - EU legal system (GDPR, Competition, Consumer Rights)
22. `legalis-sg` - Singapore legal system (Companies Act, Employment, PDPA)
23. `legalis-uk` - UK legal system (Employment, Consumer Rights, Companies)

## Step-by-Step Publishing Process

### Step 1: Pre-Publishing Checks

```bash
# 1. Ensure you're on the main branch
git checkout main

# 2. Ensure working directory is clean
git status

# 3. Verify all tests pass
cargo nextest run

# 4. Verify release build works
cargo build --release

# 5. Run dry-run test
./publish_dryrun.sh
```

### Step 2: Commit and Tag

```bash
# Commit any final changes
git add -A
git commit -m "Release v0.1.0"

# Create and push tag
git tag v0.1.0
git push origin main
git push origin v0.1.0
```

### Step 3: Publish to crates.io

```bash
# Run the main publishing script
/Users/kitasan/work/pub_legalis.sh

# The script will:
# 1. Show the publishing order
# 2. Ask for confirmation
# 3. Publish each crate with 20-second intervals
# 4. Report success or stop on error
```

### Step 4: Verify Publication

After publishing, verify all crates on crates.io:

- https://crates.io/crates/legalis-core
- https://crates.io/crates/legalis-dsl
- etc.

### Step 5: Create GitHub Release

1. Go to: https://github.com/cool-japan/legalis-rs/releases/new
2. Select tag: `v0.1.0`
3. Title: `Legalis-RS v0.1.0`
4. Copy release notes from `/tmp/RELEASE-0.1.0-SUMMARY.md`
5. Publish release

## Publishing Examples (Optional)

Examples are commented out in `pub_legalis.sh` by default. To publish them:

1. Edit `/Users/kitasan/work/pub_legalis.sh`
2. Uncomment desired examples in the `EXAMPLES` array
3. Add them to the `CRATES` array
4. Run the publishing script

## Troubleshooting

### Error: "crate already exists"

This means the crate was already published. You cannot publish the same version twice. Either:
- Bump the version number in `Cargo.toml`
- Skip this crate (comment it out in the script)

### Error: "failed to verify package tarball"

Check the error message for details. Common causes:
- Missing files in `Cargo.toml` `include` field
- Path dependencies not properly configured
- Z3 environment variables not set

### Error: "Z3 library not found"

This error only occurs if you're building with `--features z3-solver`.

**Solution 1:** Don't use the z3-solver feature (default build doesn't need it):
```bash
cargo build  # No Z3 required
```

**Solution 2:** If you need Z3, install it and set environment variables:
```bash
brew install z3
export Z3_SYS_Z3_HEADER=/opt/homebrew/opt/z3/include/z3.h
export LIBRARY_PATH=/opt/homebrew/opt/z3/lib:$LIBRARY_PATH
```

### Publishing stopped midway

If publishing stops due to an error:
1. Fix the error in the problematic crate
2. Edit the publish script and comment out already-published crates
3. Re-run the script

## Environment Variables

**No environment variables are required** for default features.

Z3 environment variables are only needed if building with `--features z3-solver`:

```bash
# Only if using z3-solver feature
Z3_SYS_Z3_HEADER=/opt/homebrew/opt/z3/include/z3.h
LIBRARY_PATH=/opt/homebrew/opt/z3/lib:$LIBRARY_PATH
DYLD_LIBRARY_PATH=/opt/homebrew/opt/z3/lib:$DYLD_LIBRARY_PATH
```

## Rate Limiting

crates.io recommends waiting at least 10-20 seconds between publishes. Our script uses 20 seconds to be safe.

If you need to publish faster:
- Edit `INTERVAL` in `pub_legalis.sh`
- Minimum recommended: 10 seconds

## Yanking a Release

If you need to yank a published version:

```bash
# Yank a specific crate version
cargo yank --vers 0.1.0 legalis-core

# Un-yank if needed
cargo yank --undo --vers 0.1.0 legalis-core
```

## Post-Publication Checklist

- [ ] All 23 crates published successfully
- [ ] All crates visible on crates.io
- [ ] GitHub release created with release notes
- [ ] Documentation updated (if needed)
- [ ] Announcement made (if desired)
- [ ] Update roadmap for next version

## Support

For issues with publishing:
- Check crates.io status: https://status.crates.io/
- Contact: COOLJAPAN OU (Team Kitasan)
- License: MIT OR Apache-2.0

---

**Version:** 0.1.0
**Last Updated:** 2026-01-05
**Author:** COOLJAPAN OU (Team Kitasan)
