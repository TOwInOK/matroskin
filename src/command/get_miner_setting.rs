//! Implement `get.miner.setting` command
//!
//! This command is used to get the miner settings summary.
//!
//! - Command: [GetMinerSettings]
//! - ApiDoc: <https://apidoc.whatsminer.com/#api-Miner-btminer_get_settings>
use core::str;

use serde::Deserialize;

use crate::{command::Command, error::Result, response::Response};

/// This command represents the `get.miner.setting` operation.
///
/// It is used to get the miner settings summary.
///
/// - ApiDoc: <https://apidoc.whatsminer.com/#api-Miner-btminer_get_settings>
///
/// # Example
/// ```rust,ignore
/// use matroskin::actor::Actor;
/// use matroskin::command::get_miner_setting::{GetMinerSettings};
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
///     let cmd = GetMinerSettings::default();
///     let response = actor.send(&cmd).await?;
///     println!("Response: {:#?}", response);
///
///     Ok(())
/// }
///
#[derive(Debug, Default)]
pub struct GetMinerSettings;

/// [GetMinerSettings] Response
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetMinerSettingsResponse {
    pub power_limit: i64,
    pub upfreq_speed: i64,
    pub power_mode: String,
    pub fast_boot: String,
    pub target_freq: i64,
}

impl Command for GetMinerSettings {
    type Params = ();
    type Response = Response<GetMinerSettingsResponse>;
    const CMD_NAME: &'static str = "get.miner.setting";
    fn params(&self) -> Result<Option<String>> {
        Ok(None)
    }
}
