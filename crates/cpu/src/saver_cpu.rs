use crate::cpu::Cpu;

pub type Save = String;

pub trait Saver {
    fn serialize(&self) -> Save;
}

impl Saver for Cpu {
    fn serialize(&self) -> Save {
        serde_json::to_string_pretty(self).unwrap()
    }
}

// pub trait Loader {
//     fn deserialize(&self) -> Cpu;
// }
//
// impl Loader for Save {
//     fn deserialize(&self) -> Cpu {
//         serde_json::from_slice(self).unwrap()
//     }
// }

#[cfg(test)]
mod test_saver_cpu {
    use crate::cpu::Cpu;
    use crate::registers::{Bits16, Bus, Flag};
    use crate::saver_cpu::Saver;

    #[test]
    fn test_serde_cpu() {
        let mut cpu: Cpu = Cpu::default();

        cpu.registers.set(Bits16::AF, 0001);
        cpu.registers.set(Bits16::BC, 0302);
        cpu.registers.set(Bits16::DE, 0504);
        cpu.registers.set(Bits16::HL, 0706);
        cpu.registers.set(Flag::N, true);

        let serialized = cpu.serialize();
        println!("serialized cpu = {:?}", serialized);

        // let deserialized: Cpu = serialized.deserialize();
        // assert_eq!(println!("{:?}", deserialized), println!("{:?}", cpu));
    }
}
