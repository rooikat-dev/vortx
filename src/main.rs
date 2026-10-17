use clap::{Parser, Args, Subcommand};
use tracing::{info, Level};
use tracing_subscriber;

/// P2P encrypted file sending program
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // Send a file over VORTX
    Send(SendArgs),
    // Receive a file over VORTX
    Receive(ReceiveArgs),
}

#[derive(Args)]
struct SendArgs {
    // Path to file to send
    file: String,
}

#[derive(Args)]
struct ReceiveArgs {
    // Peer connection phrase
    phrase: String,
}

fn main() {
    let cli = Cli::parse();

    // install global subscriber configured based on RUST_LOG envvar.
    tracing_subscriber::fmt::init(); 

    match &cli.command {
        Commands::Send(file) => {
            info!(file = file.file, "Sending")
        }
        Commands::Receive(phrase) => {
            info!(phrase = phrase.phrase, "Receivin with");
        }
    }
}
