#![allow(rustdoc::bare_urls)]
#![warn(unreachable_pub)]
#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

mod core;
pub use crate::core::*;
mod signed;
mod unsigned;

#[cfg(target_arch = "x86_64")]
mod simd;

#[cfg(target_arch = "x86_64")]
mod imp {
    pub(crate) use crate::simd::parse_16;
}

#[cfg(not(target_arch = "x86_64"))]
mod fallback;

#[cfg(not(target_arch = "x86_64"))]
mod imp {
    pub(crate) use crate::fallback::parse_16;
}

pub trait FromRadix10Checked {
    fn from_radix_10_checked(_: &[u8]) -> Result<Self, ()>
    where
        Self: Sized;
}

pub trait FromRadix10CheckedNoPlus {
    fn from_radix_10_checked_no_plus(_: &[u8]) -> Result<Self, ()>
    where
        Self: Sized;
}

/// Parses an integer from the bytes of the pattern:
/// - unsigned: `r"\+?[0-9]+$"`
/// - signed: `r"(\+|-)?[0-9]+$"`
///
/// The expected format is the exact same as `std::str::FromStr`.
#[inline(always)]
pub fn atoi<I: FromRadix10Checked>(text: &[u8]) -> Result<I, ()> {
    I::from_radix_10_checked(text)
}

/// Parses an integer from the bytes of the pattern:
/// - unsigned: `r"[0-9]+$"`
///
/// The expected format is the exact same as `std::str::FromStr`, without the optional leading '+'.
#[inline(always)]
pub fn atoi_no_plus<I: FromRadix10CheckedNoPlus>(text: &[u8]) -> Result<I, ()> {
    I::from_radix_10_checked_no_plus(text)
}

/// Parses an integer from the bytes of the pattern:
/// - unsigned: `r"\+?[0-9]+$"`
/// - signed: `r"(\+|-)?[0-9]+$"`
///
/// The expected format is the exact same as `std::str::FromStr`.
pub trait ToRadix10Checked<T> {
    fn parse_radix10(&self) -> Result<T, ()>;
}

impl<T: FromRadix10Checked> ToRadix10Checked<T> for str {
    #[inline(always)]
    fn parse_radix10(&self) -> Result<T, ()> {
        T::from_radix_10_checked(self.as_bytes())
    }
}

impl<T: FromRadix10Checked> ToRadix10Checked<T> for [u8] {
    #[inline(always)]
    fn parse_radix10(&self) -> Result<T, ()> {
        T::from_radix_10_checked(self)
    }
}
