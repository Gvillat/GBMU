use crate::Registers;

pub type Save = Vec<u8>;

pub trait Saver {
    fn serialize(&self) -> Save;
}

impl Saver for Registers {
    fn serialize(&self) -> Save {
        serde_json::to_vec(self).unwrap()
    }
}

pub trait Loader {
    fn deserialize(&self) -> Registers;
}

impl Loader for Save {
    fn deserialize(&self) -> Registers {
        serde_json::from_slice(self).unwrap()
    }
}

#[cfg(test)]
mod test_saver_registers {
    use crate::registers::{Bits16, Bus, Flag, Registers};
    use crate::saver_registers::{Loader, Saver};

    #[test]
    fn test_serde_registers() {
        let mut registers: Registers = Registers::default();

        registers.set(Bits16::AF, 0001);
        registers.set(Bits16::BC, 0302);
        registers.set(Bits16::DE, 0504);
        registers.set(Bits16::HL, 0706);
        registers.set(Flag::N, true);
        let serialized = registers.serialize();
        println!("serialized registers = {:?}", serialized);

        let deserialized: Registers = serialized.deserialize();
        assert_eq!(println!("{:?}", deserialized), println!("{:?}", registers));
    }
}
