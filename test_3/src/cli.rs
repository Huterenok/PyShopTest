use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// Number of zeros in the end of hash (natural number)
    #[arg(short = 'N')]
    pub n: u8,
    /// Number of hashes you need (natural number)
    #[arg(short = 'F')]
    pub f: u8,
}

impl Args {
    pub fn receive() -> Self {
        Args::parse()
    }
}
