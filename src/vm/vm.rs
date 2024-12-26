use crate::cpu::cpu::CPU;
use crate::cpu::errors::MicroprocessorErrors;
use crate::ram::ram::Ram;
pub struct VM {
    cpu: CPU,
    ram: Ram
}

impl VM {
    pub fn new() -> Self {
        VM {
            cpu: CPU::new(),
            ram: Ram::new()
        }
    }

    pub fn execute(&mut self) -> Result<(), MicroprocessorErrors> {
        self.ram.test();
        self.cpu.execute(&mut self.ram)
    }
}