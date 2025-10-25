//! Implement `set.miner.pools` command
//!
//! It is used to set the miner's pools [SetMinerPoolsParam] for the miner.
//!
//! - Command: [SetMinerPools]
//! - ApiDoc: <https://apidoc.whatsminer.com/#api-Miner-btminer_set_pools>
use core::str;

use serde::Serialize;

use crate::{command::Command, error::Result, response::Response};

/// This command represents the `set.miner.pools` operation.
///
/// It is used to set the miner's pools [SetMinerPoolsParam] for the miner.
///
/// **note:** unstable
///
/// - ApiDoc: <https://apidoc.whatsminer.com/#api-Miner-btminer_set_pools>
///
/// ## Example
/// ```rust,ignore
/// use waru::actor::Actor;
/// use waru::account::Account;
/// use waru::password::Password;
/// use waru::command::set_miner_pools::{SetMinerPools, SetMinerPoolsParam};
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let actor = Actor::new("1.1.1.1:4433", Account::Super, Password::Super).await?;
///
///     let mut pools = SetMinerPoolsParam::new();
///
///     pools.push(SetMinerPoolsParamItem {
///         pool: "stratum+tcp://1.1.1.1:3333".to_string(),
///         worker: "waru.777".to_string(),
///         password: "test".to_string(),
///     });
///
///     let cmd = SetMinerPools(pools);
///     let res = cmd.execute(&actor).await?;
///     println!("res: {:#?}", res);
/// }
/// ```
#[derive(Debug, Default)]
pub struct SetMinerPools(pub SetMinerPoolsParam);

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
/// Pool configuration
pub struct SetMinerPoolsParamItem {
    /// Stratum url
    /// - required
    pub pool: String,
    /// Worker username
    /// - required
    ///
    /// like: 1-9A-z.1-9A-z
    pub worker: String,
    #[serde(rename = "passwd")]
    /// Password
    /// - not required
    pub password: String,
}
/// Represents a vector of [SetMinerPoolsParamItem].
pub type SetMinerPoolsParam = Vec<SetMinerPoolsParamItem>;

impl Command for SetMinerPools {
    type Params = SetMinerPoolsParam;
    type Response = Response<String>;
    const CMD_NAME: &'static str = "set.miner.pools";
    const SECURED: bool = true;
    const ENCRYPTED: bool = true;
    fn params(&self) -> Result<Option<String>> {
        Ok(Some(serde_json::to_string(&self.0)?))
    }
}

#[cfg(test)]
mod set_miner_fastboot {

    use crate::{account::Account, actor::Actor, auth_data::AuthData, password::Password};

    use super::*;

    #[test]
    fn view() {
        let cmd = SetMinerPools::default();
        let c = cmd
            .to_request(Some(
                AuthData::new::<SetMinerPools>(Account::Super, "test123", "test123").unwrap(),
            ))
            .unwrap();
        println!("{}", c)
    }

    #[tokio::test]
    async fn send_to_miner() {
        let actor = Actor::new("1.1.1.1:4433", Account::Super, Password::Super)
            .await
            .unwrap();
        let mut pools = SetMinerPoolsParam::new();
        pools.push(SetMinerPoolsParamItem {
            pool: "stratum+tcp://1.1.1.1:3333".to_string(),
            worker: "waru.777".to_string(),
            password: "test".to_string(),
        });

        let cmd = SetMinerPools(pools);
        let a = cmd.execute(&actor).await.unwrap();
        println!("{:#?}", a)
    }
}
