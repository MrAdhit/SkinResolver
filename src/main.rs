mod packet;
mod resolver;

use std::{env, sync::Arc};

use anyhow::Result;
use tokio::{io::BufReader, net::UdpSocket};

use crate::packet::*;
use crate::resolver::*;

type Resolver = MineskinResolver;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = env::args().nth(1).unwrap_or("0.0.0.0:9912".to_string());

    resolver::restore_cache();

    let socket = Arc::new(UdpSocket::bind(&addr).await?);
    println!("Listening at \"{addr}\"");

    let mut buf = [0u8; 256];
    loop {
        let socket = socket.clone();

        if let Ok((size, peer)) = socket.recv_from(&mut buf).await {
            
            println!(
                "Received {size} bytes from \"{peer}\"",
                peer = peer.to_string()
            );

            tokio::spawn(async move {
                let bytes = &buf[..size];

                if let Some(packet) = SkinRequestPacket::decode(&mut BufReader::new(bytes)).await {
                    let Some(resolved) = <Resolver as SkinResolver>::resolve(&packet.username.0, true).await else {
                        socket.send_to(&OptionPacket::None.encode().await, peer).await.unwrap();
                        return;
                    };

                    let skin = SkinResponsePacket {
                        username: packet.username,
                        value: SkinValue(resolved.value),
                        signature: SkinSignature(resolved.signature),
                    };

                    socket.send_to(&OptionPacket::Some.encode().await, peer).await.unwrap();
                    socket.send_to(&skin.encode().await, peer).await.unwrap();
                }

                if let Some(packet) = SkinRefreshPacket::decode(&mut BufReader::new(bytes)).await {
                    let Some(_) = <Resolver as SkinResolver>::resolve(&packet.username.0, false).await else {
                        socket.send_to(&OptionPacket::None.encode().await, peer).await.unwrap();
                        return;
                    };

                    socket.send_to(&OptionPacket::Some.encode().await, peer).await.unwrap();
                    socket.send_to(&packet.encode().await, peer).await.unwrap();
                }
            });
        }
    }
}
