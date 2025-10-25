use core::str;

use serde::Serialize;

use crate::{account::Password, command::Command, error::Result, response::Response};

// Command
#[derive(Debug, Default)]
pub struct SetMinerPools(pub SetMinerPoolsParam);

// Params
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SetMinerPoolsParamItem {
    pub pool: String,
    pub worker: String,
    #[serde(rename = "passwd")]
    pub password: Password,
}
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

    use crate::{account::Account, actor::Actor, auth_data::AuthData};

    use super::*;

    #[test]
    fn view() {
        let cmd = SetMinerPools::default();
        let c = cmd
            .to_request(Some(
                AuthData::new::<SetMinerPools>(Account::Super, "test123", "test123").unwrap(),
            ))
            .unwrap();
        println!("{:#?}", c)
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
            password: Password::Super,
        });

        let cmd = SetMinerPools(pools);
        let a = cmd.execute(&actor).await.unwrap();
        println!("{:#?}", a)
    }
}
