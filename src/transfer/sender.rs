use std::error::Error;
use std::time::Duration;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::sleep;
use tracing::debug;

use crate::discovery::beacon::Beacon;

pub async fn init_listen(keyphrase: String) -> Result<TcpStream, Box<dyn Error + Send + Sync>> {
    let listener = TcpListener::bind("0.0.0.0:0").await?;
    let address = listener.local_addr()?;
    debug!("Tcp Connection open on: {}", address);

    let beacon = Beacon::init(&keyphrase, &address.to_string()).await?;

    let beacon_task = tokio::spawn(async move {
        loop {
            if let Err(e) = beacon.transmit().await {
                eprintln!("Beacon broadcast error: {e}");
            }
            sleep(Duration::from_secs(1)).await;
        }
    });

    let (socket, client_addr) = listener.accept().await?;
    debug!("Client connected from: {client_addr}");

    beacon_task.abort();

    Ok(socket)
}
