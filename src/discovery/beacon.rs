use bytes::{BufMut, Bytes, BytesMut};
use clap::error::Result;
use std::error::Error;
use std::time::Duration;

use tokio::net::UdpSocket;
use tokio::time::sleep;
use tracing::debug;

pub struct BcPacket<'a> {
    pub hash: &'a str,
    pub address: &'a str,
}

impl<'a> BcPacket<'a> {
    pub fn encode(&self) -> Result<Bytes, &'static str> {
        let mut buf = BytesMut::new();
        let parts = [self.hash, self.address];

        for part in parts {
            let len: u16 = part.len().try_into().map_err(|_| "String too large")?;
            buf.put_u16(len);
            buf.put_slice(part.as_bytes());
        }

        Ok(buf.freeze())
    }
}

pub struct Beacon {
    packet: Bytes,
    socket: UdpSocket,
}

impl Beacon {
    pub async fn init(
        keyphrase: &str,
        address: &str,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let socket = UdpSocket::bind("0.0.0.0:0").await?;
        socket.set_broadcast(true)?;

        let packet = BcPacket {
            hash: keyphrase,
            address,
        }
        .encode()
        .map_err(Box::<dyn Error + Send + Sync>::from)?;

        Ok(Self { packet, socket })
    }

    pub async fn transmit(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let target = "239.255.42.99:7890";

        self.socket.send_to(&self.packet, target).await?;
        debug!("Broadcasted");

        sleep(Duration::from_secs(1)).await;

        Ok(())
    }
}
