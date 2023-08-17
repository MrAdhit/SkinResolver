mod skin_request;
mod skin_response;
mod skin_texture;
mod text;
mod traits;
mod username;
mod skin_refresh;

use tokio::io::{AsyncReadExt, BufReader};

pub use self::traits::*;

pub use self::skin_refresh::*;
pub use self::skin_request::*;
pub use self::skin_response::*;
pub use self::skin_texture::*;
pub use self::text::*;
pub use self::username::*;

#[derive(Debug, Clone, Copy)]
pub enum RequestPacketID {
    SkinFetch = 0x00,
    SkinRefresh = 0x01,
}

#[async_trait::async_trait]
impl PacketDecoder for RequestPacketID {
    type Output = RequestPacketID;

    async fn decode(bytes: &mut BufReader<&[u8]>) -> Option<Self::Output> {
        let mut packet = [0u8; 1];
        let Ok(_) = bytes.read(&mut packet).await else { return None };

        match packet {
            [byte] if byte == RequestPacketID::SkinFetch as u8 => Some(RequestPacketID::SkinFetch),
            [byte] if byte == RequestPacketID::SkinRefresh as u8 => Some(RequestPacketID::SkinRefresh),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ResponsePacketID {
    SkinFetch = 0x00,
    SkinRefresh = 0x01,
}

#[async_trait::async_trait]
impl PacketEncoder for ResponsePacketID {
    async fn encode(&self) -> Box<[u8]> {
        Box::new([(self.to_owned() as u8).clone()])
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OptionPacket {
    Some = 0x00,
    None = 0x01,
}

#[async_trait::async_trait]
impl PacketEncoder for OptionPacket {
    async fn encode(&self) -> Box<[u8]> {
        Box::new([(self.to_owned() as u8).clone()])
    }
}
