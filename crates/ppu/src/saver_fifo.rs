use crate::fifo::Fifo;

pub type Save = Vec<u8>;

pub trait Saver {
    fn serialize(&self) -> Save;
}

impl Saver for Fifo {
    fn serialize(&self) -> Save {
        serde_json::to_vec(self).unwrap()
    }
}

pub trait Loader {
    fn deserialize(&self) -> Fifo;
}

impl Loader for Save {
    fn deserialize(&self) -> Fifo {
        serde_json::from_slice(self).unwrap()
    }
}

#[cfg(test)]
mod test_saver_fifo {
    use crate::fifo::Fifo;
    use crate::saver_fifo::{Loader, Saver};

    #[test]
    fn test_serde_fifo() {
        let fifo: Fifo = Fifo::new();

        let serialized = fifo.serialize();
        println!("serialized fifo = {:?}", serialized);

        let deserialized: Fifo = serialized.deserialize();
        assert_eq!(println!("{:?}", deserialized), println!("{:?}", fifo));
    }
}
