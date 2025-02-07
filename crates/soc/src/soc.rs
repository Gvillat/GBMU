use crate::runner::Runner;
use crate::Status;
use shared::Redraw;
use std::fs;

use crate::header::Header;
use memory;

const ROM_START: usize = 0x150;
const HEADER_START: usize = 0x100;
const HEADER_LEN: usize = 0x50;
const HEAD_LEN: usize = 0x100;

/// The SOC is the GBMU async executor
pub struct SOC {
    status: Status,
    processor: Runner,
}

impl TryFrom<&str> for SOC {
    type Error = std::io::Error;

    fn try_from(path: &str) -> Result<Self, Self::Error> {
        let mut head = fs::read(path)?;
        let rom = head.split_off(ROM_START);
        let raw_header = head.split_off(HEADER_START);

        assert_eq!(head.len(), HEAD_LEN);
        assert_eq!(raw_header.len(), HEADER_LEN);

        let header = Header::try_from(raw_header).expect("Invalid data in raw_header");
        println!("Header: {:#?}", header);

        let state = memory::state::State::Bios;
        let memory: memory::Memory = memory::memory::Memory::new(header.cartridge, rom, state);
        let processor = Runner::new(memory, state);
        let status = Status::new(processor.cpu());

        Ok(SOC { processor, status })
    }
}

impl SOC {
    pub fn get_ppu(&self) -> ppu::Ppu {
        self.processor.ppu()
    }

    pub fn get_cpu(&self) -> cpu::Cpu {
        self.processor.cpu()
    }

    pub fn get_memory(&self) -> memory::Memory {
        self.processor.memory.clone()
    }

    pub fn get_status(&self) -> Status {
        self.status.clone()
    }

    pub fn run(&mut self) -> Redraw {
        let mut status = self.status.borrow_mut();
        status.redraw.clear();
        if status.is_idle() {
            return Redraw::Nope;
        }
        while status.processing() {
            status.step();
            let finished = self.processor.run();
            status.check_redraw(finished)
        }
        println!("[SOC] Finished Run. Redraw: {:?}", status.redraw);
        status.redraw
    }
}
