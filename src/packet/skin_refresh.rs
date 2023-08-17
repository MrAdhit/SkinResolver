use super::*;

#[derive(Debug)]
pub struct SkinRefreshPacket {
    pub username: Username,
}

#[async_trait::async_trait]
impl PacketEncoder for SkinRefreshPacket {
    async fn encode(&self) -> Box<[u8]> {
        let mut buffer = Vec::new();

        buffer.extend_from_slice(&ResponsePacketID::SkinRefresh.encode().await);
        buffer.extend_from_slice(&self.username.encode().await);

        buffer.into_boxed_slice()
    }
}

#[async_trait::async_trait]
impl PacketDecoder for SkinRefreshPacket {
    type Output = SkinRefreshPacket;

    async fn decode(buffer: &mut BufReader<&[u8]>) -> Option<Self::Output> {
        let Some(RequestPacketID::SkinRefresh) = RequestPacketID::decode(buffer).await else { return None };
        let Some(username) = Username::decode(buffer).await else { return None };

        Some(Self { username })
    }
}