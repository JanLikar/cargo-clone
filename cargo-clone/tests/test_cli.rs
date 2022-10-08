use std::process::Command;
use std::{env, fs};

use tempdir::TempDir;

fn cargo_clone_cmd() -> String {
    let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "../target".to_string());
    format!("{target_dir}/debug/cargo-clone")
}

#[test]
fn test_cli() {
    let temp_dir = TempDir::new("cargo-clone-tests").unwrap();
    let output_path = temp_dir.path().join("cargo-clone");

    assert!(!output_path.exists());

    let status = Command::new(cargo_clone_cmd())
        .arg("clone")
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

    let status = Command::new(cargo_clone_cmd())
        .arg("clone")
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

    let status = Command::new(cargo_clone_cmd())
        .arg("clone")
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

    let status = Command::new(cargo_clone_cmd())
        .arg("clone")
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
