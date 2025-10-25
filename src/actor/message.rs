use tokio::sync::oneshot;

use crate::error::Result;

#[derive(Debug)]
/// Message for Actor
pub struct ActorMessage {
    pub message: Vec<u8>,
    pub rev: oneshot::Sender<Result<String>>,
}
