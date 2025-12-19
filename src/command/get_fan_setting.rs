//! Implement `get.fan.setting` command
//!
//! This command is used to get the miner settings summary.
//!
//! - Command: [GetFanSettings]
//! - ApiDoc: <https://apidoc.whatsminer.com/#api-Fan-get_fan_settings>
use core::str;

use serde::Deserialize;

use crate::{command::Command, error::Result, response::Response};

/// This command represents the `get.fan.setting` operation.
///
/// It is used to get the miner settings summary.
///
/// - ApiDoc: <https://apidoc.whatsminer.com/#api-Fan-get_fan_settings>
///
/// # Example
/// ```rust,ignore
/// use matroskin::actor::Actor;
/// use matroskin::command::get_fan_settings::GetFanSettings;
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
///     let cmd = GetFanSettings::default();
///     let response = actor.send(&cmd).await?;
///     println!("Response: {:#?}", response);
///
///     Ok(())
/// }
///
#[derive(Debug, Default)]
pub struct GetFanSettings;

/// [GetFanSettings] Response
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetFanSettingsResponse {
    pub fan_poweroff_cool: u8,
    pub fan_zero_speed: i64,
    pub fan_temp_offset: i64,
}

impl Command for GetFanSettings {
    type Params = ();
    type Response = Response<GetFanSettingsResponse>;
    const CMD_NAME: &'static str = "get.fan.setting";
    fn params(&self) -> Result<Option<String>> {
        Ok(None)
    }
}

#[cfg(test)]
mod get_fan_settings {

    use crate::{account::Account, actor::Actor, password::Password};

    use super::*;

    #[test]
    fn view() {
        let cmd = GetFanSettings::default();
        let c = serde_json::to_string_pretty(&cmd.to_request(None).unwrap()).unwrap();
        assert_eq!(c, "{\n  \"cmd\": \"get.fan.setting\"\n}")
    }

    #[tokio::test]
    async fn to_miner() {
        let actor = Actor::new("10.10.10.20:4433", Account::Super, Password::Super)
            .await
            .unwrap();
        let cmd = GetFanSettings;
        let a = cmd.execute(&actor).await.unwrap();
        actor.send(&cmd).await.unwrap();
        println!("{:#?}", a)
    }
}
