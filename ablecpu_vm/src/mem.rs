use crate::{Device, errors::CPUError};

pub struct InstructionCache {
    instructions: [u8; 196605]
}

impl InstructionCache {
    pub fn new(source: [u8; 196605]) -> InstructionCache {
        InstructionCache {
            instructions: source
        }
    }
}

impl Device for InstructionCache {
    fn get_address_space(&self) -> (u64, u64) {
        (65537, 131071)
    }

    fn load(&mut self, address: u64) -> Result<u64, CPUError> {
        let index = address - 65537;
        if index % 3 == 0 {
            Ok(*self.instructions.get(index as usize).unwrap_or(return Err(CPUError::OutOfInstructions)) as u64)
        } else if index +1 % 3 == 0 {
            self
        }
    }

    fn push(&mut self, address: u64, value:u64) {
        todo!()
    }
}

struct Memory {
    memory: Vec<u64>
}

impl Memory {
    fn new() -> Memory {
        Memory {
            memory: vec![]
        }
    }
}

impl Device for Memory {
    fn get_address_space(&self) -> (u64, u64) {
        (131072, 131072)
    }

    fn load(&mut self, address: u64) -> u64 {
        todo!()
    }

    fn push(&mut self, address: u64, value: u64) {
        todo!()
    }
}