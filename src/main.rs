// Copyright 2015 Jan Likar.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use cargo::core::SourceId;
use cargo::util::{into_url::IntoUrl, Config};

use docopt::Docopt;

use anyhow::bail;

use serde::Deserialize;

type Result<T> = std::result::Result<T, anyhow::Error>;

#[derive(Deserialize, Debug)]
pub struct Options {
    flag_verbose: Option<bool>,
    flag_quiet: Option<bool>,
    flag_color: Option<String>,

    flag_prefix: Option<String>,

    arg_crate: Option<String>,
    flag_vers: Option<String>,

    flag_path: Option<String>,

    flag_alt_registry: Option<String>,

    flag_registry_url: Option<String>,

    flag_local_registry: Option<String>,
}

pub const USAGE: &str = "
Clone source code of a Rust crate

Usage:
    cargo clone [options] [<crate>]

Options:
    --prefix DIR              Directory to clone the package into

    --vers VERS               Specify a version to clone from crates.io

    --path PATH               Filesystem path to local crate to clone

    --alt-registry NAME       A registry name from Cargo config to clone the specified crate from

    --registry-url URL        A registry url to clone the specified crate from

    --local-registry PATH     A local registry path to clone the specified crate from

    -h, --help                Print this message
    -V, --version             Print version information
    -v, --verbose             Use verbose output
    -q, --quiet               Less output printed to stdout
    --color WHEN              Coloring: auto, always, never
";

fn main() {
    let options: Options = Docopt::new(USAGE)
        .and_then(|d| d.version(Some(version())).deserialize())
        .unwrap_or_else(|e| e.exit());

    let mut config = Config::default().expect("Unable to get config.");

    if let Err(e) = execute(options, &mut config) {
        config.shell().error(e).unwrap();
        std::process::exit(101);
    }
}

fn version() -> String {
    format!(
        "cargo-clone {}.{}.{}{}",
        option_env!("CARGO_PKG_VERSION_MAJOR").unwrap_or("X"),
        option_env!("CARGO_PKG_VERSION_MINOR").unwrap_or("X"),
        option_env!("CARGO_PKG_VERSION_PATCH").unwrap_or("X"),
        option_env!("CARGO_PKG_VERSION_PRE").unwrap_or("")
    )
}

pub fn execute(options: Options, config: &mut Config) -> Result<Option<()>> {
    let verbose = match options.flag_verbose {
        Some(v) => {
            if v {
                1
            } else {
                0
            }
        }
        None => 0,
    };

    let flag_quiet = options.flag_quiet.unwrap_or(false);

    config.configure(
        verbose,
        flag_quiet,
        options.flag_color.as_deref(),
        false,
        false,
        false,
        &None,
        &[],
        &[],
    )?;

    let source_id = if let Some(path) = options.flag_path {
        SourceId::for_path(&config.cwd().join(path))?
    } else if let Some(registry) = options.flag_alt_registry.as_ref() {
        SourceId::alt_registry(config, registry)?
    } else if let Some(url) = options.flag_registry_url.as_ref() {
        let url = url.into_url()?;
        SourceId::for_registry(&url)?
    } else if let Some(path) = options.flag_local_registry.as_ref() {
        SourceId::for_local_registry(&config.cwd().join(path))?
    } else if options.arg_crate.is_none() {
        bail!(
            "must specify a crate to clone from \
             crates.io, or use --path or --git to \
             specify alternate source"
        );
    } else {
        SourceId::crates_io(config)?
    };

    let krate = options.arg_crate.as_deref();
    let prefix = options.flag_prefix.as_ref().map(|s| &s[..]);
    let vers = options.flag_vers.as_ref().map(|s| &s[..]);

    cargo_clone::ops::clone(krate, &source_id, prefix, vers, config)?;

    Ok(None)
}
