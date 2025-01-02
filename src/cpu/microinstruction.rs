use crate::cpu::cpu::CPU;
use crate::cpu::errors::MicroprocessorErrors;
use crate::cpu::registers::Registers;
use crate::ram::ram::Ram;

type Microinstuction = Box<dyn Fn(&mut CPU, &mut Ram) -> Result<(), MicroprocessorErrors>>;

pub struct Microinstruction {

}

pub enum MemoryDataRegister {
    MDR0, MDR1
}

struct Register {
    value: u32,
    size: u32,
}

///Registers offset in stack after pushall
const PARAM0_REGISTER_OFFSET: usize   = 0x04;
const OPTION_REGISTER_OFFSET: usize   = 0x08;
const NEGATIVE_FLAG_OFFSET: usize     = 0x0c;
const ZERO_FLAG_OFFSET: usize         = 0x0d;
const CARRY_FLAG_OFFSET: usize        = 0x0e;
const OVERFLOW_FLAG_OFFSET: usize     = 0x0f;
const GLOBAL_REGISTER_OFFSET: usize   = 0x10;
const MDR1_REGISTER_OFFSET: usize     = 0x14;
const MDR0_REGISTER_OFFSET: usize     = 0x18;

const BP_REGISTER_OFFSET: usize       = 0x1C;
const PC_REGISTER_OFFSET: usize       = 0x20;
const SP_REGISTER_OFFSET: usize       = 0x24;


impl Microinstruction {
    #[allow(dead_code)]
    pub fn sp_inc(inc_val: usize) -> Microinstuction {
        Box::new(move |cpu: &mut CPU, _ram: &mut Ram| -> Result<(), MicroprocessorErrors> {
            cpu.registers_mut().sp_inc(inc_val)
        })
    }

    pub fn pc_inc(inc_val: usize) -> Microinstuction {
        Box::new(move |cpu: &mut CPU, _ram: &mut Ram| -> Result<(), MicroprocessorErrors> {
            cpu.registers_mut().pc_inc(inc_val)
        })
    }

    pub fn pc_inc_option_offset(offset: usize)  -> Microinstuction {
        Box::new(move |cpu: &mut CPU, _ram: &mut Ram| -> Result<(), MicroprocessorErrors> {
            let option = cpu.registers().instruction_register().option();
            cpu.registers_mut().pc_inc((option + offset as u32) as usize)
        })
    }

    pub fn read_istr_param(param_len: usize, pc_offset: usize) -> Microinstuction {
        Box::new(move |cpu: &mut CPU, ram: &mut Ram| -> Result<(), MicroprocessorErrors> {
            let istr_param = ram.read_ram(cpu.registers().pc() + pc_offset, param_len)?;
            cpu.registers_mut().set_mdr0(istr_param);
            Ok(())
        })
    }

    pub fn store_mdr_option() -> Microinstuction {
        Box::new(move |cpu: &mut CPU, _ram: &mut Ram| -> Result<(), MicroprocessorErrors> {
            let mdr = cpu.registers().mdr0();
            cpu.registers_mut().instruction_registers_mut().set_option(mdr);
            Ok(())
        })
    }

    pub fn store_mdr_pc() -> Microinstuction {
        Box::new(move |cpu: &mut CPU, _ram: &mut Ram| -> Result<(), MicroprocessorErrors> {
            let mdr = cpu.registers().mdr0();
            cpu.registers_mut().set_pc(mdr as usize)?;
            Ok(())
        })
    }


    pub fn read_istr_param_mdr_len(pc_offset: usize) -> Microinstuction {
        Box::new(move |cpu: &mut CPU, ram: &mut Ram| -> Result<(), MicroprocessorErrors> {
            let cpu_registers = cpu.registers();
            let istr_param = ram.read_ram(cpu_registers.pc() + pc_offset, cpu_registers.mdr0() as usize)?;
            cpu.registers_mut().set_mdr0(istr_param);
            Ok(())
        })
    }

    pub fn write_mdr_on_sp() -> Microinstuction {
        Box::new(move |cpu: &mut CPU, ram: &mut Ram| -> Result<(), MicroprocessorErrors> {
            let cpu_registers = cpu.registers();
            ram.push_unsigned(cpu_registers.sp(), cpu_registers.mdr0())?;
            Ok(())
        })
    }

    pub fn sp_inc_option() -> Microinstuction {
        Box::new(move |cpu: &mut CPU, _ram: &mut Ram| -> Result<(), MicroprocessorErrors> {
            let option = cpu.registers().instruction_register().option() as usize;
            cpu.registers_mut().sp_inc(option)?;
            Ok(())
        })
    }

    pub fn sp_dec_option()-> Microinstuction {
        Box::new(move |cpu: &mut CPU, _ram: &mut Ram| -> Result<(), MicroprocessorErrors> {
            let option = cpu.registers().instruction_register().option() as usize;
            cpu.registers_mut().sp_dec(option)?;
            Ok(())
        })
    }

    pub fn poweroff() -> Microinstuction {
        Box::new(move |cpu: &mut CPU, _ram: &mut Ram| -> Result<(), MicroprocessorErrors> {
            cpu.registers_mut().poweroff();
            Ok(())
        })
    }

    pub fn add_mdr0_mdr1() -> Microinstuction {
        Box::new(move |cpu: &mut CPU, _ram: &mut Ram| -> Result<(), MicroprocessorErrors> {
            let mdr0 = cpu.registers().mdr0();
            let mdr1 = cpu.registers().mdr1();
            cpu.registers_mut().set_mdr0(mdr0 + mdr1);
            Ok(())
        })
    }

    pub fn read_from_sp(mdr: MemoryDataRegister) -> Microinstuction {
        Box::new(move |cpu: &mut CPU, ram: &mut Ram| -> Result<(), MicroprocessorErrors> {
            let data = ram.read_ram(cpu.registers().sp() - cpu.registers().instruction_register().option() as usize, cpu.registers().instruction_register().option() as usize)?;

            if let MemoryDataRegister::MDR0 = mdr {
                cpu.registers_mut().set_mdr0(data);
            } else {
                cpu.registers_mut().set_mdr1(data);
            }

            Ok(())
        })
    }

    pub fn store_sp_bp()  -> Microinstuction {
        Box::new(move |cpu: &mut CPU, _ram: &mut Ram| -> Result<(), MicroprocessorErrors> {
            let sp = cpu.registers().sp();
            cpu.registers_mut().set_bp(sp)?;

            Ok(())
        })
    }

    fn get_registers_to_push(registers: &Registers) -> Vec<Register> {
        vec![
            Register { value: registers.sp() as u32, size: 4 },
            Register { value: registers.pc() as u32, size: 4 },
            Register { value: registers.bp() as u32, size: 4 },
            Register { value: registers.mdr0(), size: 4 },
            Register { value: registers.mdr1(), size: 4 },
            Register { value: registers.global_register() as u32, size: 4 },
            Register { value: registers.flags().overflow() as u32, size: 1 },
            Register { value: registers.flags().carry() as u32, size: 1 },
            Register { value: registers.flags().zero() as u32, size: 1 },
            Register { value: registers.flags().negative() as u32, size: 1 },
            Register { value: registers.instruction_register().option(), size: 4 },
            Register { value: registers.instruction_register().param0(), size: 4 },
            Register { value: registers.instruction_register().param1(), size: 4 },
        ]
    }

    pub fn push_all() -> Microinstuction {
        Box::new(move |cpu: &mut CPU, ram: &mut Ram| -> Result<(), MicroprocessorErrors> {
            let registers = cpu.registers();
            let registers_to_push = Self::get_registers_to_push(registers);
            let mut offset = 0;
            for reg in &registers_to_push {
                ram.push_unsigned(registers.sp() + offset, reg.value)?;
                offset += reg.size as usize;
            }

            cpu.registers_mut().sp_inc(offset)?;

            Ok(())
        })
    }

    pub fn pop_all() -> Microinstuction {
        Box::new(move |cpu: &mut CPU, ram: &mut Ram| -> Result<(), MicroprocessorErrors> {
            let registers = cpu.registers_mut();
            let curr_sp = registers.sp();

            //Pop instruction registers
            registers.instruction_registers_mut().set_param1(ram.read_stack(curr_sp, 4)?);
            registers.instruction_registers_mut().set_param0(ram.read_stack(curr_sp - PARAM0_REGISTER_OFFSET , 4)?);
            registers.instruction_registers_mut().set_option(ram.read_stack(curr_sp - OPTION_REGISTER_OFFSET, 4)?);

            //Pop flags registers
            registers.flags_mut().set_negative(ram.read_stack(curr_sp - NEGATIVE_FLAG_OFFSET, 1)? != 0);
            registers.flags_mut().set_zero(ram.read_stack(curr_sp - ZERO_FLAG_OFFSET, 1)? != 0);
            registers.flags_mut().set_carry(ram.read_stack(curr_sp - CARRY_FLAG_OFFSET, 1)? != 0);
            registers.flags_mut().set_overflow(ram.read_stack(curr_sp - OVERFLOW_FLAG_OFFSET, 1)? != 0);

            //Pop registers
            registers.set_global_register(ram.read_stack(curr_sp - GLOBAL_REGISTER_OFFSET, 4)? as usize);
            registers.set_mdr1(ram.read_stack(curr_sp - MDR1_REGISTER_OFFSET, 4)?);
            registers.set_mdr0(ram.read_stack(curr_sp - MDR0_REGISTER_OFFSET, 4)?);
            registers.set_bp(ram.read_stack(curr_sp - BP_REGISTER_OFFSET, 4)? as usize)?;
            registers.set_pc(ram.read_stack(curr_sp - PC_REGISTER_OFFSET, 4)? as usize)?;
            registers.set_sp(ram.read_stack(curr_sp - SP_REGISTER_OFFSET, 4)? as usize)?;

            Ok(())
        })
    }
}