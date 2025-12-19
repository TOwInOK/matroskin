//! Trait for Whatsminer commands
//!
//! ## Base trait for buildings commands
//! - [Command]
//! ### Example
//! - example command [SetMinerFastboot]
//! ```
//! use matroskin::{command::Command, error::Result, response::Response};
//! #[derive(Debug, Default)]
//! pub struct SetMinerFastboot(SetMinerPoolsParam);
//!
//! /// Type alias for the fastboot parameter (enable/disable)
//! pub type SetMinerPoolsParam = bool;
//!
//! impl Command for SetMinerFastboot {
//!     type Params = SetMinerPoolsParam;
//!     type Response = Response<String>;
//!     const CMD_NAME: &'static str = "set.miner.fastboot";
//!     const SECURED: bool = true;
//!     fn params(&self) -> Result<Option<String>> {
//!         Ok(Some(if self.0 { "enable" } else { "disable" }.to_string()))
//!     }
//! }
//! ```
//! ## Marks:
//! - Ready to use: ✅
//! - Unstable (do not use it in any ways, only at your own risk): ⚠️
//! - without marks: not tested, only at your own risk
//!
//! ## [list of commands](https://apidoc.whatsminer.com/):
//! - [x] [get.device.custom_data](https://apidoc.whatsminer.com/#api-Device-device_get_custom_data)
//! - [x] ✅ [get.device.info](https://apidoc.whatsminer.com/#api-Device-device_get_info)
//! - [ ] [set.device.custom_data](https://apidoc.whatsminer.com/#api-Device-device_set_custom_data)
//! - [x] ✅ [get.fan.setting](https://apidoc.whatsminer.com/#api-Fan-btminer_get_fansettings)
//! - [ ] [set.fan.poweroff_cool](https://apidoc.whatsminer.com/#api-Fan-btminer_poweroff_cool)
//! - [ ] [set.fan.temp_offset](https://apidoc.whatsminer.com/#api-Fan-fan_set_temp_offset)
//! - [ ] [set.fan.zero_speed](https://apidoc.whatsminer.com/#api-Fan-btminer_zero_speed)
//! - [ ] [get.log.download](https://apidoc.whatsminer.com/#api-Log-syslog_download)
//! - [ ] [set.log.upload](https://apidoc.whatsminer.com/#api-Log-syslog_upload)
//! - [ ] [get.miner.history](https://apidoc.whatsminer.com/#api-Miner-btminer_get_history)
//! - [x] [get.miner.setting](https://apidoc.whatsminer.com/#api-Miner-btminer_get_settings)
//! - [ ] [get.miner.status](https://apidoc.whatsminer.com/#api-Miner-btminer_get_status)
//! - [ ] [set.miner.cointype](https://apidoc.whatsminer.com/#api-Miner-btminer_set_cointype)
//! - [ ] [set.miner.fast_hash](https://apidoc.whatsminer.com/#api-Miner-set_fast_mining)
//! - [x] ✅ [set.miner.fastboot](https://apidoc.whatsminer.com/#api-Miner-btminer_set_fastboot)
//! - [ ] [set.miner.heat_mode](https://apidoc.whatsminer.com/#api-Miner-btminer_set_heat_mode)
//! - [x] ⚠️ [set.miner.pools](https://apidoc.whatsminer.com/#api-Miner-btminer_set_pools)
//! - [ ] [set.miner.power](https://apidoc.whatsminer.com/#api-Miner-btminer_set_power)
//! - [ ] [set.miner.power_limit](https://apidoc.whatsminer.com/#api-Miner-btminer_power_limit)
//! - [ ] [set.miner.power_mode](https://apidoc.whatsminer.com/#api-Miner-btminer_power_mode)
//! - [ ] [set.miner.power_percent](https://apidoc.whatsminer.com/#api-Miner-btminer_set_power_percent)
//! - [ ] [set.miner.report](https://apidoc.whatsminer.com/#api-Miner-btminer_report)
//! - [ ] [set.miner.restore_setting](https://apidoc.whatsminer.com/#api-Miner-btminer_restore)
//! - [ ] [set.miner.service](https://apidoc.whatsminer.com/#api-Miner-btminer_service_set)
//! - [ ] [set.miner.target_freq](https://apidoc.whatsminer.com/#api-Miner-btminer_set_targetfreq)
//! - [ ] [set.miner.upfreq_speed](https://apidoc.whatsminer.com/#api-Miner-btminer_upfreq_speed)
//! - [ ] [get.system.setting](https://apidoc.whatsminer.com/#api-System-btminer_get_systemsettings)
//! - [ ] [set.system.factory_reset](https://apidoc.whatsminer.com/#api-System-system_factory_reset)
//! - [ ] [set.system.hostname](https://apidoc.whatsminer.com/#api-System-system_set_hostname)
//! - [ ] [set.system.led](https://apidoc.whatsminer.com/#api-System-system_set_led)
//! - [ ] [set.system.net_config](https://apidoc.whatsminer.com/#api-System-system_net_config)
//! - [ ] [set.system.ntp_server](https://apidoc.whatsminer.com/#api-System-system_set_ntp)
//! - [ ] [set.system.reboot](https://apidoc.whatsminer.com/#api-System-system_reboot)
//! - [ ] [set.system.time_randomized](https://apidoc.whatsminer.com/#api-System-system_set_time_randomized)
//! - [ ] [set.system.timezone](https://apidoc.whatsminer.com/#api-System-system_set_timezone)
//! - [ ] [set.system.update_firmware](https://apidoc.whatsminer.com/#api-System-system_update_firmware)
//! - [ ] [set.system.webpools](https://apidoc.whatsminer.com/#api-System-system_set_webpools)
//! - [ ] [set.user.change_passwd](https://apidoc.whatsminer.com/#api-User-user_set_passwd)
//! - [ ] [set.user.permission](https://apidoc.whatsminer.com/#api-User-user_set_permission)

pub mod get_device_custom_data;
pub mod get_device_info;
pub mod get_fan_settings;
pub mod get_miner_setting;
pub mod get_system_setting;
pub mod set_miner_fastboot;
pub mod set_miner_pools;

#[cfg(doc)]
use crate::command::set_miner_fastboot::SetMinerFastboot;
#[cfg(doc)]
use crate::response::Response;
use crate::{
    actor::{Actor, message::ActorMessage},
    auth_data::AuthData,
    error::{Error, Result},
    request::Request,
};
use core::str;
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;
use tracing::{debug, trace};

/// Base trait for buildings commands
///
/// # Example
/// - example command [SetMinerFastboot]
/// ```
/// use matroskin::{command::Command, error::Result, response::Response};
/// #[derive(Debug, Default)]
/// pub struct SetMinerFastboot(SetMinerPoolsParam);
///
/// /// Type alias for the fastboot parameter (enable/disable)
/// pub type SetMinerPoolsParam = bool;
///
/// impl Command for SetMinerFastboot {
///     type Params = SetMinerPoolsParam;
///     type Response = Response<String>;
///     const CMD_NAME: &'static str = "set.miner.fastboot";
///     const SECURED: bool = true;
///     fn params(&self) -> Result<Option<String>> {
///         Ok(Some(if self.0 { "enable" } else { "disable" }.to_string()))
///     }
/// }
/// ```
pub trait Command {
    /// Parameter for command
    type Params: Serialize + Default;
    /// [Response] from command
    type Response: for<'a> Deserialize<'a>;
    /// Represents the command name
    const CMD_NAME: &'static str;
    /// Include AuthData in request?
    ///
    /// - data: [AuthData]
    /// - where: [Command::execute]
    const SECURED: bool = false;
    /// Encrypt params data?
    ///
    /// - data: [AuthData]
    /// - where [AuthData::encrypt]
    const ENCRYPTED: bool = false;

    /// Return local params
    fn params(&self) -> Result<Option<String>>;

    /// Convert command to request
    fn to_request<'a, 'b: 'a>(&'a self, auth_data: Option<AuthData>) -> Result<Request> {
        let parameter = self.params()?;

        let parameter = if let Some(data) = &auth_data {
            if let Some(x) = parameter {
                if Self::ENCRYPTED {
                    Some(data.encrypt(x.as_bytes())?)
                } else {
                    Some(x)
                }
            } else {
                parameter
            }
        } else if Self::ENCRYPTED {
            return Err(Error::CommandSholdHaveAuthData(Self::CMD_NAME));
        } else {
            parameter
        };

        Ok(Request::new(Self::CMD_NAME, auth_data, parameter))
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
