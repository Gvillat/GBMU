use crate::apu::Apu;

pub type Save = Vec<u8>;

pub trait Saver {
    fn serialize(&self) -> Save;
}

impl Saver for Apu {
    fn serialize(&self) -> Save {
        serde_json::to_vec(self).unwrap()
    }
}

pub trait Loader {
    fn deserialize(&self) -> Apu;
}

impl Loader for Save {
    fn deserialize(&self) -> Apu {
        serde_json::from_slice(self).unwrap()
    }
}

#[cfg(test)]
mod test_saver_apu {
    use crate::apu::Apu;
    use crate::saver_apu::{Loader, Saver};

    #[test]
    fn test_serde_apu() {
        let mut apu: Apu = Apu::new();

        apu.data.push(42);
        let serialized = apu.serialize();
        println!("serialized apu = {:?}", serialized);

        let deserialized: Apu = serialized.deserialize();
        assert_eq!(println!("{:?}", deserialized), println!("{:?}", apu));
    }
}
