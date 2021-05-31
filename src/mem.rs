use crate::Device;

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
        todo!()
    }

    fn load(&self, address: u64) -> u64 {
        todo!()
    }

    fn push(&self, address: u64, value: u64) {
        todo!()
    }
}