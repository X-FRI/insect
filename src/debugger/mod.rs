use crate::register;
use crate::utils as internal_utils;
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
                None => {
                    println!("bye.");
                    break;
                }
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

    pub fn dump_registers(self) {
        register::consts::GLOBAL_REGISTER_DESCRIPTORS
            .iter()
            .for_each(|reg_desc| {
                info!(
                    "{} = 0x{:x}",
                    reg_desc.name,
                    internal_utils::result::unwrap(
                        reg_desc.register.get_register_value(self.pid),
                        format!("Cannot get the register {} value", reg_desc.name).as_str()
                    )
                )
            })
    }
}
