use crate::instructions::Instruction;

pub struct CpuState {
    pub a: u64,
    pub b: u64,
    pub s: u64,
    pub x: u64,
    pub cache: Vec<u64>,
    pub upcoming: Instruction
}