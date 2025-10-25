use std::fmt::Display;

use serde::Serialize;
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Debug, Serialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
/// Static, unchangeable users
///
/// https://apidoc.whatsminer.com/#api-Getting_Started-getting_start
pub enum Account {
    Super,
    User1,
    User2,
    User3,
}

impl From<Account> for Password {
    fn from(value: Account) -> Self {
        match value {
            Account::Super => Self::Super,
            Account::User1 => Self::User1,
            Account::User2 => Self::User2,
            Account::User3 => Self::User3,
        }
    }
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

#[derive(Debug, Serialize, Clone, PartialEq, Zeroize, ZeroizeOnDrop)]
#[serde(rename_all = "lowercase")]
/// Account's passwords.
/// By default it is the same as account name.
/// but it can be changed.
///
/// https://apidoc.whatsminer.com/#api-Getting_Started-getting_start
pub enum Password {
    Super,
    User1,
    User2,
    User3,
    // TODO: it can be useless
    Custom(String),
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        match self {
            Password::Super => "super",
            Password::User1 => "user1",
            Password::User2 => "user2",
            Password::User3 => "user3",
            Password::Custom(e) => e,
        }
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Password::Super => write!(f, "super"),
            Password::User1 => write!(f, "user1"),
            Password::User2 => write!(f, "user2"),
            Password::User3 => write!(f, "user3"),
            Password::Custom(e) => write!(f, "{}", e),
        }
    }
}
