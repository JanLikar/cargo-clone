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

use cargo::core::SourceId;
use cargo::util::{Config, CliResult};

use docopt::Docopt;

#[derive(RustcDecodable, Debug)]
pub struct Options {
    flag_verbose: Option<bool>,
    flag_quiet: Option<bool>,
    flag_color: Option<String>,
    flag_version: Option<bool>,

    flag_prefix: Option<String>,

    arg_crate: Option<String>,
    flag_vers: Option<String>,
}

pub const USAGE: &'static str = "
Clone source code of a Rust crate

Usage:
    cargo clone [options] [<crate>]

Options:
    --prefix DIR              Directory to clone the package into

    --vers VERS               Specify a version to clone from crates.io

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

    // Make a SourceId for the central Registry (usually crates.io)
    let source_id = try!(SourceId::for_central(&config));

    let krate = options.arg_crate.as_ref().map(|s| &s[..]);
    let prefix = options.flag_prefix.as_ref().map(|s| &s[..]);
    let vers = options.flag_vers.as_ref().map(|s| &s[..]);

    try!(cargo_clone::ops::clone(krate,
                                 &source_id,
                                 prefix,
                                 vers,
                                 config));

    Ok(None)
}
