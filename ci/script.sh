# This script takes care of testing your crate

set -ex

main() {
    cross build --target $TARGET
    cross build --target $TARGET --release

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cross test --target $TARGET
    cross test --target $TARGET --release

    # Test that running the binary works in a few common cases.
    ./target/rls/cargo-clone clone time
    ./target/rls/cargo-clone clone --prefix /tmp/output/ time
    mkdir /tmp/clone-into-existing
    ./target/rls/cargo-clone clone --prefix /tmp/clone-into-existing time
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
