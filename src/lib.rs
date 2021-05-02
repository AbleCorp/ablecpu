use crate::Instruction::{NoOp, LoadBusA};
use crate::CPUError::{IllegalInstruction, OutOfInstructions};

#[cfg(test)]
mod tests {
    use std::time::Instant;

    #[test]
    fn it_works() {
        use crate::*;
        let mut cpu = CPU::new([0; 65535], vec![]);
        let now = Instant::now();
        loop {
            match cpu.tick() {
            Ok(_) => continue, // println!("TICKED!"),
            Err(e) => {
                eprintln!("{:?}", e);
                break;
            }
            }
        }
        println!("{}", now.elapsed().as_secs_f64())
    }
}

pub struct CPU {
    a: u64,
    b: u64,
    s: u64,
    x: u64,
    cache: [u64; 65535],
    instructions: [u8; 65535],
    memory: Vec<u64>,
    devices: Vec<Box<dyn Device>>
}

pub trait Device {
    fn get_address_space(&self) -> (u64, u64);
    fn load(&self, address: u64) -> u64;
    fn push(&self, address: u64, value:u64);
}

enum Instruction {
    NoOp,
    LoadBusA(u64),
    LoadBusB(u64),
    Add,
    Subtract,
    Multiply,
    Divide,
    CopyAB,
    CopyBA,
    SwapAB,
    PushABus(u64),
    PushBBus(u64),
    LoadA(u64),
    LoadB(u64),
    LoadBusX(u64),
    CopyAX,
    CopyBX,
    PushXBus(u64),
    LoadX(u64),
    CopyXA,
    CopyXB,
    LoadBusAS,
    LoadBusBS,
    CopyAS,
    CopyBS,
    CopyXS,
    CopySA,
    CopySB,
    CopySX,
    SwapAS,
    SwapBS,
    PushABusS,
    PushBBusS,
    LoadBusXS,
    PushXBusS,
    SkipEq,
    SkipGrEq,
    SkipGr,
    SkipLe,
    SkipLeEq
}

#[derive(Debug)]
pub enum CPUError{
    IllegalInstruction(String),
    OutOfInstructions(String)
}

impl CPU {
    pub fn new(instructions: [u8; 65535], devices: Vec<Box<dyn Device>>) -> CPU {
        CPU {
            a: 0,
            b: 0,
            s: 0,
            x: 0,
            cache: [0; 65535],
            instructions,
            memory: vec![],
            devices
        }
    }

    pub fn tick(&mut self) -> Result<(), CPUError>{
        self.x += 1;
        self.process_instruction(self.read_instruction()?)
    }
    fn read_instruction(&self) -> Result<Instruction, CPUError>{
        match self.instructions.get((self.x - 1 )as usize) {
            None => Err(OutOfInstructions(format!("Out of instructions at position {}", self.x))),
            Some(i) => {
                match i {
                    0 => Ok(NoOp),
                    1 => Ok(LoadBusA(5)),
                    e => Err(IllegalInstruction(format!("{} is not a valid instruction", e)))
                }
            }
        }
    }
    fn process_instruction(&self, inst: Instruction) -> Result<(), CPUError> {
        Ok(())
    }
}
