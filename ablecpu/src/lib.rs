pub struct CPU {
    pub reg_zero: u8,
    pub inst_mem: [u8; 127],
    pub data_mem: [u8; 64],
    pub devices: Vec<Box<dyn Device>>,
}

impl CPU {
    pub fn new(inst_mem: [u8; 127], devices: Vec<Box<dyn Device>>) -> CPU {
        CPU {
            reg_zero: 0,
            inst_mem,
            data_mem: [0; 64],
            devices,
        }
    }

    pub fn tick(&mut self) -> Halted {
        let inst = self.fetch();
        self.process(inst)
    }

    pub fn fetch(&self) -> Instruction {
        Instruction::from_3bytes([
            self.inst_mem[self.reg_zero as usize],
            self.inst_mem[(self.reg_zero + 1) as usize],
            self.inst_mem[(self.reg_zero + 2) as usize],
        ])
    }

    fn process(&mut self, inst: Instruction) -> Halted {
        match inst {
            Instruction::NoOp(_, _, _, _) => (),
            Instruction::And(_, _, _, _, arg1, arg2) => {
                let data1 = self.load(arg1);
                let data2 = self.load(arg2);
                self.push(arg1, data1 & data2);
            }
            Instruction::Or(_, _, _, _, arg1, arg2) => {
                let data1 = self.load(arg1);
                let data2 = self.load(arg2);
                self.push(arg1, data1 | data2);
            }
            Instruction::Not(_, _, _, _, arg1) => {
                let data1 = self.load(arg1);
                self.push(arg1, !data1);
            }
            Instruction::Add(_, _, sign1, sign2, arg1, arg2) => {
                match (sign1, sign2) {
                    (true, true) => {
                        let data1 = i8::from_be_bytes([self.load(arg1)]);
                        let data2 = i8::from_be_bytes([self.load(arg2)]);
                        self.push(arg1, (data1 + data2) as u8);
                    }
                    (true, false) => {
                        let data1 = i8::from_be_bytes([self.load(arg1)]);
                        let data2 = self.load(arg2);
                        self.push(arg1, (data1 as i16 + data2 as i16) as u8);
                    }
                    (false, true) => {
                        let data1 = self.load(arg1);
                        let data2 = i8::from_be_bytes([self.load(arg2)]);
                        self.push(arg1, (data1 as i16 + data2 as i16) as u8);
                    }
                    (false, false) => {
                        let data1 = self.load(arg1);
                        let data2 = self.load(arg2);
                        self.push(arg1,data1 + data2);
                    }
                };
            }
            Instruction::Sub(_, _, sign1, sign2, arg1, arg2) => match (sign1, sign2) {
                (true, true) => {
                    let data1 = i8::from_be_bytes([self.load(arg1)]);
                    let data2 = i8::from_be_bytes([self.load(arg2)]);
                    self.push(arg1, (data1 - data2) as u8);
                }
                (true, false) => {
                    let data1 = i8::from_be_bytes([self.load(arg1)]);
                    let data2 = self.load(arg2);
                    self.push(arg1, (data1 as i16 - data2 as i16) as u8);
                }
                (false, true) => {
                    let data1 = self.load(arg1);
                    let data2 = i8::from_be_bytes([self.load(arg2)]);
                    self.push(arg1, (data1 as i16 - data2 as i16) as u8);
                }
                (false, false) => {
                    let data1 = self.load(arg1);
                    let data2 = self.load(arg2);
                    self.push(arg1, data1 - data2);
                }
            },
            Instruction::Mul(_, _, sign1, sign2, arg1, arg2) => match (sign1, sign2) {
                (true, true) => {
                    let data1 = i8::from_be_bytes([self.load(arg1)]);
                    let data2 = i8::from_be_bytes([self.load(arg2)]);
                    self.push(arg1, (data1 * data2) as u8);
                }
                (true, false) => {
                    let data1 = i8::from_be_bytes([self.load(arg1)]);
                    let data2 = self.load(arg2);
                    self.push(arg1, (data1 as i16 * data2 as i16) as u8);
                }
                (false, true) => {
                    let data1 = self.load(arg1);
                    let data2 = i8::from_be_bytes([self.load(arg2)]);
                    self.push(arg1, (data1 as i16 * data2 as i16) as u8);
                }
                (false, false) => {
                    let data1 = self.load(arg1);
                    let data2 = self.load(arg2);
                    self.push(arg1, data1 * data2);
                }
            },
            Instruction::Div(_, _, sign1, sign2, arg1, arg2) => match (sign1, sign2) {
                (true, true) => {
                    let data1 = i8::from_be_bytes([self.load(arg1)]);
                    let data2 = i8::from_be_bytes([self.load(arg2)]);
                    self.push(arg1, (data1 / data2) as u8);
                }
                (true, false) => {
                    let data1 = i8::from_be_bytes([self.load(arg1)]);
                    let data2 = self.load(arg2);
                    self.push(arg1, (data1 as i16 / data2 as i16) as u8);
                }
                (false, true) => {
                    let data1 = self.load(arg1);
                    let data2 = i8::from_be_bytes([self.load(arg2)]);
                    self.push(arg1, (data1 as i16 / data2 as i16) as u8);
                }
                (false, false) => {
                    let data1 = self.load(arg1);
                    let data2 = self.load(arg2);
                    self.push(arg1, data1 / data2);
                }
            },
            Instruction::SL(_, _, _, _, arg1) => {
                let data1 = self.load(arg1);
                self.push(arg1, data1 << 1)
            }
            Instruction::SR(_, _, _, _, arg1) => {
                let data1 = self.load(arg1);
                self.push(arg1, data1 >> 1)
            }
            Instruction::RL(_, _, _, _, arg1) => {
                let data1 = self.load(arg1);
                self.push(arg1, u8::rotate_left(data1, 1));
            }
            Instruction::RR(_, _, _, _, arg1) => {
                let data1 = self.load(arg1);
                self.push(arg1, u8::rotate_right(data1, 1));
            }
            Instruction::Copy(_, _, _, _, arg1, arg2) => {
                let data1 = self.load(arg1);
                self.push(arg2, data1);
            }
            Instruction::CompEq(_, _, _, _, arg1, arg2) => {
                let data1 = self.load(arg1);
                let data2 = self.load(arg2);
                if data1 != data2 {
                    self.reg_zero += 3;
                }
            }
            Instruction::CompGt(_, _, _, _, arg1, arg2) => {
                let data1 = self.load(arg1);
                let data2 = self.load(arg2);
                if data1 <= data2 {
                    self.reg_zero += 3;
                }
            }
            Instruction::CompLt(_, _, _, _, arg1, arg2) => {
                let data1 = self.load(arg1);
                let data2 = self.load(arg2);
                if data1 >= data2 {
                    self.reg_zero += 3;
                }
            }
        };
        self.reg_zero += 3;
        Halted::Running
    }

    fn load(&mut self, addr: u8) -> u8 {
        match addr {
            0 => self.reg_zero,
            1..=127 => self.inst_mem[addr as usize],
            128..=191 => self.data_mem[(addr - 128) as usize],
            _ => panic!("Invalid address: {}", addr),
        }
    }

    fn push(&mut self, addr: u8, data: u8) {
        println!("From push(): addr:{}, data:{}", addr, data);
        match addr {
            0 => self.reg_zero = data,
            1..=127 => self.inst_mem[addr as usize] = data,
            128..=191 => self.data_mem[(addr - 128) as usize] = data,
            _ => panic!("Invalid address: {}", addr),
        }
    }
}

pub trait Device {}

#[derive(Debug)]
pub enum Instruction {
    NoOp(bool, bool, bool, bool),
    And(bool, bool, bool, bool, u8, u8),
    Or(bool, bool, bool, bool, u8, u8),
    Not(bool, bool, bool, bool, u8),
    Add(bool, bool, bool, bool, u8, u8),
    Sub(bool, bool, bool, bool, u8, u8),
    Mul(bool, bool, bool, bool, u8, u8),
    Div(bool, bool, bool, bool, u8, u8),
    SL(bool, bool, bool, bool, u8),
    SR(bool, bool, bool, bool, u8),
    RL(bool, bool, bool, bool, u8),
    RR(bool, bool, bool, bool, u8),
    Copy(bool, bool, bool, bool, u8, u8),
    CompEq(bool, bool, bool, bool, u8, u8),
    CompGt(bool, bool, bool, bool, u8, u8),
    CompLt(bool, bool, bool, bool, u8, u8),
}

impl Instruction {
    pub fn from_3bytes(bytes: [u8; 3]) -> Instruction {
        let opcode = bytes[0] & 0b0000_1111;
        let halt_on_error = bytes[0] & 0b1000_0000 == 0b1000_0000;
        let store_debug_info = bytes[0] & 0b0100_0000 == 0b0100_0000;
        let arg1_signed = bytes[0] & 0b0010_0000 == 0b0010_0000;
        let arg2_signed = bytes[0] & 0b0001_0000 == 0b0001_0000;
        let arg1 = bytes[1];
        let arg2 = bytes[2];

        match opcode {
            0 => Instruction::NoOp(halt_on_error, store_debug_info, arg1_signed, arg2_signed),
            1 => Instruction::And(
                halt_on_error,
                store_debug_info,
                arg1_signed,
                arg2_signed,
                arg1,
                arg2,
            ),
            2 => Instruction::Or(
                halt_on_error,
                store_debug_info,
                arg1_signed,
                arg2_signed,
                arg1,
                arg2,
            ),
            3 => Instruction::Not(
                halt_on_error,
                store_debug_info,
                arg1_signed,
                arg2_signed,
                arg1,
            ),
            4 => Instruction::Add(
                halt_on_error,
                store_debug_info,
                arg1_signed,
                arg2_signed,
                arg1,
                arg2,
            ),
            5 => Instruction::Sub(
                halt_on_error,
                store_debug_info,
                arg1_signed,
                arg2_signed,
                arg1,
                arg2,
            ),
            6 => Instruction::Mul(
                halt_on_error,
                store_debug_info,
                arg1_signed,
                arg2_signed,
                arg1,
                arg2,
            ),
            7 => Instruction::Div(
                halt_on_error,
                store_debug_info,
                arg1_signed,
                arg2_signed,
                arg1,
                arg2,
            ),
            8 => Instruction::SL(
                halt_on_error,
                store_debug_info,
                arg1_signed,
                arg2_signed,
                arg1,
            ),
            9 => Instruction::SR(
                halt_on_error,
                store_debug_info,
                arg1_signed,
                arg2_signed,
                arg1,
            ),
            10 => Instruction::RL(
                halt_on_error,
                store_debug_info,
                arg1_signed,
                arg2_signed,
                arg1,
            ),
            11 => Instruction::RR(
                halt_on_error,
                store_debug_info,
                arg1_signed,
                arg2_signed,
                arg1,
            ),
            12 => Instruction::Copy(
                halt_on_error,
                store_debug_info,
                arg1_signed,
                arg2_signed,
                arg1,
                arg2,
            ),
            13 => Instruction::CompEq(
                halt_on_error,
                store_debug_info,
                arg1_signed,
                arg2_signed,
                arg1,
                arg2,
            ),
            14 => Instruction::CompGt(
                halt_on_error,
                store_debug_info,
                arg1_signed,
                arg2_signed,
                arg1,
                arg2,
            ),
            15 => Instruction::CompLt(
                halt_on_error,
                store_debug_info,
                arg1_signed,
                arg2_signed,
                arg1,
                arg2,
            ),
            _ => panic!("Invalid opcode (This should never ever happen)"),
        }
    }
}

pub enum Halted {
    Running,
    Errored,
    Halted,
}
