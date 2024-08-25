// Copyright 2015 Jan Likar.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{env, path::PathBuf};

use anyhow::Context;
use cargo::CargoResult;
use cargo::util::context::GlobalContext;

use crate::{Cloner, ClonerSource};

/// Builder for [`Cloner`].
#[derive(Debug, Default)]
pub struct ClonerBuilder {
    context: Option<GlobalContext>,
    directory: Option<PathBuf>,
    source: ClonerSource,
    use_git: bool,
}

impl ClonerBuilder {
    /// Creates a new [`ClonerBuilder`] that:
    /// - Uses crates.io as source.
    /// - Clones the crates into the current directory.
    pub fn new() -> Self {
        Self::default()
    }

    /// Use the specified cargo configuration, instead of the default one.
    pub fn with_context(self, context: GlobalContext) -> Self {
        Self {
            context: Some(context),
            ..self
        }
    }

    /// Clone into a different directory, instead of the current one.
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

    /// Build the [`Cloner`].
    pub fn build(self) -> CargoResult<Cloner> {
        let context = match self.context {
            Some(context) => context,
            None => GlobalContext::default().context("Unable to get cargo context.")?,
        };

        let directory = match self.directory {
            Some(directory) => directory,
            None => env::current_dir().context("Unable to get current directory.")?,
        };

        let srcid = self
            .source
            .cargo_source
            .to_source_id(&context)
            .context("can't determine the source id")?;

        Ok(Cloner {
            context,
            directory,
            srcid,
            use_git: self.use_git,
        })
    }
}
