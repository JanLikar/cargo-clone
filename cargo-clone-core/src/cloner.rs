use std::{env, path::PathBuf};

use anyhow::Context;
use cargo::{core::SourceId, util::IntoUrl, CargoResult, Config};

pub struct Cloner {
    /// Cargo configuration.
    config: Config,
    /// Directory where the crates will be cloned.
    /// Each crate is cloned into a subdirectory of this directory.
    directory: PathBuf,
    /// Where the crates will be cloned from.
    source_id: SourceId,
}

/// Builder for [`Cloner`].
/// By default the cloner will clone from crates.io.
#[derive(Debug, Default)]
pub struct ClonerBuilder {
    config: Option<Config>,
    directory: Option<PathBuf>,
    source: CargoSource,
}

#[derive(Debug, Default)]
pub enum CargoSource {
    #[default]
    CratesIo,
    ///
    Index(String),
    LocalRegistry(String),
    Registry(String),
}

impl CargoSource {
    /// Creates a [`Source`] from the name of the remote registry.
    pub fn registry(self, key: impl Into<String>) -> Self {
        CargoSource::Registry(key.into())
    }

    /// Creates a [`Source`] from a local registry path.
    pub fn local_registry(self, path: impl Into<String>) -> Self {
        CargoSource::LocalRegistry(path.into())
    }

    /// Creates a [`Source`] from a remote registry URL.
    pub fn index(self, index: impl Into<String>) -> Self {
        CargoSource::Index(index.into())
    }

    fn to_source_id(&self, config: &Config) -> CargoResult<SourceId> {
        match self {
            CargoSource::CratesIo => SourceId::crates_io(config),
            CargoSource::Index(url) => SourceId::for_registry(&url.into_url()?),
            CargoSource::LocalRegistry(path) => SourceId::for_local_registry(&config.cwd().join(path)),
            CargoSource::Registry(key) => SourceId::alt_registry(config, key),
        }
    }
}

impl ClonerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_config(self, config: Config) -> Self {
        Self {
            config: Some(config),
            ..self
        }
    }

    pub fn with_directory(self, directory: impl Into<PathBuf>) -> Self {
        Self {
            directory: Some(directory.into()),
            ..self
        }
    }

    /// Clone from an alternative source, instead of crates.io.
    pub fn with_source(self, source: CargoSource) -> Self {
        Self { source, ..self }
    }

    pub fn build(self) -> CargoResult<Cloner> {
        let config = match self.config {
            Some(config) => config,
            None => Config::default().context("Unable to get cargo config.")?,
        };
        let directory = match self.directory {
            Some(directory) => directory,
            None => env::current_dir().context("Unable to get current directory.")?,
        };

        let source_id = self
            .source
            .to_source_id(&config)
            .context("can't determine the source id")?;
        Ok(Cloner {
            config,
            directory,
            source_id,
        })
    }
}
