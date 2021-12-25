                                          _                  
      ___ __ _ _ __ __ _  ___         ___| | ___  _ __   ___ 
     / __/ _` | '__/ _` |/ _ \ _____ / __| |/ _ \| '_ \ / _ \
    | (_| (_| | | | (_| | (_) |_____| (__| | (_) | | | |  __/
     \___\__,_|_|  \__, |\___/       \___|_|\___/|_| |_|\___|
                   |___/                                     

cargo-clone can be used to fetch the source code of a Rust crate.

cargo-clone is a [Cargo subcommand](https://github.com/rust-lang/cargo/wiki/Third-party-cargo-subcommands).

It can be installed using the install subcommand

    cargo install cargo-clone

and can be used like this:

    cargo clone [options] [<crate>]...

For example, to download version 0.2.0 of cargo-clone's source from crates.io, you would run

    cargo clone --vers 0.2.0 cargo-clone
