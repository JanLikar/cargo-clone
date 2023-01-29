use std::fmt;

use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
pub enum Command {
    #[command(name = "clone")]
    #[command(about, author, version)]
    Clone(CloneOpt),
}

#[derive(Debug, Clone, clap::Args)]
pub struct CloneOpt {
    /// Terminal coloring.
    #[clap(long, value_enum, value_name = "COLORING")]
    pub color: Option<Color>,
    /// Use verbose output.
    #[clap(short)]
    pub verbose: bool,
    /// Print less output to stdout.
    #[clap(short)]
    pub quiet: bool,
    /// A registry name from Cargo config to clone the specified crate from.
    #[clap(long, conflicts_with("index"), value_name = "REGISTRY")]
    pub registry: Option<String>,
    /// Registry index to install from.
    #[clap(long, conflicts_with("registry"), value_name = "URL")]
    pub index: Option<String>,
    /// A local registry path to clone the specified crate from.
    #[clap(
        long,
        conflicts_with("index"),
        conflicts_with("registry"),
        value_name = "PATH"
    )]
    pub local_registry: Option<String>,
    /// Clone from a repository specified in package's metadata.
    #[clap(long)]
    pub git: bool,
    /// The crates to be downloaded. Versions may also be specified and are matched exactly by default.
    /// Examples: 'cargo-clone@1.0.0' 'cargo-clone@~1.0.0'.
    pub crate_: Vec<String>,
    /// The destination directory. If it ends in a slash, crates will be placed into its subdirectories.
    #[clap(last = true)]
    pub directory: Option<String>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Color {
    Auto,
    Always,
    Never,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Always => write!(f, "always"),
            Self::Auto => write!(f, "auto"),
            Self::Never => write!(f, "never"),
        }
    }
}
