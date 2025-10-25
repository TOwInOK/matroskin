use serde::Serialize;

use crate::auth_data::AuthData;

/// Represents a request for building public endpoints
#[derive(Debug, Serialize)]
pub struct Request {
    /// Command name
    cmd: &'static str,
    /// Authentication data (optional)
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_data: Option<AuthData>,
    /// Command parameters (optional)
    #[serde(rename = "param")]
    #[serde(skip_serializing_if = "Option::is_none")]
    parametr: Option<String>,
}

impl Request {
    /// Creates a new Request instance.
    pub fn new(cmd: &'static str, auth_data: Option<AuthData>, parametr: Option<String>) -> Self {
        Self {
            cmd,
            auth_data,
            parametr,
        }
    }
}
