use cargo_clone_core::ClonerBuilder;
use tempdir::TempDir;

#[test]
fn test_from_registry() {
    let temp_dir = TempDir::new("cargo-clone-tests").unwrap();
    let output_path = temp_dir.path().join("cargo-clone");

    assert!(!output_path.exists());

    let crates = vec![cargo_clone_core::Crate::new(
        String::from("cargo-clone"),
        Some(String::from("0.2.0")),
    )];
    let directory = output_path.to_str().unwrap();

    let cloner = ClonerBuilder::new()
        .with_directory(directory)
        .build()
        .unwrap();

    cloner.clone(&crates).unwrap();

    assert!(output_path.exists());
    assert!(output_path.join("Cargo.toml").exists());
}

#[test]
fn test_dir_path() {
    // Test cargo clone CRATE DIR/ dumps into DIR/CRATE.
    let temp_dir = TempDir::new("cargo-clone-tests").unwrap();
    let output_path = temp_dir.path().join("foo");

    assert!(!output_path.exists());

    let crates = vec![cargo_clone_core::Crate::new(
        String::from("cargo-clone"),
        Some(String::from("0.2.0")),
    )];
    let directory = format!("{}/", output_path.to_str().unwrap());

    let cloner = ClonerBuilder::new()
        .with_directory(directory)
        .build()
        .unwrap();

    cloner.clone(&crates).unwrap();

    assert!(output_path.join("cargo-clone").exists());
    assert!(output_path.join("cargo-clone").join("Cargo.toml").exists());
}

#[test]
fn test_multi_crates() {
    let temp_dir = TempDir::new("cargo-clone-tests").unwrap();
    let output_path = temp_dir.path().join("Test");

    assert!(!output_path.exists());

    let crates = vec![
        cargo_clone_core::Crate::new(String::from("cargo-clone"), Some(String::from("0.2.0"))),
        cargo_clone_core::Crate::new(String::from("tokio"), None),
    ];
    let directory = format!("{}/", output_path.to_str().unwrap());

    let cloner = ClonerBuilder::new()
        .with_directory(directory)
        .build()
        .unwrap();
    cloner.clone(&crates).unwrap();

    assert!(output_path.join("cargo-clone").exists());
    assert!(output_path.join("cargo-clone").join("Cargo.toml").exists());
    assert!(output_path.join("tokio").exists());
    assert!(output_path.join("tokio").join("Cargo.toml").exists());
}
