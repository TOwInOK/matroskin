use tokio::{io::AsyncWriteExt, net::TcpStream};
use tracing::{debug, info, instrument};

use crate::error::Result;

#[instrument(level = "debug", skip_all)]
/// Send data to the stream
pub async fn send(stream: &mut TcpStream, data: &[u8]) -> Result<()> {
    debug!("Sending data to stream. Data length: {} bytes.", data.len());
    let len = data.len() as u32;
    let len_bytes = len.to_le_bytes();
    debug!("Writing 4-byte length prefix: {:?}.", len_bytes);
    stream.write_all(&len_bytes).await?;
    debug!("Writing {} bytes of actual data.", len);
    stream.write_all(data).await?;
    debug!("Flushing stream.");
    stream.flush().await?;
    info!("Data sent and stream flushed successfully.");
    Ok(())
}
