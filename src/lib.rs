use base64::prelude::*;
use rand::Rng;
use std::vec;

pub mod cli;

#[derive(Debug, Clone)]
pub struct Sequence {
    bytes: Vec<u8>,
}

impl Sequence {
    pub fn new(size: u32) -> Self {
        let mut rng = rand::thread_rng();
        let mut bytes = vec![0u8; size as usize];
        rng.fill(&mut bytes[..]);

        Sequence { bytes }
    }

    pub fn to_hex(&self) -> String {
        self.bytes
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect()
    }

    pub fn to_base64(&self) -> String {
        BASE64_STANDARD_NO_PAD.encode(&self.bytes)
    }
}
