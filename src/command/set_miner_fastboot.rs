use core::str;

use crate::{command::Command, error::Result, response::Response};

/// Command to set miner fastboot
#[derive(Debug, Default)]
pub struct SetMinerFastboot(SetMinerPoolsParam);

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

    use crate::{
        account::{Account, Password},
        actor::Actor,
        auth_data::AuthData,
    };

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
        let actor = Actor::new(
            "1.1.1.1:4433",
            crate::account::Account::Super,
            crate::account::Account::Super,
        )
        .await
        .unwrap();
        let cmd = SetMinerFastboot(false);
        let a = cmd.execute(&actor).await.unwrap();
        println!("{:#?}", a)
    }
}
