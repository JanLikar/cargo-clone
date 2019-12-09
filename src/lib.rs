// Copyright 2015 Jan Likar.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate failure;



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
            let path = srcid.url().to_file_path().ok().expect("path must be valid");
            let mut src = PathSource::new(&path, srcid.clone(), config);
            src.update()?;

            select_pkg(config, src, krate, vers, &mut |path| path.read_packages())?
        } else if srcid.is_git() {
            select_pkg(
                config,
                GitSource::new(srcid.clone(), config)?,
                krate,
                vers,
                &mut |git| git.read_packages(),
            )?
        } else {
            select_pkg(
                config,
                map.load(srcid.clone(), &Default::default())?,
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
        let mut dest_path = match prefix {
            Some(path) => PathBuf::from(path),
            None => r#try!(env::current_dir()),
        };

        dest_path.push(format!("{}", pkg.name()));

        r#try!(clone_directory(&pkg.root(), &dest_path));

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
                let vers = vers.as_ref().map(|s| &**s);
                let dep = r#try!(Dependency::parse_no_deprecated(name, vers, src.source_id()));
                let mut summaries = vec![];
                r#try!(src.query(&dep, &mut |summary| summaries.push(summary.clone())));

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

    fn clone_directory(from: &Path, to: &Path) -> CargoResult<()> {
        for entry in WalkDir::new(from) {
            let entry = entry.unwrap();
            let file_type = entry.file_type();
            let mut to = to.to_owned();
            to.push(entry.path().strip_prefix(from).unwrap());

            if file_type.is_file() && entry.file_name() != ".cargo-ok" {
                // .cargo-ok is not wanted in this context
                r#try!(fs::copy(&entry.path(), &to));
            } else if file_type.is_dir() {
                r#try!(fs::create_dir(&to));
            }
        }

        Ok(())
    }
}
