//! Implement `get.device.custom_data` command
//!
//! This command is used to get miner device custom information.
//!
//! - Command: [GetDeviceCustomData]
//! - ApiDoc: <https://apidoc.whatsminer.com/#api-Device-device_get_custom_data>
use core::str;

use serde::Deserialize;

use crate::{command::Command, error::Result, response::Response};

/// This command represents the `get.device.custom_data` operation.
///
/// It is used to get miner device custom information.
///
/// - ApiDoc: <https://apidoc.whatsminer.com/#api-Device-device_get_custom_data>
///
/// # Example
/// ```rust,ignore
/// use matroskin::actor::Actor;
/// use matroskin::command::get_device_custom_data::{GetDeviceCustomData};
/// use matroskin::account::Account;
/// use matroskin::password::Password;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let addr = "10.10.10.10:4433";
///     let username = Account::Super;
///     let password = Password::Super;
///
///     let actor = Actor::new(addr, username, password).await?;
///
///     let cmd = GetDeviceCustomData::default();
///     let response = actor.send(&cmd).await?;
///     println!("Response: {:#?}", response);
///
///     Ok(())
/// }
///
#[derive(Debug, Default)]
pub struct GetDeviceCustomData;

/// [GetDeviceCustomData] Response
#[derive(Debug, Default, Deserialize)]
pub struct GetDeviceCustomDataResponse {
    pub custom_sn: String,
    pub msg0: String,
    pub msg1: String,
    pub msg2: String,
    pub msg3: String,
    pub msg4: String,
    pub msg5: String,
    pub msg6: String,
    pub msg7: String,
    pub msg8: String,
    pub msg9: String,
}

impl Command for GetDeviceCustomData {
    type Params = ();
    type Response = Response<GetDeviceCustomDataResponse>;
    const CMD_NAME: &'static str = "get.device.custom_data";
    fn params(&self) -> Result<Option<String>> {
        Ok(None)
    }
}
