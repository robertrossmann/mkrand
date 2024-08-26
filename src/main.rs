use clap::{builder::styling, Parser, Subcommand};
use toolkit::rand::RandArgs;

const STYLES: styling::Styles = styling::Styles::styled()
    .header(styling::AnsiColor::Green.on_default().bold())
    .usage(styling::AnsiColor::Green.on_default().bold())
    .literal(styling::AnsiColor::Blue.on_default().bold())
    .placeholder(styling::AnsiColor::Cyan.on_default())
    .error(styling::AnsiColor::Red.on_default().bold());

#[derive(Debug, Parser)]
#[clap(author, about, version, styles = STYLES)]
struct ProgramArgs {
    #[clap(subcommand)]
    service: Service,
}

#[derive(Debug, Clone, Subcommand)]
enum Service {
    /// Generate random data
    Rand(RandArgs),
}

fn main() {
    let args = ProgramArgs::parse();

    match args.service {
        Service::Rand(args) => toolkit::rand::execute(&args),
    }
}
