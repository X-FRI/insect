use crate::utils as internal_utils;

use super::Debugger;
use nix::sys;

pub trait Command {
    fn exec(debugger: &Debugger) -> ();
}

pub struct Continue;

impl Command for Continue {
    fn exec(debugger: &Debugger) -> () {
        nix::sys::ptrace::cont(debugger.pid, None)
            .map_err(|errno| internal_utils::errno::exit("Failure to continue debugging", errno))
            .unwrap();

        let _wait_status = sys::wait::waitpid(Some(debugger.pid), None).unwrap();
    }
}
