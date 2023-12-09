use linenoise;
use nix::{libc, sys, unistd};

mod command;

struct Debugger {
    program: String,
    pid: unistd::Pid,
}

impl Debugger {
    pub fn run(&self) {
        let wait_status = sys::wait::waitpid(Some(self.pid), None).unwrap();

        loop {
            match linenoise::input("insect> ") {
                None => break,
                Some(input) => {
                    self.command_handler(&input);
                    linenoise::history_add(input.as_str());
                }
            }
        }
    }

    pub fn new(program: String, raw_pid: libc::pid_t) -> Self {
        Debugger {
            program,
            pid: unistd::Pid::from_raw(raw_pid),
        }
    }

    pub fn command_handler(&self, input: &String) {}
}
