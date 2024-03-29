mod consts;
mod register_descriptor;

enum Register {
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
    RFLAGS,
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

pub(self) struct RegisterDescriptor {
    register: Register,
    dwarf_r: i32,
    name: String,
}
