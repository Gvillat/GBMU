use super::{Bits16, Bits8, Bus, Carry, Flag, Registers};

pub trait IncDec<T, U> {
    fn increase(&mut self, _: T, n: U) -> u8;

    fn decrease(&mut self, _: T, n: U) -> u8;
}

impl IncDec<Bits8, u8> for Registers {
    fn increase(&mut self, area: Bits8, n: u8) -> u8 {
        let data = self.get(area);
        self.f.is_half_carry(data, 1);
        let data = data.wrapping_add(n);
        self.set(area, data);
        self.set(Flag::N, false);
        self.set(Flag::Z, data == 0);
        0
    }

    fn decrease(&mut self, area: Bits8, n: u8) -> u8 {
        let data = self.get(area);
        self.f.is_half_borrow(data, 1);
        let data = data.wrapping_sub(n);
        self.set(area, data);
        self.set(Flag::N, true);
        self.set(Flag::Z, data == 0);
        0
    }
}

impl IncDec<u8, u8> for Registers {
    fn increase(&mut self, data: u8, n: u8) -> u8 {
        self.f.is_half_carry(data, 1);
        let data = data.wrapping_add(n);
        self.set(Flag::N, false);
        self.set(Flag::Z, data == 0);
        data
    }

    fn decrease(&mut self, data: u8, n: u8) -> u8 {
        self.f.is_half_borrow(data, 1);
        let data = data.wrapping_sub(n);
        self.set(Flag::N, true);
        self.set(Flag::Z, data == 0);
        data
    }
}

impl IncDec<Bits16, u16> for Registers {
    fn increase(&mut self, area: Bits16, n: u16) -> u8 {
        let data = self.get(area).wrapping_add(n);
        self.set(area, data);
        0
    }

    fn decrease(&mut self, area: Bits16, n: u16) -> u8 {
        let data = self.get(area).wrapping_sub(n);
        self.set(area, data);
        0
    }
}
