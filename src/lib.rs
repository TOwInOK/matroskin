#![doc(html_favicon_url = ".content/favicon.ico")]

//! # `matroskin`
//!
//! WhatsMiner v3 ApiDoc: <https://apidoc.whatsminer.com>
//!
//! `matroskin` is a Rust library designed for interacting with WhatsMiner devices.
//! It provides an asynchronous API to send commands and receive responses from WhatsMiner hardware,
//! facilitating monitoring and control functionalities.
//!
//! **Warning:** This library is developed primarily for personal use and may not be
//! fully optimized or extensively tested for public production environments.
//!
//! # Usage Example
//!
//! ```rust,ignore
//! use matroskin::actor::{Actor, Account, Password};
//! use matroskin::command::SetMinerFastboot;
//!
//! #[tokio::main]
//! async fn main() {
//!     // 0. (Optional) init logger
//!     use tracing_subscriber::FmtSubscriber;
//!     tracing::subscriber::set_global_default(
//!         FmtSubscriber::builder()
//!             .compact()
//!             .with_max_level(level)
//!             .without_time()
//!             .finish(),
//!     ).expect("Fail to set global default subscriber");
//!
//!     // 1. Initialize the actor with the device IP, account, and password.
//!     let actor = Actor::new("1.1.1.1:4433", Account::Super, Password::Super)
//!        .await
//!        .expect("Failed to create actor");
//!
//!     // 2. Create a command to send to the WhatsMiner device.
//!     let cmd = SetMinerFastboot(false);
//!
//!     // 3.1 Execute the command directly.
//!     let response = cmd.execute(&actor).await.expect("Failed to execute command");
//!     println!("Response via command.execute: {:#?}", response);
//!
//!     // 3.2 Alternatively, send the command via the actor.
//!     let response_from_actor = actor.send(&cmd).await.expect("Failed to send command via actor");
//!     println!("Response via actor.send: {:#?}", response_from_actor);
//! }
//! ```

pub mod account;
pub mod actor;
pub mod auth_data;
pub mod command;
pub mod error;
pub mod password;
pub mod request;
pub mod response;
