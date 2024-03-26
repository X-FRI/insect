use nix::{libc, sys, unistd};

use crate::utils as internal_utils;

pub struct Breakpoint {
    pid: unistd::Pid,
    addr: i64,
    enabled: bool,
    saved_data: i64,
}

impl Breakpoint {
    pub fn new(pid: unistd::Pid, addr: i64) -> Self {
        Breakpoint {
            pid,
            addr,
            enabled: false,
            saved_data: 0x00,
        }
    }

    pub fn enable(&mut self) -> () {
        match sys::ptrace::read(self.pid, self.addr as *mut libc::c_void) {
            Err(errno) => internal_utils::errno::error(
                format!(
                    "Failed to enable the breakpoint, unable to read address: {:#02x}",
                    self.addr
                )
                .as_str(),
                errno,
            ),
            Ok(data) => {
                self.saved_data = data & 0xff;

                unsafe {
                    sys::ptrace::write(
                        self.pid,
                        self.addr as *mut libc::c_void,
                        ((data & (!0xff)) | 0xcc) as *mut libc::c_void,
                    )
                    .map_err(|errno| {
                        internal_utils::errno::error(
                            format!(
                                "Failed to enable the breakpoint, unabled to write address: {:#02x}",
                                self.addr
                            )
                            .as_str(),
                            errno,
                        )
                    })
                    .unwrap();
                }

                self.enabled = true
            }
        }
    }

    pub fn disable(&mut self) -> () {
        let data = sys::ptrace::read(self.pid, self.addr as *mut libc::c_void)
            .map_err(|errno| {
                internal_utils::errno::error(
                    format!(
                        "Failed to disbale breakpoint, unable to read address: {:#02x}",
                        self.addr
                    )
                    .as_str(),
                    errno,
                )
            })
            .unwrap();

        unsafe {
            sys::ptrace::write(
                self.pid,
                self.addr as *mut libc::c_void,
                ((data & (!0xff)) | self.saved_data) as *mut libc::c_void,
            )
            .map_err(|errno| {
                internal_utils::errno::error(
                    format!(
                        "Failed to disbale breakpoint, unable to write address: {:#02x}",
                        self.addr
                    )
                    .as_str(),
                    errno,
                )
            })
            .unwrap();
        }

        self.enabled = false
    }
}
