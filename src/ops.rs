use crate::{False, True, Ubool, Unknown};
use core::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
};

impl BitAnd for Ubool {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self.into_bool(), rhs.into_bool()) {
            (Some(lhs), Some(rhs)) => (lhs && rhs).into(),
            (Some(false), None) | (None, Some(false)) => False,
            _ => Unknown,
        }
    }
}

impl BitAndAssign for Ubool {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl BitOr for Ubool {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self.into_bool(), rhs.into_bool()) {
            (Some(lhs), Some(rhs)) => (lhs || rhs).into(),
            (Some(true), None) | (None, Some(true)) => True,
            _ => Unknown,
        }
    }
}

impl BitOrAssign for Ubool {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitXor for Ubool {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self.into_bool(), rhs.into_bool()) {
            (Some(lhs), Some(rhs)) => (lhs ^ rhs).into(),
            _ => Unknown,
        }
    }
}

impl BitXorAssign for Ubool {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl Not for Ubool {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.into_bool().map_or(Unknown, |b| (!b).into())
    }
}

impl Ubool {
    #[must_use]
    pub fn strict_eq(self, other: Self) -> Self {
        !(self ^ other)
    }
}
