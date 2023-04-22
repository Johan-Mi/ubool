#![forbid(unsafe_code)]
#![warn(clippy::nursery, clippy::pedantic)]
#![no_std]

mod checks;
mod conversions;
mod misc;
mod ops;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Ubool {
    True,
    False,
    Unknown,
}

pub use Ubool::{False, True, Unknown};
