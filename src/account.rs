//! Define account module
//!
//! - Item: [Account]
//! - ApiDoc: <https://apidoc.whatsminer.com/#api-Getting_Started-getting_start>
use std::fmt::Display;

use serde::Serialize;

#[derive(Debug, Serialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
/// Static, unchangeable users
///
/// - ApiDoc: <https://apidoc.whatsminer.com/#api-Getting_Started-getting_start>
pub enum Account {
    Super,
    User1,
    User2,
    User3,
}

impl Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Account::Super => write!(f, "super"),
            Account::User1 => write!(f, "user1"),
            Account::User2 => write!(f, "user2"),
            Account::User3 => write!(f, "user3"),
        }
    }
}

impl AsRef<str> for Account {
    fn as_ref(&self) -> &str {
        match self {
            Account::Super => "super",
            Account::User1 => "user1",
            Account::User2 => "user2",
            Account::User3 => "user3",
        }
    }
}
