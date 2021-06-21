use u_risc_interpreter::Device;

pub(crate) struct TerminalOut {

}

pub(crate) struct TerminalIn {

}

impl Device for TerminalOut {
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