use super::{Empty, CMD};

impl<'debugger> CMD<'debugger> for Empty {
    fn run(&mut self) -> () {}
}

impl Empty {
    pub fn new() -> Empty {
        Empty()
    }
}
