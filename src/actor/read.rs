//! This module provides functions for reading data from a TCP stream
//! - when the command type is known and unknown.
//!
//!- Item: [read], [read_unknown]
//! - ApiDoc: <https://apidoc.whatsminer.com/#api-TCP_Translate_Protocol-tcp_protocol>
use tokio::{
    io::{self, AsyncReadExt},
    net::TcpStream,
};
use tracing::{debug, error, info, instrument, trace};

use crate::{command::Command, error::Result};

#[instrument(level = "debug", skip(stream))]
/// Read the stream when we can know command type
///
/// - ApiDoc:
/// <https://apidoc.whatsminer.com/#api-TCP_Translate_Protocol-tcp_protocol>
pub async fn read<C: Command>(stream: &mut TcpStream) -> Result<<C as Command>::Response> {
    debug!("Reading response for known command: {}.", C::CMD_NAME);
    let raw_response = read_unknown(stream).await?;
    debug!(
        "Raw response received for {}. Attempting to parse.",
        C::CMD_NAME
    );
    C::response_from_str(&raw_response)
}

#[instrument(level = "debug", skip(stream))]
/// Read the stream when we can't know command type at compile time
///
/// - ApiDoc:
/// <https://apidoc.whatsminer.com/#api-TCP_Translate_Protocol-tcp_protocol>
pub async fn read_unknown(stream: &mut TcpStream) -> Result<String> {
    debug!("Reading unknown response from stream.");
    const MAX_BUF: usize = 8192;
    let mut buf = Vec::with_capacity(MAX_BUF);
    '_READ_RES: {
        let mut len_buf = [0u8; 4];
        info!("Reading response length (4 bytes).");
        stream.read_exact(&mut len_buf).await?;
        let resp_len = u32::from_le_bytes(len_buf) as usize;
        debug!("Response length header received: {} bytes.", resp_len);

        if resp_len == 0 {
            error!("Received a zero-length response, which is invalid.");
            // TODO: Change to out type
            return Err(io::Error::from(io::ErrorKind::InvalidData).into());
        }
        if resp_len > MAX_BUF {
            trace!(
                response = %resp_len,
                "Response length exceeds the limit of {}. This response will be truncated if buf is not resized.",
                MAX_BUF
            );
        }
        buf.resize(resp_len, 0);
        info!("Reading {} bytes of response data.", resp_len);
        stream.read_exact(&mut buf).await?;
        debug!("Response data read successfully.");
    }
    info!("Attempting to parse response bytes to UTF-8 string.");
    Ok(String::from_utf8(buf).map_err(|e| {
        error!("Failed to decode response as UTF-8: {}", e);
        io::Error::new(io::ErrorKind::InvalidData, e)
    })?)
}
