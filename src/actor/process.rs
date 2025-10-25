//! Provides functions for processing commands by sending data to and receiving responses from a TCP stream.
//!
//!- Item: [process], [process_unknown]
//!
//! It's just send/read layer
use tokio::net::TcpStream;
use tracing::{debug, instrument};

use crate::{
    actor::{
        read::{read, read_unknown},
        send::send,
    },
    command::Command,
    error::Result,
};

#[instrument(level = "debug", skip(stream, data))]
/// Execute some data into Actor
///
/// You are need specify the Command
pub async fn process<C: Command>(stream: &mut TcpStream, data: &[u8]) -> Result<C::Response> {
    debug!(cmd=%C::CMD_NAME, "Processing command.",);
    send(stream, data).await?;
    debug!(cmd=%C::CMD_NAME,"Data sent. Reading response for command.");
    read::<C>(stream).await
}

#[instrument(level = "debug", skip(stream, data))]
/// Execute some data into Actor
///
/// It is return any positive result
pub async fn process_unknown(stream: &mut TcpStream, data: &[u8]) -> Result<String> {
    debug!("Processing unknown command.");
    debug!("Sending data for unknown command.");
    send(stream, data).await?;
    debug!("Data sent. Reading unknown response.");
    read_unknown(stream).await
}
