use clap::Parser;
use mkrand::cli::Args;
use mkrand::Sequence;

fn main() {
    let args = Args::parse();
    let sequence = Sequence::new(args.size);

    match args.format {
        mkrand::cli::Format::Hex => println!("{}", sequence.to_hex()),
        mkrand::cli::Format::Base64 => println!("{}", sequence.to_base64()),
    }
}
