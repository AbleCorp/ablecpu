use std::{fs::File, io::Read};

use ablecpu_vm::Cpu;

#[test]
fn main() {
    println!("Started!");
    // read file main-example.bin to buffer
    let mut buffer = vec![0u8; 27305];
    let mut file = File::open("tests/example.bin").unwrap();
    file.read(&mut buffer).unwrap();
    
    let mut cpu: Cpu<u16> = Cpu::new(
        buffer.into_boxed_slice(),
        Vec::new(),
    );
    loop {
        cpu.tick().unwrap();
    }
}

#[test]
fn inst_fetch() {
    let mut buffer = vec![0u8; 27305];
    let mut file = File::open("tests/example.bin").unwrap();
    file.read(&mut buffer).unwrap();

    dbg!(buffer.len());

    let cpu: Cpu<u16> = Cpu::new(
        buffer.into_boxed_slice(),
        Vec::new(),
    );
    dbg!(cpu.get_instruction(0).unwrap());
    dbg!(cpu.get_instruction(3).unwrap());
    dbg!(cpu.get_instruction(6).unwrap());
    dbg!(cpu.get_instruction(9).unwrap());
    dbg!(cpu.get_instruction(12).unwrap());
    dbg!(cpu.get_instruction(15).unwrap());
    dbg!(cpu.get_instruction(18).unwrap());
}

use std::io;
use std::io::prelude::*;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}