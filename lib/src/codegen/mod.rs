mod bytecode;
pub mod ir;
mod reljumps;
mod stack;
pub use ir::*;

pub fn high_byte(short: u16) -> u8 {
    (short >> 8) as u8
}

pub fn low_byte(short: u16) -> u8 {
    short as u8
}
