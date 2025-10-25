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

/// Send builded data and return mapped response to gived stream
#[instrument(level = "debug", skip(stream, data))]
/// Execute some data into Actor
///
/// You are need specify the Command
pub async fn process<C: Command>(stream: &mut TcpStream, data: &[u8]) -> Result<C::Response> {
    debug!("Processing command: {}.", C::CMD_NAME);
    debug!("Sending data for command {}.", C::CMD_NAME);
    send(stream, data).await?;
    debug!("Data sent. Reading response for command {}.", C::CMD_NAME);
    read::<C>(stream).await
}

/// Send builded data and return mapped response to gived stream
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
