use std::env;
use std::fs::File;
mod run;
mod debug;
mod devices;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        None => print_help(""),
        Some(command) => {
            let file = find_file(&args);
            match command.as_str() {
            "run" => {run::main(args, file)}
            "debug" => {debug::main(args, file)}
            _ => {print_help("")}
        }}
    }
    std::process::exit(0)
}

fn print_help(err: &str) -> ! {
    println!("{}", err);
    println!("U-RISC-LAUNCHER v{} using U-RISC-INTERPRETER v{}\nAvailable commands:\n  run FILENAME\n  debug FILENAME", env!("CARGO_PKG_VERSION"), u_risc_interpreter::get_version());
    std::process::exit(1)
}

fn find_file(args: &[String]) -> File{
    match args.get(2) {
        None => { print_help("ERROR: Please enter a filename after the command"); },
        Some(filename) => {
            match File::open(filename) {
                Ok(file) => file,
                Err(e) => print_help(&format!("ERROR: error while opening specified file {}", e))
            }
        }
    }
}