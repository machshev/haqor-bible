use clap::{Parser, Subcommand};
use haqor_engine::{Bible, Library, ResourceRepo};
use log::info;
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
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Fetch bible from remote repository
    #[command(arg_required_else_help = true)]
    Fetch {
        /// Name of the bible to download
        bible: String,
    },
    /// Read bible verse
    Read { bible: String },
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

    let library = Library::default();

    match cli.command {
        Commands::Fetch { bible } => {
            let repo = ResourceRepo::default();
            repo.fetch_bible(library, &bible);
        }
        Commands::Read { bible } => {
            let bible: Bible = library.get_bible(&bible);

            info!("Bible: {:?}", bible);
        }
    }
}
