use super::Register;
use super::RegisterDescriptor;

pub const REGISTERS_NUM: usize = 27;

pub const GLOBAL_REGISTER_DESCRIPTORS: [RegisterDescriptor; REGISTERS_NUM] = [
    RegisterDescriptor {
        register: Register::R15,
        dwarf_r: 15,
        name: "r15",
    },
    RegisterDescriptor {
        register: Register::R14,
        dwarf_r: 14,
        name: "r14",
    },
    RegisterDescriptor {
        register: Register::R13,
        dwarf_r: 13,
        name: "r13",
    },
    RegisterDescriptor {
        register: Register::R12,
        dwarf_r: 12,
        name: "r12",
    },
    RegisterDescriptor {
        register: Register::RBP,
        dwarf_r: 6,
        name: "rbp",
    },
    RegisterDescriptor {
        register: Register::RBX,
        dwarf_r: 3,
        name: "rbx",
    },
    RegisterDescriptor {
        register: Register::R11,
        dwarf_r: 11,
        name: "r11",
    },
    RegisterDescriptor {
        register: Register::R10,
        dwarf_r: 10,
        name: "r10",
    },
    RegisterDescriptor {
        register: Register::R9,
        dwarf_r: 9,
        name: "r9",
    },
    RegisterDescriptor {
        register: Register::R8,
        dwarf_r: 8,
        name: "r8",
    },
    RegisterDescriptor {
        register: Register::RAX,
        dwarf_r: 0,
        name: "rax",
    },
    RegisterDescriptor {
        register: Register::RCX,
        dwarf_r: 2,
        name: "rcx",
    },
    RegisterDescriptor {
        register: Register::RDX,
        dwarf_r: 1,
        name: "rdx",
    },
    RegisterDescriptor {
        register: Register::RSI,
        dwarf_r: 4,
        name: "rsi",
    },
    RegisterDescriptor {
        register: Register::RDI,
        dwarf_r: 5,
        name: "rdi",
    },
    RegisterDescriptor {
        register: Register::ORIGRAX,
        dwarf_r: -1,
        name: "orig_rax",
    },
    RegisterDescriptor {
        register: Register::RIP,
        dwarf_r: -1,
        name: "rip",
    },
    RegisterDescriptor {
        register: Register::CS,
        dwarf_r: 51,
        name: "cs",
    },
    RegisterDescriptor {
        register: Register::EFLAGS,
        dwarf_r: 49,
        name: "eflags",
    },
    RegisterDescriptor {
        register: Register::RSP,
        dwarf_r: 7,
        name: "rsp",
    },
    RegisterDescriptor {
        register: Register::SS,
        dwarf_r: 52,
        name: "ss",
    },
    RegisterDescriptor {
        register: Register::FSBASE,
        dwarf_r: 58,
        name: "fs_base",
    },
    RegisterDescriptor {
        register: Register::GSBASE,
        dwarf_r: 59,
        name: "gs_base",
    },
    RegisterDescriptor {
        register: Register::DS,
        dwarf_r: 53,
        name: "ds",
    },
    RegisterDescriptor {
        register: Register::ES,
        dwarf_r: 50,
        name: "es",
    },
    RegisterDescriptor {
        register: Register::FS,
        dwarf_r: 54,
        name: "fs",
    },
    RegisterDescriptor {
        register: Register::GS,
        dwarf_r: 55,
        name: "gs",
    },
];
