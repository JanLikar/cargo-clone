// Copyright 2015 Jan Likar.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate cargo;
extern crate docopt;
extern crate rustc_serialize;
extern crate cargo_clone;

use std::io::{self, Write};

use cargo::core::{SourceId, GitReference};
use cargo::util::{Config, CliResult, ToUrl, human};

use docopt::Docopt;

#[derive(RustcDecodable, Debug)]
pub struct Options {
    flag_verbose: Option<bool>,
    flag_quiet: Option<bool>,
    flag_color: Option<String>,
    flag_version: Option<bool>,

    arg_crate: Option<String>,
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
    cargo clone [options] [<crate>]

Options:
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
                                  .and_then(|d| d.version(Some(version())).decode())
                                  .unwrap_or_else(|e| e.exit());

    let config = Config::default().expect("Unable to get config");

    if let Err(e) = execute(options, config) {
        write!(io::stderr(), "{}\n", e.to_string()).unwrap();
        std::process::exit(101);
    }
}

fn version() -> String {
    format!("cargo-clone {}.{}.{}{}",
            option_env!("CARGO_PKG_VERSION_MAJOR").unwrap_or("X"),
            option_env!("CARGO_PKG_VERSION_MINOR").unwrap_or("X"),
            option_env!("CARGO_PKG_VERSION_PATCH").unwrap_or("X"),
            option_env!("CARGO_PKG_VERSION_PRE").unwrap_or(""))
}

pub fn execute(options: Options, config: Config) -> CliResult<Option<()>> {
    try!(config.configure_shell(options.flag_verbose,
                                options.flag_quiet,
                                &options.flag_color));

    let source_id = match (options.flag_path, options.flag_git) {
        (Some(path), None) => {
            try!(SourceId::for_path(path.as_ref()))
        }
        (None, Some(git)) => {
            unimplemented!();
            // SourceId for Git registry
            let git = try!(git.to_url().map_err(human));
            let gref = if let Some(branch) = options.flag_branch {
                GitReference::Branch(branch)
            } else if let Some(tag) = options.flag_tag {
                GitReference::Tag(tag)
            } else if let Some(rev) = options.flag_rev {
                GitReference::Rev(rev)
            } else {
                GitReference::Branch("master".to_string())
            };

            SourceId::for_git(&git, gref)
        },
        (None, None) => {
            // Make a SourceId for the central Registry (usually crates.io)
            try!(SourceId::for_central(&config))
        }
        (Some(_), Some(_)) => panic!("--path and --git flags are incompatible."),
    };

    try!(cargo_clone::ops::clone(&options.arg_crate, &source_id, options.flag_vers, config));

    Ok(None)
}
