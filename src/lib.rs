//! # freerdp-sys
//!
//! Raw FFI bindings to [FreeRDP 3.x](https://github.com/FreeRDP/FreeRDP) —
//! a free, open-source implementation of the Remote Desktop Protocol (RDP).
//!
//! This crate provides low-level, unsafe Rust bindings generated via
//! [`bindgen`](https://crates.io/crates/bindgen). It is intended as a
//! foundational building block for higher-level safe Rust wrappers.
//!
//! ## Features
//!
//! | Feature | Default | Description |
//! |---------|---------|-------------|
//! | `vendored` | ✓ | Build FreeRDP from vendored source via CMake |
//! | `system` | | Link against a system-installed FreeRDP 3.x (pkg-config) |
//! | `generate-bindings` | | Generate bindings at build time (requires libclang) |
//!
//! ## Usage
//!
//! ```rust,no_run
//! use freerdp_sys::*;
//!
//! unsafe {
//!     let settings = freerdp_settings_new(FREERDP_SETTINGS_INSTANCE_COUNT);
//!     // ... configure settings ...
//!     freerdp_settings_free(settings);
//! }
//! ```
//!
//! ## Vendored Build
//!
//! With the default `vendored` feature, FreeRDP is compiled from source during
//! `cargo build`. Ensure you have:
//!
//! - CMake ≥ 3.13
//! - A C compiler (gcc/clang)
//! - OpenSSL development headers
//! - zlib development headers
//!
//! The FreeRDP source lives in `vendor/FreeRDP` as a git submodule.
//!
//! ## System Linking
//!
//! Disable default features and enable `system`:
//!
//! ```toml
//! [dependencies]
//! freerdp-sys = { version = "0.1", default-features = false, features = ["system"] }
//! ```
//!
//! Requires FreeRDP 3.x development packages installed and discoverable via pkg-config.
//!
//! ## Safety
//!
//! All functions in this crate are `unsafe` FFI calls. Refer to the
//! [FreeRDP documentation](https://pub.freerdp.com/api/) for correct usage,
//! memory ownership, and threading constraints.
//!
//! ## License
//!
//! Licensed under MIT OR Apache-2.0 at your option.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]
#![cfg_attr(docsrs, feature(doc_cfg))]

// When `generate-bindings` is active, include the freshly generated bindings from OUT_DIR.
// Otherwise, include the pre-generated bindings committed to the repository.
#[cfg(feature = "generate-bindings")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(not(feature = "generate-bindings"))]
#[path = "bindings.rs"]
mod bindings;

#[cfg(not(feature = "generate-bindings"))]
pub use bindings::*;

/// Re-export the FreeRDP version components this crate was built against.
pub mod version {
    /// Major version of the FreeRDP API this crate targets.
    pub const FREERDP_API_MAJOR: u32 = 3;
    /// The crate version.
    pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");
}
