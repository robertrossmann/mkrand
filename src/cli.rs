use clap::{builder::styling, command, Parser, ValueEnum};

const STYLES: styling::Styles = styling::Styles::styled()
    .header(styling::AnsiColor::Green.on_default().bold())
    .usage(styling::AnsiColor::Green.on_default().bold())
    .literal(styling::AnsiColor::Blue.on_default().bold())
    .placeholder(styling::AnsiColor::Cyan.on_default())
    .error(styling::AnsiColor::Red.on_default().bold());

#[derive(Debug, Clone, ValueEnum)]
pub enum Format {
    /// Print the random data as hexadecimal string.
    Hex,
    /// Print the random data as base64 string.
    Base64,
}

#[derive(Parser, Debug)]
#[command(
    name = "mkrand",
    about = "Generate random data of specific size and format.",
    styles = STYLES,
)]
pub struct Args {
    /// Number of bytes to generate.
    pub size: u32,
    /// Format of the random data, to be printed to the console.
    pub format: Format,
}
