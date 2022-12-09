use std::{env, path::PathBuf};

use anyhow::Context;
use cargo::{CargoResult, Config};

use crate::{ClonerSource, Cloner};

/// Builder for [`Cloner`].
/// By default the cloner will clone from crates.io.
#[derive(Debug, Default)]
pub struct ClonerBuilder {
    config: Option<Config>,
    directory: Option<PathBuf>,
    source: ClonerSource,
    use_git: bool,
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

    /// Clone the git repository present in the manifest metadata.
    pub fn with_git(self, use_git: bool) -> Self {
        Self { use_git, ..self }
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

        let srcid = self
            .source
            .cargo_source
            .to_source_id(&config)
            .context("can't determine the source id")?;

        Ok(Cloner {
            config,
            directory,
            srcid,
            use_git: self.use_git,
        })
    }
}
