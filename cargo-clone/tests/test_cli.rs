use::assert_cmd::prelude::*;
use std::process::Command;
use std::fs;
use tempdir::TempDir;

fn cargo_clone_cmd() -> Command {
    Command::cargo_bin("cargo-clone").expect("Unable to get the cargo-clone command.")
}

#[test]
fn test_cli() {
    let temp_dir = TempDir::new("cargo-clone-tests").unwrap();
    let output_path = temp_dir.path().join("cargo-clone");

    assert!(!output_path.exists());

    let status = cargo_clone_cmd()
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
fn test_cli_no_directory() {
    let temp_dir = TempDir::new("cargo-clone-tests").unwrap();
    let output_path = temp_dir.path().join("cargo-clone");

    assert!(!output_path.exists());

    //assert!(temp_dir.path().exists());
    let status = cargo_clone_cmd()
        .current_dir(temp_dir.path())
        .arg("clone")
        .arg("cargo-clone")
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

    let status = cargo_clone_cmd()
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

    let status = cargo_clone_cmd()
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

    let status = cargo_clone_cmd()
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
