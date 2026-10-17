use clap::{Parser, Args, Subcommand};

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

    match &cli.command {
        Commands::Send(file) => {
            println!("'send' was used, file is: {:?}", file.file);
        }
        Commands::Receive(phrase) => {
            println!("'receive' was ysed, phrase is: {:?}", phrase.phrase);
        }
    }
}
