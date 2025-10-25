use hex::FromHexError;
use thiserror::Error;
use tokio::sync::mpsc::error::SendError;

use crate::actor::message::ActorMessage;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Aes error: {0}")]
    SendMPSC(#[from] SendError<ActorMessage>),
    #[error("Aes error: {0}")]
    RecvOneshot(#[from] tokio::sync::oneshot::error::RecvError),
    #[error("Salt not found")]
    SaltNotFound,
    #[error("Corrupted token")]
    CurraptedToken,
    #[error("Command {0}: should have AuthData ")]
    CommandSholdHaveAuthData(&'static str),
    #[error("Hex got error")]
    Hex(#[from] FromHexError),
    #[error("Encryption failed")]
    EncryptionFailed,
}

pub type Result<T> = std::result::Result<T, Error>;
