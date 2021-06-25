use ablecpu_vm::Device;

pub(crate) struct TerminalOut {

}

pub(crate) struct TerminalIn {

}

impl Device for TerminalOut {
    fn get_address_space(&self) -> (u64, u64) {
        (131073, 131073)
    }

    fn load(&self, address: u64) -> u64 {
        println!();
        0
    }

    fn push(&self, address: u64, value: u64) {
        print!("{}", std::str::from_utf8(&value.to_be_bytes()).unwrap())
    }
}

impl Device for TerminalIn {
    fn get_address_space(&self) -> (u64, u64) {
        todo!()
    }

    fn load(&self, address: u64) -> u64 {
        todo!()
    }

    fn push(&self, address: u64, value: u64) {
        todo!()
    }
}