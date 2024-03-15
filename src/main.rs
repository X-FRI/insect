#[macro_use]
extern crate log;

mod cli;
mod debugger;
mod utils;

use colog;
use nix::{self, libc, sys, unistd};
use std::{
    ffi::{c_char, c_void, CString},
    ptr,
};
use utils as internal_utils;

use crate::debugger::Debugger;

fn main() {
    colog::init();

    let args = cli::Cli::parse();
    match unsafe { unistd::fork() }
        .map_err(|errno| internal_utils::errno::exit("Failed to create the program process", errno))
        .unwrap()
    {
        unistd::ForkResult::Parent { child } => {
            info!("Successfully started debugging process");
            Debugger::new(args.program.to_str().unwrap(), child).run()
        }

        unistd::ForkResult::Child => {
            sys::ptrace::traceme().unwrap();

            unsafe {
                internal_utils::libc::return_or_exit(
                    libc::execl(
                        CString::new(args.program.clone().to_str().unwrap())
                            .unwrap()
                            .as_c_str()
                            .as_ptr(),
                        CString::new(args.program.clone().to_str().unwrap())
                            .unwrap()
                            .as_c_str()
                            .as_ptr(),
                        ptr::null() as *const c_void,
                    ),
                    "Failed to create the program process",
                );
            }

            info!("Successfully started program process")
        }
    }
}
