use crate::{False, True, Ubool, Unknown};
use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
};

impl BitAnd for Ubool {
    type Output = Ubool;

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
        *self = *self & rhs
    }
}

impl BitOr for Ubool {
    type Output = Ubool;

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
        *self = *self | rhs
    }
}

impl BitXor for Ubool {
    type Output = Ubool;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self.into_bool(), rhs.into_bool()) {
            (Some(lhs), Some(rhs)) => (lhs ^ rhs).into(),
            _ => Unknown,
        }
    }
}

impl BitXorAssign for Ubool {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs
    }
}

impl Not for Ubool {
    type Output = Ubool;

    fn not(self) -> Self::Output {
        match self.into_bool() {
            Some(b) => (!b).into(),
            None => Unknown,
        }
    }
}
