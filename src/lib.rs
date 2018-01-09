// Copyright 2015 Jan Likar.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate cargo;
extern crate walkdir;

macro_rules! bail {
    ($($fmt:tt)*) => (
        return Err(human(&format_args!($($fmt)*)))
    )
}

pub mod ops {
    use std::path::{Path, PathBuf};
    use std::fs;
    use std::env;

    use cargo::util::{CargoResult, Config, human};
    use cargo::util::to_semver::ToSemver;
    use cargo::core::Package;
    use cargo::core::source::{Source, SourceId};
    use cargo::core::dependency::Dependency;
    use cargo::sources::{GitSource, PathSource, SourceConfigMap};

    use walkdir::WalkDir;

    pub fn clone(krate: Option<&str>,
                 srcid: &SourceId,
                 prefix: Option<&str>,
                 vers: Option<&str>,
                 config: &Config)
                 -> CargoResult<()> {
        let map = SourceConfigMap::new(config)?;
        let pkg = if srcid.is_path(){
            let path = srcid.url().to_file_path().ok().expect("path must be valid");
            let mut src = PathSource::new(&path, srcid, config);
            src.update()?;

            select_pkg(src, krate, vers, &mut |path| path.read_packages())?
        }
        else if srcid.is_git() {
            select_pkg(GitSource::new(srcid, config),
                       krate, vers, &mut |git| git.read_packages())?
        } else {
            select_pkg(map.load(srcid)?,
                       krate, vers,
                       &mut |_| Err(human("must specify a crate to clone from \
                                          crates.io, or use --path or --git to \
                                          specify alternate source")))?
        };



        // If prefix was not supplied, clone into current dir
        let dest_path = match prefix {
            Some(path) => PathBuf::from(path),
            None => {
                let mut dest = try!(env::current_dir());
                dest.push(pkg.name());
                dest
            }
        };

        // Cloning into an existing directory is only allowed if the directory is empty.
        if !dest_path.exists() {
            try!(fs::create_dir_all(&dest_path));
        } else {
            let is_empty = try!(dest_path.read_dir()).next().is_none();
            if !is_empty {
                bail!("destination path '{}' already exists and is not an empty directory.", dest_path.display());
            }
        }

        try!(clone_directory(&pkg.root(), &dest_path));

        Ok(())
    }

    fn select_pkg<'a, T>(mut src: T,
                          name: Option<&str>,
                          vers: Option<&str>,
                          list_all: &mut FnMut(&mut T) -> CargoResult<Vec<Package>>)
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
                let dep = try!(Dependency::parse_no_deprecated(
                    name, vers, src.source_id()));
                let summaries = try!(src.query(&dep));

                let latest = summaries.iter().max_by_key(|s| s.version());

                match latest {
                    Some(l) => {
                        let pkg = src.download(l.package_id())?;
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
                try!(fs::copy(&entry.path(), &to));
            }
            else if file_type.is_dir() {
                try!(fs::create_dir(&to));
            }
        }

        Ok(())
    }
}
