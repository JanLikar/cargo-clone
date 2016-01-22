extern crate cargo;
extern crate docopt;
extern crate rustc_serialize;
extern crate cargo_clone;

use cargo::core::SourceId;
use cargo::util::{Config, CliResult, human};
use cargo::util::errors::CliError;

use docopt::Docopt;

#[derive(RustcDecodable, Debug)]
pub struct Options {
    flag_verbose: bool,
    flag_quiet: bool,
    flag_color: Option<String>,
    flag_root: Option<String>,

    arg_crate: Option<String>,
    flag_vers: Option<String>,

    flag_path: Option<String>,
}

pub const USAGE: &'static str = "
Download source code of a Rust crate
Usage:
    cargo-clone [options] [<crate>]

Specifying what crate to clone:
    --vers VERS               Specify a version to clone from crates.io
Build and install options:
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

    match execute(options, config) {
        Ok(_) => {}
        Err(e) => println!("{}", e.to_string()),
    }
}

pub fn execute(options: Options, config: Config) -> CliResult<Option<()>> {
    try!(config.shell().set_verbosity(options.flag_verbose, options.flag_quiet));
    try!(config.shell().set_color_config(options.flag_color.as_ref().map(|s| &s[..])));

    let source_id = try!(SourceId::for_central(&config));

    try!(cargo_clone::ops::clone(&options.arg_crate, &source_id, &options.flag_vers, config));

    Ok(None)
}
