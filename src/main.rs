use tracing::Level;
use waru::{
    account::Account,
    actor::Actor,
    command::{Command, set_miner_fastboot::SetMinerFastboot},
    password::Password,
};

#[tokio::main]
async fn main() {
    // 0. (Optional) init logger
    use tracing_subscriber::FmtSubscriber;
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .compact()
            .with_max_level(Level::INFO)
            .without_time()
            .finish(),
    )
    .expect("Fail to set global default subscriber");
    // 1. Initialize the actor with the device IP, account, and password.
    let actor = Actor::new("1.1.1.1:4433", Account::Super, Password::Super)
        .await
        .expect("Failed to create actor");
    // 2. Create a command to send to the WhatsMiner device.
    let cmd = SetMinerFastboot(false);
    // 3.1 Execute the command directly.
    let response = cmd
        .execute(&actor)
        .await
        .expect("Failed to execute command");
    println!("Response via command.execute: {:#?}", response);
    // 3.2 Alternatively, send the command via the actor.
    let response_from_actor = actor
        .send(&cmd)
        .await
        .expect("Failed to send command via actor");
    println!("Response via actor.send: {:#?}", response_from_actor);
}
