use crate::bios::Bios;

pub type Save = Vec<u8>;

pub trait Saver {
    fn serialize(&self) -> Save;
}

impl Saver for Bios {
    fn serialize(&self) -> Save {
        serde_json::to_vec(self).unwrap()
    }
}

// impl Saver for Rc<RefCell<Box<(dyn MemoryBus + 'static)>>>

// pub trait Loader {
//     fn deserialize(&self) -> Bios;
// }
//
// impl Loader for Save {
//     fn deserialize(&self) -> Bios {
//         serde_json::from_slice(self).unwrap()
//     }
// }

#[cfg(test)]
mod test_saver_bios {
    use crate::bios::Bios;
    use crate::saver_bios::Saver;

    #[test]
    fn test_serde_bios() {
        let bios: Bios = Bios::default();

        let serialized = bios.serialize();
        println!("serialized bios = {:?}", serialized);

        // let deserialized: Bios = serialized.deserialize();
        // assert_eq!(println!("{:?}", deserialized), println!("{:?}", bios));
    }
}
