//! Compile-pass and compile-fail checks for the exported macros and their diagnostics.

use std::process::Command;

#[test]
fn ui() {
    let cases = trybuild::TestCases::new();
    cases.pass("tests/ui/pass/*.rs");
    cases.compile_fail("tests/ui/fail/*.rs");
}

/// A consumer that depends on `umbriel` under another name must still expand. It is a separate
/// crate because `trybuild` cases always see this crate's own `umbriel` dependency.
#[test]
fn renamed_dependency() {
    let root = env!("CARGO_MANIFEST_DIR");
    let status = Command::new(env!("CARGO"))
        .args(["check", "--offline", "--quiet", "--manifest-path"])
        .arg(format!("{root}/fixtures/renamed-dependency/Cargo.toml"))
        .arg("--target-dir")
        .arg(format!("{root}/../../target/fixtures"))
        .status()
        .expect("failed to run cargo");
    assert!(
        status.success(),
        "renamed-dependency fixture failed to compile"
    );
}
