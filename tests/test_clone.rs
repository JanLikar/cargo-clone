use cargo::core::SourceId;
use cargo::util::Config;

use tempdir::TempDir;

#[test]
fn test_from_registry() {
    let temp_dir = TempDir::new("cargo-clone-tests").unwrap();
    let output_path = temp_dir.path().join("cargo-clone");

    assert!(!output_path.exists());

    let config = Config::default().unwrap();

    let krate = Some("cargo-clone");
    let source_id = SourceId::crates_io(&config).unwrap();
    let prefix = Some(output_path.to_str().unwrap());
    let vers = Some("0.2.0");

    cargo_clone::ops::clone(krate, &source_id, prefix, vers, &config).unwrap();

    assert!(output_path.exists());
    assert!(output_path.join("Cargo.toml").exists());
}
