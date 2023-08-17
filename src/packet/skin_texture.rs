use super::*;

#[derive(Debug)]
pub struct SkinValue(pub Text);

#[derive(Debug)]
pub struct SkinSignature(pub Text);

#[async_trait::async_trait]
impl PacketEncoder for SkinValue {
    async fn encode<W: AsyncWrite + Send + Unpin>(&self, writer: &mut W) -> Result<()> {
        Text::encode(&self.0, writer).await
    }
}

#[async_trait::async_trait]
impl PacketDecoder for SkinValue {
    type Output = SkinValue;

    async fn decode(buffer: &mut BufReader<&[u8]>) -> Option<Self::Output> {
        Text::decode(buffer).await.map(|v| Self(v))
    }
}

#[async_trait::async_trait]
impl PacketEncoder for SkinSignature {
    async fn encode<W: AsyncWrite + Send + Unpin>(&self, writer: &mut W) -> Result<()> {
        Text::encode(&self.0, writer).await
    }
}

#[async_trait::async_trait]
impl PacketDecoder for SkinSignature {
    type Output = SkinSignature;

    async fn decode(buffer: &mut BufReader<&[u8]>) -> Option<Self::Output> {
        Text::decode(buffer).await.map(|v| Self(v))
    }
}
