use tokio::io::{AsyncReadExt, BufReader};

use super::*;

pub type Text = String;

#[async_trait::async_trait]
impl PacketEncoder for Text {
    async fn encode<W: AsyncWrite + Send + Unpin>(&self, writer: &mut W) -> Result<()> {
        writer.write(&(self.len() as u32).to_be_bytes()).await?;
        writer.write(self.as_bytes()).await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl PacketDecoder for Text {
    type Output = Text;

    async fn decode(buffer: &mut BufReader<&[u8]>) -> Option<Self::Output> {
        let mut length = [0u8; 4];
        let Ok(_) = buffer.read(&mut length).await else { return None };
        let length = u32::from_be_bytes(length) as usize;

        let mut value = [0u8; 512];
        let Ok(_) = buffer.read(&mut value).await else { return None };
        let value = String::from_utf8_lossy(&value[..length]).to_string();

        Some(value)
    }
}
