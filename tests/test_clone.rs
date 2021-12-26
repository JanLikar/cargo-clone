use cargo::core::SourceId;
use cargo::util::Config;

use tempdir::TempDir;

#[test]
fn test_from_registry() {
    let temp_dir = TempDir::new("cargo-clone-tests").unwrap();
    let output_path = temp_dir.path().join("cargo-clone");

    assert!(!output_path.exists());

    let config = Config::default().unwrap();

    let crates = vec![cargo_clone::Crate::new(
        String::from("cargo-clone"),
        Some(String::from("0.2.0")),
    )];
    let source_id = SourceId::crates_io(&config).unwrap();
    let git = false;
    let directory = Some(output_path.to_str().unwrap());

    let opts = cargo_clone::CloneOpts::new(&crates, &source_id, directory, git);

    cargo_clone::clone(&opts, &config).unwrap();

    assert!(output_path.exists());
    assert!(output_path.join("Cargo.toml").exists());
}

#[test]
fn test_dir_path() {
    // Test cargo clone CRATE DIR/ dumps into DIR/CRATE.
    let temp_dir = TempDir::new("cargo-clone-tests").unwrap();
    let output_path = temp_dir.path().join("foo");

    assert!(!output_path.exists());

    let config = Config::default().unwrap();

    let crates = vec![cargo_clone::Crate::new(
        String::from("cargo-clone"),
        Some(String::from("0.2.0")),
    )];
    let source_id = SourceId::crates_io(&config).unwrap();
    let git = false;
    let directory = Some(format!("{}/", output_path.to_str().unwrap()));

    let opts = cargo_clone::CloneOpts::new(&crates, &source_id, directory.as_deref(), git);

    cargo_clone::clone(&opts, &config).unwrap();

    assert!(output_path.join("cargo-clone").exists());
    assert!(output_path.join("cargo-clone").join("Cargo.toml").exists());
}

#[test]
fn test_multi_crates() {
    let temp_dir = TempDir::new("cargo-clone-tests").unwrap();
    let output_path = temp_dir.path().join("Test");

    assert!(!output_path.exists());

    let config = Config::default().unwrap();

    let crates = vec![
        cargo_clone::Crate::new(String::from("cargo-clone"), Some(String::from("0.2.0"))),
        cargo_clone::Crate::new(String::from("tokio"), None),
    ];
    let source_id = SourceId::crates_io(&config).unwrap();
    let git = false;
    let directory = Some(format!("{}/", output_path.to_str().unwrap()));

    let opts = cargo_clone::CloneOpts::new(&crates, &source_id, directory.as_deref(), git);

    cargo_clone::clone(&opts, &config).unwrap();

    assert!(output_path.join("cargo-clone").exists());
    assert!(output_path.join("cargo-clone").join("Cargo.toml").exists());
    assert!(output_path.join("tokio").exists());
    assert!(output_path.join("tokio").join("Cargo.toml").exists());
}
