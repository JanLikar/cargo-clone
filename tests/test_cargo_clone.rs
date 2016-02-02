use std::path::MAIN_SEPARATOR as SEP;

use support::{execs, project};
use support::{COMPILING, RUNNING};

use hamcrest::assert_that;

fn setup() {
}

test!(no_package {
    let p = project("foo");
    assert_that(p.process("cargo")
                 .arg("run")
                 .arg("--")
                 .arg("clone"), // To make docopt happy
                execs().with_status(101).with_stderr("\
specify which package to clone.
"));
});

test!(unknown_package {
    let p = project("foo");
    assert_that(p.process("cargo")
                 .arg("run")
                 .arg("--")
                 .arg("clone") // To make docopt happy
                 .arg("foobarfoofoobarbar"), // Hopefully this package does not exist
                execs().with_status(101).with_stderr("\
BLABLA
"));
});

test!(clone_package {
    let p = project("foo");
    assert_eq!(p.process("top"), 101);
});
