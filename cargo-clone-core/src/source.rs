// Copyright 2015 Jan Likar.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use cargo::{core::SourceId, util::IntoUrl, CargoResult, Config};

/// Where to clone the crate from.
#[derive(Debug, Default)]
pub struct ClonerSource {
    pub(crate) cargo_source: CargoSource,
}

#[derive(Debug, Default)]
pub(crate) enum CargoSource {
    #[default]
    CratesIo,
    ///
    Index(String),
    LocalRegistry(String),
    Registry(String),
}

impl ClonerSource {
    /// Creates a [`ClonerSource`] from the name of the remote registry.
    pub fn registry(key: impl Into<String>) -> Self {
        Self {
            cargo_source: CargoSource::Registry(key.into()),
        }
    }

    /// Creates a [`ClonerSource`] from a local registry path.
    pub fn local_registry(path: impl Into<String>) -> Self {
        Self {
            cargo_source: CargoSource::LocalRegistry(path.into()),
        }
    }

    /// Creates a [`ClonerSource`] from a remote registry URL.
    pub fn index(index: impl Into<String>) -> Self {
        Self {
            cargo_source: CargoSource::Index(index.into()),
        }
    }

    /// Creates a [`ClonerSource`] from [crates.io](https://crates.io/).
    pub fn crates_io() -> Self {
        Self {
            cargo_source: CargoSource::CratesIo,
        }
    }
}

impl CargoSource {
    pub(crate) fn to_source_id(&self, config: &Config) -> CargoResult<SourceId> {
        match self {
            CargoSource::CratesIo => SourceId::crates_io(config),
            CargoSource::Index(url) => SourceId::for_registry(&url.into_url()?),
            CargoSource::LocalRegistry(path) => {
                SourceId::for_local_registry(&config.cwd().join(path))
            }
            CargoSource::Registry(key) => SourceId::alt_registry(config, key),
        }
    }
}
