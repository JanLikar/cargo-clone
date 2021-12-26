                                          _                  
      ___ __ _ _ __ __ _  ___         ___| | ___  _ __   ___ 
     / __/ _` | '__/ _` |/ _ \ _____ / __| |/ _ \| '_ \ / _ \
    | (_| (_| | | | (_| | (_) |_____| (__| | (_) | | | |  __/
     \___\__,_|_|  \__, |\___/       \___|_|\___/|_| |_|\___|
                   |___/                                     

cargo-clone can be used to fetch the source code of a Rust crate.


    cargo clone [FLAGS] [OPTIONS] <crate>... [-- <directory>]

cargo-clone is a [Cargo subcommand](https://github.com/rust-lang/cargo/wiki/Third-party-cargo-subcommands).

It can be installed using the install subcommand

    cargo install cargo-clone

and can be used like this:

    cargo clone [options] [<crate>]

For example, to download version 1.0.0 of cargo-clone's source from crates.io, you would run

    cargo clone cargo-clone@1.0.0

Downloading multiple packages is also supported:

    cargo clone cargo-clone@1.0.0 serde time

The output dir can be specified as the last argument:

    cargo clone serde time -- packages/

To checkout a git repo specified in the package's Cargo.toml, you can use the `--git` flag:

    cargo clone --git cargo-clone


## Contributing

When running locally, you can run using `cargo run -- clone CRATE` or `cargo-clone clone CRATE`.
