// Copyright 2015 Jan Likar.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate failure;
extern crate cargo;
extern crate walkdir;

pub mod ops {
    use std::path::{Path, PathBuf};
    use std::fs;
    use std::env;

    use cargo::util::{CargoResult, Config};
    use cargo::util::to_semver::ToSemver;
    use cargo::core::Package;
    use cargo::core::source::{Source, SourceId, MaybePackage};
    use cargo::core::dependency::Dependency;
    use cargo::sources::{GitSource, PathSource, SourceConfigMap};

    use walkdir::WalkDir;
    use std::collections::HashSet;

    pub fn clone(krate: Option<&str>,
                 srcid: &SourceId,
                 prefix: Option<&str>,
                 vers: Option<&str>,
                 config: &Config)
                 -> CargoResult<()> {
        let map = SourceConfigMap::new(config)?;
        let lock = config.acquire_package_cache_lock().unwrap_or_else(|v| {panic!();});
        let pkg = if srcid.is_path(){
            let path = srcid.url().to_file_path().ok().expect("path must be valid");
            let mut src = PathSource::new(&path, srcid.clone(), config);
            src.update()?;
            select_pkg(src, krate, vers, &mut |path| path.read_packages(), config)?
        } else if srcid.is_git() {
            select_pkg(GitSource::new(srcid.clone(), config)?,
                       krate, vers, &mut |git| git.read_packages(), config)?
        } else {
            select_pkg(map.load(srcid.clone(), &HashSet::new())?,
                       krate, vers,
                       &mut |_| bail!("must specify a crate to clone from \
                                          crates.io, or use --path or --git to \
                                          specify alternate source"), config)?
        };
        config.release_package_cache_lock();



        // If prefix was not supplied, clone into current dir
        let mut dest_path = match prefix {
            Some(path) => PathBuf::from(path),
            None => env::current_dir()?
        };

        dest_path.push(format!("{}", pkg.name()));


        clone_directory(&pkg.root(), &dest_path)?;

        Ok(())
    }

    fn select_pkg<'a, T>(mut src: T,
                          name: Option<&str>,
                          vers: Option<&str>,
                          list_all: &mut FnMut(&mut T) -> CargoResult<Vec<Package>>,
                        config: &Config)
                          -> CargoResult<Package>
        where T: Source + 'a
    {

        src.update()?;

        match name {
            Some(name) => {
                let vers = match vers {
                    Some(v) => {
                        match v.to_semver() {
                            Ok(v) => Some(v.to_string()),
                            Err(e) => bail!("{}", e),
                        }
                    },
                    None => None
                };
                let vers = vers.as_ref().map(|s| &**s);
                let dep = Dependency::parse_no_deprecated(
                    name, vers, src.source_id())?;
                let mut summaries = vec![];
                src.query(&dep, &mut |summary| summaries.push(summary.clone()))?;

                let latest = summaries.iter().max_by_key(|s| s.version());

                match latest {
                    Some(l) => {
                        let pkg = src.download(l.package_id())?;
                        match pkg {
                            MaybePackage::Ready(pkg) => Ok(pkg),
                            MaybePackage::Download {
                                url: url, descriptor: desc
                            } => Box::new(src).download_now(l.package_id(), config),
                        }
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

    fn clone_directory(from: &Path, to: &Path) -> CargoResult<()> {
        for entry in WalkDir::new(from) {
            let entry = entry.unwrap();
            let file_type = entry.file_type();
            let mut to = to.to_owned();
            to.push(entry.path().strip_prefix(from).unwrap());

            if file_type.is_file() && entry.file_name() != ".cargo-ok" {
                // .cargo-ok is not wanted in this context
                fs::copy(&entry.path(), &to)?;
            }
            else if file_type.is_dir() {
                fs::create_dir(&to)?;
            }
        }

        Ok(())
    }
}
