use crate::state::State;

pub type Save = Vec<u8>;

pub trait Saver {
    fn serialize(&self) -> Save;
}

impl Saver for State {
    fn serialize(&self) -> Save {
        serde_json::to_vec(self).unwrap()
    }
}

pub trait Loader {
    fn deserialize(&self) -> State;
}

impl Loader for Save {
    fn deserialize(&self) -> State {
        serde_json::from_slice(self).unwrap()
    }
}

#[cfg(test)]
mod test_saver_state {
    use crate::state::State;
    use crate::saver_state::{Loader, Saver};

    #[test]
    fn test_serde_state() {
        let state: State = State::default();

        let serialized = state.serialize();
        println!("serialized state = {:?}", serialized);

        let deserialized: State = serialized.deserialize();
        assert_eq!(println!("{:?}", deserialized), println!("{:?}", state));
    }
}
