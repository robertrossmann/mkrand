use base64::engine::{general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::{Args, ValueEnum};
use rand::Rng;
use std::vec;

#[derive(Debug, Clone)]
struct Sequence {
    bytes: Vec<u8>,
}

impl Sequence {
    fn new(size: usize) -> Self {
        let mut bytes = vec![0u8; size];
        rand::thread_rng().fill(&mut bytes[..]);

        Sequence { bytes }
    }

    fn to_hex(&self) -> String {
        self.bytes
            .iter()
            .map(|&byte| format!("{:02x}", &byte))
            .collect()
    }

    fn to_base64(&self) -> String {
        URL_SAFE_NO_PAD.encode(&self.bytes)
    }
}

#[derive(Debug, Clone, ValueEnum)]
enum Format {
    /// Print the random data as hexadecimal string.
    Hex,
    /// Print the random data as base64 string.
    Base64,
}

#[derive(Debug, Clone, Args)]
pub struct RandArgs {
    /// Number of bytes to generate.
    size: usize,
    /// Format of the random data, to be printed to the console.
    format: Format,
}

pub fn execute(args: &RandArgs) {
    let sequence = Sequence::new(args.size);

    match args.format {
        Format::Hex => println!("{}", sequence.to_hex()),
        Format::Base64 => println!("{}", sequence.to_base64()),
    }
}
