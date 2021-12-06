use crate::registers::Coordinates;

pub type Save = Vec<u8>;

pub trait Saver {
    fn serialize(&self) -> Save;
}

impl Saver for Coordinates {
    fn serialize(&self) -> Save {
        serde_json::to_vec(self).unwrap()
    }
}

pub trait Loader {
    fn deserialize(&self) -> Coordinates;
}

impl Loader for Save {
    fn deserialize(&self) -> Coordinates {
        serde_json::from_slice(self).unwrap()
    }
}

#[cfg(test)]
mod test_saver_coordinates {
    use crate::registers::Coordinates;
    use crate::registers::saver_coordinates::{Loader, Saver};

    #[test]
    fn test_serde_coordinates() {
        let coordinates: Coordinates = Coordinates::default();

        let serialized = coordinates.serialize();
        println!("serialized coordinates = {:?}", serialized);

        let deserialized: Coordinates = serialized.deserialize();
        assert_eq!(println!("{:?}", deserialized), println!("{:?}", coordinates));
    }
}
