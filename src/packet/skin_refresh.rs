use super::*;

#[derive(Debug)]
pub struct SkinRefreshPacket {
    pub username: Username,
}

#[async_trait::async_trait]
impl PacketEncoder for SkinRefreshPacket {
    async fn encode<W: AsyncWrite + Send + Unpin>(&self, writer: &mut W) -> Result<()> {
        ResponsePacketID::SkinRefresh.encode(writer).await?;
        self.username.encode(writer).await?;

        Ok(())
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