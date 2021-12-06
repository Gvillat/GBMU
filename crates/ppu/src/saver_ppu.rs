use crate::ppu::Ppu;

pub type Save = Vec<u8>;

pub trait Saver {
    fn serialize(&self) -> Save;
}

impl Saver for Ppu {
    fn serialize(&self) -> Save {
        serde_json::to_vec(self).unwrap()
    }
}

pub trait Loader {
    fn deserialize(&self) -> Ppu;
}

impl Loader for Save {
    fn deserialize(&self) -> Ppu {
        serde_json::from_slice(self).unwrap()
    }
}

#[cfg(test)]
mod test_saver_ppu {
    use shared::Interrupts;
    use crate::ppu::Ppu;
    use crate::saver_ppu::{Loader, Saver};

    #[test]
    fn test_serde_ppu() {
        let ppu = Ppu::no_bios(Interrupts::default());

        let serialized = ppu.serialize();
       // println!("serialized ppu = {:?}", serialized);

        let deserialized: Ppu = serialized.deserialize();
        assert_eq!(println!("{:?}", deserialized), println!("{:?}", ppu));
    }
}
