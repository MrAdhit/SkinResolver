use super::*;

// pub type SkinValue = Username;
// pub type SkinSignature = Text;

#[derive(Debug)]
pub struct SkinValue(pub Text);

#[derive(Debug)]
pub struct SkinSignature(pub Text);

#[async_trait::async_trait]
impl PacketEncoder for SkinValue {
    async fn encode(&self) -> Box<[u8]> {
        Text::encode(&self.0).await
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
    async fn encode(&self) -> Box<[u8]> {
        Text::encode(&self.0).await
    }
}

#[async_trait::async_trait]
impl PacketDecoder for SkinSignature {
    type Output = SkinSignature;

    async fn decode(buffer: &mut BufReader<&[u8]>) -> Option<Self::Output> {
        Text::decode(buffer).await.map(|v| Self(v))
    }
}
