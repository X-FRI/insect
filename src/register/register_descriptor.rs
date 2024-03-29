use super::{Register, RegisterDescriptor};

impl RegisterDescriptor {
    pub fn new(register: Register, dwarf_r: i32, name: &str) -> Self {
        Self {
            register,
            dwarf_r,
            name: name.to_string(),
        }
    }
}
