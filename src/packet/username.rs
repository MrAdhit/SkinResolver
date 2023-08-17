use tokio::io::BufReader;

use super::*;

#[derive(Debug)]
pub struct Username(pub Text);

#[async_trait::async_trait]
impl PacketEncoder for Username {
    async fn encode(&self) -> Box<[u8]> {
        Text::encode(&self.0).await
    }
}

#[async_trait::async_trait]
impl PacketDecoder for Username {
    type Output = Username;

    async fn decode(buffer: &mut BufReader<&[u8]>) -> Option<Self> {
        let mut length = [0u8; 4];
        let Ok(_) = buffer.read(&mut length).await else { return None };
        let length = u32::from_be_bytes(length) as usize;

        if length > 16 { return None }

        let mut username = [0u8; 16];
        let Ok(_) = buffer.read(&mut username).await else { return None };
        let username = String::from_utf8_lossy(&username[..length]).to_string();

        Some(Username(username))
    }
}
