use std::{env, path::PathBuf};

use anyhow::Context;
use cargo::{core::SourceId, CargoResult, Config};

use crate::ClonerSource;

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
    source: ClonerSource,
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
    pub fn with_source(self, source: ClonerSource) -> Self {
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
            .cargo_source
            .to_source_id(&config)
            .context("can't determine the source id")?;

        Ok(Cloner {
            config,
            directory,
            source_id,
        })
    }
}
