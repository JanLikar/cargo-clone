use std::{env, path::PathBuf};

use anyhow::Context;
use cargo::{CargoResult, Config};

pub struct Cloner {
    /// Cargo configuration.
    config: Config,
    /// Directory where the crates will be cloned.
    /// Each crate is cloned into a subdirectory of this directory.
    directory: PathBuf,
}

#[derive(Debug, Default)]
pub struct ClonerBuilder {
    config: Option<Config>,
    directory: Option<PathBuf>,
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

    pub fn with_directory(self, directory: PathBuf) -> Self {
        Self {
            directory: Some(directory),
            ..self
        }
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

        Ok(Cloner { config, directory })
    }
}
