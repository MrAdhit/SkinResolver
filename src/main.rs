mod packet;
mod resolver;

use std::{env, sync::Arc};

use anyhow::Result;
use tokio::io::AsyncWrite;
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
                let mut writer = Vec::new();
                let mut handler = PacketHandler {
                    writer: &mut writer
                };

                if let Some(packet) = SkinRequestPacket::decode(&mut BufReader::new(bytes)).await {
                    handler.skin_request(packet).await;
                }

                if let Some(packet) = SkinRefreshPacket::decode(&mut BufReader::new(bytes)).await {
                    handler.skin_refresh(packet).await;
                }
                
                socket.send_to(&writer, peer).await.unwrap();
            });
        }
    }
}

struct PacketHandler<'a, W: AsyncWrite + Send + Unpin> {
    writer: &'a mut W
}

impl<'a, W: AsyncWrite + Send + Unpin> PacketHandler<'a, W> {
    async fn skin_request(&mut self, packet: SkinRequestPacket) {
        let Some(resolved) = <Resolver as SkinResolver>::resolve(&packet.username.0, true).await else {
            OptionPacket::None.encode(self.writer).await.unwrap();
            return;
        };
    
        let skin = SkinResponsePacket {
            username: packet.username,
            value: SkinValue(resolved.value),
            signature: SkinSignature(resolved.signature),
        };
    
        OptionPacket::Some.encode(self.writer).await.unwrap();
        skin.encode(self.writer).await.unwrap();
    }

    async fn skin_refresh(&mut self, packet: SkinRefreshPacket) {
        let Some(_) = <Resolver as SkinResolver>::resolve(&packet.username.0, false).await else {
            OptionPacket::None.encode(self.writer).await.unwrap();
            return;
        };

        OptionPacket::Some.encode(self.writer).await.unwrap();
        packet.encode(self.writer).await.unwrap();
    }
}