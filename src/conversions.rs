use std::convert::TryFrom;

use crate::{False, True, Ubool, Unknown};

impl Ubool {
    pub const fn from_bool(b: bool) -> Self {
        match b {
            true => True,
            false => False,
        }
    }
}

impl Ubool {
    pub const fn into_bool(self) -> Option<bool> {
        match self {
            True => Some(true),
            False => Some(false),
            Unknown => None,
        }
    }
}

impl From<bool> for Ubool {
    fn from(b: bool) -> Self {
        Self::from_bool(b)
    }
}

impl From<Option<bool>> for Ubool {
    fn from(ob: Option<bool>) -> Self {
        match ob {
            Some(true) => True,
            Some(false) => False,
            None => Unknown,
        }
    }
}

impl From<Ubool> for Option<bool> {
    fn from(ub: Ubool) -> Self {
        match ub {
            True => Some(true),
            False => Some(false),
            Unknown => None,
        }
    }
}

impl TryFrom<Ubool> for bool {
    type Error = ();

    fn try_from(ub: Ubool) -> Result<Self, Self::Error> {
        match ub {
            True => Ok(true),
            False => Ok(false),
            Unknown => Err(()),
        }
    }
}
