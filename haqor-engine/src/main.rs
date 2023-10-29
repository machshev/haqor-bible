use clap::Parser;
use haqor_engine::{Bible, Library};
use log::info;

/// Summarise bible resource
#[derive(Parser)]
#[command(name = "haqor-engine")]
#[command(author = "James McCorrie <djmccorrie@gmail.com>")]
#[command(version = "0.1")]
#[command(
    about = "CLI for haqor",
    long_about = "This tool is mostly for testing purposes. It allows basic 
    operations with the backend rust based engine. It's not expected to have 
    utility beyond that at this stage."
)]
struct Args {
    /// The bible to load
    bible: String,
}

fn main() {
    env_logger::init();

    let args = Args::parse();

    let library = Library::default();
    let bible: Bible = library.get_bible(&*args.bible);

    info!("Bible: {:?}", bible);
}
