use std::ops::RangeInclusive;

use arch::Arch;
use errors::CpuError;
use instruction_cache::InstructionCache;
use instructions::Instruction;

mod arch;
mod bus;
pub mod errors;
mod instruction_cache;
mod instructions;

pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub struct Cpu<T: Arch> {
    REG_ZERO: T,
    DATA_START: T,
    DATA_END: T,
    INSTRUCTION_START: T,
    INSTRUCTION_END: T,
    DATA_SIZE: T,
    INSTRUCTION_SIZE: T,
    reg_zero: T,
    data_cache: Vec<T>,
    pub instruction_cache: InstructionCache<T>,
    devices: Vec<Box<dyn Device<T>>>,
}

impl<T: Arch> Cpu<T> {
    pub fn tick(&mut self) -> Result<(), CpuError<T>> {
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
                let mut value: T = 0.into();

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
                let mut value1: T = 0.into();
                let mut value2: T = 0.into();

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
                        self.reg_zero += 1.into();
                    }
                    std::cmp::Ordering::Greater => {
                        self.reg_zero += 2.into();
                    }
                }
            }
            Instruction::Add(arg1, arg2, ignore_errors, no_halt_if_error, no_debug_info, _) => {
                let mut value1: T = 0.into();
                let mut value2: T = 0.into();

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
                let mut value1: T = 0.into();
                let mut value2: T = 0.into();

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
                let mut value1: T = 0.into();
                let mut value2: T = 0.into();

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
                let mut value1: T = 0.into();
                let mut value2: T = 0.into();

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
        self.reg_zero += 17.into();
        Ok(())
    }

    fn get_instruction(&self, index: T) -> Result<Instruction<T>, CpuError<T>> {
        Instruction::from_tuple((
            self.instruction_cache.get(index).as_u8(),
            self.instruction_cache.get(index + 1.into()),
            self.instruction_cache.get(index + 2.into()),
        ))
    }

    fn handle_error(
        &mut self,
        e: CpuError<T>,
        ignore_errors: bool,
        no_halt_if_error: bool,
        no_debug_info: bool,
    ) -> Result<(), CpuError<T>> {
        if !ignore_errors {
            if !no_halt_if_error {
                return Err(e);
            } else {
                if !no_debug_info {
                    self.push(T::from_i32(420), self.reg_zero)?;
                }
            }
        }
        Ok(())
    }
}

impl<T: Arch> Cpu<T> {
    pub fn new(instructions: Box<[u8]>, devices: Vec<Box<dyn Device<T>>>) -> Cpu<T> {
        Cpu {
            reg_zero: 0.into(),
            data_cache: vec![0.into(); T::DATA_SIZE().as_usize()],
            instruction_cache: InstructionCache::new(instructions),
            devices,
            REG_ZERO: 0.into(),
            DATA_START: 1.into(),
            DATA_END: T::DATA_SIZE(),
            INSTRUCTION_START: T::DATA_SIZE() + 1.into(),
            INSTRUCTION_END: T::DATA_SIZE() + 1.into() + T::INSTRUCTION_SIZE(),
            DATA_SIZE: T::DATA_SIZE(),
            INSTRUCTION_SIZE: T::INSTRUCTION_SIZE(),
        }
    }
}

mod tests {
    use crate::Cpu;

    #[test]
    fn it_works() {
        let mut cstm_vec: Vec<u8> = vec![1, 0, 0, 0, 0, 0, 0, 0, 8];
        let mut fill_vec: Vec<u8> = vec![0; 371365];
        cstm_vec.append(&mut fill_vec);

        let test_cpu: Cpu<u16> = super::Cpu::new(cstm_vec.into_boxed_slice(), Vec::new());
        println!("{:?}", test_cpu.instruction_cache.instructions[0]);
    }
}

pub trait Device<T> {
    fn get_address_space(&self) -> RangeInclusive<T>;

    fn load(&self, address: T) -> Result<T, CpuError<T>>;

    fn push(&self, address: T, value: T) -> Result<(), CpuError<T>>;
}
