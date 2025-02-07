pub(crate) mod area;
pub(crate) mod arithmetic;
pub(crate) mod bitwise;
pub mod bus;
pub(crate) mod complement;
pub(crate) mod flags;
pub(crate) mod incdec;
pub(crate) mod jump;
pub(crate) mod load;
pub(crate) mod logical;
pub(crate) mod rotation;
pub(crate) mod shift;

pub use area::{Bits16, Bits8, Flag};
pub(crate) use arithmetic::Arithmetic;
pub(crate) use bitwise::Bitwise;
pub use bus::Bus;
pub(crate) use complement::Complement;
pub(crate) use flags::{Carry, Flags};
pub(crate) use incdec::IncDec;
pub(crate) use jump::{Absolute, Relative};
pub(crate) use load::Load;
pub(crate) use logical::Logical;
pub(crate) use rotation::Rotation;
pub(crate) use shift::Shift;

#[derive(Debug, Default)]
pub struct Registers {
    pub(crate) a: u8,
    pub(crate) f: Flags,
    pub(crate) b: u8,
    pub(crate) c: u8,
    pub(crate) d: u8,
    pub(crate) e: u8,
    pub(crate) h: u8,
    pub(crate) l: u8,
    pub(crate) sp: u16,
    pub pc: u16,
}

impl Registers {
    pub fn update(&self, dst: &mut Self) {
        *dst = Self { ..*self };
    }

    // Registers are set to these specific values after GB BIOS runs
    pub fn no_bios() -> Self {
        let mut registers = Self::default();
        println!("I'm passing in nobios");
        registers.set(Bits8::F, 0xB0);
        Self {
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            sp: 0xFFFE,
            pc: 0x0100,
            ..registers
        }
    }
}
