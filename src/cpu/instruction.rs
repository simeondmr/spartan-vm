use crate::cpu::cpu::CPU;
use crate::cpu::errors::MicroprocessorErrors;
use crate::cpu::microinstruction::{MemoryDataRegister, Microinstruction};
use crate::ram::ram::Ram;

type MicroinstuctionList = Vec<Box<dyn Fn(&mut CPU, &mut Ram) -> Result<(), MicroprocessorErrors>>>;

pub trait Instruction {
    fn exec(&self) -> MicroinstuctionList;
}

pub struct Push;
pub struct Pop;
pub struct Halt;
pub struct Add;

pub const HLT_OPCODE: u32   = 0x0000;
pub const PUSH_OPCODE: u32  = 0x0001;
pub const POP_OPCODE: u32   = 0x0002;

pub const ADD_OPCODE: u32   = 0x0003;

pub const OPCODE_SIZE: usize = 2;

impl Instruction for Push {
    fn exec(&self) -> MicroinstuctionList {
        vec![
            Microinstruction::pc_inc(OPCODE_SIZE),
            Microinstruction::read_istr_param(1),
            Microinstruction::pc_inc(1),
            Microinstruction::store_mdr_option(),
            Microinstruction::read_istr_param_mdr_len(),
            Microinstruction::write_mdr_on_sp(),
            Microinstruction::sp_inc_option(),
            Microinstruction::pc_inc(1)
        ]
    }
}

impl Instruction for Pop {
    fn exec(&self) -> MicroinstuctionList {
        todo!()
    }
}

impl Instruction for Halt {
    fn exec(&self) -> MicroinstuctionList {
        vec![
            Microinstruction::poweroff()
        ]
    }
}

impl Instruction for Add {
    fn exec(&self) -> MicroinstuctionList {
        vec![
            Microinstruction::pc_inc(OPCODE_SIZE),
            Microinstruction::read_istr_param(1),
            Microinstruction::store_mdr_option(),
            Microinstruction::read_from_sp(MemoryDataRegister::MDR0),
            Microinstruction::sp_dec_option(),
            Microinstruction::read_from_sp(MemoryDataRegister::MDR1),
            Microinstruction::sp_dec_option(),
            Microinstruction::add_mdr0_mdr1(),
            Microinstruction::write_mdr_on_sp(),
            Microinstruction::sp_inc_option(),
            Microinstruction::pc_inc(1)
        ]
    }
}
