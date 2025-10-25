//! Implement `set.miner.fastboot` command
//!
//! This command is used to set the fastboot flag [SetMinerPoolsParam] for the miner.\
//!
//! - Command: [SetMinerFastboot]
//! - ApiDoc: <https://apidoc.whatsminer.com/#api-Miner-btminer_set_fastboot>
use core::str;

use crate::{command::Command, error::Result, response::Response};

/// This command represents the `set.miner.fastboot` operation.
///
/// It is used to set the fastboot flag for the miner.
///
/// - ApiDoc: <https://apidoc.whatsminer.com/#api-Miner-btminer_set_fastboot>
///
/// # Example
/// ```rust,ignore
/// use matroskin::actor::Actor;
/// use matroskin::command::set_miner_fastboot::{SetMinerFastboot};
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
///     let cmd = SetMinerFastboot(true); // Enable fastboot
///     let response = actor.send(&cmd).await?;
///     println!("Response: {:#?}", response);
///
///     Ok(())
/// }
///
#[derive(Debug, Default)]
pub struct SetMinerFastboot(pub SetMinerPoolsParam);

/// Type alias for the fastboot parameter (enable/disable)
pub type SetMinerPoolsParam = bool;

impl Command for SetMinerFastboot {
    type Params = SetMinerPoolsParam;
    type Response = Response<String>;
    const CMD_NAME: &'static str = "set.miner.fastboot";
    const SECURED: bool = true;
    fn params(&self) -> Result<Option<String>> {
        Ok(Some(if self.0 { "enable" } else { "disable" }.to_string()))
    }
}

#[cfg(test)]
mod set_miner_fastboot {

    use crate::{account::Account, actor::Actor, auth_data::AuthData, password::Password};

    use super::*;

    #[test]
    fn view() {
        let cmd = SetMinerFastboot::default();
        let c = serde_json::to_string_pretty(
            &cmd.to_request(Some(
                AuthData::new::<SetMinerFastboot>(
                    Account::Super,
                    Password::Super.as_ref(),
                    "test123",
                )
                .unwrap(),
            ))
            .unwrap(),
        )
        .unwrap();
        println!("{}", c)
    }

    #[tokio::test]
    async fn send_to_miner() {
        let actor = Actor::new("1.1.1.1:4433", Account::Super, Password::Super)
            .await
            .unwrap();
        let cmd = SetMinerFastboot(false);
        let a = cmd.execute(&actor).await.unwrap();
        actor.send(&cmd).await.unwrap();
        println!("{:#?}", a)
    }
}
