// Copyright 2015 Jan Likar.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate cargo;
extern crate walkdir;
extern crate itertools;

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
    use cargo::core::package_id::PackageId;
    use cargo::core::source::{Source, SourceId};
    use cargo::core::registry::Registry;
    use cargo::core::dependency::Dependency;
    use cargo::sources::RegistrySource;

    use walkdir::WalkDir;
    use itertools::Itertools;
    use itertools::EitherOrBoth::{Both, Left, Right};

    pub fn clone(krate: &Option<String>,
                 srcid: &SourceId,
                 flag_version: Option<String>,
                 config: Config)
                 -> CargoResult<()> {

        let krate = match *krate {
                Some(ref k) => k,
                None => bail!("specify which package to clone!"),
        };

        let mut regsrc = RegistrySource::new(&srcid, &config);
        try!(regsrc.update());

        let version = match flag_version {
            Some(v) => {
                match v.to_semver() {
                    Ok(v) => v,
                    Err(e) => bail!("{}", e),
                }
            },
            None => {
                let dep = try!(Dependency::parse(krate, flag_version.as_ref().map(|s| &s[..]), &srcid));
                let summaries = try!(regsrc.query(&dep));

                let latest = summaries.iter().max_by_key(|s| s.version());

                match latest {
                    Some(l) => l.version().to_semver().unwrap(),
                    None => bail!("package '{}' not found", krate),
                }
            },
        };

        let pkgid = try!(PackageId::new(&krate, version, srcid));

        let krate = try!(regsrc.download(&pkgid.clone()));

        let mut dest_path = try!(env::current_dir());
        dest_path.push(krate.name());

        try!(clone_directory(&krate.root(), &dest_path));

        Ok(())
    }

    fn clone_directory(from: &Path, to: &Path) -> CargoResult<()> {
        for entry in WalkDir::new(from) {
            let entry = entry.unwrap();
            let file_type = entry.file_type();
            let mut to = to.to_owned();
            to.push(&strip_prefix(from, entry.path()));

            if file_type.is_file() {
                try!(fs::copy(&entry.path(), &to));
            }
            else if file_type.is_dir() {
                try!(fs::create_dir(&to));
            }
        }

        Ok(())
    }

    // When Path.strip_prefix() lands, this can be removed
    fn strip_prefix(prefix: &Path, path: &Path) -> PathBuf {
        assert!(path.starts_with(prefix));
        let mut ret = PathBuf::new();
        for e in path.iter().zip_longest(prefix) {
            match e {
                Both(..) => continue,
                Left(a) => ret.push(a),
                Right(_) => unreachable!(),
            }
        }

        ret
    }

    #[test]
    fn test_strip_prefix() {
        let r = Path::new("foo/bar.py");
        let prefix = Path::new("/home/john/");
        let path = Path::new("/home/john/foo/bar.py");
        assert_eq!(r, strip_prefix(prefix, path));
    }
}
