use std::process::Command;

use tempdir::TempDir;

#[test]
fn test_cli() {
    let temp_dir = TempDir::new("cargo-clone-tests").unwrap();
    let output_path = temp_dir.path().join("cargo-clone");

    assert!(!output_path.exists());

    let status = Command::new("target/debug/cargo-clone")
        .arg("cargo-clone")
        .arg(output_path.to_str().unwrap())
        .status()
        .unwrap();

    assert!(status.success());
    assert!(output_path.exists());
    assert!(output_path.join("Cargo.toml").exists());
}

#[test]
fn test_custon_index() {
    let temp_dir = TempDir::new("cargo-clone-tests").unwrap();
    let output_path = temp_dir.path().join("cargo-clone");

    assert!(!output_path.exists());

    let status = Command::new("target/debug/cargo-clone")
        .arg("--index")
        .arg("https://github.com/rust-lang/crates.io-index")
        .arg("cargo-clone")
        .arg(output_path.to_str().unwrap())
        .status()
        .unwrap();

    assert!(status.success());
    assert!(output_path.exists());
    assert!(output_path.join("Cargo.toml").exists());
}
