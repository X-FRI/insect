use crate::cmd::{Break, Continue, Empty, CMD};

use super::Debugger;
use crate::utils as internal_utils;
use std::str;

pub struct Command<'debugger> {
    debugger: &'debugger mut Debugger,
    command: String,
    args: Vec<String>,
}

impl<'debugger> Command<'debugger> {
    pub fn new(command: String, debugger: &'debugger mut Debugger) -> Command<'debugger> {
        let args = command
            .split(" ")
            .map(String::from)
            .collect::<Vec<String>>();

        Command {
            command: args[0].to_string(),
            args,
            debugger,
        }
    }

    pub fn parse(self) -> Box<dyn CMD<'debugger> + 'debugger> {
        if self.command.starts_with("continue") {
            info!("Continue debugging...");
            Box::new(Continue::new(self.debugger))
        } else if self.command.starts_with("break") {
            match i64::from_str_radix(
                self.args[1]
                    .as_str()
                    .strip_prefix("0x")
                    .unwrap_or_else(|| self.args[1].as_str()),
                16,
            ) {
                Err(_) => {
                    error!("Invalid address format: {}", self.args[1].clone());
                    Box::new(Empty::new())
                }
                Ok(addr) => Box::new(Break::new(self.debugger, addr)),
            }
        } else {
            panic!("Unknown command {}", self.command)
        }
    }
}
