use crate::memory::Memory;

pub type Save = Vec<u8>;

pub trait Saver {
    fn serialize(&self) -> Save;
}

impl Saver for Memory {
    fn serialize(&self) -> Save {
        serde_json::to_vec(self).unwrap()
        // let mut save: Save = Vec::new();

        // save.append(Saver::serialize(&self.state).as_mut());
        // save.append(crate::saver_bios::Saver::serialize(&self.bios).as_mut());
        // save.append(Saver::serialize(&self.wram).as_mut());
        // save.append(crate::ppu::saver_ppu::Saver::serialize(&self.ppu).as_mut());
        // save.append(Saver::serialize(&self.hram).as_mut());
        // save.append(crate::saver_io::Saver::serialize(&self.io).as_mut());
        // save.append(crate::saver_interrupts::Saver::serialize(&self.interrupts).as_mut());
        // save
    }
}

// pub trait Loader {
//     fn deserialize(&self) -> Memory;
// }
//
// impl Loader for Save {
//     fn deserialize(&self) -> Memory {
//         serde_json::from_slice(self).unwrap()
//     }
// }

#[cfg(test)]
mod test_saver_memory {
    use crate::memory::Memory;
    use crate::saver_memory::Saver;

    #[test]
    fn test_serde_memory() {
        let memory: Memory = Memory::default();

        let serialized = memory.serialize();
        println!("serialized memory = {:?}", memory);

     //   let deserialized: Memory = serialized.deserialize();
    //    assert_eq!(println!("{:?}", deserialized), println!("{:?}", memory));
    }
}
