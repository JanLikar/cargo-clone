// Copyright 2015 Jan Likar.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Fetch the source code of a Rust crate from a registry.

#![warn(missing_docs)]

mod cloner_builder;
mod source;

pub use cloner_builder::*;
pub use source::*;

use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use anyhow::{bail, Context};

use cargo::core::dependency::Dependency;
use cargo::core::source::Source;
use cargo::core::Package;
use cargo::core::QueryKind;
use cargo::sources::{PathSource, SourceConfigMap};

use semver::VersionReq;

use walkdir::WalkDir;

// Re-export cargo types.
pub use cargo::{
    core::SourceId,
    util::{CargoResult, Config},
};

/// Rust crate.
#[derive(PartialEq, Eq, Debug)]
pub struct Crate {
    name: String,
    version: Option<String>,
}

impl Crate {
    /// Create a new [`Crate`].
    /// If `version` is not specified, the latest version is chosen.
    pub fn new(name: String, version: Option<String>) -> Crate {
        Crate { name, version }
    }
}

/// Clones a crate.
pub struct Cloner {
    /// Cargo configuration.
    pub(crate) config: Config,
    /// Directory where the crates will be cloned.
    /// Each crate is cloned into a subdirectory of this directory.
    pub(crate) directory: PathBuf,
    /// Where the crates will be cloned from.
    pub(crate) srcid: SourceId,
    /// If true, use `git` to clone the git repository present in the manifest metadata.
    pub(crate) use_git: bool,
}

impl Cloner {
    /// Creates a new [`ClonerBuilder`], that:
    /// - uses crates.io as source.
    /// - clones the crates into the current directory.
    pub fn builder() -> ClonerBuilder {
        ClonerBuilder::new()
    }

    /// Clone the specified crate from registry or git repository.
    pub fn clone(&self, crates: &[Crate]) -> CargoResult<()> {
        let _lock = self.config.acquire_package_cache_lock()?;

        // If prefix was not supplied, use current dir.

        let mut src = get_source(&self.srcid, &self.config)?;
        src.invalidate_cache();

        for crate_ in crates {
            let mut dest_path = self.directory.clone();

            dest_path.push(&crate_.name);

            if !dest_path.exists() {
                fs::create_dir_all(&dest_path)?;
            }

            self.config
                .shell()
                .verbose(|s| s.note(format!("Cloning into {:?}", &self.directory)))?;

            // Cloning into an existing directory is only allowed if the directory is empty.
            let is_empty = dest_path.read_dir()?.next().is_none();
            if !is_empty {
                bail!(
                    "destination path '{}' already exists and is not an empty directory.",
                    dest_path.display()
                );
            }

            self.clone_single(crate_, &dest_path, &self.config, &mut src)?;
        }

        Ok(())
    }

    fn clone_single<'a, T>(
        &self,
        crate_: &Crate,
        dest_path: &Path,
        config: &Config,
        src: &mut T,
    ) -> CargoResult<()>
    where
        T: Source + 'a,
    {
        let pkg = select_pkg(config, src, &crate_.name, crate_.version.as_deref())?;

        if self.use_git {
            let repo = &pkg.manifest().metadata().repository;

            if repo.is_none() {
                bail!(
                    "Cannot clone {} from git repo because it is not specified in package's manifest.",
                    &crate_.name
                )
            }

            clone_git_repo(repo.as_ref().unwrap(), dest_path)?;
        } else {
            clone_directory(pkg.root(), dest_path)?;
        }

        Ok(())
    }
}

fn get_source<'a>(srcid: &SourceId, config: &'a Config) -> CargoResult<Box<dyn Source + 'a>> {
    let source = if srcid.is_path() {
        let path = srcid.url().to_file_path().expect("path must be valid");
        Box::new(PathSource::new(&path, *srcid, config))
    } else {
        let map = SourceConfigMap::new(config)?;
        map.load(*srcid, &Default::default())?
    };
    Ok(source)
}

fn select_pkg<'a, T>(
    config: &Config,
    src: &mut T,
    name: &str,
    vers: Option<&str>,
) -> CargoResult<Package>
where
    T: Source + 'a,
{
    let dep = Dependency::parse(name, vers, src.source_id())?;
    let mut summaries = vec![];

    loop {
        match src.query(&dep, QueryKind::Exact, &mut |summary| {
            summaries.push(summary)
        })? {
            std::task::Poll::Ready(()) => break,
            std::task::Poll::Pending => src.block_until_ready()?,
        }
    }

    let latest = summaries.iter().max_by_key(|s| s.version());

    match latest {
        Some(l) => {
            config
                .shell()
                .note(format!("Downloading {} {}", name, l.version()))?;
            let pkg = Box::new(src).download_now(l.package_id(), config)?;
            Ok(pkg)
        }
        None => bail!("Package `{}@{}` not found", name, vers.unwrap_or("*.*.*")),
    }
}

fn parse_version_req(version: &str) -> CargoResult<String> {
    // This function's main purpose is to treat "x.y.z" as "=x.y.z"
    // so specifying the version in CLI works as expected.
    let first = version.chars().next();

    if first.is_none() {
        bail!("Version cannot be empty.")
    };

    let is_req = "<>=^~".contains(first.unwrap()) || version.contains('*');

    if is_req {
        let vers = VersionReq::parse(version)
            .with_context(|| format!("Invalid version requirement: `{}`.", version))?;
        Ok(vers.to_string())
    } else {
        Ok(format!("={}", version))
    }
}

// clone_directory copies the contents of one directory into another directory, which must
// already exist.
fn clone_directory(from: &Path, to: &Path) -> CargoResult<()> {
    if !to.is_dir() {
        bail!("Not a directory: {}", to.to_string_lossy());
    }
    for entry in WalkDir::new(from) {
        let entry = entry.unwrap();
        let file_type = entry.file_type();
        let mut dest_path = to.to_owned();
        dest_path.push(entry.path().strip_prefix(from).unwrap());

        if entry.file_name() == ".cargo-ok" {
            continue;
        }

        if file_type.is_file() {
            // .cargo-ok is not wanted in this context
            fs::copy(entry.path(), &dest_path)?;
        } else if file_type.is_dir() {
            if dest_path == to {
                continue;
            }
            fs::create_dir(&dest_path)?;
        }
    }

    Ok(())
}

fn clone_git_repo(repo: &str, to: &Path) -> CargoResult<()> {
    let status = Command::new("git")
        .arg("clone")
        .arg(repo)
        .arg(to.to_str().unwrap())
        .status()
        .context("Failed to clone from git repo.")?;

    if !status.success() {
        bail!("Failed to clone from git repo.")
    }

    Ok(())
}

/// Parses crate specifications like: crate, crate@x.y.z, crate@~23.4.5.
pub fn parse_name_and_version(spec: &str) -> CargoResult<Crate> {
    if !spec.contains('@') {
        return Ok(Crate::new(spec.to_owned(), None));
    }

    let mut parts = spec.split('@');
    let crate_ = parts
        .next()
        .context(format!("Crate name missing in `{}`.", spec))?;
    let version = parts
        .next()
        .context(format!("Crate version missing in `{}`.", spec))?;

    Ok(Crate::new(
        crate_.to_owned(),
        Some(parse_version_req(version)?),
    ))
}

#[cfg(test)]
mod tests {
    use std::{env, path::PathBuf};

    use super::*;
    use tempdir::TempDir;

    #[test]
    fn test_parse_version_req() {
        assert_eq!("=12.4.5", parse_version_req("12.4.5").unwrap());
        assert_eq!("=12.4.5", parse_version_req("=12.4.5").unwrap());
        assert_eq!("12.2.*", parse_version_req("12.2.*").unwrap());
    }

    #[test]
    fn test_parse_version_req_invalid_req() {
        assert_eq!(
            "Invalid version requirement: `=foo`.",
            parse_version_req("=foo").unwrap_err().to_string()
        );
    }

    #[test]
    fn test_clone_directory() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let from = PathBuf::from(manifest_dir).join("tests/data");
        let to = TempDir::new("cargo-clone-tests").unwrap();
        let to_path = to.path();

        clone_directory(&from, to_path).unwrap();

        assert!(to_path.join("Cargo.toml").exists());
        assert!(!to_path.join("cargo-ok").exists());
    }

    #[test]
    fn test_clone_repo() {
        let to = TempDir::new("cargo-clone-tests").unwrap();
        let to_path = to.path();

        clone_git_repo("https://github.com/janlikar/cargo-clone", to_path).unwrap();

        assert!(to_path.exists());
        assert!(to_path.join(".git").exists());
    }

    #[test]
    fn test_parse_name_and_version() {
        assert_eq!(
            parse_name_and_version("foo").unwrap(),
            Crate::new(String::from("foo"), None)
        );
        assert_eq!(
            parse_name_and_version("foo@1.1.3").unwrap(),
            Crate::new(String::from("foo"), Some(String::from("=1.1.3")))
        );
        assert_eq!(
            parse_name_and_version("foo@~1.1.3").unwrap(),
            Crate::new(String::from("foo"), Some(String::from("~1.1.3")))
        );
        assert_eq!(
            parse_name_and_version("foo@1.1.*").unwrap(),
            Crate::new(String::from("foo"), Some(String::from("1.1.*")))
        );
    }
}
