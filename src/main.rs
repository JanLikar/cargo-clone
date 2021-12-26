// Copyright 2015 Jan Likar.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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

    let source_id = if let Some(registry) = matches.value_of("registry") {
        SourceId::alt_registry(config, registry)?
    } else if let Some(index) = matches.value_of("index") {
        SourceId::for_registry(&index.into_url()?)?
    } else if let Some(path) = matches.value_of("local-registry") {
        SourceId::for_local_registry(&config.cwd().join(path))?
    } else {
        SourceId::crates_io(config)?
    };

    let directory = matches.value_of("directory");
    let use_git = matches.is_present("git");

    let crates = matches
        .values_of("crate")
        .unwrap()
        .map(cargo_clone::parse_name_and_version)
        .collect::<Result<Vec<cargo_clone::Crate>>>()?;

    let opts = cargo_clone::CloneOpts::new(&crates, &source_id, directory, use_git);

    cargo_clone::clone(&opts, config)?;

    Ok(None)
}
