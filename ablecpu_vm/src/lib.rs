use std::convert::TryInto;

pub use crate::debug::CpuState;
use crate::errors::{CPUError};
use crate::instructions::{Instruction};
use crate::mem::InstructionCache;

mod debug;
mod errors;
mod instructions;
mod mem;

pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub struct Cpu {
    reg_x: u64,
    ect: [u64; 32],
    cache: [u64; 65504],
    instructions: InstructionCache,
    devices: Vec<Box<dyn Device>>,
}

pub trait Device {
    fn get_address_space(&self) -> (u64, u64);
    fn load(&mut self, address: u64) -> Result<u64, CPUError>;
    fn push(&mut self, address: u64, value:u64) -> Result<(), CPUError>;
}

impl Cpu {
    pub fn new(ect: Option<[u64; 32]>, cache: Option<[u64; 65504]>, instructions: [u8; 196605], devices: Vec<Box<dyn Device>>) -> Cpu {
        Cpu {
            reg_x: 0,
            ect: match ect {
                Some(ect) => ect,
                None => [0; 32],
            },
            cache: match cache {
                Some(cache) => cache,
                None => [0; 65504],
            },
            instructions: InstructionCache::new(instructions),
            devices,
        }
    }

    pub fn tick(&mut self) -> Result<(), CPUError> {
        let inst = self.read_instruction(self.reg_x);
        Ok(())
    }

    fn read_instruction(&self, reg_x: u64) -> Result<(), CPUError> {
        match self.instructions {
            
        }
        Ok(())
    }
}
