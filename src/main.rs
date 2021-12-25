// Copyright 2015 Jan Likar.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use anyhow::bail;

use cargo::core::SourceId;
use cargo::util::{into_url::IntoUrl, Config};

use clap::Arg;

type Result<T> = std::result::Result<T, anyhow::Error>;

fn main() {
    let version = version();

    let app = clap::App::new("cargo-clone")
        .bin_name("cargo clone")
        .version(&*version)
        .arg(
            Arg::with_name("vers")
                .long("vers")
                .value_name("VERSION")
                .help("Specify crate version."),
        )
        .arg(
            Arg::with_name("color")
                .long("color")
                .value_name("COLORING")
                .help("Coloring: auto, always, never")
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
            Arg::with_name("prefix")
                .long("prefix")
                .value_name("PREFIX")
                .help("Install under a different prefix."),
        )
        .arg(
            Arg::with_name("path")
                .long("path")
                .value_name("PATH")
                .help("Filesystem path to local crate to clone."),
        )
        .arg(
            Arg::with_name("alt-registry")
                .long("alt-registry")
                .value_name("REGISTRY")
                .help("A registry name from Cargo config to clone the specified crate from."),
        )
        .arg(
            Arg::with_name("registry-url")
                .long("registry-url")
                .value_name("URL")
                .help(" A registry url to clone the specified crate from."),
        )
        .arg(
            Arg::with_name("local-registry")
                .long("local-registry")
                .value_name("PATH")
                .help("A local registry path to clone the specified crate from."),
        )
        .arg(
            Arg::with_name("git")
                .long("git")
                .help("Clone from repository specified in package's metadata."),
        )
        .arg(Arg::with_name("crate").help("The name of the crate to be downloaded."));

    let matches = app.get_matches();
    let mut config = Config::default().expect("Unable to get config.");

    if let Err(e) = execute(matches, &mut config) {
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

pub fn execute(matches: clap::ArgMatches, config: &mut Config) -> Result<Option<()>> {
    let verbose = if matches.is_present("verbose") { 1 } else { 0 };

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

    let source_id = if let Some(path) = matches.value_of("path") {
        SourceId::for_path(&config.cwd().join(path))?
    } else if let Some(registry) = matches.value_of("registry-name") {
        SourceId::alt_registry(config, registry)?
    } else if let Some(url) = matches.value_of("registry-url") {
        let url = url.into_url()?;
        SourceId::for_registry(&url)?
    } else if let Some(path) = matches.value_of("local-registry") {
        SourceId::for_local_registry(&config.cwd().join(path))?
    } else if matches.value_of("crate").is_none() {
        bail!(
            "must specify a crate to clone from \
             crates.io, or use --path or --git to \
             specify alternate source"
        );
    } else {
        SourceId::crates_io(config)?
    };

    let krate = matches.value_of("crate");
    let prefix = matches.value_of("prefix");
    let vers = matches.value_of("vers");
    let git = matches.is_present("git");

    cargo_clone::clone(krate, &source_id, prefix, git, vers, config)?;

    Ok(None)
}
