use clap::Parser;
use std::path::PathBuf;

/// Summarise bible resource
#[derive(Parser)]
#[command(name = "haqor-engine")]
#[command(author = "James McCorrie <djmccorrie@gmail.com>")]
#[command(version = "0.1")]
#[command(
    about = "CLI for haqor",
    long_about = "This tool is mostly for testing purposes. It allows basic operations with the backend rust based engine. It's not expected to have utility beyond that at this stage."
)]
struct Args {
    /// The bible to load
    bible: String,
    /// Where to find the bible
    path: PathBuf,
}

fn main() {
    let args = Args::parse();

    println!("Bible: {}", args.bible);
    println!("Path: {}", args.path.display());
}
