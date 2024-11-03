use crate::memory::Memory;
use crate::constants::*;
use std::sync::{Arc, Mutex};

pub struct ARM7TDMI {
    memory: Arc<Mutex<Memory>>,
    registers: [u32; 16],  // 16 general-purpose registers
    cpsr: u32,             // Current Program Status Register
    pc: usize,             // Program counter
}

impl ARM7TDMI {
    pub fn new(memory: Arc<Mutex<Memory>>) -> Self {
        Self {
            memory,
            registers: [0; 16],
            cpsr: 0,
            pc: 0,
        }
    }

    fn fetch(&mut self) -> u32 {
        let memory = self.memory.lock().unwrap();
        let instruction = memory.read_word(self.pc);
        self.pc += 4;
        instruction
    }

    fn decode(&self, instruction: u32) -> u32 {
        (instruction >> 26) & 0b11  // Simplified opcode extraction
    }

    pub fn execute_instruction(&mut self) {
        let instruction = self.fetch();
        let opcode = self.decode(instruction);

        match opcode {
            OPCODE_DATA_PROCESSING => self.handle_data_processing(instruction),
            OPCODE_MEMORY_ACCESS => self.handle_memory_access(instruction),
            OPCODE_BRANCH => self.handle_branch(instruction),
            _ => panic!("Unknown opcode: {:08x}", opcode),
        }
    }

    fn handle_data_processing(&mut self, instruction: u32) {
        let opcode = (instruction >> 21) & 0xF;
        let rd = (instruction >> 12) & 0xF;
        let rn = (instruction >> 16) & 0xF;
        let operand2 = instruction & 0xFFF;  // Immediate or register operand

        match opcode {
            0xD => self.registers[rd as usize] = operand2,  // MOV instruction (simplified)
            0x4 => self.registers[rd as usize] = self.registers[rn as usize].wrapping_add(operand2),  // ADD instruction
            _ => panic!("Unhandled data processing opcode: {:08x}", opcode),
        }
    }

    fn handle_memory_access(&mut self, instruction: u32) {
        let load = (instruction >> 20) & 1 == 1;
        let rd = (instruction >> 12) & 0xF;
        let base = (instruction >> 16) & 0xF;
        let offset = instruction & 0xFFF;

        let address = self.registers[base as usize].wrapping_add(offset);
        let mut memory = self.memory.lock().unwrap();

        if load {
            self.registers[rd as usize] = memory.read_word(address as usize);
        } else {
            memory.write_word(address as usize, self.registers[rd as usize]);
        }
    }

    fn handle_branch(&mut self, instruction: u32) {
        let offset = instruction & 0xFFFFFF;
        let sign_extended_offset = ((offset << 8) as i32 >> 6) as u32;
        self.pc = self.pc.wrapping_add(sign_extended_offset);
    }
}
