# Legalis-RS Platform-Specific Notes

## Supported Platforms

Legalis-RS is designed to be cross-platform and works on:
- ✅ macOS (Apple Silicon & Intel)
- ✅ Linux (Ubuntu, Debian, Fedora, Arch, etc.)
- ✅ Windows (via WSL, MSYS2, or Git Bash)

## Installation by Platform

### macOS

#### Using Homebrew (Recommended)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Note: OxiZ SMT solver is Pure Rust - no external dependencies needed

# Clone and build
git clone https://github.com/cool-japan/legalis-rs
cd legalis-rs
cargo build --release
```

#### Homebrew Locations
- **Apple Silicon (M1/M2/M3):** `/opt/homebrew/`
- **Intel:** `/usr/local/`

### Linux

#### Ubuntu/Debian

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Optional: Install Z3 (only for z3-solver feature)
sudo apt update
sudo apt install libz3-dev

# Clone and build
git clone https://github.com/cool-japan/legalis-rs
cd legalis-rs
cargo build --release
```

#### Fedora/RHEL/CentOS

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Note: OxiZ SMT solver is Pure Rust - no external dependencies needed

# Clone and build
git clone https://github.com/cool-japan/legalis-rs
cd legalis-rs
cargo build --release
```

#### Arch Linux

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Optional: Install Z3
# Note: OxiZ SMT solver is Pure Rust - no external dependencies needed

# Clone and build
git clone https://github.com/cool-japan/legalis-rs
cd legalis-rs
cargo build --release
```

### Windows

#### Option 1: WSL (Windows Subsystem for Linux) - Recommended

```bash
# Install WSL if not already installed
wsl --install

# Inside WSL, follow Linux instructions
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Optional: Install Z3
sudo apt install libz3-dev

# Clone and build
git clone https://github.com/cool-japan/legalis-rs
cd legalis-rs
cargo build --release
```

#### Option 2: MSYS2

```bash
# Install MSYS2 from https://www.msys2.org/

# In MSYS2 terminal
pacman -S mingw-w64-x86_64-rust

# Optional: Install Z3
# Note: OxiZ SMT solver is Pure Rust - no external dependencies needed

# Clone and build
git clone https://github.com/cool-japan/legalis-rs
cd legalis-rs
cargo build --release
```

#### Option 3: Native Windows (Visual Studio)

```powershell
# Install Rust from https://rustup.rs/

# Optional: Install Z3
# Download from https://github.com/Z3Prover/z3/releases
# Set environment variables in PowerShell:
$env:Z3_SYS_Z3_HEADER = "C:\path\to\z3\include\z3.h"
$env:LIB = "C:\path\to\z3\lib;$env:LIB"

# Clone and build
git clone https://github.com/cool-japan/legalis-rs
cd legalis-rs
cargo build --release
```

## SMT Solver (OxiZ)

**OxiZ is a Pure Rust SMT solver** - no external dependencies or installation needed!

The `smt-solver` feature uses OxiZ, which compiles directly with your Rust code:

```bash
# Enable SMT solver features (Pure Rust, no external dependencies)
cargo build --features smt-solver
```

### Why Pure Rust?

- ✅ **No C/C++ dependencies**: Works on all platforms without external libraries
- ✅ **Easy to build**: Just `cargo build`, no system packages needed
- ✅ **Portable**: Works everywhere Rust works
- ✅ **No environment variables**: Everything is handled by Cargo

## Publishing from Different Platforms

The publishing scripts work on all platforms without external dependencies:

```bash
# macOS
/Users/kitasan/work/pub_legalis.sh

# Linux
~/work/pub_legalis.sh

# Windows (WSL)
~/work/pub_legalis.sh

# Windows (Git Bash)
/c/Users/YourName/work/pub_legalis.sh
```

## Testing Platform Compatibility

### Quick Platform Test

```bash
# Test basic build
cargo build

# Test with SMT solver (Pure Rust OxiZ)
cargo build --features smt-solver

# Run tests
cargo nextest run

# Check platform info
uname -a
rustc --version --verbose
```

### Platform-Specific CI/CD

GitHub Actions configuration supports all platforms:

```yaml
strategy:
  matrix:
    os: [ubuntu-latest, macos-latest, windows-latest]

steps:
  - name: Install Z3 (Ubuntu)
    if: matrix.os == 'ubuntu-latest'
    run: sudo apt install libz3-dev

  - name: Install Z3 (macOS)
    if: matrix.os == 'macos-latest'
    run: brew install z3

  - name: Install Z3 (Windows)
    if: matrix.os == 'windows-latest'
    run: choco install z3
```

## Cross-Compilation

Legalis-RS supports cross-compilation to various targets:

```bash
# List available targets
rustup target list

# Add a target
rustup target add aarch64-unknown-linux-gnu

# Cross-compile
cargo build --target aarch64-unknown-linux-gnu --release
```

### Common Targets

- **Linux ARM64:** `aarch64-unknown-linux-gnu`
- **Linux x86_64:** `x86_64-unknown-linux-gnu`
- **macOS ARM64:** `aarch64-apple-darwin`
- **macOS x86_64:** `x86_64-apple-darwin`
- **Windows x86_64:** `x86_64-pc-windows-msvc`
- **WebAssembly:** `wasm32-unknown-unknown` (limited support)

## Docker Support

Platform-independent deployment via Docker:

```bash
# Build Docker image
docker build -t legalis-rs:0.1.0 .

# Run
docker run -p 8080:8080 legalis-rs:0.1.0
```

See `Dockerfile` and `docker-compose.yml` in the repository.

## Troubleshooting by Platform

### macOS: "Library not loaded"

```bash
# Add Z3 to library path
export DYLD_LIBRARY_PATH=/opt/homebrew/opt/z3/lib:$DYLD_LIBRARY_PATH
```

### Linux: "cannot find -lz3"

```bash
# Install development package
sudo apt install libz3-dev  # Ubuntu/Debian
sudo dnf install z3-devel    # Fedora
# Note: OxiZ SMT solver is Pure Rust - no external dependencies needed            # Arch

# Or set library path
export LD_LIBRARY_PATH=/usr/lib:$LD_LIBRARY_PATH
```

### Windows: "link.exe failed"

```bash
# Use WSL or MSYS2 instead of native Windows
# OR install Visual Studio Build Tools
# OR use the Windows Subsystem for Linux (WSL)
```

## Performance Notes by Platform

### Compilation Times

- **macOS M1/M2:** ~1-2 minutes (release build)
- **Linux (modern CPU):** ~2-3 minutes (release build)
- **Windows (WSL):** ~3-4 minutes (release build)
- **Windows (native):** ~4-5 minutes (release build)

### Runtime Performance

All platforms show similar runtime performance. Rust's zero-cost abstractions ensure consistent performance across platforms.

## Recommended Platforms for Development

1. **macOS** (Apple Silicon) - Best performance, native Homebrew support
2. **Linux** (Ubuntu 22.04+) - Best for CI/CD, package manager support
3. **Windows** (WSL2) - Good compatibility, Linux-like environment

## Recommended Platforms for Production

1. **Linux** (Docker containers) - Best for production deployments
2. **macOS** (Server) - If using macOS Server infrastructure
3. **Windows** (WSL2 or native) - If Windows infrastructure required

---

**Last Updated:** 2026-01-05
**Author:** COOLJAPAN OU (Team Kitasan)
**License:** MIT OR Apache-2.0
