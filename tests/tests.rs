extern crate bufstream;
extern crate cargo;
extern crate filetime;
extern crate flate2;
extern crate git2;
extern crate hamcrest;
extern crate libc;
extern crate rustc_serialize;
extern crate tar;
extern crate tempdir;
extern crate term;
extern crate url;
#[cfg(windows)] extern crate kernel32;
#[cfg(windows)] extern crate winapi;

#[macro_use]
extern crate log;

use cargo::util::Rustc;
use std::ffi::OsStr;

mod support;
macro_rules! test {
    ($name:ident $expr:expr) => (
        #[test]
        fn $name() {
            ::support::paths::setup();
            setup();
            $expr;
        }
    )
}

mod test_cargo_clone;

thread_local!(static RUSTC: Rustc = Rustc::new("rustc").unwrap());

fn rustc_host() -> String {
    RUSTC.with(|r| r.host.clone())
}

fn is_nightly() -> bool {
    RUSTC.with(|r| {
        r.verbose_version.contains("-nightly") ||
            r.verbose_version.contains("-dev")
    })
}

fn can_panic() -> bool {
    RUSTC.with(|r| !(r.host.contains("msvc") && !r.host.contains("x86_64")))
}

fn process<T: AsRef<OsStr>>(t: T) -> cargo::util::ProcessBuilder {
    let mut p = cargo::util::process(t.as_ref());
    p.cwd(&support::paths::root())
     .env("HOME", &support::paths::home())
     .env_remove("CARGO_HOME")
     .env_remove("CARGO_TARGET_DIR") // we assume 'target'
     .env_remove("MSYSTEM");    // assume cmd.exe everywhere on windows
    return p
}

fn cargo_process() -> cargo::util::ProcessBuilder {
    process(&support::cargo_dir().join("cargo"))
}

#[allow(deprecated)] // sleep_ms is now deprecated in favor of sleep()
fn sleep_ms(ms: u32) {
    std::thread::sleep_ms(ms);
}
