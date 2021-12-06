use crate::ram::Ram;

pub type Save = Vec<u8>;

pub trait Saver {
    fn serialize(&self) -> Save;
}

impl Saver for Ram {
    fn serialize(&self) -> Save {
        serde_json::to_vec(self).unwrap()
    }
}

pub trait Loader {
    fn deserialize(&self) -> Ram;
}

impl Loader for Save {
    fn deserialize(&self) -> Ram {
        serde_json::from_slice(self).unwrap()
    }
}

#[cfg(test)]
mod test_saver_ram {
    use crate::ram::Ram;
    use crate::saver_ram::{Loader, Saver};

    #[test]
    fn test_serde_ram() {
        let ram: Ram = Ram::default();

        let serialized = ram.serialize();
        println!("serialized ram = {:?}", serialized);

        let deserialized: Ram = serialized.deserialize();
        assert_eq!(println!("{:?}", deserialized), println!("{:?}", ram));
    }
}
