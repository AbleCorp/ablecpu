use std::convert::TryInto;

mod instructions;

pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub struct InstructionCache {
    pub instructions: Box<[(u8, u64, u64)]>,
}

impl InstructionCache {
    fn new(raw: Box<[u8]>) -> InstructionCache {
        println!("This still worked! 3");
        let mut i: usize = 0;
        let mut assembled: Box<[(u8, u64, u64)]> = vec![(0, 0, 0); 21845].into_boxed_slice();
        while i<371365 {
            let inst = raw[i];
            let arg_one = u64::from_be_bytes(raw[i+1..=i+8].try_into().unwrap());
            let arg_two = u64::from_be_bytes(raw[i+9..=i+16].try_into().unwrap());
            assembled[i / 17] = (inst, arg_one, arg_two);
            i+=17;
        }

        println!("This still worked! 4");

        InstructionCache {
            instructions: assembled,
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
        println!("This still worked! 2");
        Cpu{
            reg_zero: 65536,
            data_cache: [0; 65535],
            instruction_cache: InstructionCache::new(instructions),
            devices: Vec::new(),
        }
    }
}

mod tests {
    #[test]
    fn it_works() {
        println!("This still worked! 1");
        let test_cpu = super::Cpu::new(vec![0; 371365].into_boxed_slice());
        

        println!("{:?}", test_cpu.instruction_cache.instructions[0]);
    }
}

pub trait Device {

}