use crate::CommandHandler;
use base64::engine::{general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::{Args, ValueEnum};
use rand::Rng;
use std::{
    fmt::{Formatter, LowerHex, Result},
    vec,
};

#[derive(Debug, Clone)]
struct Sequence {
    bytes: Vec<u8>,
}

impl Sequence {
    fn new(size: usize) -> Self {
        let mut bytes = vec![0u8; size];
        rand::thread_rng().fill(&mut bytes[..]);

        Self { bytes }
    }

    fn to_hex(&self) -> String {
        format!("{:02x}", &self)
    }

    fn to_base64(&self) -> String {
        URL_SAFE_NO_PAD.encode(&self.bytes)
    }
}

impl LowerHex for Sequence {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        self.bytes
            .iter()
            .for_each(|&byte| LowerHex::fmt(&byte, formatter).unwrap_or(()));
        Ok(())
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

pub struct RandCommand {
    args: RandArgs,
}

impl RandCommand {
    pub fn new(args: RandArgs) -> Self {
        RandCommand { args }
    }
}

impl CommandHandler for RandCommand {
    fn execute(&self) {
        let sequence = Sequence::new(self.args.size);
        println!("{}", self.args.size);

        match self.args.format {
            Format::Hex => println!("{}", sequence.to_hex()),
            Format::Base64 => println!("{}", sequence.to_base64()),
        }
    }
}
