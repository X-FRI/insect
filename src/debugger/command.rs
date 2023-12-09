use super::Debugger;
use nix::sys;

trait Command {
    fn exec(self);
}

pub struct Continue(Debugger);

impl Command for Debugger {
    fn exec(self) {
        nix::sys::ptrace::cont(self.pid, None).unwrap();
        let wait_status = sys::wait::waitpid(Some(self.pid), None).unwrap();
    }
}
