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
pub struct Nop;
pub struct PushAll;
pub struct Jmp;
pub struct Call;
pub struct Ret;

pub const HLT_OPCODE: u32       = 0x0000;
pub const PUSH_OPCODE: u32      = 0x0001;
pub const POP_OPCODE: u32       = 0x0002;

pub const ADD_OPCODE: u32       = 0x0003;

pub const NOP_OPCODE: u32       = 0x0004;

pub const PUSHALL_OPCODE: u32   = 0x0005;

pub const JMP_OPCODE: u32       = 0x0006;
pub const CALL_OPCODE: u32      = 0x0007;
pub const RET_OPCODE: u32       = 0x0008;

pub const OPCODE_SIZE: usize                = 2;
const MEMORY_ADDRESS_SIZE: usize        = 4;

const OPTION_PARAM_SIZE: usize              = 1;
const POP_INSTRUCTION_SIZE : usize          = 3;
const ADD_INSTRUCTION_SIZE: usize           = 3;
const PUSH_INSTRUCTION_WITHOUT_PARAM_SIZE: usize = 3;



impl Instruction for Push {
    fn exec(&self) -> MicroinstuctionList {
        vec![
            Microinstruction::read_istr_param(OPTION_PARAM_SIZE, OPCODE_SIZE),
            Microinstruction::store_mdr_option(),
            Microinstruction::read_istr_param_mdr_len(OPCODE_SIZE + OPTION_PARAM_SIZE),
            Microinstruction::write_mdr_on_sp(),
            Microinstruction::sp_inc_option(),
            Microinstruction::pc_inc_option_offset(PUSH_INSTRUCTION_WITHOUT_PARAM_SIZE)
        ]
    }
}

impl Instruction for Pop {
    fn exec(&self) -> MicroinstuctionList {
        vec![
            Microinstruction::read_istr_param(OPTION_PARAM_SIZE, OPCODE_SIZE),
            Microinstruction::store_mdr_option(),
            Microinstruction::sp_dec_option(),
            Microinstruction::pc_inc(POP_INSTRUCTION_SIZE)
        ]
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
            Microinstruction::read_istr_param(OPTION_PARAM_SIZE, OPCODE_SIZE),
            Microinstruction::store_mdr_option(),
            Microinstruction::read_from_sp(MemoryDataRegister::MDR0),
            Microinstruction::sp_dec_option(),
            Microinstruction::read_from_sp(MemoryDataRegister::MDR1),
            Microinstruction::sp_dec_option(),
            Microinstruction::add_mdr0_mdr1(),
            Microinstruction::write_mdr_on_sp(),
            Microinstruction::sp_inc_option(),
            Microinstruction::pc_inc(ADD_INSTRUCTION_SIZE)
        ]
    }
}

impl Instruction for Nop {
    fn exec(&self) -> MicroinstuctionList {
        vec![
            Microinstruction::pc_inc(OPCODE_SIZE)
        ]
    }
}

impl Instruction for PushAll {
    fn exec(&self) -> MicroinstuctionList {
        vec![
            Microinstruction::push_all(),
            Microinstruction::pc_inc(OPCODE_SIZE)
        ]
    }
}

impl Instruction for Jmp {
    fn exec(&self) -> MicroinstuctionList {
        vec![
            Microinstruction::read_istr_param(MEMORY_ADDRESS_SIZE, OPCODE_SIZE),
            Microinstruction::store_mdr_pc()
        ]
    }
}

impl Instruction for Call {
    fn exec(&self) -> MicroinstuctionList {
        vec![
            Microinstruction::read_istr_param(MEMORY_ADDRESS_SIZE, OPCODE_SIZE),
            Microinstruction::store_sp_bp(),
            Microinstruction::pc_inc(OPCODE_SIZE +  MEMORY_ADDRESS_SIZE),
            Microinstruction::push_all(),
            Microinstruction::store_mdr_pc()
        ]
    }
}

impl Instruction for Ret {
    fn exec(&self) -> MicroinstuctionList {
        vec![
            Microinstruction::pop_all()
        ]
    }
}