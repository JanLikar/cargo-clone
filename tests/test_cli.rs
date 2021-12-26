use std::fs;
use std::process::Command;

use tempdir::TempDir;

#[test]
fn test_cli() {
    let temp_dir = TempDir::new("cargo-clone-tests").unwrap();
    let output_path = temp_dir.path().join("cargo-clone");

    assert!(!output_path.exists());

    let status = Command::new("target/debug/cargo-clone")
        .arg("cargo-clone")
        .arg("--")
        .arg(output_path.to_str().unwrap())
        .status()
        .unwrap();

    assert!(status.success());
    assert!(output_path.exists());
    assert!(output_path.join("Cargo.toml").exists());
}

#[test]
fn test_custom_index() {
    let temp_dir = TempDir::new("cargo-clone-tests").unwrap();
    let output_path = temp_dir.path().join("cargo-clone");

    assert!(!output_path.exists());

    let status = Command::new("target/debug/cargo-clone")
        .arg("--index")
        .arg("https://github.com/rust-lang/crates.io-index")
        .arg("cargo-clone")
        .arg("--")
        .arg(output_path.to_str().unwrap())
        .status()
        .unwrap();

    assert!(status.success());
    assert!(output_path.exists());
    assert!(output_path.join("Cargo.toml").exists());
}

#[test]
fn test_clone_into_existing() {
    let temp_dir = TempDir::new("cargo-clone-tests").unwrap();
    let output_path = temp_dir.path();

    let status = Command::new("target/debug/cargo-clone")
        .arg("time")
        .arg("--")
        .arg(output_path.to_str().unwrap())
        .status()
        .unwrap();

    assert!(status.success());
    assert!(output_path.exists());
    assert!(output_path.join("Cargo.toml").exists());
}

#[test]

fn test_with_version() {
    let temp_dir = TempDir::new("cargo-clone-tests").unwrap();
    let output_path = temp_dir.path().join("cargo-clone");

    assert!(!output_path.exists());

    let status = Command::new("target/debug/cargo-clone")
        .arg("tokei@6.1.2")
        .arg("--")
        .arg(output_path.to_str().unwrap())
        .status()
        .unwrap();

    assert!(status.success());
    assert!(output_path.exists());
    assert!(output_path.join("Cargo.toml").exists());

    fs::read_to_string(output_path.join("Cargo.toml"))
        .unwrap()
        .contains("version = \"6.1.2\"");
}
