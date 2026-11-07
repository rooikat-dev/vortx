mod crypto;
mod discovery;
mod transfer;

use clap::{Args, Parser, Subcommand};
use tracing::{debug, info};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

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

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // install global subscriber configured based on RUST_LOG envvar.
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let keyphrase: String;
    let is_sending: bool;

    match &cli.command {
        Commands::Send(file) => {
            info!("Sending file: {}", file.file);

            keyphrase = match crypto::keyphrase::keyphrase_gen() {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Error generating keyphrase: {}", e);
                    e
                }
            };

            info!("Phrase is: {}", keyphrase);

            is_sending = true;
        }

        Commands::Receive(phrase) => {
            keyphrase = phrase.phrase.clone();
            is_sending = false;
        }
    }

    info!("Are we sending: {}", is_sending);

    let hash_keyphrase = blake3::hash(keyphrase.as_bytes()).to_hex().to_lowercase();

    debug!("The blake3 hash is: {}", hash_keyphrase);

    if is_sending {
        let _socket = match transfer::sender::init_listen(hash_keyphrase).await {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error creating socket: {}", e);
                return;
            }
        };
    }
    // debug!("This is a debug");
    // warn!("WARNINGGGGG");
    // error!("does this end the program?");
    // info!("nope just logging");
}
