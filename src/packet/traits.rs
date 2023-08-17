use tokio::io::BufReader;

#[async_trait::async_trait]
pub trait PacketEncoder {
    async fn encode(&self) -> Box<[u8]>;
}

#[async_trait::async_trait]
pub trait PacketDecoder {
    type Output;

    async fn decode(bytes: &mut BufReader<&[u8]>) -> Option<Self::Output>
    where
        Self: Sized;
}
