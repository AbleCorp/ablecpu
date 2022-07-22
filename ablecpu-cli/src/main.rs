use std::{io::{self, Read}, fs::File};

use ablecpu::CPU;

fn main() {
    let mut file = File::open("example.bin").unwrap();
    let mut binary = [0_u8; 127];

    file.read(&mut binary).unwrap();

    let mut cpu = CPU::new(binary, Vec::new());

    

    loop {
        println!("Next Instruction: {:?}", cpu.fetch());
        cpu.tick();
        println!("RegZero: {} Data: {:?}", cpu.reg_zero, &cpu.data_mem[0..3]);
        pause();
    }
}

fn pause() {
    let mut stdin = io::stdin();
    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}