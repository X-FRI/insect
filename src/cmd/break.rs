use crate::debugger::Debugger;

use super::{Break, CMD};

impl<'debugger> CMD<'debugger> for Break<'debugger> {
    fn run(&mut self) -> () {
        self.debugger.set_breakpoint_at(self.addr)
    }
}

impl<'debugger> Break<'debugger> {
    pub fn new(debugger: &'debugger mut Debugger, addr: i64) -> Break {
        Break { debugger, addr }
    }
}
