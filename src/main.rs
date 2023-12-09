mod cli;
mod debugger;

use std::ffi::{c_char, CString};

use nix::{self, libc, sys};

fn main() {
    let args = cli::Cli::parse();

    let pid = unsafe { libc::fork() };
    if pid == 0 {
        // In the child process
        sys::ptrace::traceme().unwrap();

        unsafe {
            let program = CString::new(args.program.to_str().unwrap())
                .unwrap()
                .as_ptr() as *const c_char;

            let _ = libc::execl(program, program);
        }
    } else if pid >= 1 {
        // In the parent process
    } else {
        panic!("WTF????")
    }
}
