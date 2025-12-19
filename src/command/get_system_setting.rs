//! Implement `get.system.setting` command
//!
//! This command is used to get the miner settings summary.
//!
//! - Command: [GetSystemSetting]
//! - ApiDoc: <https://apidoc.whatsminer.com/#api-System-get_system_settings>
use core::str;

use serde::Deserialize;

use crate::{command::Command, error::Result, response::Response};

/// This command represents the `get.system.setting` operation.
///
/// Retrieves a summary of the device's current system settings.
///
/// - ApiDoc: <https://apidoc.whatsminer.com/#api-System-get_system_settings>
///
/// # Example
/// ```rust,ignore
/// use matroskin::actor::Actor;
/// use matroskin::command::get_miner_setting::{GetSystemSetting};
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
///     let cmd = GetSystemSetting::default();
///     let response = actor.send(&cmd).await?;
///     println!("Response: {:#?}", response);
///
///     Ok(())
/// }
///
#[derive(Debug, Default)]
pub struct GetSystemSetting;

/// [GetSystemSetting] Response
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetSystemSettingResponse {
    /// Is web panel on?
    pub web_pool: i64,
    /// TimeZone UTC
    pub timezone: String,
    /// Zone name
    pub zonename: String,
    /// Current hostname for NET
    pub hostname: String,
    // idk, my miner does't has it
    pub log_upload: Option<GetSystemSettingsResponseLogUpload>,
    pub time_randomized: GetSystemSettingsResponseTimeRandomized,
    /// Time servers
    pub ntp_server: Vec<String>,
}
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetSystemSettingsResponseLogUpload {
    pub ip: String,
    pub port: String,
    pub proto: String,
}
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetSystemSettingsResponseTimeRandomized {
    pub start: i64,
    pub stop: i64,
}

impl Command for GetSystemSetting {
    type Params = ();
    type Response = Response<GetSystemSettingResponse>;
    const CMD_NAME: &'static str = "get.system.setting";
    fn params(&self) -> Result<Option<String>> {
        Ok(None)
    }
}

#[cfg(test)]
mod get_system_setting {

    use crate::{account::Account, actor::Actor, password::Password};

    use super::*;

    #[test]
    fn view() {
        let cmd = GetSystemSetting::default();
        let c = serde_json::to_string_pretty(&cmd.to_request(None).unwrap()).unwrap();
        println!("{:#?}", c);
        assert_eq!(c, "{\n  \"cmd\": \"get.system.setting\"\n}")
    }

    #[tokio::test]
    async fn to_miner() {
        let actor = Actor::new("10.10.10.20:4433", Account::Super, Password::Super)
            .await
            .unwrap();
        let cmd = GetSystemSetting;
        let a = cmd.execute(&actor).await.unwrap();
        actor.send(&cmd).await.unwrap();
        println!("{:#?}", a)
    }
}
