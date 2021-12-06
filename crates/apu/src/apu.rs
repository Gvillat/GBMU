use serde::{Deserialize, Serialize};
/// Audio Processing Unit

#[derive(Debug, Serialize, Deserialize)]
pub struct Apu {
    pub data: Vec<u8>,
}

impl Apu {
    pub fn new() -> Self {
        let data = vec![0; 0x30];
        Self { data }
    }
}

impl Default for Apu {
    fn default() -> Self {
        Self::new()
    }
}
