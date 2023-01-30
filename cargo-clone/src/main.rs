// Copyright 2015 Jan Likar.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod args;

use anyhow::Context;
use args::{CloneOpt, Command};
use cargo::util::Config;

use cargo_clone_core::{ClonerBuilder, ClonerSource};
use clap::Parser;

type Result<T> = std::result::Result<T, anyhow::Error>;

fn main() {
    let Command::Clone(ref args) = Command::parse();

    if let Err(e) = execute(args) {
        let config = cargo_config(args).expect("Unable to get config.");
        let error_msg = format!("{:?}", e);
        config.shell().error(error_msg).unwrap();
        std::process::exit(101);
    }
}

fn cargo_config(matches: &CloneOpt) -> Result<Config> {
    let verbose = u32::from(matches.verbose);

    let mut config = Config::default().expect("Unable to get config.");
    let color = matches.color.map(|c| c.to_string());
    config.configure(
        verbose,
        matches.quiet,
        color.as_deref(),
        false,
        false,
        false,
        &None,
        &[],
        &[],
    )?;
    Ok(config)
}

fn source(opts: &CloneOpt) -> Result<ClonerSource> {
    let source = if let Some(registry) = &opts.registry {
        ClonerSource::registry(registry)
    } else if let Some(index) = &opts.index {
        ClonerSource::index(index)?
    } else if let Some(path) = &opts.local_registry {
        ClonerSource::local_registry(path)
    } else {
        ClonerSource::crates_io()
    };
    Ok(source)
}

pub fn execute(opts: &CloneOpt) -> Result<()> {
    let source = source(opts).context("invalid source")?;

    let crates = opts
        .crate_
        .iter()
        .map(|c| c.as_str())
        .map(cargo_clone_core::parse_name_and_version)
        .collect::<Result<Vec<cargo_clone_core::Crate>>>()?;

    let config = cargo_config(opts)?;
    let mut cloner_builder = ClonerBuilder::new().with_source(source).with_config(config);
    let directory = opts.directory.as_deref();
    if let Some(directory) = directory {
        cloner_builder = cloner_builder.with_directory(directory);
    }
    if opts.git {
        cloner_builder = cloner_builder.with_git(true);
    }

    let cloner = cloner_builder
        .build()
        .context("Failed to setup cargo-clone")?;

    let should_append_crate_dir = {
        let multiple_crates = crates.len() > 1;
        let can_clone_in_dir = directory.map(|d| d.ends_with('/')).unwrap_or(true);
        multiple_crates || can_clone_in_dir
    };

    if should_append_crate_dir {
        cloner.clone(&crates)
    } else {
        cloner.clone_in_dir(&crates[0])
    }
    .context("Error while cloning")
}
