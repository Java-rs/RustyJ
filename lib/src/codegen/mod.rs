mod bytecode;
pub mod ir;
mod stack;
pub use ir::*;

pub fn high_byte(short: u16) -> u8 {
    (short >> 8) as u8
}

pub fn low_byte(short: u16) -> u8 {
    short as u8
}

pub fn shigh_byte(short: i16) -> u8 {
    (short >> 8) as u8
}

pub fn slow_byte(short: i16) -> u8 {
    short as u8
}
