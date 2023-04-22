use crate::{False, True, Ubool, Unknown};

impl Ubool {
    #[must_use]
    pub const fn is_known(self) -> bool {
        matches!(self, True | False)
    }

    #[must_use]
    pub const fn is_unknown(self) -> bool {
        !self.is_known()
    }

    #[must_use]
    pub const fn is_true(self) -> bool {
        matches!(self, True)
    }

    #[must_use]
    pub const fn is_false(self) -> bool {
        matches!(self, False)
    }

    #[must_use]
    pub const fn can_be_true(self) -> bool {
        matches!(self, True | Unknown)
    }

    #[must_use]
    pub const fn can_be_false(self) -> bool {
        matches!(self, False | Unknown)
    }
}
