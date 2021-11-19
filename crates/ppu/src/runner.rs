use crate::blanks::Blank;
use crate::oam::Oam;
use crate::registers as lcd;
use crate::registers::Mode;
use crate::transfert::Transfert;
use crate::Ppu;
use shared::{Error, Finished, Interrupt, Output, Run};

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Runner<T> {
    inner: Pin<Box<dyn Future<Output = T>>>,
}

impl<T> Runner<T> {
    pub fn new(inner: Pin<Box<dyn Future<Output = T>>>) -> Self {
        Self { inner }
    }
}

impl<T> Future for Runner<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        self.inner.as_mut().poll(context)
    }
}

impl Run for Ppu {
    fn run(self) -> Output {
        let inner = Box::pin(run(self));
        Box::pin(Runner::new(inner))
    }
}

async fn run(ppu: Ppu) -> Result<Finished, Error> {
    if !ppu.borrow().registers.control.lcd_enabled {
        println!("[PPU] Ppu disabled");
        Ok(Finished::Nope)
    } else if ppu.borrow_mut().registers.is_equal(lcd::Field::Ly, 144) {
        // Vblank
        ppu.borrow_mut().registers.vblank_interupt = true;
        ppu.borrow().raise_interrupt(Interrupt::Vblank);
        ppu.borrow_mut().registers.mode.update(Mode::Vblank);
        Blank::new(ppu.clone(), Mode::Vblank).await;
        ppu.borrow_mut().registers.clear(lcd::Field::Ly);
        println!("[PPU] Frame finished");
        Ok(Finished::Frame)
    } else {
        // Oam Search
        let mut cycles = Oam::search(ppu.clone()).await?;
        println!("[PPU] oam cycles: {}", cycles);

        // Pixel transfert
        cycles += Transfert::new(ppu.clone()).start().await?;

        // Hblank
        ppu.borrow_mut().registers.hblank_interupt = true;
        ppu.borrow().raise_interrupt(Interrupt::Lcd);
        cycles += Blank::new(ppu.clone(), Mode::Hblank(204)).await;

        ppu.borrow_mut().registers.increase(lcd::Field::Ly);
        let ly = ppu.borrow().registers.coordinates.get(lcd::Field::Ly);
        println!("[PPU] ly: {}", ly);
        Ok(Finished::Line(cycles))
    }
}
