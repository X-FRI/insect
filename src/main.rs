#[macro_use]
extern crate log;

mod breakpoint;
mod cli;
mod cmd;
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
        .map_err(|errno| {
            internal_utils::errno::error("Failed to create the program process", errno)
        })
        .unwrap()
    {
        unistd::ForkResult::Parent { child } => {
            info!("Successfully started debugging process");

            // Disable address space layout randomization for the programs we launch
            unsafe { libc::personality(libc::ADDR_NO_RANDOMIZE as u64) };
            Debugger::new(args.program.to_path_buf(), child).run()
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
