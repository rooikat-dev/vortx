use clap::{Args, Parser, Subcommand};
use getrandom::{SysRng, rand_core::TryRng};
use sha2::{Digest, Sha512};
use tracing::{debug, info};

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

            let mut key = [0u8; 16];
            SysRng.try_fill_bytes(&mut key).unwrap();
            debug!("Random Key is: {}", hex::encode(key));

            let hash512 = hex::encode(Sha512::digest(key));
            debug!("Hash is {}", hash512);

            let hash44 = hash512.get(0..11).unwrap();
            debug!("44 Hex is {}", hash44);

            let big_int = u64::from_str_radix(hash44, 16).expect("Invalid hex char");

            let mask = 0x7ff;

            let int_0 = big_int & mask;
            let int_1 = (big_int >> 11) & mask;
            let int_2 = (big_int >> 22) & mask;
            let int_3 = (big_int >> 33) & mask;
            debug!("Ints are: {} {} {} {}", int_0, int_1, int_2, int_3);
        }
        Commands::Receive(phrase) => {
            info!("Receiving with phrase: {}", phrase.phrase);
        }
    }

    // debug!("This is a debug");
    // warn!("WARNINGGGGG");
    // error!("does this end the program?");
    // info!("nope just logging");
}
