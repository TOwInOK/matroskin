use serde::Deserialize;

/// Represents a response from the WhatsMiner API.
#[derive(Debug, Clone, Deserialize)]
pub struct Response<T> {
    /// Response code:
    /// - -3: Parameter item is null
    /// - -2: Incorrect parameters and commands
    /// - -1: Invalid JSON or password is wrong
    /// - 0: OK
    // TODO: make enum
    pub code: i8,
    /// UNIX Timestamp
    pub when: u64,
    pub msg: T,
    pub desc: String,
}
