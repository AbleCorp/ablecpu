use std::convert::TryInto;

use instructions::Instruction;

mod instructions;

pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub struct InstructionCache {
    instructions: Box<[(u8, u64, u64)]>,
}

impl InstructionCache {
    fn new(raw: Box<[u8]>) -> InstructionCache {
        let mut i: usize = 0;
        let mut assembled: Box<[(u8, u64, u64)]> = vec![(0, 0, 0); 21845].into_boxed_slice();
        while i<371365 {
            let inst = raw[i];
            let arg_one = u64::from_be_bytes(raw[i+1..=i+8].try_into().unwrap());
            let arg_two = u64::from_be_bytes(raw[i+9..=i+16].try_into().unwrap());
            assembled[i / 17] = (inst, arg_one, arg_two);
            i+=17;
        }

        InstructionCache {
            instructions: assembled,
        }
    }

    pub fn get(&self, index: u64) -> u64 {
        match index % 3 {
            0 => self.instructions[(index/3) as usize].0 as u64,
            1 => self.instructions[(index/3) as usize].1,
            _ => self.instructions[(index/3) as usize].2,
        }
    }

    pub fn set(&mut self, index: u64, value: u64) {
        match index % 3 {
            0 => self.instructions[(index/3) as usize].0 = value as u8,
            1 => self.instructions[(index/3) as usize].1 = value,
            _ => self.instructions[(index/3) as usize].2 = value,
        }
    }
}

pub struct Cpu {
    reg_zero: u64,
    data_cache: [u64; 65535],
    pub instruction_cache: InstructionCache,
    devices: Vec<Box<dyn Device>>
}

impl Cpu {
    pub fn new(instructions: Box<[u8]>) -> Cpu{
        Cpu{
            reg_zero: 65536,
            data_cache: [0; 65535],
            instruction_cache: InstructionCache::new(instructions),
            devices: Vec::new(),
        }
    }

    pub fn tick(&self) {

    }

    fn get_instruction(&self, index: u64) -> Instruction {
        
    }
}

mod tests {
    #[test]
    fn it_works() {
        let mut cstm_vec: Vec<u8> = vec![1, 0, 0, 0, 0, 0, 0, 0, 8];
        let mut fill_vec: Vec<u8> = vec![0; 371356];
        cstm_vec.append(&mut fill_vec);

        let test_cpu = super::Cpu::new(cstm_vec.into_boxed_slice());
        println!("{:?}", test_cpu.instruction_cache.instructions[0]);
    }
}

pub trait Device {

}