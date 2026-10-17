use clap::{Args, Parser, Subcommand};
use tracing::{debug, error, info, warn};

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
            info!("Sending file: {}", file.file);
        }
        Commands::Receive(phrase) => {
            info!("Receiving with phrase: {}", phrase.phrase);
        }
    }

    debug!("This is a debug");
    warn!("WARNINGGGGG");
    error!("does this end the program?");
    info!("nope just logging");
}
