mod r#break;
mod r#continue;
mod empty;

use crate::debugger::Debugger;

pub trait CMD<'debugger> {
    fn run(&mut self) -> ();
}

pub struct Continue<'debugger>(pub &'debugger Debugger);

pub struct Break<'debugger> {
    debugger: &'debugger mut Debugger,
    addr: i64,
}

/// The puporse of this `Empty` command is to do nothing when encountering certain non-fatal errors.
pub struct Empty();
