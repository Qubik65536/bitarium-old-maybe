//! Build the sibling C3 static library and link it into the Rust runner.
//!
//! Cargo runs this script before compiling the runner. C3 must be on PATH.
//! Generated archives, headers, and intermediate objects stay inside OUT_DIR.

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

/// Read a Cargo-provided filesystem path without requiring UTF-8 encoding.
fn cargo_path(name: &str) -> PathBuf {
    PathBuf::from(env::var_os(name).unwrap_or_else(|| panic!("Cargo must provide {name}")))
}

/// Ensure that the build is not cross-compiling. The C3 library does not yet support
/// cross-compilation, so the `HOST` and `TARGET` must be the same.
fn require_native_build() {
    let host = env::var("HOST").expect("HOST must be provided in the environment.");
    let target = env::var("TARGET").expect("TARGET must be provided in the environment.");
    assert_eq!(
        host, target,
        "Cross-compilation is not configured for the C3 library."
    );
}

/// Configure Cargo to rerun this build script if any of the C3 source files or project
/// configuration change, or if the PATH environment variable changes. This ensures that
/// Cargo rebuilds the C3 library when necessary.
fn track_build_inputs(memory_dir: &Path) {
    println!(
        "cargo::rerun-if-changed={}",
        memory_dir.join("src").display()
    );
    println!(
        "cargo::rerun-if-changed={}",
        memory_dir.join("project.json").display()
    );
    println!("cargo::rerun-if-env-changed=PATH");
}

/// Build the C3 static library. Direct the output of the built library
/// to the specified `output_dir`. The C3 compiler is invoked in the `memory_dir`
/// which should contain the C3 source code and project configuration.
fn build_c3_library(project_dir: &Path, output_dir: &Path) {
    // Use c3c compiler to build the C3 static library.
    let status = Command::new("c3c")
        .arg("build")
        // Set the output directory for generated files, including the static library.
        .arg("--output-dir")
        .arg(output_dir)
        .arg("--build-dir")
        .arg(output_dir.join("c3-build"))
        // Run the C3 compiler in the memory project directory,
        // which contains the C3 source and project.json.
        .current_dir(project_dir)
        // Run the command and check for success.
        .status()
        // If the command fails to start, provide a clear error message.
        .expect("Failed to run c3c; ensure the C3 compiler is on PATH");
    // If the command runs but returns a non-zero exit code, panic with a message.
    assert!(status.success(), "C3 static-library build failed");
}

/// C3 emits memory.a, while Rust's static library lookup expects libmemory.a.
/// Keep C3's output and make a linker-compatible copy beside it.
fn prepare_archive(output_dir: &Path) {
    fs::copy(output_dir.join("memory.a"), output_dir.join("libmemory.a"))
        .expect("Failed to prepare the C3 archive for Rust");
}

/// Add the archive directory to Rust's native search path and link it statically.
/// These Cargo directives configure linking; they do not declare Rust FFI types.
fn link_library(output_dir: &Path) {
    println!("cargo::rustc-link-search=native={}", output_dir.display());
    println!("cargo::rustc-link-lib=static=memory");
}

fn main() {
    // For now cross-compiling the C3 library is not supported.
    // The HOST and TARGET must therefore the same.
    require_native_build();

    // Get respective Cargo-provided paths.
    let memory_dir = cargo_path("CARGO_MANIFEST_DIR").join("../memory");
    let output_dir = cargo_path("OUT_DIR");

    // Build and link the C3 library.
    track_build_inputs(&memory_dir);
    build_c3_library(&memory_dir, &output_dir);
    prepare_archive(&output_dir);
    link_library(&output_dir);
}
