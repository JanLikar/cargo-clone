// Copyright 2015 Jan Likar.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{bail, Context};

use cargo::core::dependency::Dependency;
use cargo::core::source::{Source, SourceId};
use cargo::core::Package;
use cargo::sources::{PathSource, SourceConfigMap};
use cargo::util::{CargoResult, Config};

use semver::VersionReq;

use walkdir::WalkDir;

pub fn clone(
    krate: Option<&str>,
    srcid: &SourceId,
    prefix: Option<&str>,
    git: bool,
    vers: Option<&str>,
    config: &Config,
) -> CargoResult<()> {
    let _lock = config.acquire_package_cache_lock()?;

    let map = SourceConfigMap::new(config)?;
    let pkg = if srcid.is_path() {
        let path = srcid.url().to_file_path().expect("path must be valid");
        let src = PathSource::new(&path, *srcid, config);

        select_pkg(config, src, krate, vers, &mut |path| path.read_packages())?
    } else {
        select_pkg(
            config,
            map.load(*srcid, &Default::default())?,
            krate,
            vers,
            &mut |_| {
                bail!(
                    "must specify a crate to clone from \
                        crates.io, or use --path to \
                        specify alternate source"
                )
            },
        )?
    };

    // If prefix was not supplied, clone into current dir
    let dest_path = match prefix {
        Some(path) => PathBuf::from(path),
        None => {
            let mut dest = env::current_dir()?;
            dest.push(format!("{}", pkg.name()));
            dest
        }
    };

    if !dest_path.exists() {
        fs::create_dir_all(&dest_path)?;
    }

    // Cloning into an existing directory is only allowed if the directory is empty.
    let is_empty = dest_path.read_dir()?.next().is_none();
    if !is_empty {
        bail!(
            "destination path '{}' already exists and is not an empty directory.",
            dest_path.display()
        );
    }

    if git {
        let repo = &pkg.manifest().metadata().repository;

        if repo.is_none() {
            bail!("Cannot clone from git repo because it is not specified in package's manifest.")
        }

        clone_git_repo(repo.as_ref().unwrap(), &dest_path)?;
    } else {
        clone_directory(pkg.root(), &dest_path)?;
    }

    Ok(())
}

fn select_pkg<'a, T>(
    config: &Config,
    mut src: T,
    name: Option<&str>,
    vers: Option<&str>,
    list_all: &mut dyn FnMut(&mut T) -> CargoResult<Vec<Package>>,
) -> CargoResult<Package>
where
    T: Source + 'a,
{
    src.update()?;

    match name {
        Some(name) => {
            let vers = match vers {
                Some(v) => Some(parse_version_req(v)?),
                None => None,
            };

            let dep = Dependency::parse(name, vers.as_deref(), src.source_id())?;
            let mut summaries = vec![];
            src.query(&dep, &mut |summary| summaries.push(summary))?;

            let latest = summaries.iter().max_by_key(|s| s.version());

            match latest {
                Some(l) => {
                    let pkg = Box::new(src).download_now(l.package_id(), config)?;
                    Ok(pkg)
                }
                None => bail!("package '{}' not found", name),
            }
        }
        None => {
            let candidates = list_all(&mut src)?;
            Ok(candidates[0].clone())
        }
    }
}

fn parse_version_req(version: &str) -> CargoResult<String> {
    // This function's main purpose is to treat "x.y.z" as "=x.y.z"
    // so specifying the version in CLI works as expected.
    let first = version.chars().next();

    if first.is_none() {
        bail!("No version provided for the `--vers` flag")
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

        if file_type.is_file() && entry.file_name() != ".cargo-ok" {
            // .cargo-ok is not wanted in this context
            fs::copy(&entry.path(), &dest_path)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
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
        let from = Path::new("tests/data");
        let to = TempDir::new("cargo-clone-tests").unwrap();
        let to_path = to.path();

        clone_directory(&from, &to_path).unwrap();

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
}
