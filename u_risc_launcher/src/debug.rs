use crate::print_help;
use std::fs::File;
use std::io::Read;
use u_risc_interpreter::{Cpu, CpuState};
use std::io;

pub fn main(args: Vec<String>, mut file: File) {
    let mut bin: [u8; 65535] = [0; 65535];
    match file.read(&mut bin) {
        Ok(_) => {println!("Opened file successfully!")}
        Err(e) => {print_help(&format!("ERROR: error while reading specified file {}", e))}
    }
    let mut cpu = Cpu::new(bin, vec![]);
    loop {
        print_debug(cpu.debug());
        cpu.tick().unwrap();
        pause();
    }
}

fn print_debug(state: CpuState) {
    println!("Registers: \nA: {} B: {} X: {} S: {} \nUpcoming Instruction: {:?}", state.a, state.b, state.x, state.s, state.upcoming);
}

fn pause() {
    let mut stdin = io::stdin();

    let _ = stdin.read(&mut [0u8]).unwrap();
}