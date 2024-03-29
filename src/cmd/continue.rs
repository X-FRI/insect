use crate::debugger::Debugger;

use crate::utils as internal_utils;
use nix::sys;

use super::{Continue, CMD};

impl<'debugger> CMD<'debugger> for Continue<'debugger> {
    fn run(&mut self) -> () {
        nix::sys::ptrace::cont(self.0.pid, None)
            .map_err(|errno| internal_utils::errno::error("Failure to continue debugging", errno))
            .unwrap();

        let _wait_status = sys::wait::waitpid(Some(self.0.pid), None).unwrap();
    }
}

impl<'debugger> Continue<'debugger> {
    pub fn new(debugger: &Debugger) -> Self {
        Self(debugger)
    }
}
