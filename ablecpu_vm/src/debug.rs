use crate::instructions::Instruction;

pub struct CpuState {
    pub subscriber: Vec<u64>,
    pub subscriber_results: Vec<u64>,
    pub upcoming: Option<Instruction>
}