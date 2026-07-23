# freerdp-sys

[![CI](https://github.com/GOODBOY008/freerdp-sys/actions/workflows/ci.yml/badge.svg)](https://github.com/GOODBOY008/freerdp-sys/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/freerdp-sys.svg)](https://crates.io/crates/freerdp-sys)
[![Docs.rs](https://docs.rs/freerdp-sys/badge.svg)](https://docs.rs/freerdp-sys)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![MSRV](https://img.shields.io/badge/rustc-1.71+-orange.svg)](#requirements)

Low-level Rust FFI bindings to [FreeRDP 3.x](https://github.com/FreeRDP/FreeRDP) — a free,
open-source implementation of the Remote Desktop Protocol (RDP).

## Overview

`freerdp-sys` is a **general-purpose, standalone** `-sys` crate that exposes the FreeRDP 3.x C API
to Rust. It is not tied to any specific application or framework — any Rust project that needs RDP
client or server functionality can build on top of it.

**Key characteristics:**

- Bindings generated via [`bindgen`](https://crates.io/crates/bindgen) from FreeRDP 3.x headers
- Vendored CMake build compiles FreeRDP from source (zero system dependencies at link time)
- Optional system linking via `pkg-config` for distro-packaged FreeRDP
- Pre-generated bindings included so downstream users don't need `libclang`
- Dual-licensed (MIT / Apache-2.0) for maximum compatibility

## Installation

### Vendored build (default, recommended)

Compiles FreeRDP 3.x from source during `cargo build`. No system FreeRDP installation required.

```toml
[dependencies]
freerdp-sys = "0.1"
```

### System linking

Links against a system-installed FreeRDP 3.x discovered via `pkg-config`.

```toml
[dependencies]
freerdp-sys = { version = "0.1", default-features = false, features = ["system"] }
```

### With runtime binding generation

Generates fresh bindings at build time using `bindgen` (requires `libclang`).

```toml
[dependencies]
freerdp-sys = { version = "0.1", features = ["generate-bindings"] }
```

## Feature Flags

| Feature | Default | Description |
|---------|:-------:|-------------|
| `vendored` | ✓ | Compile FreeRDP 3.x from vendored source via CMake. Produces static libraries. |
| `system` | | Link against a system-installed FreeRDP 3.x discovered via `pkg-config`. |
| `generate-bindings` | | Run `bindgen` at build time to regenerate FFI bindings (requires `libclang`). When disabled, pre-generated `src/bindings.rs` is used. |

> **Note:** `vendored` and `system` are mutually exclusive. Enable exactly one.

## Requirements

### Vendored build

| Dependency | Minimum Version | Purpose |
|------------|----------------|---------|
| CMake | 3.13 | Build system for FreeRDP |
| C compiler | C11 | gcc, clang, or MSVC |
| OpenSSL | 1.1.1+ | TLS/crypto support |
| zlib | 1.2+ | Compression |
| Rust | 1.71+ | Crate MSRV |

**Platform-specific:**

- **Linux:** `libssl-dev`, `zlib1g-dev`, `cmake`, `build-essential`
- **macOS:** `brew install cmake openssl zlib`
- **Windows:** Visual Studio Build Tools, vcpkg OpenSSL + zlib

### System linking

- FreeRDP 3.x development packages (`libfreerdp3-dev` or equivalent)
- `pkg-config`
- WinPR 3.x development packages

### Binding generation (optional)

- `libclang` (used by the `bindgen` crate)
  - Linux: `libclang-dev`
  - macOS: `brew install llvm` (set `LIBCLANG_PATH` if needed)

## Usage

```rust
use freerdp_sys::*;

fn main() {
    unsafe {
        // Create a FreeRDP instance
        let instance = freerdp_new();
        assert!(!instance.is_null());

        // Access settings
        let settings = freerdp_get_settings(instance);
        // ... configure hostname, port, credentials, etc. ...

        // Connect
        let result = freerdp_connect(instance);
        if result != 0 {
            println!("Connected!");
            // ... RDP session ...
            freerdp_disconnect(instance);
        }

        // Cleanup
        freerdp_free(instance);
    }
}
```

> **Safety:** All functions in this crate are `unsafe` FFI calls. Refer to the
> [FreeRDP API documentation](https://pub.freerdp.com/api/) for correct usage,
> memory ownership, and threading constraints.

## Build Configuration

### Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `FREERDP_CMAKE_DEFS` | Extra CMake definitions (semicolon-separated) | `WITH_CAIRO=ON;WITH_FFMPEG=ON` |
| `FREERDP_BINDGEN_CLANG_ARGS` | Extra clang arguments passed to bindgen | `-DWITH_SMARTCARD=1` |
| `LIBCLANG_PATH` | Path to libclang (if not in default search path) | `/usr/lib/llvm-18/lib` |
| `CMAKE` | Path to cmake binary | `/usr/local/bin/cmake` |

### CMake Build Options

The vendored build configures FreeRDP with these defaults (optimized for a minimal static library):

```
BUILD_SHARED_LIBS=OFF    WITH_SERVER=OFF
WITH_CLIENT=OFF          WITH_CHANNELS=OFF
WITH_WINPR_TOOLS=OFF     WITH_MANPAGES=OFF
BUILD_TESTING=OFF        WITH_SAMPLE=OFF
WITH_PROXY=OFF           WITH_SHADOW=OFF
CMAKE_BUILD_TYPE=Release
```

Override any of these via `FREERDP_CMAKE_DEFS`:

```sh
FREERDP_CMAKE_DEFS="WITH_SERVER=ON;WITH_CHANNELS=ON" cargo build
```

## Development

### Getting Started

```sh
# Clone with submodule
git clone --recurse-submodules https://github.com/GOODBOY008/freerdp-sys.git
cd freerdp-sys

# Or initialize submodule after clone
git submodule update --init --recursive
```

### Building

```sh
# Default vendored build (pre-generated bindings)
cargo build

# Vendored + regenerate bindings from headers
cargo build --features vendored,generate-bindings

# System linking
cargo build --no-default-features --features system

# Check without linking (uses pre-generated bindings)
cargo check --no-default-features
```

### Testing

```sh
cargo test --features vendored,generate-bindings
```

### Regenerating Pre-built Bindings

```sh
# Using the helper script
./scripts/regenerate-bindings.sh

# Or manually
cargo build --features vendored,generate-bindings
cp target/debug/build/freerdp-sys-*/out/bindings.rs src/bindings.rs
rustfmt src/bindings.rs
```

## Project Structure

```
freerdp-sys/
├── Cargo.toml              # Crate manifest with feature flags & crates.io metadata
├── build.rs                # Build script: cmake vendored build + bindgen orchestration
├── src/
│   ├── lib.rs              # Crate root: conditional binding inclusion, docs
│   ├── bindings.rs         # Pre-generated bindings (no libclang needed by consumers)
│   └── wrapper.h           # C header input for bindgen (FreeRDP 3.x + WinPR)
├── vendor/
│   └── FreeRDP/            # Git submodule → FreeRDP 3.x source (stable-3.0)
├── cmake/
│   └── CMakeLists.txt      # Optional CMake wrapper for standalone/test builds
├── scripts/
│   └── regenerate-bindings.sh  # Helper to regenerate src/bindings.rs
├── .github/workflows/
│   └── ci.yml              # CI: check, clippy, test, fmt, publish
├── .gitmodules             # Submodule config (FreeRDP stable-3.0)
├── rustfmt.toml            # Formatting configuration
├── LICENSE-MIT             # MIT license
├── LICENSE-APACHE          # Apache 2.0 license
└── README.md               # This file
```

## Troubleshooting

### "vendored FreeRDP source not found"

```
freerdp-sys: vendored FreeRDP source not found at `vendor/FreeRDP`.
```

**Fix:** Initialize the git submodule:
```sh
git submodule update --init --recursive
```

### "could not find FreeRDP 3.x via pkg-config"

```
freerdp-sys: could not find FreeRDP 3.x via pkg-config.
```

**Fix:** Install FreeRDP 3.x development packages:
```sh
# Debian/Ubuntu
sudo apt install libfreerdp3-dev libwinpr3-dev

# Fedora
sudo dnf install freerdp-devel

# Or switch to vendored build
```

### bindgen / libclang errors

```
thread 'main' panicked: Unable to find libclang
```

**Fix:** Install libclang and set the path:
```sh
# Linux
sudo apt install libclang-dev

# macOS
brew install llvm
export LIBCLANG_PATH="$(brew --prefix llvm)/lib"
```

### CMake not found

```
Failed to run cmake: No such file or directory
```

**Fix:** Install CMake ≥ 3.13:
```sh
# Linux
sudo apt install cmake

# macOS
brew install cmake

# Or set CMAKE env var
export CMAKE=/path/to/cmake
```

### Linker errors (missing symbols)

If you see undefined symbol errors at link time, ensure all platform dependencies are installed:

```sh
# Linux
sudo apt install libssl-dev zlib1g-dev

# macOS (if using Homebrew OpenSSL)
export OPENSSL_ROOT_DIR="$(brew --prefix openssl)"
```

## Publishing to crates.io

This crate is configured for automated publishing via GitHub Actions on tag push:

```sh
# Tag a release
git tag v0.1.0
git push origin v0.1.0
# CI will run checks, then publish to crates.io
```

Manual publishing:
```sh
cargo package --no-verify   # Verify the package
cargo publish               # Publish to crates.io
```

## Contributing

Contributions are welcome! Please follow these guidelines:

1. **Fork** the repository and create a feature branch
2. **Initialize submodules:** `git submodule update --init --recursive`
3. **Make your changes** following the existing code style
4. **Format:** `cargo fmt --all`
5. **Check:** `cargo clippy --features vendored,generate-bindings`
6. **Test:** `cargo test --features vendored,generate-bindings`
7. **Commit** with a descriptive message
8. **Open a Pull Request** against `main`

### Code Style

- Follow `rustfmt` defaults (configured in `rustfmt.toml`)
- All public items must have doc comments
- Unsafe code must have safety comments explaining invariants
- Keep bindings generation reproducible — commit updated `src/bindings.rs` when headers change

### Reporting Issues

When reporting bugs, please include:
- OS and architecture
- Rust version (`rustc --version`)
- Feature flags used
- Full error output
- CMake version (for vendored builds)

## License

Licensed under either of:

- **MIT License** ([LICENSE-MIT](LICENSE-MIT))
- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE))

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate shall be dual licensed as above, without any
additional terms or conditions.

## Acknowledgments

- [FreeRDP](https://github.com/FreeRDP/FreeRDP) — The FreeRDP project and its contributors
- [bindgen](https://github.com/rust-lang/rust-bindgen) — Automatic Rust FFI binding generation
- [cmake-rs](https://github.com/rust-lang/cmake-rs) — CMake build integration for Cargo
