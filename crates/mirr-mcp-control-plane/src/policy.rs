#![forbid(unsafe_code)]

use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Role {
    Reader,
    Builder,
    Committer,
    Admin,
}

impl Role {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reader => "reader",
            Self::Builder => "builder",
            Self::Committer => "committer",
            Self::Admin => "admin",
        }
    }
}

impl FromStr for Role {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "reader" => Ok(Self::Reader),
            "builder" => Ok(Self::Builder),
            "committer" => Ok(Self::Committer),
            "admin" => Ok(Self::Admin),
            _ => Err(format!("invalid role: {}", value)),
        }
    }
}
