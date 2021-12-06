use crate::io::IO;

pub type Save = Vec<u8>;

pub trait Saver {
    fn serialize(&self) -> Save;
}

impl Saver for IO {
    fn serialize(&self) -> Save {
        serde_json::to_vec(self).unwrap()
    }
}

pub trait Loader {
    fn deserialize(&self) -> IO;
}

impl Loader for Save {
    fn deserialize(&self) -> IO {
        serde_json::from_slice(self).unwrap()
    }
}

#[cfg(test)]
mod test_saver_io {
    use crate::io::IO;
    use crate::saver_io::{Loader, Saver};

    #[test]
    fn test_serde_io() {
        let io: IO = IO::default();

        let serialized = io.serialize();
        println!("serialized io = {:?}", serialized);

        let deserialized: IO = serialized.deserialize();
        assert_eq!(println!("{:?}", deserialized), println!("{:?}", io));
    }
}
