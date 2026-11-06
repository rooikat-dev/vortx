use bytes::{BufMut, Bytes, BytesMut};
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time::interval;

pub struct BcPacket<'a> {
    pub hash: &'a str,
    pub address: &'a str,
    pub port: &'a str,
}

impl<'a> BcPacket<'a> {
    pub fn encode(&self) -> Result<Bytes, &'static str> {
        let mut buf = BytesMut::new();
        let parts = [self.hash, self.address, self.port];

        for part in parts {
            let len: u16 = part.len().try_into().map_err(|_| "String too large")?;
            buf.put_u16(len);
            buf.put_slice(part.as_bytes());
        }

        Ok(buf.freeze())
    }
}

#[tokio::main]
pub async fn transmit(keyphrase: String) -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    println!("Hello from transmit");

    socket.set_broadcast(true)?;

    let target = "239.255.42.99:7890";

    let mut ticker = interval(Duration::from_secs(2));

    loop {
        ticker.tick().await;

        let packet = BcPacket {
            hash: &keyphrase,
            address: "2",
            port: "3",
        };

        let raw = packet.encode()?;

        match socket.send_to(&raw, target).await {
            Ok(_bytes) => println!("broadcasted"),
            Err(_e) => eprintln!("broadcast failed"),
        }
    }
}
