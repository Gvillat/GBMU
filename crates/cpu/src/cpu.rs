use crate::registers::Registers;
use memory::Memory;
use serde::Serialize;

#[derive(Default, Debug, Serialize)]
pub struct Cpu {
    #[serde(skip)]
    memory: Memory,
    pub registers: Registers,
    pub(crate) halt: bool,
    pub(crate) stop: bool,
}

impl Cpu {
    pub fn new(memory: Memory) -> Self {
        Self {
            memory,
            registers: Registers::default(),
            halt: false,
            stop: false,
        }
    }

    pub fn no_bios(memory: Memory) -> Self {
        Self {
            memory,
            registers: Registers::no_bios(),
            halt: false,
            stop: false,
        }
    }

    pub fn get_memory(&self) -> Memory {
        self.memory.clone()
    }

    pub fn interrupt_enabled(&self) -> bool {
        self.memory.borrow().is_enabled().is_ok()
    }
}
