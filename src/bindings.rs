//! Pre-generated FreeRDP 3.x bindings.
//!
//! This file is checked in so that downstream users can build without libclang.
//! To regenerate, build with the `generate-bindings` feature enabled:
//!
//! ```sh
//! cargo build --features generate-bindings
//! # Then copy from target/<profile>/build/freerdp-sys-*/out/bindings.rs
//! ```
//!
//! Alternatively, use the `scripts/regenerate-bindings.sh` helper.
//!
//! WARNING: This is a placeholder. Run the regeneration script after cloning
//! with the FreeRDP submodule initialized to produce the actual bindings.

// Placeholder — replace with actual generated bindings via:
//   cargo build --features vendored,generate-bindings
//   cp target/debug/build/freerdp-sys-*/out/bindings.rs src/bindings.rs

#![allow(unused)]

use std::os::raw::{c_char, c_int, c_uint, c_void};

/// Opaque FreeRDP instance handle.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rdp_freerdp {
    _unused: [u8; 0],
}

/// Opaque FreeRDP settings handle.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rdp_settings {
    _unused: [u8; 0],
}

/// Opaque FreeRDP context handle.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rdp_context {
    _unused: [u8; 0],
}

/// Opaque FreeRDP input handle.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rdp_input {
    _unused: [u8; 0],
}

/// Opaque WinPR stream handle.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wStream {
    _unused: [u8; 0],
}

/// Settings instance count constant.
pub const FREERDP_SETTINGS_INSTANCE_COUNT: c_uint = 1;

extern "C" {
    /// Create a new FreeRDP instance.
    pub fn freerdp_new() -> *mut rdp_freerdp;

    /// Free a FreeRDP instance.
    pub fn freerdp_free(instance: *mut rdp_freerdp);

    /// Create new settings with the given instance count.
    pub fn freerdp_settings_new(count: c_uint) -> *mut rdp_settings;

    /// Free settings.
    pub fn freerdp_settings_free(settings: *mut rdp_settings);

    /// Connect to the remote server.
    pub fn freerdp_connect(instance: *mut rdp_freerdp) -> c_int;

    /// Disconnect from the remote server.
    pub fn freerdp_disconnect(instance: *mut rdp_freerdp) -> c_int;

    /// Get the context associated with an instance.
    pub fn freerdp_get_context(instance: *const rdp_freerdp) -> *mut rdp_context;

    /// Get the settings associated with an instance.
    pub fn freerdp_get_settings(instance: *const rdp_freerdp) -> *mut rdp_settings;

    /// Get the input handle for sending input events.
    pub fn freerdp_get_input(instance: *const rdp_freerdp) -> *mut rdp_input;

    /// Check if the instance is connected.
    pub fn freerdp_is_connected(instance: *const rdp_freerdp) -> c_int;

    /// Get the last error code.
    pub fn freerdp_get_last_error(context: *const rdp_context) -> c_uint;

    /// Get a human-readable error string.
    pub fn freerdp_get_last_error_name(code: c_uint) -> *const c_char;

    /// Get a human-readable error description.
    pub fn freerdp_get_last_error_info(code: c_uint) -> *const c_char;
}
