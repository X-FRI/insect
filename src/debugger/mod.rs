use std::{collections::HashMap, path::PathBuf};

use linenoise;
use nix::{
    sys,
    unistd::{self, Pid},
};

use crate::breakpoint::Breakpoint;

mod command;

pub struct Debugger {
    pub program: PathBuf,
    pub pid: unistd::Pid,
    pub breakpoints: HashMap<i64, Breakpoint>,
}

impl Debugger {
    pub fn new(program: PathBuf, pid: Pid) -> Self {
        Debugger {
            program,
            pid,
            breakpoints: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        let _wait_status = sys::wait::waitpid(Some(self.pid), None).unwrap();

        loop {
            match linenoise::input("\ninsect> ") {
                None => break,
                Some(input) => {
                    self.command_handler(&input);
                    linenoise::history_add(input.as_str());
                }
            }
        }
    }

    pub fn command_handler(&mut self, input: &String) {
        command::Command::new(input.clone(), self).parse().run();
    }

    pub fn set_breakpoint_at(&mut self, addr: i64) -> () {
        info!("Set breakpoint at {:#02x}", addr);
        let mut breakpoint = Breakpoint::new(self.pid, addr);
        breakpoint.enable();
        self.breakpoints.insert(addr, breakpoint);
    }
}
