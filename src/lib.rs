mod debug;
mod errors;
mod instructions;
mod mem;

use std::convert::TryInto;
use crate::instructions::{Instruction, Instruction::*};
use crate::errors::{CPUError, CPUError::*};
pub use crate::debug::CpuState;

pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub struct Cpu {
    reg_a: u64,
    reg_b: u64,
    reg_s: u64,
    reg_x: u64,
    cache: [u64; 65535],
    instructions: [u8; 65535],
    devices: Vec<Box<dyn Device>>
}

pub trait Device {
    fn get_address_space(&self) -> (u64, u64);
    fn load(&self, address: u64) -> u64;
    fn push(&self, address: u64, value:u64);
}

impl Cpu {
    pub fn new(instructions: [u8; 65535], devices: Vec<Box<dyn Device>>) -> Cpu {
        Cpu {
            reg_a: 0,
            reg_b: 0,
            reg_s: 0,
            reg_x: 0,
            cache: [0; 65535],
            instructions,
            devices
        }
    }

    pub fn debug(&self) -> CpuState {
        CpuState {
            a: self.reg_a,
            b: self.reg_b,
            s: self.reg_s,
            x: self.reg_x,
            cache: self.cache.to_vec(),
            upcoming: self.read_instruction().unwrap()
        }
    }

    pub fn tick(&mut self) -> Result<(), CPUError>{
        self.process_instruction(self.read_instruction()?)?;
        self.reg_x += 1;
        Ok(())
    }
    fn read_instruction(&self) -> Result<Instruction, CPUError>{
        match self.instructions.get((self.reg_x)as usize) {
            None => Err(OutOfInstructions(format!("Out of instructions at position {}", self.reg_x))),
            Some(i) => {
                match i {
                    0 => Ok(NoOp),
                    1 => Ok(LoadBusA(self.get_args(self.reg_x)?)),
                    2 => Ok(LoadBusB(self.get_args(self.reg_x)?)),
                    3 => Ok(Add),
                    4 => Ok(Subtract),
                    5 => Ok(Multiply),
                    6 => Ok(Divide),
                    7 => Ok(CopyAB),
                    8 => Ok(CopyBA),
                    9 => Ok(SwapAB),
                    10 => Ok(PushABus(self.get_args(self.reg_x)?)),
                    11 => Ok(PushBBus(self.get_args(self.reg_x)?)),
                    12 => Ok(LoadA(self.get_args(self.reg_x)?)),
                    13 => Ok(LoadBusX(self.get_args(self.reg_x)?)),
                    14 => Ok(CopyAX),
                    15 => Ok(CopyBX),
                    16 => Ok(PushXBus(self.get_args(self.reg_x)?)),
                    17 => Ok(LoadX(self.get_args(self.reg_x)?)),
                    18 => Ok(CopyXA),
                    19 => Ok(CopyXB),
                    20 => Ok(LoadBusAS),
                    21 => Ok(LoadBusBS),
                    22 => Ok(CopyAS),
                    23 => Ok(CopyBS),
                    24 => Ok(CopyXS),
                    25 => Ok(CopySA),
                    26 => Ok(CopySB),
                    27 => Ok(CopySX),
                    28 => Ok(SwapAS),
                    29 => Ok(SwapBS),
                    30 => Ok(PushABusS),
                    31 => Ok(PushBBusS),
                    32 => Ok(LoadBusXS),
                    33 => Ok(PushXBusS),
                    34 => Ok(SkipEq),
                    35 => Ok(SkipGrEq),
                    36 => Ok(SkipGr),
                    37 => Ok(SkipLe),
                    38 => Ok(SkipLeEq),
                    e => Err(IllegalInstruction(format!("{} is not a valid instruction", e)))
                }
            }
        }
    }
    fn get_args(&self, start: u64) -> Result<u64, CPUError> {
        print!("{}", self.reg_x);
        println!("{:?}", &self.instructions[(start+1) as usize..(start+9) as usize]);
        Ok(u64::from_be_bytes(self.instructions[(start+1) as usize..(start+9) as usize].try_into().unwrap()))

    }
    fn process_instruction(&mut self, inst: Instruction) -> Result<(), CPUError> {
        match inst {
            NoOp => {Ok(())}
            LoadBusA(arg) => {
                self.reg_a = self.load_base(arg)?;
                self.arg_skip();
                Ok(())
            }
            LoadBusB(arg) => {
                self.reg_b = self.load_base(arg)?;
                self.arg_skip();
                Ok(())
            }
            Add => {
                self.reg_a += self.reg_b;
                Ok(())
            }
            Subtract => {
                self.reg_a -= self.reg_b;
                Ok(())
            }
            Multiply => {
                self.reg_a *= self.reg_b;
                Ok(())
            }
            Divide => {
                self.reg_a /= self.reg_b;
                Ok(())
            }
            CopyAB => {
                self.reg_b = self.reg_a;
                Ok(())
            }
            CopyBA => {
                self.reg_a = self.reg_b;
                Ok(())
            }
            SwapAB => {
                std::mem::swap(&mut self.reg_a, &mut self.reg_b);
                Ok(())
            }
            PushABus(arg) => {
                self.push_base(arg, self.reg_a)?;
                self.arg_skip();
                Ok(())
            }
            PushBBus(arg) => {
                self.push_base(arg, self.reg_b)?;
                self.arg_skip();
                Ok(())
            }
            LoadA(arg) => {
                self.reg_a = arg;
                self.arg_skip();
                Ok(())
            }
            LoadB(arg) => {
                self.reg_b = arg;
                self.arg_skip();
                Ok(())
            }
            LoadBusX(arg) => {
                self.reg_x = self.load_base(arg)?;
                self.arg_skip();
                Ok(())
            }
            CopyAX => {
                self.reg_x = self.reg_a;
                Ok(())
            }
            CopyBX => {
                self.reg_x = self.reg_b;
                Ok(())
            }
            PushXBus(arg) => {
                self.push_base(arg, self.reg_x)?;
                self.arg_skip();
                Ok(())
            }
            LoadX(arg) => {
                self.reg_x = arg;
                self.arg_skip();
                Ok(())
            }
            CopyXA => {
                self.reg_a = self.reg_x;
                Ok(())
            }
            CopyXB => {
                self.reg_b = self.reg_x;
                Ok(())
            }
            LoadBusAS => {
                self.reg_a = self.load_base(self.reg_s)?;
                Ok(())
            }
            LoadBusBS => {
                self.reg_b = self.load_base(self.reg_s)?;
                Ok(())
            }
            CopyAS => {
                self.reg_s = self.reg_a;
                Ok(())
            }
            CopyBS => {
                self.reg_s = self.reg_b;
                Ok(())
            }
            CopyXS => {
                self.reg_s = self.reg_x;
                Ok(())
            }
            CopySA => {
                self.reg_a = self.reg_s;
                Ok(())
            }
            CopySB => {
                self.reg_b = self.reg_s;
                Ok(())
            }
            CopySX => {
                self.reg_x = self.reg_s;
                Ok(())
            }
            SwapAS => {
                std::mem::swap(&mut self.reg_a, &mut self.reg_s);
                Ok(())
            }
            SwapBS => {
                std::mem::swap(&mut self.reg_b, &mut self.reg_s);
                Ok(())
            }
            PushABusS => {
                self.push_base(self.reg_s, self.reg_a)?;
                Ok(())
            }
            PushBBusS => {
                self.push_base(self.reg_s, self.reg_b)?;
                Ok(())
            }
            LoadBusXS => {
                self.reg_x = self.load_base(self.reg_s)?;
                Ok(())
            }
            PushXBusS => {
                self.push_base(self.reg_s, self.reg_x)?;
                Ok(())
            }
            SkipEq => {
                if self.reg_a == self.reg_b {
                    self.reg_x += 1;
                }
                Ok(())
            }
            SkipGrEq => {
                if self.reg_a >= self.reg_b {
                    self.reg_x += 1;
                }
                Ok(())
            }
            SkipGr => {
                if self.reg_a > self.reg_b {
                    self.reg_x += 1;
                }
                Ok(())
            }
            SkipLe => {
                if self.reg_a < self.reg_b {
                    self.reg_x += 1;
                }
                Ok(())
            }
            SkipLeEq => {
                if self.reg_a <= self.reg_b {
                    self.reg_x += 1;
                }
                Ok(())
            }
        }
    }

    fn load_base(&self, arg: u64) -> Result<u64, CPUError> {
        match arg {
            0..=65535 => {
                Ok(self.cache[arg as usize])
            }
            65536..=131071 => {
                Ok(self.instructions[arg as usize] as u64)
            }
            _ => {
                let mut success = None;
                for device in &self.devices {
                    let (min, max) = device.get_address_space();
                    if (min..=max).contains(&arg) {
                        success = Some(device.load(arg));
                    }
                }
                match success {
                    Some(_) => Ok(success.unwrap()),
                    _ => Err(IllegalAddressLoad(format!("{} is not a populated address", arg)))
                }
            }
        }
    }

    fn push_base(&mut self, arg: u64, val: u64) -> Result<(), CPUError> {
        match arg {
            0..=65535 => {
                self.cache[arg as usize] = val;
                Ok(())
            }
            65536..=131071 => {
                self.instructions[arg as usize] = val as u8;
                Ok(())
            }
            _ => {
                let mut success= false;
                for device in &self.devices {
                    let (min, max) = device.get_address_space();
                    if (min..=max).contains(&arg) {
                        success = true;
                        device.push(arg, val)
                    }
                }
                if success {
                    Ok(())
                }
                else {
                    Err(IllegalAddressPush(format!("{} is not a populated address", arg)))
                }
            }
        }
    }

    fn arg_skip(&mut self) {
        self.reg_x += 7;
    }
}
