use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Rustly {
    /// Input options
    #[arg(short, long)]
    pub input: String,
}
