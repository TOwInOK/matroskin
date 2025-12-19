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
    // TODO: Make enum with power mode types
    pub power_mode: String,
    // TODO: swap to `bool` type,
    // create bool <--> string converter
    pub fast_boot: String,
    pub target_freq: i64,
    // TODO: swap to `bool` type,
    // create bool <--> string converter
    //
    // since 3.0.3v
    pub fast_mining: Option<String>,
    // since 3.0.3v
    pub power: Option<i64>,
    // since 3.0.3v
    pub power_percent: Option<i64>,
}

impl Command for GetMinerSettings {
    type Params = ();
    type Response = Response<GetMinerSettingsResponse>;
    const CMD_NAME: &'static str = "get.miner.setting";
    fn params(&self) -> Result<Option<String>> {
        Ok(None)
    }
}

#[cfg(test)]
mod get_miner_setting {

    use crate::{account::Account, actor::Actor, password::Password};

    use super::*;

    #[test]
    fn view() {
        let cmd = GetMinerSettings::default();
        let c = serde_json::to_string_pretty(&cmd.to_request(None).unwrap()).unwrap();
        assert_eq!(c, "{\n  \"cmd\": \"get.miner.setting\"\n}")
    }

    #[tokio::test]
    async fn to_miner() {
        let actor = Actor::new("10.10.10.20:4433", Account::Super, Password::Super)
            .await
            .unwrap();
        let cmd = GetMinerSettings;
        let a = cmd.execute(&actor).await.unwrap();
        actor.send(&cmd).await.unwrap();
        println!("{:#?}", a)
    }
}
