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
mod test_saver_ppu_registers {
    use crate::Registers;
    use crate::saver_registers::{Loader, Saver};

    #[test]
    fn test_serde_ppu_registers() {
        let registers: Registers = Registers::default();

        let serialized = registers.serialize();
        println!("serialized Ppu registers = {:?}", serialized);

        let deserialized: Registers = serialized.deserialize();
        assert_eq!(println!("{:?}", deserialized), println!("{:?}", registers));
    }
}
