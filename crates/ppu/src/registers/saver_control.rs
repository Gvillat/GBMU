use crate::registers::Control;

pub type Save = Vec<u8>;

pub trait Saver {
    fn serialize(&self) -> Save;
}

impl Saver for Control {
    fn serialize(&self) -> Save {
        serde_json::to_vec(self).unwrap()
    }
}

pub trait Loader {
    fn deserialize(&self) -> Control;
}

impl Loader for Save {
    fn deserialize(&self) -> Control {
        serde_json::from_slice(self).unwrap()
    }
}

#[cfg(test)]
mod test_saver_control {
    use crate::registers::Control;
    use crate::registers::saver_control::{Loader, Saver};

    #[test]
    fn test_serde_control() {
        let control: Control = Control::default();

        let serialized = control.serialize();
        println!("serialized control = {:?}", serialized);

        let deserialized: Control = serialized.deserialize();
        assert_eq!(println!("{:?}", deserialized), println!("{:?}", control));
    }
}
