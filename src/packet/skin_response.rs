use super::*;

#[derive(Debug)]
pub struct SkinResponsePacket {
    pub username: Username,
    pub value: SkinValue,
    pub signature: SkinSignature,
}

#[async_trait::async_trait]
impl PacketEncoder for SkinResponsePacket {
    async fn encode(&self) -> Box<[u8]> {
        let mut buffer = Vec::new();

        buffer.extend_from_slice(&ResponsePacketID::SkinFetch.encode().await);
        buffer.extend_from_slice(&self.username.encode().await);
        buffer.extend_from_slice(&self.value.encode().await);
        buffer.extend_from_slice(&self.signature.encode().await);

        buffer.into_boxed_slice()
    }
}
