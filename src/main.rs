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

use cargo::core::SourceId;
use cargo::util::{Config, CliResult};

use docopt::Docopt;

#[derive(RustcDecodable, Debug)]
pub struct Options {
    flag_verbose: bool,
    flag_quiet: bool,
    flag_color: Option<String>,

    arg_crate: Option<String>,
    flag_vers: Option<String>,

    flag_path: Option<String>,
}

pub const USAGE: &'static str = "
Clone source code of a Rust crate

Usage:
    cargo clone [options] [<crate>]

Options:
    --vers VERS               Specify a version to clone from crates.io
    -h, --help                Print this message
    -v, --verbose             Use verbose output
    -q, --quiet               Less output printed to stdout
    --color WHEN              Coloring: auto, always, never
";

fn main() {
    let options: Options = Docopt::new(USAGE)
                               .and_then(|d| d.decode())
                               .unwrap_or_else(|e| e.exit());

    let config = Config::default().expect("Unable to get config");

    if let Err(e) = execute(options, config) {
        println!("{}", e.to_string())
    }
}

pub fn execute(options: Options, config: Config) -> CliResult<Option<()>> {
    try!(config.shell().set_verbosity(options.flag_verbose, options.flag_quiet));
    try!(config.shell().set_color_config(options.flag_color.as_ref().map(|s| &s[..])));

    // Make a SourceId for the central Registry (usually crates.io)
    let source_id = try!(SourceId::for_central(&config));

    try!(cargo_clone::ops::clone(&options.arg_crate, &source_id, options.flag_vers, config));

    Ok(None)
}
