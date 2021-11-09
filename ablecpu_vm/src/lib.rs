use std::convert::TryInto;

mod instructions;

pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

struct InstructionCache {
    instructions: [(u8, u64, u64); 21845],
}

impl InstructionCache {
    fn new(raw: [u8; 371365]) -> InstructionCache {
        let mut i: usize = 0;
        let mut assembled: [(u8, u64, u64); 21845] = [(0, 0, 0); 21845];
        while i<371365 {
            let inst = raw[i];
            let arg_one = u64::from_be_bytes(raw[i+1..=i+8].try_into().unwrap());
            let arg_two = u64::from_be_bytes(raw[i+9..=i+16].try_into().unwrap());
            assembled[i / 17] = (inst, arg_one, arg_two);
        }

        InstructionCache {
            instructions: assembled,
        }
    }
}

pub struct Cpu {
    reg_zero: u64,
    data_cache: [u64; 65535],
    instruction_cache: InstructionCache,
    devices: Vec<Box<dyn Device>>
}

impl Cpu {
    pub fn new(instructions: [u8; 371365]) -> Cpu{
        Cpu{
            reg_zero: 65536,
            data_cache: [0; 65535],
            instruction_cache: InstructionCache::new(instructions),
            devices: Vec::new(),
        }
    }
}

pub trait Device {

}