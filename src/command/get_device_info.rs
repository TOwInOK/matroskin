use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{command::Command, error::Result, response::Response};

/// Command to get device info
#[derive(Debug, Default)]
pub struct GetDeviceInfo(pub GetDeviceInfoParam);

/// Parameters for the GetDeviceInfo command
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetDeviceInfoParam {
    /// Include miner (ASIC) information
    pub miner: bool,
    /// Include power information
    pub power: bool,
    /// Include network information
    pub network: bool,
    /// Include system information
    pub system: bool,
    /// Include salt information
    pub salt: bool,
    /// Include error code information
    pub error_code: bool,
}
impl Default for GetDeviceInfoParam {
    fn default() -> Self {
        Self {
            miner: true,
            power: true,
            network: true,
            system: true,
            salt: true,
            error_code: true,
        }
    }
}

impl Command for GetDeviceInfo {
    type Params = GetDeviceInfoParam;
    type Response = Response<DeviceInfo>;
    const CMD_NAME: &'static str = "get.device.info";

    fn params(&self) -> Result<Option<String>> {
        Ok({
            if self.0.miner
                && self.0.error_code
                && self.0.network
                && self.0.power
                && self.0.salt
                && self.0.system
                || !self.0.miner
                    && !self.0.error_code
                    && !self.0.network
                    && !self.0.power
                    && !self.0.salt
                    && !self.0.system
            {
                None
            } else {
                let mut out = Vec::with_capacity(5);
                if self.0.error_code {
                    out.push("error-code");
                }
                if self.0.network {
                    out.push("network");
                }
                if self.0.power {
                    out.push("power");
                }
                if self.0.salt {
                    out.push("salt");
                }
                if self.0.system {
                    out.push("system");
                }
                Some(out.join(","))
            }
        })
    }
}

/// Response containing device information
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DeviceInfo {
    /// Network information
    pub network: Option<Network>,
    /// Miner information
    pub miner: Option<Miner>,
    /// System information
    pub system: Option<System>,
    /// Power information
    pub power: Option<Power>,
    /// Salt value
    pub salt: Option<String>,
    // TODO: make type for that
    /// Error codes
    ///
    /// Expect reason and number of error
    pub error_codes: Option<Vec<HashMap<String, String>>>,
}

/// Network information
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Network {
    /// IP address
    pub ip: String,
    /// Protocol
    pub proto: String,
    /// Netmask
    pub netmask: String,
    /// DNS server
    pub dns: String,
    /// MAC address
    pub mac: String,
    /// Gateway address
    pub gateway: String,
    /// Hostname
    pub hostname: String,
}

/// Miner information
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Miner {
    /// Working status
    pub working: String,
    /// Miner type
    #[serde(rename = "type")]
    pub r#type: String,
    /// Hash board type
    pub hash_board: String,
    /// Detected hash rate
    pub detect_hash_rate: String,
    // TODO: make type, cause it's static
    /// Coin type
    pub cointype: String,
    /// Pool strategy
    pub pool_strategy: String,
    /// Heat mode
    pub heatmode: String,
    /// Hash percent
    pub hash_percent: String,
    /// info about liquid cooling
    pub eeprom_liquid_cooling: Option<String>,
    /// Chip data 0
    pub chipdata0: String,
    /// Chip data 1
    pub chipdata1: String,
    /// Chip data 2
    pub chipdata2: String,
    // TODO: make it bool
    /// Fast boot status
    pub fast_boot: String,
    /// Board number
    pub board_num: String,
    /// PCB serial number 0
    pub pcbsn0: String,
    /// PCB serial number 1
    pub pcbsn1: String,
    /// PCB serial number 2
    pub pcbsn2: String,
    /// Miner serial number
    pub miner_sn: String,
    /// Power limit set
    pub power_limit_set: String,
    /// Web pool
    pub web_pool: u32,
    #[serde(rename = "UpfreqSpeed")]
    pub upfreq_speed: Option<String>,
    /// Permission
    pub permission: Option<String>,
}

/// System information
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct System {
    /// API version
    pub api: String,
    /// Platform
    pub platform: String,
    /// Firmware version
    pub fwversion: String,
    /// Control board version
    pub control_board_version: String,
    // TODO: IDK, needs to find out
    pub btrom: Option<String>,
    /// API switch status
    ///
    /// - 0 (disable)
    /// - 1 (enable)
    pub apiswitch: String,
    /// LED status
    pub ledstatus: String,
}

/// Power supply information
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Power {
    #[serde(rename = "type")]
    /// Power supply type
    pub r#type: String,
    /// Power supply mode
    pub mode: String,
    /// Hardware version
    pub hwversion: String,
    /// Software version
    pub swversion: String,
    /// Model name
    pub model: String,
    /// Input current (A)
    pub iin: f32,
    /// Input voltage (V)
    pub vin: f32,
    /// Output voltage (100mV unit), i.e., 12.09V
    pub vout: i32,
    /// Output power (W)
    pub pin: i32,
    /// Fan speed (RPM)
    pub fanspeed: i32,
    /// Temperature (°C)
    pub temp0: f32,
    /// Serial number
    pub sn: String,
    /// Vendor ID
    pub vendor: String,
}
