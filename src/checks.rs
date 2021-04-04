use crate::{False, True, Ubool, Unknown};

impl Ubool {
    pub const fn is_known(self) -> bool {
        matches!(self, True | False)
    }

    pub const fn is_unknown(self) -> bool {
        !self.is_known()
    }

    pub const fn is_true(self) -> bool {
        matches!(self, True)
    }

    pub const fn is_false(self) -> bool {
        matches!(self, False)
    }

    pub const fn can_be_true(self) -> bool {
        matches!(self, True | Unknown)
    }

    pub const fn can_be_false(self) -> bool {
        matches!(self, False | Unknown)
    }
}
