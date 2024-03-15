use linenoise;
use nix::{
    libc, sys,
    unistd::{self, Pid},
};

use crate::debugger::command::{Command, Continue};

mod command;

pub struct Debugger<'program> {
    program: &'program str,
    pid: unistd::Pid,
}

impl<'program> Debugger<'program> {
    pub fn new(program: &'program str, pid: Pid) -> Self {
        Debugger { program, pid }
    }

    pub fn run(&self) {
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

    pub fn command_handler(&self, input: &String) {
        let args = input.split(" ").collect::<Vec<&str>>();
        let command = args[0];

        if command.starts_with("continue") {
            info!("Continue debugging...");
            Continue::exec(&self)
        }
    }
}
