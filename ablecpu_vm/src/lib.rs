use std::{convert::TryInto, ops::RangeInclusive};

use arch::Arch;
use errors::CpuError;
use instructions::Instruction;
use num::Num;

mod arch;
pub mod errors;
mod instructions;

pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub struct InstructionCache<T: Arch + Num> {
    instructions: Box<[(u8, T, T)]>,
}

impl<T: Arch + Num> InstructionCache<T> {
    fn new(raw: Box<[u8]>) -> InstructionCache<T> {
        let mut i: usize = 0;
        let mut assembled: Box<[(u8, T, T)]> = vec![(0, 0, 0); 21845].into_boxed_slice();
        while i < 371365 {
            let inst = raw[i];
            let arg_one = T::from_be_bytes(raw[i + 1..=i + 8].try_into().unwrap());
            let arg_two = u64::from_be_bytes(raw[i + 9..=i + 16].try_into().unwrap());
            assembled[i / 17] = (inst, arg_one, arg_two);
            i += 17;
        }

        InstructionCache {
            instructions: assembled,
        }
    }

    pub fn get(&self, index: u64) -> u64 {
        match index % 3 {
            0 => self.instructions[(index / 3) as usize].0 as u64,
            1 => self.instructions[(index / 3) as usize].1,
            _ => self.instructions[(index / 3) as usize].2,
        }
    }

    pub fn set(&mut self, index: u64, value: u64) {
        match index % 3 {
            0 => self.instructions[(index / 3) as usize].0 = value as u8,
            1 => self.instructions[(index / 3) as usize].1 = value,
            _ => self.instructions[(index / 3) as usize].2 = value,
        }
    }
}

pub struct Cpu<T: Arch + Num> {
    reg_zero: T,
    data_cache: [T; 65535],
    pub instruction_cache: InstructionCache<T>,
    devices: Vec<Box<dyn Device>>,
}

impl<T: Arch + Num> Cpu<T> {
    pub fn new(instructions: Box<[u8]>) -> Cpu<T> {
        Cpu {
            reg_zero: 65536,
            data_cache: [0; 65535],
            instruction_cache: InstructionCache::new(instructions),
            devices: Vec::new(),
        }
    }

    pub fn tick(&mut self) -> Result<(), CpuError> {
        let instruction = self.get_instruction(self.reg_zero)?;

        match instruction {
            Instruction::NoOp(_, _, _, _, _, _) => {}
            Instruction::Load(arg1, arg2, ignore_errors, no_halt_if_error, no_debug_info, _) => {
                match self.push(arg1, arg2) {
                    Ok(_) => {}
                    Err(e) => {
                        self.handle_error(e, ignore_errors, no_halt_if_error, no_debug_info)?;
                    }
                }
            }
            Instruction::Copy(arg1, arg2, ignore_errors, no_halt_if_error, no_debug_info, _) => {
                let mut value: T = 0;

                match self.load(arg1) {
                    Ok(v) => {
                        value = v;
                    }
                    Err(e) => {
                        self.handle_error(e, ignore_errors, no_halt_if_error, no_debug_info)?;
                    }
                }
                match self.push(arg2, value) {
                    Ok(_) => {}
                    Err(e) => {
                        self.handle_error(e, ignore_errors, no_halt_if_error, no_debug_info)?;
                    }
                }
            }
            Instruction::Comp(arg1, arg2, ignore_errors, no_halt_if_error, no_debug_info, _) => {
                let mut value1: u64 = 0;
                let mut value2: u64 = 0;

                match self.load(arg1) {
                    Ok(v) => {
                        value1 = v;
                    }
                    Err(e) => {
                        self.handle_error(e, ignore_errors, no_halt_if_error, no_debug_info)?;
                    }
                }

                match self.load(arg2) {
                    Ok(v) => {
                        value2 = v;
                    }
                    Err(e) => {
                        self.handle_error(e, ignore_errors, no_halt_if_error, no_debug_info)?;
                    }
                }

                match value1.cmp(&value2) {
                    std::cmp::Ordering::Less => {}
                    std::cmp::Ordering::Equal => {
                        self.reg_zero += 1;
                    }
                    std::cmp::Ordering::Greater => {
                        self.reg_zero += 2;
                    }
                }
            }
            Instruction::Add(arg1, arg2, ignore_errors, no_halt_if_error, no_debug_info, _) => {
                let mut value1: u64 = 0;
                let mut value2: u64 = 0;

                match self.load(arg1) {
                    Ok(v) => {
                        value1 = v;
                    }
                    Err(e) => {
                        self.handle_error(e, ignore_errors, no_halt_if_error, no_debug_info)?;
                    }
                }

                match self.load(arg2) {
                    Ok(v) => {
                        value2 = v;
                    }
                    Err(e) => {
                        self.handle_error(e, ignore_errors, no_halt_if_error, no_debug_info)?;
                    }
                }

                match self.push(arg1, value1 + value2) {
                    Ok(_) => {}
                    Err(e) => {
                        self.handle_error(e, ignore_errors, no_halt_if_error, no_debug_info)?;
                    }
                }
            }
            Instruction::Sub(arg1, arg2, ignore_errors, no_halt_if_error, no_debug_info, _) => {
                let mut value1: u64 = 0;
                let mut value2: u64 = 0;

                match self.load(arg1) {
                    Ok(v) => {
                        value1 = v;
                    }
                    Err(e) => {
                        self.handle_error(e, ignore_errors, no_halt_if_error, no_debug_info)?;
                    }
                }

                match self.load(arg2) {
                    Ok(v) => {
                        value2 = v;
                    }
                    Err(e) => {
                        self.handle_error(e, ignore_errors, no_halt_if_error, no_debug_info)?;
                    }
                }

                match self.push(arg1, value1 - value2) {
                    Ok(_) => {}
                    Err(e) => {
                        self.handle_error(e, ignore_errors, no_halt_if_error, no_debug_info)?;
                    }
                }
            }
            Instruction::Mul(arg1, arg2, ignore_errors, no_halt_if_error, no_debug_info, _) => {
                let mut value1: u64 = 0;
                let mut value2: u64 = 0;

                match self.load(arg1) {
                    Ok(v) => {
                        value1 = v;
                    }
                    Err(e) => {
                        self.handle_error(e, ignore_errors, no_halt_if_error, no_debug_info)?;
                    }
                }

                match self.load(arg2) {
                    Ok(v) => {
                        value2 = v;
                    }
                    Err(e) => {
                        self.handle_error(e, ignore_errors, no_halt_if_error, no_debug_info)?;
                    }
                }

                match self.push(arg1, value1 * value2) {
                    Ok(_) => {}
                    Err(e) => {
                        self.handle_error(e, ignore_errors, no_halt_if_error, no_debug_info)?;
                    }
                }
            }
            Instruction::Div(arg1, arg2, ignore_errors, no_halt_if_error, no_debug_info, _) => {
                let mut value1: u64 = 0;
                let mut value2: u64 = 0;

                match self.load(arg1) {
                    Ok(v) => {
                        value1 = v;
                    }
                    Err(e) => {
                        self.handle_error(e, ignore_errors, no_halt_if_error, no_debug_info)?;
                    }
                }

                match self.load(arg2) {
                    Ok(v) => {
                        value2 = v;
                    }
                    Err(e) => {
                        self.handle_error(e, ignore_errors, no_halt_if_error, no_debug_info)?;
                    }
                }

                match self.push(arg1, value1 / value2) {
                    Ok(_) => {}
                    Err(e) => {
                        self.handle_error(e, ignore_errors, no_halt_if_error, no_debug_info)?;
                    }
                }
            }
        }
        self.reg_zero += 17;
        Ok(())
    }

    fn get_instruction(&self, index: u64) -> Result<Instruction, CpuError> {
        Instruction::from_tuple((
            self.instruction_cache.get(index) as u8,
            self.instruction_cache.get(index + 1),
            self.instruction_cache.get(index + 2),
        ))
    }

    fn load(&self, arg: T) -> Result<T, CpuError> {
        match arg {
            0 => Ok(self.reg_zero),
            1..=65535 => Ok(self.data_cache[(arg - 1) as usize]),
            65536..=131071 => Ok(self.instruction_cache.get(arg - 65536)),
            address => {
                match self
                    .devices
                    .iter()
                    .filter_map(|dev| match dev.get_address_space().contains(&address) {
                        true => Some(dev),
                        false => None,
                    })
                    .collect::<Vec<&Box<dyn Device>>>()
                    .get(0)
                {
                    Some(dev) => return dev.load(address),
                    None => return Err(CpuError::AddressNotPopulated(address)),
                }
            }
        }
    }

    fn push(&mut self, arg1: u64, arg2: u64) -> Result<(), CpuError> {
        match arg1 {
            0 => {
                self.reg_zero = arg2;
                Ok(())
            }
            1..=65535 => {
                self.data_cache[(arg1 - 1) as usize] = arg2;
                Ok(())
            }
            65536..=131071 => {
                self.instruction_cache.set(arg1 - 65536, arg2);
                Ok(())
            }
            address => {
                match self
                    .devices
                    .iter()
                    .filter_map(|dev| match dev.get_address_space().contains(&address) {
                        true => Some(dev),
                        false => None,
                    })
                    .collect::<Vec<&Box<dyn Device>>>()
                    .get(0)
                {
                    Some(dev) => return dev.push(address, arg2),
                    None => return Err(CpuError::AddressNotPopulated(address)),
                }
            }
        }
    }

    fn handle_error(
        &mut self,
        e: CpuError,
        ignore_errors: bool,
        no_halt_if_error: bool,
        no_debug_info: bool,
    ) -> Result<(), CpuError> {
        if !ignore_errors {
            if !no_halt_if_error {
                return Err(e);
            } else {
                if !no_debug_info {
                    self.push(420, self.reg_zero)?;
                }
            }
        }
        Ok(())
    }
}

mod tests {
    use crate::Cpu;

    #[test]
    fn it_works() {
        let mut cstm_vec: Vec<u8> = vec![1, 0, 0, 0, 0, 0, 0, 0, 8];
        let mut fill_vec: Vec<u8> = vec![0; 371356];
        cstm_vec.append(&mut fill_vec);

        let test_cpu: Cpu<u64> = super::Cpu::new(cstm_vec.into_boxed_slice());
        println!("{:?}", test_cpu.instruction_cache.instructions[0]);
    }
}

pub trait Device {
    fn get_address_space(&self) -> RangeInclusive<u64>;

    fn load(&self, address: u64) -> Result<u64, CpuError>;

    fn push(&self, address: u64, value: u64) -> Result<(), CpuError>;
}
