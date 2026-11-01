pub type Register = u16;

pub const PROGRAM_COUNTER: u16 = 7;
pub const CMP_REGISTER: u16 = 8;

#[derive(Default)]
pub struct Registers {
    pub registers: [u16; 7],
    pub program_counter: u16,
    pub cmp_register: u16,
}

pub fn read_register(registers: &Registers, register: u16) -> u16 {
    match register {
        0..7 => registers.registers[register as usize],
        PROGRAM_COUNTER => registers.program_counter,
        CMP_REGISTER => registers.cmp_register,
        _ => 0,
    }
}

pub fn store_register(registers: &mut Registers, register: u16, value: u16) {
    match register {
        // start from 1 because can't write to register 0
        1..7 => registers.registers[register as usize] = value,
        PROGRAM_COUNTER => registers.program_counter = value,
        CMP_REGISTER => registers.cmp_register = value,
        _ => {}
    }
}
