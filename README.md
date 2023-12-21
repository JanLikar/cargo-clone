# cargo-clone

cargo-clone can be used to fetch the source code of a Rust crate from a registry.

    cargo clone [FLAGS] [OPTIONS] <crate>... [-- <directory>]

cargo-clone is a [Cargo subcommand](https://github.com/rust-lang/cargo/wiki/Third-party-cargo-subcommands).

# Installation & upgrading

    cargo install cargo-clone

## Usage

    cargo clone [FLAGS] [OPTIONS] <crate>... [-- <directory>]

To download cargo-clone's code you would use

    cargo clone cargo-clone


### Specifying versions
The latest available version is downloaded by default.
If specific versions are desired, semver specifiers can be appended to crate names. 


    cargo clone cargo-clone@1.0.0

Versions are matched exactly by default, but other kinds of matching are also allowed.

    cargo clone cargo-clone@~1.0.0


### Cloning from git repositories
Using the `--git` flag runs `git clone` on each git repository url extracted from crate's metadata.

These lines are roughly equivalent:

    cargo clone --git cargo-clone
    git clone https://github.com/janlikar/cargo-clone

The command fails if a crate does not have the repository field set to a valid git repository.


### Output directory
Crates are downloaded into `$PWD/$CRATE_NAME` by default.

The output dir can be specified as the last argument:

    cargo clone cargo-clone -- foo  # Downloads into $PWD/foo

If multiple packages are downloaded at the same time or if the directory contains a trailing slash,
the packages will be downloaded into subdirectories of the path provided.

    cargo clone cargo-clone -- pkgs/  # Creates pkgs/cargo-clone/
    cargo clone cargo serde -- pkgs2/  # Creates pkgs2/cargo and pkgs2/serde


## Contributing
Contributions are welcome. Feel free to open a PR into develop branch.

When running locally, you can run using `cargo run -- clone CRATE` or `cargo-clone clone CRATE`.

By opening a PR you agree to license your code under Apache/MIT licenses.
