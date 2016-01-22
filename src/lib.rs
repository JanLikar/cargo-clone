extern crate cargo;

macro_rules! bail {
    ($($fmt:tt)*) => (
        return Err(util::human(&format_args!($($fmt)*)))
    )
}

pub mod ops {
    use std::path::Path;

    use cargo::util::{self, CargoResult, Config};
    use cargo::core::package_id::PackageId;
    use cargo::core::source::{Source, SourceId};
    use cargo::core::registry::Registry;
    use cargo::core::dependency::Dependency;
    use cargo::sources::RegistrySource;

    pub fn clone(krate: String, source_id: &SourceId, version: Option<String>, config: Config) -> CargoResult<()> {
        let version = match version {
            Some(v) => v,
            None => {unimplemented!()}
        };

        let package_id = try!(PackageId::new(&krate, &version, source_id));

        let mut registry_source = RegistrySource::new(&source_id, &config);

        try!(registry_source.update());
        try!(registry_source.download(&[package_id.clone()]));

        let crates = try!(registry_source.get(&[package_id.clone()]));

        println!("DONE: {:?}", crates[0].root());

        // create dir and copy files over
        Ok(())
    }

    fn clone_directory(from: &Path, to: &Path) -> CargoResult<()> {
        // Alla Walkdir
        unimplemented!();
    }

}
