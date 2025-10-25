<div align="center">
    <h1>matroskin</h1>
    <img src=".content/logo.webp"/>
    <p><i>Library for interacting with WhatsMiner ASIC miners.</i></p>
    <p><i><b>⚠️ Currently under active development ⚠️</b></i></p>

</div>

---

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Apache licensed][Apache2-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/matroskin.svg
[crates-url]: https://crates.io/crates/matroskin
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/tokio-rs/tokio/blob/master/LICENSE-MIT
[Apache2-badge]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[mit-url]: https://github.com/tokio-rs/tokio/blob/master/LICENSE-APACHE

## Overview

matroskin is an actor-tokio based library for creating tool for ASIC infrastructure.
It provides:
* Easy to use Actor system
* Build in commands
* Easy scalable architecture

## 🚧 Development Status
Marks:
- Ready to use: ✅
- Unstable (do not use it in any ways, only at your own risk): ⚠️
---
[list of commands](https://apidoc.whatsminer.com/):
- [ ] [get.device.custom_data](https://apidoc.whatsminer.com/#api-Device-device_get_custom_data)
- [x] ✅ [get.device.info](https://apidoc.whatsminer.com/#api-Device-device_get_info)
- [ ] [set.device.custom_data](https://apidoc.whatsminer.com/#api-Device-device_set_custom_data)
- [ ] [get.fan.setting](https://apidoc.whatsminer.com/#api-Fan-btminer_get_fansettings)
- [ ] [set.fan.poweroff_cool](https://apidoc.whatsminer.com/#api-Fan-btminer_poweroff_cool)
- [ ] [set.fan.temp_offset](https://apidoc.whatsminer.com/#api-Fan-fan_set_temp_offset)
- [ ] [set.fan.zero_speed](https://apidoc.whatsminer.com/#api-Fan-btminer_zero_speed)
- [ ] [get.log.download](https://apidoc.whatsminer.com/#api-Log-syslog_download)
- [ ] [set.log.upload](https://apidoc.whatsminer.com/#api-Log-syslog_upload)
- [ ] [get.miner.history](https://apidoc.whatsminer.com/#api-Miner-btminer_get_history)
- [ ] [get.miner.setting](https://apidoc.whatsminer.com/#api-Miner-btminer_get_settings)
- [ ] [get.miner.status](https://apidoc.whatsminer.com/#api-Miner-btminer_get_status)
- [ ] [set.miner.cointype](https://apidoc.whatsminer.com/#api-Miner-btminer_set_cointype)
- [x] ✅ [set.miner.fastboot](https://apidoc.whatsminer.com/#api-Miner-btminer_set_fastboot)
- [ ] [set.miner.heat_mode](https://apidoc.whatsminer.com/#api-Miner-btminer_set_heat_mode)
- [x] ⚠️ [set.miner.pools](https://apidoc.whatsminer.com/#api-Miner-btminer_set_pools)
- [ ] [set.miner.power](https://apidoc.whatsminer.com/#api-Miner-btminer_set_power)
- [ ] [set.miner.power_limit](https://apidoc.whatsminer.com/#api-Miner-btminer_power_limit)
- [ ] [set.miner.power_mode](https://apidoc.whatsminer.com/#api-Miner-btminer_power_mode)
- [ ] [set.miner.power_percent](https://apidoc.whatsminer.com/#api-Miner-btminer_set_power_percent)
- [ ] [set.miner.report](https://apidoc.whatsminer.com/#api-Miner-btminer_report)
- [ ] [set.miner.restore_setting](https://apidoc.whatsminer.com/#api-Miner-btminer_restore)
- [ ] [set.miner.service](https://apidoc.whatsminer.com/#api-Miner-btminer_service_set)
- [ ] [set.miner.target_freq](https://apidoc.whatsminer.com/#api-Miner-btminer_set_targetfreq)
- [ ] [set.miner.upfreq_speed](https://apidoc.whatsminer.com/#api-Miner-btminer_upfreq_speed)
- [ ] [get.system.setting](https://apidoc.whatsminer.com/#api-System-btminer_get_systemsettings)
- [ ] [set.system.factory_reset](https://apidoc.whatsminer.com/#api-System-system_factory_reset)
- [ ] [set.system.hostname](https://apidoc.whatsminer.com/#api-System-system_set_hostname)
- [ ] [set.system.led](https://apidoc.whatsminer.com/#api-System-system_set_led)
- [ ] [set.system.net_config](https://apidoc.whatsminer.com/#api-System-system_net_config)
- [ ] [set.system.ntp_server](https://apidoc.whatsminer.com/#api-System-system_set_ntp)
- [ ] [set.system.reboot](https://apidoc.whatsminer.com/#api-System-system_reboot)
- [ ] [set.system.time_randomized](https://apidoc.whatsminer.com/#api-System-system_set_time_randomiz)
- [ ] [set.system.timezone](https://apidoc.whatsminer.com/#api-System-system_set_timezone)
- [ ] [set.system.update_firmware](https://apidoc.whatsminer.com/#api-System-system_update_firmware)
- [ ] [set.system.webpools](https://apidoc.whatsminer.com/#api-System-system_set_webpools)
- [ ] [set.user.change_passwd](https://apidoc.whatsminer.com/#api-User-user_set_passwd)
- [ ] [set.user.permission](https://apidoc.whatsminer.com/#api-User-user_set_permission)

## Example
```toml
[dependencies]
tokio = { version = "1.48.0", features = ["full"] }
matroskin = { version = "0" }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "fmt"] }
```

Then, on your main.rs:

```rust,no_run
use tracing::Level;
use matroskin::{
    account::Account,
    actor::Actor,
    command::{Command, set_miner_fastboot::SetMinerFastboot},
    password::Password,
};

#[tokio::main]
async fn main() {
    // Init logger
    use tracing_subscriber::FmtSubscriber;
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .compact()
            .with_max_level(level)
            .without_time()
            .finish(),
    ).expect("Fail to set global default subscriber");

    // Create connection with ASIC
    let actor = Actor::new("10.10.10.10:4433", Account::Super, Password::Super).await.expect("fail to establish connection");

    // Create command
    let command = GetDeviceInfo::default();
    // Run command
    //
    // - First variant (actor execute cmd)
    // - Second variant (run cmd using actor)
    let response = actor.send(&command).await.expect("fail to execute"); // or  let response = cmd.execute(&actor).await.unwrap();

    println!("All Device Info: {:#?}", response);
    Ok(())
```
