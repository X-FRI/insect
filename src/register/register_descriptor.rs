use super::{Register, RegisterDescriptor};

impl RegisterDescriptor {
    pub fn new(register: Register, dwarf_r: i32, name: &'static str) -> Self {
        Self {
            register,
            dwarf_r,
            name,
        }
    }
}
