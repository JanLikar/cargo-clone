// Copyright 2015 Jan Likar.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub mod ops {
    use std::env;
    use std::fs;
    use std::path::{Path, PathBuf};

    use cargo::core::dependency::Dependency;
    use cargo::core::source::{Source, SourceId};
    use cargo::core::Package;
    use cargo::sources::{GitSource, PathSource, SourceConfigMap};
    use cargo::util::to_semver::ToSemver;
    use cargo::util::{CargoResult, Config};

    use failure::bail;

    use walkdir::WalkDir;

    pub fn clone(
        krate: Option<&str>,
        srcid: &SourceId,
        prefix: Option<&str>,
        vers: Option<&str>,
        config: &Config,
    ) -> CargoResult<()> {
        let _lock = config.acquire_package_cache_lock()?;

        let map = SourceConfigMap::new(config)?;
        let pkg = if srcid.is_path() {
            let path = srcid.url().to_file_path().expect("path must be valid");
            let mut src = PathSource::new(&path, *srcid, config);
            src.update()?;

            select_pkg(config, src, krate, vers, &mut |path| path.read_packages())?
        } else if srcid.is_git() {
            select_pkg(
                config,
                GitSource::new(*srcid, config)?,
                krate,
                vers,
                &mut |git| git.read_packages(),
            )?
        } else {
            select_pkg(
                config,
                map.load(*srcid, &Default::default())?,
                krate,
                vers,
                &mut |_| {
                    bail!(
                        "must specify a crate to clone from \
                         crates.io, or use --path or --git to \
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

        // Cloning into an existing directory is only allowed if the directory is empty.
        if !dest_path.exists() {
            fs::create_dir_all(&dest_path)?;
        } else {
            let is_empty = dest_path.read_dir()?.next().is_none();
            if !is_empty {
                bail!(
                    "destination path '{}' already exists and is not an empty directory.",
                    dest_path.display()
                );
            }
        }

        clone_directory(&pkg.root(), &dest_path)?;

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
                    Some(v) => match v.to_semver() {
                        Ok(v) => Some(v.to_string()),
                        Err(e) => bail!("{}", e),
                    },
                    None => None,
                };
                let vers = vers.as_deref();
                let dep = Dependency::parse_no_deprecated(name, vers, src.source_id())?;
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

    // clone_directory copies the contents of one directory into another directory, which must
    // already exist.
    fn clone_directory(from: &Path, to: &Path) -> CargoResult<()> {
        if !to.is_dir() {
            bail!("not a directory: {}", to.to_string_lossy());
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
                    continue
                }
                fs::create_dir(&dest_path)?;
            }
        }

        Ok(())
    }
}
