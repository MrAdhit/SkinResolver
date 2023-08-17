use super::*;

#[derive(Debug)]
pub struct SkinResponsePacket {
    pub username: Username,
    pub value: SkinValue,
    pub signature: SkinSignature,
}

#[async_trait::async_trait]
impl PacketEncoder for SkinResponsePacket {
    async fn encode<W: AsyncWrite + Send + Unpin>(&self, writer: &mut W) -> Result<()> {
        ResponsePacketID::SkinFetch.encode(writer).await?;
        self.username.encode(writer).await?;
        self.value.encode(writer).await?;
        self.signature.encode(writer).await?;

        Ok(())
    }
}
