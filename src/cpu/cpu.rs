use std::collections::HashMap;
use crate::cpu::errors::MicroprocessorErrors;
use crate::cpu::instruction::{Add, ADD_OPCODE, Halt, HLT_OPCODE, Instruction, OPCODE_SIZE, Pop, POP_OPCODE, Push, PUSH_OPCODE};
use crate::cpu::registers::Registers;
use crate::ram::ram::Ram;

pub struct CPU {
    registers: Registers,
    instructions: HashMap<u32, Box<dyn Instruction>>,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: Registers::new(),

            instructions: HashMap::from([
                (HLT_OPCODE, Box::new(Halt) as Box<dyn Instruction>),
                (PUSH_OPCODE, Box::new(Push) as Box<dyn Instruction>),
                (POP_OPCODE, Box::new(Pop) as Box<dyn Instruction>),
                (ADD_OPCODE, Box::new(Add) as Box<dyn Instruction>)
            ]),
        }
    }

    fn fetch_istr(&mut self, ram: &mut Ram) -> Result<&Box<dyn Instruction>, MicroprocessorErrors> {
        let opcode = ram.read_ram(self.registers.pc(), OPCODE_SIZE)?; //read 2 bytes opcode
        self.instructions.get(&opcode).ok_or_else(|| MicroprocessorErrors::PushUnsignedWrongMode)
    }

    pub fn execute(&mut self, ram: &mut Ram) -> Result<(), MicroprocessorErrors> {
        loop {
            let istr = self.fetch_istr(ram)?;

            println!("--------------------------");
            istr.exec().into_iter().try_for_each(|microinstruction: Box<dyn Fn(&mut CPU, &mut Ram) -> Result<(), MicroprocessorErrors>>| -> Result<(), MicroprocessorErrors> {
                microinstruction(self, ram)
            })?;
            println!("--------------------------");

            if !self.registers.poweron() {
                break;
            }
        }

        Ok(())
    }

    pub fn registers_mut(&mut self) -> &mut Registers {
        &mut self.registers
    }

    pub fn registers(&self) -> &Registers {
        &self.registers
    }
}