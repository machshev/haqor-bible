use clap::{Parser, Subcommand};
//use haqor_engine::{Bible, Library};
use log::{debug, error, info, trace, warn};
use std::env;

/// Summarise bible resource
#[derive(Parser, Debug)]
#[command(name = "haqor-engine")]
#[command(author = "James McCorrie <djmccorrie@gmail.com>")]
#[command(version = "0.1")]
#[command(
    about = "CLI for haqor",
    long_about = "This tool is mostly for testing purposes. It allows basic 
    operations with the backend rust based engine. It's not expected to have 
    utility beyond that at this stage."
)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Fetch bible from remote repository
    Fetch {
        /// Name of the bible to download
        bible: String,
    },
}

fn main() {
    let cli = Cli::parse();

    if env::var("RUST_LOG").is_err() {
        env::set_var(
            "RUST_LOG",
            match cli.verbose {
                0 => "Error",
                1 => "Info",
                2 => "Debug",
                _ => "Trace",
            },
        );
    }
    env_logger::init();

    error!("Error");
    warn!("Warn");
    info!("Info");
    debug!("Debug");
    trace!("Trace");

    // let library = Library::default();
    // let bible: Bible = library.get_bible(&*cli.bible);

    // info!("Bible: {:?}", bible);
}
