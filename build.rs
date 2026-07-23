//! Build script for freerdp-sys.
//!
//! Supports two link modes:
//! - `vendored` (default): Compiles FreeRDP 3.x from `vendor/FreeRDP` via CMake.
//! - `system`: Links against a system-installed FreeRDP discovered via pkg-config.
//!
//! Bindings generation:
//! - With `generate-bindings` feature: runs bindgen against FreeRDP headers at build time.
//! - Without: uses the pre-generated `src/bindings.rs`.

// Functions are conditionally called based on runtime feature detection via env vars.
#![allow(dead_code)]

use std::env;
use std::path::{Path, PathBuf};

/// Check if a cargo feature is enabled (build scripts use env vars, not cfg!).
fn has_feature(name: &str) -> bool {
    env::var(format!(
        "CARGO_FEATURE_{}",
        name.to_uppercase().replace('-', "_")
    ))
    .is_ok()
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/wrapper.h");

    let vendored = has_feature("vendored");
    let system = has_feature("system");
    let gen_bindings = has_feature("generate-bindings");

    let freerdp_include = if vendored {
        build_vendored()
    } else if system {
        find_system()
    } else {
        // No link mode selected — allow `cargo check` with pre-generated bindings
        // but emit a warning.
        println!("cargo:warning=freerdp-sys: no link mode selected (vendored/system). Using pre-generated bindings only.");
        PathBuf::from("/usr/include/freerdp3") // placeholder; not used without gen_bindings
    };

    // Generate or use pre-generated bindings
    if gen_bindings {
        generate_bindings(&freerdp_include);
    } else {
        // Use pre-generated bindings; just verify the file exists
        let bindings_path = Path::new("src/bindings.rs");
        if !bindings_path.exists() {
            panic!(
                "freerdp-sys: pre-generated bindings not found at src/bindings.rs.\n\
                 Enable the `generate-bindings` feature to generate them at build time \
                 (requires libclang)."
            );
        }
    }
}

/// Build FreeRDP from vendored source using the `cmake` crate.
/// Returns the include directory path for the built library.
fn build_vendored() -> PathBuf {
    let vendor_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("vendor/FreeRDP");

    if !vendor_dir.join("CMakeLists.txt").exists() {
        panic!(
            "freerdp-sys: vendored FreeRDP source not found at `vendor/FreeRDP`.\n\
             Run: git submodule update --init --recursive\n\
             Or clone FreeRDP 3.x into vendor/FreeRDP manually."
        );
    }

    println!("cargo:rerun-if-changed=vendor/FreeRDP/CMakeLists.txt");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    // Configure the CMake build with sensible defaults for a static library
    let mut cfg = cmake::Config::new(&vendor_dir);

    cfg.define("BUILD_SHARED_LIBS", "OFF")
        .define("WITH_SERVER", "OFF")
        .define("WITH_CLIENT", "ON")
        .define("WITH_CHANNELS", "ON")
        .define("WITH_WINPR_TOOLS", "OFF")
        .define("WITH_MANPAGES", "OFF")
        .define("BUILD_TESTING", "OFF")
        .define("BUILD_TESTING_INTERNAL", "OFF")
        .define("WITH_SAMPLE", "OFF")
        .define("WITH_PROXY", "OFF")
        .define("WITH_SHADOW", "OFF")
        .define("WITH_PLATFORM_SERVER", "OFF")
        .define("CMAKE_BUILD_TYPE", "Release")
        // Disable all optional display/audio backends
        .define("WITH_X11", "OFF")
        .define("WITH_WAYLAND", "OFF")
        .define("WITH_ALSA", "OFF")
        .define("WITH_PULSE", "OFF")
        .define("WITH_OSS", "OFF")
        .define("WITH_SNDIO", "OFF")
        .define("WITH_OPUS", "OFF")
        .define("WITH_FAAC", "OFF")
        .define("WITH_FAAD2", "OFF")
        .define("WITH_LAME", "OFF")
        .define("WITH_SOXR", "OFF")
        .define("WITH_GSM", "OFF")
        // Disable optional codec/media support
        .define("WITH_FFMPEG", "OFF")
        .define("WITH_DSP_FFMPEG", "OFF")
        .define("WITH_OPENH264", "OFF")
        .define("WITH_MEDIA_FOUNDATION", "OFF")
        .define("WITH_SWSCALE", "OFF")
        .define("WITH_CAIRO", "OFF")
        .define("WITH_JPEG", "OFF")
        .define("WITH_WEBP", "OFF")
        .define("WITH_PNG", "OFF")
        // Disable optional hardware/smartcard
        .define("WITH_PCSC", "OFF")
        .define("WITH_PKCS11", "OFF")
        .define("WITH_SMARTCARD_EMULATE", "OFF")
        .define("WITH_FUSE", "OFF")
        .define("WITH_AAD", "OFF")
        .define("WITH_CJSON", "OFF")
        .define("WITH_JSON", "OFF")
        .define("WITH_INTREE_WAYLAND", "OFF")
        .define("WITH_LIBSYSTEMD", "OFF")
        .define("WITH_KRB5", "OFF")
        .define("WITH_KRB5_NO_VENDOR", "OFF")
        .define("WITH_SDL", "OFF")
        .define("WITH_UI", "OFF")
        // Disable optional SIMD
        .define("WITH_SSE2", "OFF")
        .define("WITH_NEON", "OFF");

    // Allow user overrides via environment
    if let Ok(extra_defs) = env::var("FREERDP_CMAKE_DEFS") {
        for def in extra_defs.split(';') {
            if let Some((key, value)) = def.split_once('=') {
                cfg.define(key.trim(), value.trim());
            }
        }
    }

    let dst = cfg.build();

    // Emit link directives for the static libraries
    let lib_dir = dst.join("lib");
    let lib_dir_alt = dst.join("lib64");

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    if lib_dir_alt.exists() {
        println!("cargo:rustc-link-search=native={}", lib_dir_alt.display());
    }

    // FreeRDP 3.x produces these core static libraries
    println!("cargo:rustc-link-lib=static=freerdp3");
    println!("cargo:rustc-link-lib=static=freerdp-client3");
    println!("cargo:rustc-link-lib=static=winpr3");
    println!("cargo:rustc-link-lib=static=winpr-tools3");

    // Platform system libraries that FreeRDP depends on
    emit_system_libs(&target_os);

    // Return include path for bindgen
    dst.join("include").join("freerdp3")
}

/// Find system-installed FreeRDP via pkg-config.
/// Returns the include directory path.
fn find_system() -> PathBuf {
    let freerdp = pkg_config::Config::new()
        .atleast_version("3.0")
        .probe("freerdp3")
        .or_else(|_| {
            // Fallback: some distros name it differently
            pkg_config::Config::new()
                .atleast_version("3.0")
                .probe("freerdp")
        })
        .expect(
            "freerdp-sys: could not find FreeRDP 3.x via pkg-config.\n\
             Install FreeRDP development packages or use the `vendored` feature.",
        );

    // Also probe winpr (FreeRDP's platform abstraction)
    let _ = pkg_config::Config::new()
        .atleast_version("3.0")
        .probe("winpr3")
        .or_else(|_| {
            pkg_config::Config::new()
                .atleast_version("3.0")
                .probe("winpr")
        });

    freerdp
        .include_paths
        .first()
        .cloned()
        .expect("freerdp-sys: pkg-config returned no include paths for FreeRDP")
}

/// Run bindgen to produce Rust FFI bindings from FreeRDP headers.
fn generate_bindings(include_path: &Path) {
    let wrapper = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("src/wrapper.h");

    println!("cargo:rerun-if-changed={}", wrapper.display());

    let mut builder = bindgen::Builder::default()
        .header(wrapper.to_string_lossy())
        .clang_arg(format!("-I{}", include_path.display()))
        // Also add parent include for winpr headers
        .clang_arg(format!(
            "-I{}",
            include_path
                .parent()
                .map(|p| p.display().to_string())
                .unwrap_or_default()
        ))
        // Allowlist FreeRDP and WinPR symbols
        .allowlist_function("freerdp_.*")
        .allowlist_function("rdp_.*")
        .allowlist_function("winpr_.*")
        .allowlist_function("WLog_.*")
        .allowlist_type("rdp.*")
        .allowlist_type("freerdp.*")
        .allowlist_type("FREERDP.*")
        .allowlist_type("RDP.*")
        .allowlist_type("winpr.*")
        .allowlist_type("WINPR.*")
        .allowlist_var("FREERDP.*")
        .allowlist_var("RDP.*")
        .allowlist_var("WINPR.*")
        // Layout and derive options
        .layout_tests(true)
        .derive_debug(true)
        .derive_default(true)
        .derive_eq(true)
        .derive_hash(true)
        // Use core types where possible
        .use_core()
        .ctypes_prefix("::std::os::raw")
        // Size_t handling
        .size_t_is_usize(true)
        // Generate for both 32/64 bit
        .generate_comments(true)
        .prepend_enum_name(false)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        });

    // Extra clang args from environment
    if let Ok(extra_args) = env::var("FREERDP_BINDGEN_CLANG_ARGS") {
        for arg in extra_args.split_whitespace() {
            builder = builder.clang_arg(arg);
        }
    }

    let bindings = builder
        .generate()
        .expect("freerdp-sys: bindgen failed to generate bindings");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("freerdp-sys: failed to write generated bindings");
}

/// Emit platform-specific system library links required by FreeRDP.
fn emit_system_libs(target_os: &str) {
    match target_os {
        "linux" => {
            println!("cargo:rustc-link-lib=dylib=ssl");
            println!("cargo:rustc-link-lib=dylib=crypto");
            println!("cargo:rustc-link-lib=dylib=z");
            println!("cargo:rustc-link-lib=dylib=pthread");
            println!("cargo:rustc-link-lib=dylib=dl");
            println!("cargo:rustc-link-lib=dylib=m");
        }
        "macos" => {
            println!("cargo:rustc-link-lib=framework=CoreFoundation");
            println!("cargo:rustc-link-lib=framework=CoreGraphics");
            println!("cargo:rustc-link-lib=framework=CoreAudio");
            println!("cargo:rustc-link-lib=framework=AudioToolbox");
            println!("cargo:rustc-link-lib=dylib=z");
            println!("cargo:rustc-link-lib=dylib=iconv");
        }
        "windows" => {
            println!("cargo:rustc-link-lib=dylib=ws2_32");
            println!("cargo:rustc-link-lib=dylib=advapi32");
            println!("cargo:rustc-link-lib=dylib=crypt32");
            println!("cargo:rustc-link-lib=dylib=user32");
            println!("cargo:rustc-link-lib=dylib=gdi32");
        }
        _ => {}
    }
}
