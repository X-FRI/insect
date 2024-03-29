use super::Register;
use super::RegisterDescriptor;

pub const REGISTERS_NUM: u8 = 27;

pub const GLOBAL_REGISTER_DESCRIPTORS: [Register; REGISTERS_NUM] = [
    RegisterDescriptor::new(Register::R15, 15, "r15"),
    RegisterDescriptor::new(Register::r14, 14, "r14"),
    RegisterDescriptor::new(Register::r13, 13, "r13"),
    RegisterDescriptor::new(Register::r12, 12, "r12"),
    RegisterDescriptor::new(Register::rbp, 6, "rbp"),
    RegisterDescriptor::new(Register::rbx, 3, "rbx"),
    RegisterDescriptor::new(Register::r11, 11, "r11"),
    RegisterDescriptor::new(Register::r10, 10, "r10"),
    RegisterDescriptor::new(Register::r9, 9, "r9"),
    RegisterDescriptor::new(Register::r8, 8, "r8"),
    RegisterDescriptor::new(Register::rax, 0, "rax"),
    RegisterDescriptor::new(Register::rcx, 2, "rcx"),
    RegisterDescriptor::new(Register::rdx, 1, "rdx"),
    RegisterDescriptor::new(Register::rsi, 4, "rsi"),
    RegisterDescriptor::new(Register::rdi, 5, "rdi"),
    RegisterDescriptor::new(Register::orig_rax, -1, "orig_rax"),
    RegisterDescriptor::new(Register::rip, -1, "rip"),
    RegisterDescriptor::new(Register::cs, 51, "cs"),
    RegisterDescriptor::new(Register::rflags, 49, "eflags"),
    RegisterDescriptor::new(Register::rsp, 7, "rsp"),
    RegisterDescriptor::new(Register::ss, 52, "ss"),
    RegisterDescriptor::new(Register::fs_base, 58, "fs_base"),
    RegisterDescriptor::new(Register::gs_base, 59, "gs_base"),
    RegisterDescriptor::new(Register::ds, 53, "ds"),
    RegisterDescriptor::new(Register::es, 50, "es"),
    RegisterDescriptor::new(Register::fs, 54, "fs"),
    RegisterDescriptor::new(Register::gs, 55, "gs"),
];
