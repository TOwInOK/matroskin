//! Declare response module
//!
//! - msg is any T. Usual declare into [Command::Response]
//! - Item: [Response]
//! - (random command as example) ApiDoc: <https://apidoc.whatsminer.com/#api-Device-device_set_custom_data>
use serde::Deserialize;

#[cfg(doc)]
use crate::command::Command;

/// Represents a response from the WhatsMiner API.
#[derive(Debug, Clone, Deserialize)]
pub struct Response<T> {
    // TODO: make enum
    /// Response code:
    /// - -3: Parameter item is null
    /// - -2: Incorrect parameters and commands
    /// - -1: Invalid JSON or password is wrong
    /// - 0: OK
    pub code: i8,
    /// UNIX Timestamp
    pub when: u64,
    /// Response message
    ///
    /// automatically mapped to T
    pub msg: T,
    /// Called cmd
    ///
    /// mb it's more then that
    pub desc: String,
}
