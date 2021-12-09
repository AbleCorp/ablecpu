use std::fs::File;
use std::io;
use std::io::Read;

use ableCpu64_vm::{Cpu64, Cpu64State, Device};

use crate::devices::{TerminalIn, TerminalOut};
use crate::print_help;

pub fn main(args: Vec<String>, mut file: File) {
    let mut bin: [u8; 65535] = [0; 65535];
    match file.read(&mut bin) {
        Ok(_) => {
            println!("Opened file successfully!")
        }
        Err(e) => print_help(&format!("ERROR: error while reading specified file {}", e)),
    }
    let mut devices: Vec<Box<dyn Device>> = vec![];
    match args.get(3) {
        None => {
            add_default_devices(&mut devices);
        }
        Some(mode) => {
            if mode == "--disable-devices" {
            } else {
                add_default_devices(&mut devices);
            }
        }
    }
    let mut Cpu64 = Cpu64::new(bin, devices);
    loop {
        print_debug(Cpu64.debug());
        Cpu64.tick().unwrap();
        pause();
    }
}

fn print_debug(state: Cpu64State) {
    println!(
        "Registers: \nA: {} B: {} X: {} S: {} \nUpcoming Instruction: {:?}",
        state.a, state.b, state.x, state.s, state.upcoming
    );
}

fn pause() {
    let mut stdin = io::stdin();

    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn add_default_devices(devices: &mut Vec<Box<dyn Device>>) {
    devices.push(Box::from(TerminalOut {}));
    devices.push(Box::from(TerminalIn {}));
}
