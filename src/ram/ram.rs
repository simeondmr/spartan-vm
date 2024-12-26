use std::mem::size_of;
use std::sync::atomic::{AtomicU8, Ordering};
use crate::cpu::errors::MicroprocessorErrors;

pub struct Ram {
    memory: Vec<AtomicU8>
}

impl Ram {
    const MAX_RAM_SIZE          : usize = 10000;

    pub fn new() -> Self {
        Ram {
            memory: (0..Self::MAX_RAM_SIZE).map(|_| AtomicU8::new(0)).collect(),
        }
    }

    pub fn read_ram(&self, from: usize, len: usize) -> Result<u32, MicroprocessorErrors> {
        if len > 4 {
            return Err(MicroprocessorErrors::MemoryOverflow)
        }

        let read_atomic_slice = self.memory.get(from..(from + len)).ok_or_else(||MicroprocessorErrors::MemoryOverflow)?;

        let vec_data: Vec<u8> = read_atomic_slice
            .iter()
            .map(|atomic| atomic.load(Ordering::Relaxed)).collect();

        let mut data_array = [0u8; 4];
        data_array[..vec_data.len()].copy_from_slice(&vec_data);

        Ok(u32::from_le_bytes(data_array))
    }

    pub fn push_unsigned<T: Into<u32>>(&mut self, stack_pointer: usize, value: T) -> Result<(), MicroprocessorErrors> {
        let value_bytes = value.into().to_le_bytes();
        let value_size = size_of::<T>();

        let mut index = 0;
        self.memory
            .get(stack_pointer..stack_pointer + value_size)
            .ok_or(MicroprocessorErrors::MemoryOverflow)?
            .iter()
            .for_each(|atomic| {
                atomic.store(*value_bytes.get(index).unwrap(), Ordering::Relaxed);
                index += 1;
            });

        Ok(())
    }

    pub fn test(&self) {
        //test
        //push8 10
        //push8 10

        self.memory[0].store(1, Ordering::Relaxed);//push opcode
        self.memory[1].store(0, Ordering::Relaxed);//push opcode
        self.memory[2].store(1, Ordering::Relaxed);//push param(push 1 byte)
        self.memory[3].store(8, Ordering::Relaxed);//push 8

        self.memory[4].store(1, Ordering::Relaxed);//push opcode
        self.memory[5].store(0, Ordering::Relaxed);//push opcode
        self.memory[6].store(1, Ordering::Relaxed);//push param(push 1 byte)
        self.memory[7].store(10, Ordering::Relaxed);//push 10



        self.memory[8].store(3, Ordering::Relaxed);//add opcode
        self.memory[9].store(0, Ordering::Relaxed);//push opcode
        self.memory[10].store(1, Ordering::Relaxed);//push param(push 1 byte)


        self.memory[11].store(0, Ordering::Relaxed);

    }
}
