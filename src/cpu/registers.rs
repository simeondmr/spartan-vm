use crate::cpu::errors::MicroprocessorErrors;
#[allow(dead_code)]

pub struct Flags {
    overflow: bool,
    carry: bool,
    zero: bool,
    negative: bool,

    /// This flag indicate if the CPU is currently executing an interrupt
    int_occur: bool
}

#[allow(dead_code)]
pub struct InstructionRegisters {
    option: u32,
    param0: u32,
    param1: u32
}

#[allow(dead_code)]
pub struct Registers {
    pc: usize,
    sp: usize,
    bp: usize,
    mdr0: u32,
    mdr1: u32,
    poweron: bool,
    global_register: usize,//TODO: This register will point after the .text section in order to allow access to .data and .bss section
    flags: Flags,
    instruction_registers: InstructionRegisters
}

impl InstructionRegisters {
    fn new() -> Self {
        InstructionRegisters {
            option: 0,
            param0: 0,
            param1: 0,
        }
    }

    pub fn option(&self) -> u32 {
        self.option
    }

    #[allow(dead_code)]
    pub fn param0(&self) -> u32 {
        self.param0
    }

    #[allow(dead_code)]
    pub fn param1(&self) -> u32 {
        self.param1
    }

    #[allow(dead_code)]
    pub fn set_option(&mut self, option: u32) {
        self.option = option;
    }

    #[allow(dead_code)]
    pub fn set_param0(&mut self, param0: u32) {
        self.param0 = param0;
    }

    #[allow(dead_code)]
    pub fn set_param1(&mut self, param1: u32) {
        self.param1 = param1;
    }
}


impl Flags {
    fn new() -> Self {
        Flags {
            overflow: false,
            carry: false,
            zero: false,
            negative: false,
            int_occur: false
        }
    }

    #[allow(dead_code)]
    pub fn overflow(&self) -> bool {
        self.overflow
    }

    #[allow(dead_code)]
    pub fn carry(&self) -> bool {
        self.carry
    }

    #[allow(dead_code)]
    pub fn zero(&self) -> bool {
        self.carry
    }

    #[allow(dead_code)]
    pub fn negative(&self) -> bool {
        self.negative
    }

    #[allow(dead_code)]
    pub fn int_occur(&self) -> bool {
        self.int_occur
    }

    #[allow(dead_code)]
    pub fn set_overflow(&mut self, overflow: bool) {
        self.overflow = overflow;
    }

    #[allow(dead_code)]
    pub fn set_carry(&mut self, carry: bool) {
        self.carry = carry;
    }

    #[allow(dead_code)]
    pub fn set_zero(&mut self, zero: bool) {
        self.zero = zero;
    }

    #[allow(dead_code)]
    pub fn set_negative(&mut self, negative: bool) {
        self.negative = negative;
    }

    #[allow(dead_code)]
    pub fn set_int_occur(&mut self, int_occur: bool) {
        self.int_occur = int_occur
    }
}

//TODO: al registers set method must check the target address memory in order to check if there is a segmentation fault
impl Registers {
    pub fn new() -> Self {
        Registers {
            pc: 6, //test value
            sp: 20,//test value
            bp: 20,//test value
            mdr0: 0,
            mdr1: 0,
            global_register: 0,
            poweron: true,
            instruction_registers: InstructionRegisters::new(),
            flags: Flags::new(),
        }
    }

    pub fn mdr0(&self) -> u32 {
        self.mdr0
    }

    pub fn set_mdr0(&mut self, mdr0: u32) {
        self.mdr0 = mdr0;
    }

    pub fn mdr1(&self) -> u32 {
        self.mdr1
    }

    pub fn set_mdr1(&mut self, mdr1: u32) {
        self.mdr1 = mdr1;
    }

    pub fn pc(&self) -> usize {
        self.pc
    }

    pub fn sp(&self) -> usize {
        self.sp
    }

    #[allow(dead_code)]
    pub fn bp(&self) -> usize {
        self.bp
    }

    pub fn poweron(&self) -> bool {
        self.poweron
    }

    pub fn instruction_registers_mut(&mut self) -> &mut InstructionRegisters {
        &mut self.instruction_registers
    }

    pub fn instruction_register(&self) -> &InstructionRegisters {
        &self.instruction_registers
    }

    #[allow(dead_code)]
    pub fn flags(&self) -> &Flags {
        &self.flags
    }

    #[allow(dead_code)]
    pub fn flags_mut(&mut self) -> &mut Flags {
        &mut self.flags
    }

    #[allow(dead_code)]
    pub fn set_pc(&mut self, pc: usize) -> Result<(), MicroprocessorErrors> {
        self.pc = pc;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn set_bp(&mut self, bp: usize) -> Result<(), MicroprocessorErrors> {
        self.bp = bp;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn set_sp(&mut self, sp: usize) -> Result<(), MicroprocessorErrors> {
        self.sp = sp;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn pc_inc(&mut self, inc_val: usize) -> Result<(), MicroprocessorErrors> {
        self.pc += inc_val;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn pc_dec(&mut self, dec_val: usize) {
        self.pc -= dec_val;
    }

    #[allow(dead_code)]
    pub fn bp_inc(&mut self, bp_inc_val: usize) -> Result<(), MicroprocessorErrors> {
        if self.bp >= 10000 {//cioè se bp ha superato gli indirizzi delle routine di INT in RAM, supponendo 10000 sia l'indirizzo limite
            return Err(MicroprocessorErrors::BpError)
        }

        self.bp += bp_inc_val;
        Ok(())
    }

    pub fn sp_dec(&mut self, sp_dec_val: usize) -> Result<(), MicroprocessorErrors> {
        self.sp -= sp_dec_val;
        Ok(())
    }

    pub fn sp_inc(&mut self, sp_dec_val: usize) -> Result<(), MicroprocessorErrors> {
        self.sp += sp_dec_val;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn global_register(&self) -> usize {
        self.global_register
    }

    #[allow(dead_code)]
    pub fn set_global_register(&mut self, global_register: usize) {
        self.global_register = global_register;
    }

    pub fn poweroff(&mut self) {
        self.poweron = false;
    }
}