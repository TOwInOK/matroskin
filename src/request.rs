//! Declare request module
//!
//! - Item: [Request]
//! - (about [Command::SECURED], [Command::ENCRYPTED], [AuthData::new], [AuthData::encrypt]) ApiDoc: <https://apidoc.whatsminer.com/#api-Token-generate_token>
use std::fmt::{Debug, Display};

#[cfg(doc)]
use crate::command::Command;

use serde::Serialize;

use crate::auth_data::AuthData;

/// Represents a request for building public endpoints
#[derive(Debug, Serialize)]
pub struct Request {
    /// Command name
    pub cmd: &'static str,
    /// Authentication data (optional)
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_data: Option<AuthData>,
    /// Command parameters (optional)
    #[serde(rename = "param")]
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter: Option<String>,
}

impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Request")
            .field("cmd", &self.cmd)
            .field(
                "auth_data",
                &match self.auth_data {
                    Some(_) => "<hidden>",
                    None => "None",
                },
            )
            .field("parameter", &"<hidden>")
            .finish()
    }
}

impl Request {
    /// Creates a new [Request] instance.
    pub fn new(cmd: &'static str, auth_data: Option<AuthData>, parameter: Option<String>) -> Self {
        Self {
            cmd,
            auth_data,
            parameter,
        }
    }
}
