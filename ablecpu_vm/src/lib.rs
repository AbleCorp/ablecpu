use std::ops::RangeInclusive;

use arch::Arch;
use errors::CpuError;
use instructions::Instruction;

mod arch;
pub mod errors;
mod instructions;

pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub struct InstructionCache<T: Arch> {
    instructions: Box<[(u8, T, T)]>,
}

impl<T: Arch + Clone> InstructionCache<T> {
    fn new(raw: Box<[u8]>) -> InstructionCache<T> {
        let mut i: usize = 0;
        let mut assembled: Box<[(u8, T, T)]> =
            vec![(0 as u8, 0.into(), 0.into()); 21845].into_boxed_slice();
        while i < 371365 {
            let inst = raw[i];
            let arg_one = T::from_be_bytes(&raw[i + 1..=i + 8]);
            let arg_two = T::from_be_bytes(&raw[i + 9..=i + 16]);
            assembled[i / 17] = (inst, arg_one, arg_two);
            i += 17;
        }

        InstructionCache {
            instructions: assembled,
        }
    }

    pub fn get(&self, index: T) -> T {
        match (index % 3.into()).as_u8() {
            0 => self.instructions[(index / 3.into()).as_usize()].0.into(),
            1 => self.instructions[(index / 3.into()).as_usize()].1,
            _ => self.instructions[(index / 3.into()).as_usize()].2,
        }
    }

    pub fn set(&mut self, index: T, value: T) {
        match (index % 3.into()).as_u8() {
            0 => self.instructions[(index / 3.into()).as_usize()].0 = value.as_u8(),
            1 => self.instructions[(index / 3.into()).as_usize()].1 = value,
            _ => self.instructions[(index / 3.into()).as_usize()].2 = value,
        }
    }
}

pub struct Cpu<T: Arch> {
    reg_zero: T,
    data_cache: [T; 65535],
    pub instruction_cache: InstructionCache<T>,
    devices: Vec<Box<dyn Device<T>>>,
}

impl<T: Arch> Cpu<T> {
    pub fn new(instructions: Box<[u8]>) -> Cpu<T> {
        Cpu {
            reg_zero: T::from_i32(0),
            data_cache: [0.into(); 65535],
            instruction_cache: InstructionCache::new(instructions),
            devices: Vec::new(),
        }
    }

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

    fn load(&self, arg: T) -> Result<T, CpuError<T>> {
        match arg.as_usize() {
            0 => Ok(self.reg_zero),
            1..=65535 => Ok(self.data_cache[(arg - 1.into()).as_usize()]),
            65536..=131071 => Ok(self.instruction_cache.get(arg - T::from_i32(65536))),
            _ => {
                match self
                    .devices
                    .iter()
                    .filter_map(|dev| match dev.get_address_space().contains(&arg) {
                        true => Some(dev),
                        false => None,
                    })
                    .collect::<Vec<&Box<dyn Device<T>>>>()
                    .get(0)
                {
                    Some(dev) => return dev.load(arg),
                    None => return Err(CpuError::AddressNotPopulated(arg)),
                }
            }
        }
    }

    fn push(&mut self, arg1: T, arg2: T) -> Result<(), CpuError<T>> {
        match arg1.as_usize() {
            0 => {
                self.reg_zero = arg2;
                Ok(())
            }
            1..=65535 => {
                self.data_cache[(arg1 - 1.into()).as_usize()] = arg2;
                Ok(())
            }
            65536..=131071 => {
                self.instruction_cache.set(arg1 - T::from_i32(65536), arg2);
                Ok(())
            }
            _ => {
                match self
                    .devices
                    .iter()
                    .filter_map(|dev| match dev.get_address_space().contains(&arg1) {
                        true => Some(dev),
                        false => None,
                    })
                    .collect::<Vec<&Box<dyn Device<T>>>>()
                    .get(0)
                {
                    Some(dev) => return dev.push(arg1, arg2),
                    None => return Err(CpuError::AddressNotPopulated(arg1)),
                }
            }
        }
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

mod tests {
    #[test]
    fn it_works() {
        use crate::Cpu;
        let mut cstm_vec: Vec<u8> = vec![1, 0, 0, 0, 0, 0, 0, 0, 8];
        let mut fill_vec: Vec<u8> = vec![0; 371356];
        cstm_vec.append(&mut fill_vec);

        let test_cpu: Cpu<u64> = super::Cpu::new(cstm_vec.into_boxed_slice());
        println!("{:?}", test_cpu.instruction_cache.instructions[0]);
    }
}

pub trait Device<T> {
    fn get_address_space(&self) -> RangeInclusive<T>;

    fn load(&self, address: T) -> Result<T, CpuError<T>>;

    fn push(&self, address: T, value: T) -> Result<(), CpuError<T>>;
}
