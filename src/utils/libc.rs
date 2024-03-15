use nix::{errno::Errno, libc::c_int};

pub fn return_or_exit(code: c_int, desc: &str) -> c_int {
    if code < 0 {
        let msg = format!("{}: {}", desc, Errno::last().desc());
        error!("{}", msg);
        panic!("{}", msg)
    }

    code
}
