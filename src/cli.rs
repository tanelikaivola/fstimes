pub use clap::Parser;
use std::path::PathBuf;

///
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// path to start recursively walking
    pub path: PathBuf,
    /// canonicalize path
    #[clap(short, long)]
    pub relative: bool,
}
