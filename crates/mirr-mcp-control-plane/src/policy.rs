#![forbid(unsafe_code)]

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

    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "reader" => Some(Self::Reader),
            "builder" => Some(Self::Builder),
            "committer" => Some(Self::Committer),
            "admin" => Some(Self::Admin),
            _ => None,
        }
    }
}
