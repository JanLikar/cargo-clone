## [1.2.4] - Unreleased
### Changed
  - Update dependencies
  - Update Cargo to 0.81.0 (cargo-clone-core breaking change)

## [1.2.3] - 2024-08-24
### Changed
  - Update dependencies
  - Specify workspace resolver to clear warnings (#133) by @eopb
  - Update time to fix build failures on rust 1.80.0 (#134) by @eopb

## [1.2.1] - 2023-12-13
### Changed
  - fix: log full error (#68) by @MarcoIeni
  - Show cli usage when no arguments are provided. (#93) by @RyanAvella
  - Update Cargo and other dependencies due to security concerns
  - Use matching versions for cargo-clone and cargo-clone-core

## [1.2.0] - 2023-01-23
### Changed
- Update to clap v4 (#63) by @MarcoIeni
- Improve cargo_clone_core API (#62) by @MarcoIeni
- Invalidate cache once (#58) @MarcoIeni
- Update cargo to v0.66 (#55)@MarcoIeni

## [1.1.0] - 2022-10-08
### Added
- Expose cargo's vendored-openssl feature by @dtolnay in https://github.com/JanLikar/cargo-clone/pull/49
- feat: extract library by @MarcoIeni in https://github.com/JanLikar/cargo-clone/pull/50
- Run clippy on tests by @MarcoIeni in https://github.com/JanLikar/cargo-clone/pull/51
- feat: re-export cargo types by @MarcoIeni in https://github.com/JanLikar/cargo-clone/pull/52

## [1.0.1] - 2022-04-26
### Changed
- Dependencies were updated.

## [1.0.0] - 2021-12-27
### Added
- Can now clone a package from a git repository specified in package's Cargo.toml file using `--git`.
- Test coverage was improved significantly.

### Changed
- clap.rs is used instead of Docopt.
- `--prefix` is now a positional argument named `directory`.
- `--alt-registry` is now `--registry`.
- `--registry-url` is now `--index`.
- `--vers` is removed in favor of inline version specs: `cargo-clone@1.0.0`.
- Several other minimal CLI changes.

### Removed
- Removed option to clone from git repo directly. This was deemed out-of-scope.
- Removed dependency on Serde.
- Removed `--path` as it is unneeded.

## [0.2.0] - 2021-12-25
- Fix clone_directory.
- Fix destination path creation.
- --vers is now parsed as a version requirement and uses precise matching by default.
- Dependencies updated

Thank you @jsha and @pravic!

## [0.1.4] - 2020-01-28
- Add flags for local and remote registries to clone from.
- Update dependencies.
- Remove dependency on rustc-serialize.
- Allow cloning multiple crates at once.
- Fix issue with parsing Cargo.toml.

  Thank you @dralley, @Phosphorus15, and @ErichDonGubler!

## [0.1.3] - 2018-10-22
- Update dependencies.
  Thank you, @dpc!

## [0.1.2] - 2017-04-28
### Added
- cargo-clone is now able to clone from git repositories and local directories.
  Thanks to @crazymerlyn!
