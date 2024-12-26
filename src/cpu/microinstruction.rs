use crate::cpu::cpu::CPU;
use crate::cpu::errors::MicroprocessorErrors;
use crate::ram::ram::Ram;

type Microinstuction = Box<dyn Fn(&mut CPU, &mut Ram) -> Result<(), MicroprocessorErrors>>;

pub struct Microinstruction {

}

pub enum MemoryDataRegister {
    MDR0, MDR1
}

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

    pub fn read_istr_param(param_len: usize) -> Microinstuction {
        Box::new(move |cpu: &mut CPU, ram: &mut Ram| -> Result<(), MicroprocessorErrors> {
            let istr_param = ram.read_ram(cpu.registers().pc(), param_len)?;
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

    pub fn read_istr_param_mdr_len() -> Microinstuction {
        Box::new(move |cpu: &mut CPU, ram: &mut Ram| -> Result<(), MicroprocessorErrors> {
            let cpu_registers = cpu.registers();
            let istr_param = ram.read_ram(cpu_registers.pc(), cpu_registers.mdr0() as usize)?;
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
}