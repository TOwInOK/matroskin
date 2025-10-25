//! Define actor module
//!
//! Restrictions:
//! - alive about 300 secs if idle
//! - api switch should be turned on (lib can't do that)
//!     - download official tool for that <https://www.whatsminer.com/src/views/firmware-download.html#Tool>
//! - miner should be at actual version <3.0.0
//!     - firmware <https://www.whatsminer.com/src/views/firmware-download.html#Firmware>
//!
//! - Item: [Actor]
//! - ApiDoc: <https://apidoc.whatsminer.com/#api-TCP_Translate_Protocol-tcp_protocol>
pub mod message;
pub mod process;
pub mod read;
pub mod send;

use std::fmt::Display;

use tokio::{
    net::{TcpStream, ToSocketAddrs},
    select,
    sync::mpsc::{self, Receiver},
};
use tracing::{debug, error, info, instrument, warn};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{
    account::Account,
    actor::{
        message::ActorMessage,
        process::{process, process_unknown},
    },
    auth_data::AuthData,
    command::{
        Command,
        get_device_info::{GetDeviceInfo, GetDeviceInfoParam},
    },
    error::{Error, Result},
    password::Password,
};

#[derive(Debug, Zeroize, ZeroizeOnDrop)]
/// Active connection with ASIC
///
/// Restrictions:
/// - alive about 300 secs if idle
/// - api switch should be turned on (lib can't do that)
///     - download official tool for that <https://www.whatsminer.com/src/views/firmware-download.html#Tool>
/// - miner should be at actual version <3.0.0
///     - firmware <https://www.whatsminer.com/src/views/firmware-download.html#Firmware>
pub struct Actor {
    #[zeroize(skip)]
    pub username: Account,
    pub password: Password,
    pub salt: String,
    #[zeroize(skip)]
    pub tx: tokio::sync::mpsc::Sender<ActorMessage>,
}

// TODO: add command is_alive(). It should send heartbeat data [0x00,0x00,0x00,0x00]
impl Actor {
    #[instrument(level = "info", skip(addr, username, password), fields(addr = %addr))]
    /// Make connection to ASIC
    pub async fn new(
        addr: impl Display + ToSocketAddrs,
        username: Account,
        password: impl Into<Password>,
    ) -> Result<Self> {
        info!("Creating new Actor.");

        let (tx, rx) = mpsc::channel(10);
        info!(%addr, "Connecting to TCP stream.");
        let mut stream = TcpStream::connect(&addr).await?;
        debug!(%addr, "TCP stream connected. Setting no delay.");
        stream.nodelay()?;
        info!(%addr, "Getting actor salt.");
        let salt = get_actor_salt(&mut stream).await?;
        debug!(%addr, "Salt received: {}", salt);
        tokio::spawn(run_actor(rx, stream, addr.to_string()));
        info!(%addr, "Actor created successfully.");

        Ok(Self {
            tx,
            username,
            password: password.into(),
            salt,
        })
    }
    #[instrument(level = "debug", skip(self))]
    /// Generate AuthData for Command
    pub fn auth_data<C: Command>(&self) -> Result<AuthData> {
        debug!(command = %C::CMD_NAME, "Generating authentication data.");
        AuthData::from_actor::<C>(self)
    }

    #[instrument(level = "info", skip_all, fields(command_name = %C::CMD_NAME))]
    /// Execute some Command with actor
    pub async fn send<C: Command + Send + Sync>(&self, cmd: &C) -> Result<C::Response> {
        info!("Sending command: {}.", C::CMD_NAME);
        let response = cmd.execute(self).await?;
        debug!("Command {} executed. Response received.", C::CMD_NAME);
        Ok(response)
    }
}

#[instrument(level = "info", skip(stream))]
/// Execute GetDeviceInfo with only salf parametr
async fn get_actor_salt(stream: &mut TcpStream) -> Result<String> {
    info!("Attempting to retrieve actor salt.");
    debug!("Preparing GetDeviceInfo command for salt extraction.");
    let data = process::<GetDeviceInfo>(
        stream,
        GetDeviceInfo(GetDeviceInfoParam {
            miner: false,
            power: false,
            network: false,
            system: false,
            error_code: false,
            ..Default::default()
        })
        .to_request_to_bytes(None)?
        .as_ref(),
    )
    .await?;
    debug!("{:#?}", &data);
    debug!("GetDeviceInfo command processed. Extracting salt.");
    data.msg.salt.ok_or(Error::SaltNotFound)
}

#[instrument(level = "info", skip(rx, stream), fields(addr = %addr))]
/// Run actor worker
async fn run_actor(mut rx: Receiver<ActorMessage>, mut stream: TcpStream, addr: String) {
    info!("Actor worker started for address: {}.", addr);
    '_worker: loop {
        select! {
            // TODO: Add reconnection logic
            Some(msg) = rx.recv() => {
                debug!(%addr, "Actor: received command from channel.");

                // Process the command, resulting in a custom `crate::error::Result`.
                let processing_result: crate::error::Result<String> = process_unknown(&mut stream, &msg.message).await;

                // Log any error that occurred during the command processing.
                if let Err(ref e) = processing_result {
                    error!(%addr, error=%e, "Got error when processing data.");
                }

                if let Err(unsent_value) = msg.rev.send(processing_result) {
                    match unsent_value {
                        Ok(s) => warn!(%addr, "Failed to send successful result ('{}') back to requester: oneshot receiver dropped.", s),
                        Err(e) => warn!(%addr, error=%e, "Failed to send error result back to requester: oneshot receiver dropped."),
                    };
                }
                debug!(%addr, "Response sent (or attempted to send) to oneshot channel for command.");
            }
            else => {
                info!(%addr, "Actor: command channel closed. Shutting down actor worker.");
                break '_worker;
            }
        }
    }
}
