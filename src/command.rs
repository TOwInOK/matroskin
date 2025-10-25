pub mod get_device_info;
pub mod set_miner_fastboot;
pub mod set_miner_pools;

use core::str;

use crate::{
    actor::{Actor, message::ActorMessage},
    auth_data::AuthData,
    error::{Error, Result},
    request::Request,
};
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;
use tracing::{debug, trace};

pub trait Command {
    /// Parameter for command
    type Params: Serialize + Default;
    /// Response from command
    type Response: for<'a> Deserialize<'a>;
    /// Represents the command attribute
    const CMD_NAME: &'static str;
    /// Include AuthData in request?
    const SECURED: bool = false;
    /// Convert params to base64?
    const ENCRYPTED: bool = false;

    /// Return local params
    fn params(&self) -> Result<Option<String>>;

    /// Convert command to request
    fn to_request<'a, 'b: 'a>(&'a self, auth_data: Option<AuthData>) -> Result<Request> {
        let parametr = self.params()?;

        let parametr = if let Some(data) = &auth_data {
            if let Some(x) = parametr {
                if Self::ENCRYPTED {
                    Some(data.encrypt(x.as_bytes())?)
                } else {
                    Some(x)
                }
            } else {
                parametr
            }
        } else if Self::ENCRYPTED {
            return Err(Error::CommandSholdHaveAuthData(Self::CMD_NAME));
        } else {
            parametr
        };

        Ok(Request::new(Self::CMD_NAME, auth_data, parametr))
    }

    fn is_secured(&self) -> bool {
        Self::SECURED
    }
    fn is_encrypted(&self) -> bool {
        Self::ENCRYPTED
    }
    fn cmd_name(&self) -> &'static str {
        Self::CMD_NAME
    }

    /// Allow custom implementation of deserialize to response
    fn response_from_str(json: &str) -> Result<Self::Response> {
        Ok(serde_json::from_str(json)?)
    }
    /// Convert Command to String
    fn to_request_to_string(&self, auth_data: Option<AuthData>) -> Result<String> {
        let req = self.to_request(auth_data)?;
        Ok(serde_json::to_string(&req)?)
    }
    /// Convert Command to Bytes
    fn to_request_to_bytes(&self, auth_data: Option<AuthData>) -> Result<Vec<u8>> {
        let req = self.to_request(auth_data)?;
        Ok(serde_json::to_vec(&req)?)
    }

    // Yes, I want commands to be able to pull the actor. Any questions?
    /// Run command into actor
    fn execute(
        &self,
        actor: &Actor,
    ) -> impl std::future::Future<Output = Result<Self::Response>> + Send
    where
        Self: Sync + Send + Sized,
    {
        async {
            let auth = if Self::SECURED {
                Some(actor.auth_data::<Self>()?)
            } else {
                None
            };
            let message = self.to_request_to_string(auth)?;
            debug!(cmd=%Self::CMD_NAME, "message for send {}", &message);
            let (tx, rx) = oneshot::channel();
            let message = ActorMessage {
                message: message.as_bytes().to_vec(),
                rev: tx,
            };
            actor.tx.send(message).await?;
            let out = rx.await??;
            trace!(data=%out,"got data from rx");
            let out = Self::response_from_str(&out)?;
            Ok(out)
        }
    }
}
