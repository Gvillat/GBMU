use crate::interrupts::Interrupts;

pub type Save = Vec<u8>;

pub trait Saver {
    fn serialize(&self) -> Save;
}

impl Saver for Interrupts {
    fn serialize(&self) -> Save {
        serde_json::to_vec(self).unwrap()
    }
}

pub trait Loader {
    fn deserialize(&self) -> Interrupts;
}

impl Loader for Save {
    fn deserialize(&self) -> Interrupts {
        serde_json::from_slice(self).unwrap()
    }
}

#[cfg(test)]
mod test_saver_interrupts {
    use crate::interrupts::Interrupts;
    use crate::saver_interrupts::{Loader, Saver};

    #[test]
    fn test_serde_interrupts() {
        let interrupts: Interrupts = Interrupts::default();

        let serialized = interrupts.serialize();
        println!("serialized interrupts = {:?}", serialized);

        let deserialized = serialized.deserialize();
        assert_eq!(println!("{:?}", deserialized), println!("{:?}", interrupts));
    }
}
