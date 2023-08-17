use anyhow::Result;
use tokio::io::{BufReader, AsyncWrite};

#[async_trait::async_trait]
pub trait PacketEncoder {
    async fn encode<W: AsyncWrite + Send + Unpin>(&self, writer: &mut W) -> Result<()>;
}

#[async_trait::async_trait]
pub trait PacketDecoder {
    type Output;

    async fn decode(bytes: &mut BufReader<&[u8]>) -> Option<Self::Output>
    where
        Self: Sized;
}
