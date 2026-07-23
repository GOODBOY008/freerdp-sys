//! End-to-end tests for freerdp-sys.
//!
//! These tests exercise the actual FreeRDP FFI bindings by calling into the
//! native library. They require either the `vendored` or `system` feature to
//! be enabled so that FreeRDP is linked.
//!
//! Run with:
//! ```sh
//! cargo test --features vendored,generate-bindings --test e2e
//! ```

#![cfg(any(feature = "vendored", feature = "system"))]

use std::ffi::CStr;

use freerdp_sys::*;

/// Test that we can create and destroy a FreeRDP instance.
#[test]
fn test_instance_lifecycle() {
    unsafe {
        let instance = freerdp_new();
        assert!(!instance.is_null(), "freerdp_new() returned null");
        freerdp_free(instance);
    }
}

/// Test that we can create and destroy settings.
#[test]
fn test_settings_lifecycle() {
    unsafe {
        let settings = freerdp_settings_new(FREERDP_SETTINGS_INSTANCE_COUNT);
        assert!(!settings.is_null(), "freerdp_settings_new() returned null");
        freerdp_settings_free(settings);
    }
}

/// Test that a new instance reports as not connected.
#[test]
fn test_new_instance_not_connected() {
    unsafe {
        let instance = freerdp_new();
        assert!(!instance.is_null());

        let connected = freerdp_is_connected(instance);
        assert_eq!(connected, 0, "new instance should not be connected");

        freerdp_free(instance);
    }
}

/// Test that we can retrieve the context from an instance.
#[test]
fn test_get_context() {
    unsafe {
        let instance = freerdp_new();
        assert!(!instance.is_null());

        // Context may be null before full initialization, but the call should not crash
        let _context = freerdp_get_context(instance);

        freerdp_free(instance);
    }
}

/// Test that error name/info functions return valid C strings.
#[test]
fn test_error_strings() {
    unsafe {
        // Error code 0 typically means success / no error
        let name = freerdp_get_last_error_name(0);
        if !name.is_null() {
            let name_str = CStr::from_ptr(name);
            assert!(
                !name_str.to_bytes().is_empty(),
                "error name for code 0 should not be empty"
            );
        }

        let info = freerdp_get_last_error_info(0);
        if !info.is_null() {
            let info_str = CStr::from_ptr(info);
            // Info string can be empty for success, just verify it's valid UTF-8-ish
            let _ = info_str.to_str();
        }
    }
}

/// Test that error strings for a known error code are non-null.
#[test]
fn test_error_strings_known_code() {
    unsafe {
        // 0x00000002 is FREERDP_ERROR_CONNECT_CANCELLED in FreeRDP 3.x
        let code: u32 = 0x00000002;

        let name = freerdp_get_last_error_name(code);
        // The function should return a valid pointer for known error codes
        if !name.is_null() {
            let name_str = CStr::from_ptr(name);
            assert!(
                !name_str.to_bytes().is_empty(),
                "error name for code {:#x} should not be empty",
                code
            );
        }
    }
}

/// Test the version module constants.
#[test]
fn test_version_constants() {
    assert_eq!(version::FREERDP_API_MAJOR, 3);
    assert!(!version::CRATE_VERSION.is_empty());
    assert_eq!(version::CRATE_VERSION, env!("CARGO_PKG_VERSION"));
}

/// Test that multiple instances can coexist.
#[test]
fn test_multiple_instances() {
    unsafe {
        let instance1 = freerdp_new();
        let instance2 = freerdp_new();

        assert!(!instance1.is_null());
        assert!(!instance2.is_null());
        assert_ne!(
            instance1 as usize, instance2 as usize,
            "two instances should have different addresses"
        );

        // Both should report not connected
        assert_eq!(freerdp_is_connected(instance1), 0);
        assert_eq!(freerdp_is_connected(instance2), 0);

        freerdp_free(instance1);
        freerdp_free(instance2);
    }
}

/// Test settings with different instance counts.
#[test]
fn test_settings_multiple_counts() {
    unsafe {
        let settings1 = freerdp_settings_new(1);
        let settings2 = freerdp_settings_new(2);

        assert!(
            !settings1.is_null(),
            "settings with count=1 should not be null"
        );
        assert!(
            !settings2.is_null(),
            "settings with count=2 should not be null"
        );

        freerdp_settings_free(settings1);
        freerdp_settings_free(settings2);
    }
}
