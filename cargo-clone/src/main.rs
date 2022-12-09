// Copyright 2015 Jan Likar.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use anyhow::Context;
use cargo::util::Config;

use cargo_clone_core::{ClonerBuilder, ClonerSource};
use clap::Arg;

type Result<T> = std::result::Result<T, anyhow::Error>;

fn main() {
    let version = version();

    let app = clap::App::new("cargo clone")
        .bin_name("cargo clone")
        .version(&*version)
        // A hack to make calling cargo-clone directly work.
        .arg(Arg::with_name("dummy")
            .hidden(true)
            .required(true)
            .possible_value("clone"))
        .arg(
            Arg::with_name("color")
                .long("color")
                .value_name("COLORING")
                .help("Coloring: auto, always, never.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .help("Use verbose output."),
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .help("Print less output to stdout."),
        )
        .arg(
            Arg::with_name("registry")
                .long("registry")
                .value_name("REGISTRY")
                .help("A registry name from Cargo config to clone the specified crate from.")
                .conflicts_with("index"),
        )
        .arg(
            Arg::with_name("index")
                .long("index")
                .value_name("URL")
                .help("Registry index to install from.")
                .conflicts_with("registry"),
        )
        .arg(
            Arg::with_name("local-registry")
                .long("local-registry")
                .value_name("PATH")
                .help("A local registry path to clone the specified crate from.")
                .conflicts_with("registry")
                .conflicts_with("index"),
        )
        .arg(
            Arg::with_name("git")
                .long("git")
                .help("Clone from a repository specified in package's metadata."),
        )
        .arg(
            Arg::with_name("crate")
                .help("The crates to be downloaded. Versions may also be specified and are matched exactly by default. Examples: 'cargo-clone@1.0.0' 'cargo-clone@~1.0.0'.")
                .required(true)
                .multiple(true),
        )
        .arg(Arg::with_name("directory").help("The destination directory. If it ends in a slash, crates will be placed into its subdirectories.").last(true));

    let matches = app.get_matches();

    if let Err(e) = execute(&matches) {
        let config = cargo_config(&matches).expect("Unable to get config.");
        config.shell().error(e).unwrap();
        std::process::exit(101);
    }
}

fn version() -> String {
    format!(
        "{}.{}.{}{}",
        option_env!("CARGO_PKG_VERSION_MAJOR").unwrap_or("X"),
        option_env!("CARGO_PKG_VERSION_MINOR").unwrap_or("X"),
        option_env!("CARGO_PKG_VERSION_PATCH").unwrap_or("X"),
        option_env!("CARGO_PKG_VERSION_PRE").unwrap_or("")
    )
}

fn cargo_config(matches: &clap::ArgMatches) -> Result<Config> {
    let verbose = u32::from(matches.is_present("verbose"));

    let mut config = Config::default().expect("Unable to get config.");
    config.configure(
        verbose,
        matches.is_present("quiet"),
        matches.value_of("color"),
        false,
        false,
        false,
        &None,
        &[],
        &[],
    )?;
    Ok(config)
}

fn source(matches: &clap::ArgMatches) -> Result<ClonerSource> {
    let source = if let Some(registry) = matches.value_of("registry") {
        ClonerSource::registry(registry)
    } else if let Some(index) = matches.value_of("index") {
        ClonerSource::index(index)?
    } else if let Some(path) = matches.value_of("local-registry") {
        ClonerSource::local_registry(path)
    } else {
        ClonerSource::crates_io()
    };
    Ok(source)
}

pub fn execute(matches: &clap::ArgMatches) -> Result<()> {
    let source = source(matches).context("invalid source")?;

    let crates = matches
        .values_of("crate")
        .unwrap()
        .map(cargo_clone_core::parse_name_and_version)
        .collect::<Result<Vec<cargo_clone_core::Crate>>>()?;

    let config = cargo_config(matches)?;
    let mut cloner_builder = ClonerBuilder::new().with_source(source).with_config(config);
    let directory = matches.value_of("directory");
    if let Some(directory) = directory {
        cloner_builder = cloner_builder.with_directory(directory);
    }
    if matches.is_present("git") {
        cloner_builder = cloner_builder.with_git(true);
    }

    let cloner = cloner_builder
        .build()
        .context("Failed to setup cargo-clone")?;

    let should_append_crate_dir = {
        let multiple_crates = crates.len() > 1;
        let can_clone_in_dir = directory.map(|d| d.ends_with('/')).unwrap_or(true);
        multiple_crates && can_clone_in_dir
    };

    if should_append_crate_dir {
        cloner.clone(&crates)
    } else {
        cloner.clone_in_dir(&crates[0])
    }
    .context("Error while cloning")
}
