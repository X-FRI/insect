use nix::{errno, sys, unistd};

use self::consts::GLOBAL_REGISTER_DESCRIPTORS;

pub mod consts;
pub mod register_descriptor;

#[derive(PartialEq, Copy, Clone)]
pub enum Register {
    RAX,
    RBX,
    RCX,
    RDX,
    RDI,
    RSI,
    RBP,
    RSP,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    RIP,
    EFLAGS,
    CS,
    ORIGRAX,
    FSBASE,
    GSBASE,
    FS,
    GS,
    SS,
    DS,
    ES,
}

#[derive(Copy, Clone)]
pub struct RegisterDescriptor {
    pub register: Register,
    pub dwarf_r: i32,
    pub name: &'static str,
}

impl Register {
    pub fn get_register_value(self, pid: unistd::Pid) -> Result<u64, errno::Errno> {
        let regs = sys::ptrace::getregs(pid)?;

        GLOBAL_REGISTER_DESCRIPTORS
            .iter()
            .find(|reg_desc| reg_desc.register == self)
            .map(
                |current_register_descriptor| match current_register_descriptor.register {
                    Self::RAX => regs.rax,
                    Self::RBX => regs.rbx,
                    Self::RCX => regs.rcx,
                    Self::RDX => regs.rdx,
                    Self::RDI => regs.rdi,
                    Self::RSI => regs.rsi,
                    Self::RBP => regs.rbp,
                    Self::RSP => regs.rsp,
                    Self::R8 => regs.r8,
                    Self::R9 => regs.r9,
                    Self::R10 => regs.r10,
                    Self::R11 => regs.r11,
                    Self::R12 => regs.r12,
                    Self::R13 => regs.r13,
                    Self::R14 => regs.r14,
                    Self::R15 => regs.r15,
                    Self::RIP => regs.rip,
                    Self::EFLAGS => regs.eflags,
                    Self::CS => regs.cs,
                    Self::ORIGRAX => regs.orig_rax,
                    Self::FSBASE => regs.fs_base,
                    Self::GSBASE => regs.gs_base,
                    Self::FS => regs.fs,
                    Self::GS => regs.gs,
                    Self::SS => regs.ss,
                    Self::DS => regs.ds,
                    Self::ES => regs.es,
                },
            )
            .map(|regs_value| Ok(regs_value))
            .unwrap()
    }

    pub fn set_register_value(self, pid: unistd::Pid, value: u64) -> Result<(), errno::Errno> {
        let mut regs = sys::ptrace::getregs(pid)?;
        GLOBAL_REGISTER_DESCRIPTORS
            .iter()
            .find(|reg_desc| reg_desc.register == self)
            .map(
                |current_register_descriptor| match current_register_descriptor.register {
                    Self::RAX => regs.rax = value,
                    Self::RBX => regs.rbx = value,
                    Self::RCX => regs.rcx = value,
                    Self::RDX => regs.rdx = value,
                    Self::RDI => regs.rdi = value,
                    Self::RSI => regs.rsi = value,
                    Self::RBP => regs.rbp = value,
                    Self::RSP => regs.rsp = value,
                    Self::R8 => regs.r8 = value,
                    Self::R9 => regs.r9 = value,
                    Self::R10 => regs.r10 = value,
                    Self::R11 => regs.r11 = value,
                    Self::R12 => regs.r12 = value,
                    Self::R13 => regs.r13 = value,
                    Self::R14 => regs.r14 = value,
                    Self::R15 => regs.r15 = value,
                    Self::RIP => regs.rip = value,
                    Self::EFLAGS => regs.eflags = value,
                    Self::CS => regs.cs = value,
                    Self::ORIGRAX => regs.orig_rax = value,
                    Self::FSBASE => regs.fs_base = value,
                    Self::GSBASE => regs.gs_base = value,
                    Self::FS => regs.fs = value,
                    Self::GS => regs.gs = value,
                    Self::SS => regs.ss = value,
                    Self::DS => regs.ds = value,
                    Self::ES => regs.es = value,
                },
            )
            .map(|_| sys::ptrace::setregs(pid, regs))
            .unwrap()
    }

    pub fn get_register_value_from_dwarf_register(
        pid: unistd::Pid,
        regnum: i32,
    ) -> Result<u64, errno::Errno> {
        GLOBAL_REGISTER_DESCRIPTORS
            .iter()
            .find(|reg_desc| reg_desc.dwarf_r == regnum)
            .map(|reg_desc| reg_desc.register.get_register_value(pid))
            .expect("Unknown dwarf register")
    }

    pub fn get_register_name(self) -> &'static str {
        GLOBAL_REGISTER_DESCRIPTORS
            .iter()
            .find(|reg_desc| reg_desc.register == self)
            .map(|current_register_descriptor| current_register_descriptor.name)
            .unwrap()
    }

    pub fn get_register_from_name(name: &'static str) -> Self {
        GLOBAL_REGISTER_DESCRIPTORS
            .iter()
            .find(|reg_desc| reg_desc.name == name)
            .map(|current_register_descriptor| current_register_descriptor.register)
            .unwrap()
    }
}
