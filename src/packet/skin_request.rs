use tokio::io::BufReader;

use super::*;

#[derive(Debug)]
pub struct SkinRequestPacket {
    pub username: Username,
}

#[async_trait::async_trait]
impl PacketDecoder for SkinRequestPacket {
    type Output = SkinRequestPacket;

    async fn decode(buffer: &mut BufReader<&[u8]>) -> Option<Self::Output> {
        let Some(RequestPacketID::SkinFetch) = RequestPacketID::decode(buffer).await else { return None };
        let Some(username) = Username::decode(buffer).await else { return None };

        Some(Self { username })
    }
}
