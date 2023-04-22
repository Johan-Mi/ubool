use crate::{False, True, Ubool, Unknown};
use core::{
    fmt::{self, Debug, Display},
    str::FromStr,
};

/// Returns the default value of `Unknown`
impl Default for Ubool {
    fn default() -> Self {
        Unknown
    }
}

impl Display for Ubool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            True => "true",
            False => "false",
            Unknown => "unknown",
        })
    }
}

impl Debug for Ubool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl FromStr for Ubool {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "true" | "True" => Ok(True),
            "false" | "False" => Ok(False),
            "unknown" | "Unknown" => Ok(Unknown),
            _ => Err(()),
        }
    }
}
