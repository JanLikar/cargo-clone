// Copyright 2015 Jan Likar.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.



#[macro_use]
extern crate failure;

use cargo_clone;

use cargo::core::{GitReference, SourceId};
use cargo::util::{into_url::IntoUrl, Config};

use docopt::Docopt;

use serde::Deserialize;

type Result<T> = std::result::Result<T, failure::Error>;

#[derive(Deserialize, Debug)]
pub struct Options {
    flag_verbose: Option<bool>,
    flag_quiet: Option<bool>,
    flag_color: Option<String>,
    flag_version: Option<bool>,

    flag_prefix: Option<String>,

    arg_crate: Vec<String>,
    flag_vers: Option<String>,
    flag_git: Option<String>,
    flag_branch: Option<String>,
    flag_tag: Option<String>,
    flag_rev: Option<String>,

    flag_path: Option<String>,
}

pub const USAGE: &'static str = "
Clone source code of a Rust crate

Usage:
    cargo clone [options] [<crate>]...

Options:
    --prefix DIR              Directory to clone the package into

    --vers VERS               Specify a version to clone from crates.io

    --git URL                 Git URL to clone the specified crate from
    --branch BRANCH           Branch to use when cloning from git
    --tag TAG                 Tag to use when cloning from git
    --rev SHA                 Specific commit to use when cloning from git

    --path PATH               Filesystem path to local crate to clone

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
    config.configure(
        verbose,
        options.flag_quiet,
        &options.flag_color,
        false,
        false,
        false,
        &None,
        &[],
    )?;

    let source_id = if let Some(url) = options.flag_git {
        let url = url.into_url()?;
        let gitref = if let Some(rev) = options.flag_rev {
            GitReference::Rev(rev)
        } else if let Some(tag) = options.flag_tag {
            GitReference::Tag(tag)
        } else if let Some(branch) = options.flag_branch {
            GitReference::Branch(branch)
        } else {
            GitReference::Branch("master".to_string())
        };
        SourceId::for_git(&url, gitref)?
    } else if let Some(path) = options.flag_path {
        SourceId::for_path(&config.cwd().join(path))?
    } else if options.arg_crate.len() == 0 {
        bail!(
            "must specify a crate to clone from \
             crates.io, or use --path or --git to \
             specify alternate source"
        );
    } else {
        SourceId::crates_io(config)?
    };

    let prefix = options.flag_prefix.as_ref().map(|s| &s[..]);
    let vers = options.flag_vers.as_ref().map(|s| &s[..]);
    if options.arg_crate.len() != 0 {
        for item in options.arg_crate.iter() {
            cargo_clone::ops::clone(Some(&item[..]), &source_id, prefix, vers, config)?;
        }
    } else {
        cargo_clone::ops::clone(None, &source_id, prefix, vers, config)?;
    }
    Ok(None)
}
