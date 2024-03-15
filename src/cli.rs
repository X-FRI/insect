use std::path::PathBuf;
use structopt::StructOpt;

///  A lightweight *nix debugger written in Rust
#[derive(StructOpt, Debug)]
#[structopt(name = "insect")]
pub struct Cli {
    /// The binary program file with debug info
    #[structopt(parse(from_os_str))]
    pub program: PathBuf,
}

impl Cli {
    pub fn parse() -> Cli {
        Cli::from_args()
    }
}
