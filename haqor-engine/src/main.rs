use std::path::PathBuf;
use clap::Parser;

/// Summarise bible resource
#[derive(Parser)]
struct Args {
    /// The bible to load
    bible: String,
    /// Where to find the bible
    path: PathBuf,
}


fn main() {
    let args = Args::parse();

}
