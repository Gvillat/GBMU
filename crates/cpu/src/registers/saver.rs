use crate::registers::Flags;

pub type Save = Vec<u8>;

pub trait Saver {
    fn serialize_flags(&self) -> Save;
}

impl Saver for Flags {
    fn serialize_flags(&self) -> Save {
        serde_json::to_vec(self).unwrap()
    }
}

pub trait Loader {
    fn deserialize_flags(&self) -> Flags;
}

impl Loader for Save {
    fn deserialize_flags(&self) -> Flags {
        serde_json::from_slice(self).unwrap()
    }
}

#[cfg(test)]
mod test_saver_flags {
    use crate::registers::saver::{Loader, Saver};
    use crate::registers::{Bus, Flag, Flags};

    #[test]
    fn test_serde_flags() {
        let mut flags: Flags = Flags::default();

        flags.set(Flag::H, true);
        flags.set(Flag::Z, true);
        flags.set(Flag::C, true);
        let serialized = flags.serialize_flags();
        println!("serialized flags = {:?}", serialized);

        let deserialized: Flags = serialized.deserialize_flags();
        assert_eq!(println!("{:?}", deserialized), println!("{:?}", flags));
    }
}
