use crate::registers::Monochrome;

pub type Save = Vec<u8>;

pub trait Saver {
    fn serialize(&self) -> Save;
}

impl Saver for Monochrome {
    fn serialize(&self) -> Save {
        serde_json::to_vec(self).unwrap()
    }
}

pub trait Loader {
    fn deserialize(&self) -> Monochrome;
}

impl Loader for Save {
    fn deserialize(&self) -> Monochrome {
        serde_json::from_slice(self).unwrap()
    }
}

#[cfg(test)]
mod test_saver_palette {
    use crate::registers::palette::Monochrome;
    use crate::registers::saver_palette::{Loader, Saver};

    #[test]
    fn test_serde_palette() {
        let palette: Monochrome = Monochrome::default();

        let serialized = palette.serialize();
        println!("serialized palette Monochrome = {:?}", serialized);

        let deserialized: Monochrome = serialized.deserialize();
        assert_eq!(println!("{:?}", deserialized), println!("{:?}", palette));
    }
}
