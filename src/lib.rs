#![allow(rustdoc::bare_urls)]
#![warn(unreachable_pub)]
#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

use core::ptr;

pub trait FromRadix10Checked {
    fn from_radix_10_checked(_: &[u8]) -> Result<Self, ()>
    where
        Self: Sized;
}

/// Parses an integer from the bytes of the pattern `r"[0-9]+"`.
/// Unlike `std`, the prefix of "+" is not allowed.
pub fn atoi<I: FromRadix10Checked>(text: &[u8]) -> Result<I, ()> {
    I::from_radix_10_checked(text)
}

#[inline]
fn parse_1(s: &mut &[u8], is_err: &mut u64) -> u64 {
    let u = s[0] as u64 ^ 0x30;
    *s = &s[1..];
    *is_err |= (u | u.wrapping_add(0x06)) & 0xf0;
    u
}

#[inline]
fn parse_2(s: &mut &[u8], is_err: &mut u64) -> u64 {
    let mut u = unsafe { ptr::read_unaligned(s.as_ptr() as *const u16) };
    *s = &s[2..];
    u ^= 0x3030;
    *is_err |= ((u | u.wrapping_add(0x0606)) & 0xf0f0) as u64;

    u = u.wrapping_mul(10 << 8 | 1) >> 8;
    u as u64
}

#[inline]
fn parse_4(s: &mut &[u8], is_err: &mut u64) -> u64 {
    let mut u = unsafe { ptr::read_unaligned(s.as_ptr() as *const u32) };
    *s = &s[4..];
    u ^= 0x30303030;
    *is_err |= ((u | u.wrapping_add(0x06060606)) & 0xf0f0f0f0) as u64;

    u = (u.wrapping_mul(10 << 8 | 1) >> 8) & 0xff00ff;
    u = u.wrapping_mul(100 << 16 | 1) >> 16;
    u as u64
}

#[inline]
fn parse_8(s: &mut &[u8], is_err: &mut u64) -> u64 {
    let mut u = unsafe { ptr::read_unaligned(s.as_ptr() as *const u64) };
    *s = &s[8..];
    u ^= 0x3030303030303030;
    *is_err |= (u | u.wrapping_add(0x0606060606060606)) & 0xf0f0f0f0f0f0f0f0;

    // 10 * d7 + d6, 10 * d5 + d4, 10 * d3 + d2, 10 * d1 + d0
    u = (u.wrapping_mul(10 << 8 | 1) >> 8) & 0xff00ff00ff00ff;
    // 100 * (10 * d7 + d6) + 1 * (10 * d5 + d4)), 100 * (10 * d3 + d2) + 1 * (10 * d1 + d0)
    u = (u.wrapping_mul(100 << 16 | 1) >> 16) & 0xffff0000ffff;
    // 10000 * (100 * (10 * d7 + d6) + 1 * (10 * d5 + d4)) + 1 * (100 * (10 * d3 + d2) + 1 * (10 * d1 + d0)
    u = u.wrapping_mul(10000 << 32 | 1) >> 32;
    u
}

#[inline]
fn strip_leading_zeros(s: &mut &[u8], until: usize) {
    while s.len() > until && s[0] == b'0' {
        *s = &s[1..];
    }
}

#[inline]
fn parse_u64(s: &mut &[u8], is_err: &mut u64) -> u64 {
    let mut res: u64 = 0;
    while s.len() >= 8 {
        let x = parse_8(s, is_err);
        res = res.wrapping_mul(100000000);
        res = res.wrapping_add(x);
    }
    if s.len() >= 4 {
        let x = parse_4(s, is_err);
        // Since we throw out >20 length before, we only need check overflow for len=20.
        // If len=20, then we've called parse_8 twice and parse_4 once.
        if res >= 18446744_07370955 {
            let overflow = (res as u128 * 10000 + x as u128 >> 64) as u64;
            *is_err |= overflow;
        }
        res = res.wrapping_mul(10000);
        res = res.wrapping_add(x);
    }
    if s.len() >= 2 {
        let x = parse_2(s, is_err);
        res = res.wrapping_mul(100);
        res = res.wrapping_add(x);
    }
    if !s.is_empty() {
        let x = parse_1(s, is_err);
        res = res.wrapping_mul(10);
        res = res.wrapping_add(x);
    }
    res
}

impl FromRadix10Checked for u64 {
    #[inline]
    fn from_radix_10_checked(mut s: &[u8]) -> Result<Self, ()> {
        let mut is_err = 0;
        let res: u64 = match s.len() {
            1 => parse_1(&mut s, &mut is_err),
            2 => parse_2(&mut s, &mut is_err),
            3..=20 => parse_u64(&mut s, &mut is_err),
            _ => {
                strip_leading_zeros(&mut s, 20);
                if s.is_empty() || s.len() > 20 {
                    return Err(());
                }
                parse_u64(&mut s, &mut is_err)
            }
        };
        match is_err {
            0 => Ok(res),
            _ => Err(()),
        }
    }
}

#[inline]
fn parse_u32(s: &mut &[u8], is_err: &mut u64) -> u64 {
    let mut res: u64 = 0;
    if s.len() >= 8 {
        res = parse_8(s, is_err);
    }
    if s.len() >= 4 {
        let x = parse_4(s, is_err);
        res = res.wrapping_mul(10000);
        res = res.wrapping_add(x);
    }
    if s.len() >= 2 {
        let x = parse_2(s, is_err);
        res = res.wrapping_mul(100);
        res = res.wrapping_add(x);
    }
    if !s.is_empty() {
        let x = parse_1(s, is_err);
        res = res.wrapping_mul(10);
        res = res.wrapping_add(x);
    }
    *is_err |= res >> 32;
    res
}

impl FromRadix10Checked for u32 {
    #[inline]
    fn from_radix_10_checked(mut s: &[u8]) -> Result<Self, ()> {
        let mut is_err = 0;
        let res: u64 = match s.len() {
            1 => parse_1(&mut s, &mut is_err),
            2 => parse_2(&mut s, &mut is_err),
            3..=10 => parse_u32(&mut s, &mut is_err),
            _ => {
                strip_leading_zeros(&mut s, 10);
                if s.is_empty() || s.len() > 10 {
                    return Err(());
                }
                parse_u32(&mut s, &mut is_err)
            }
        };
        match is_err {
            0 => Ok(res as u32),
            _ => Err(()),
        }
    }
}

#[inline]
fn parse_u8(s: &mut &[u8], is_err: &mut u64) -> u64 {
    let mut res = 0;
    if s.len() >= 2 {
        res = parse_2(s, is_err);
    }
    if !s.is_empty() {
        let x = parse_1(s, is_err);
        res = res.wrapping_mul(10);
        res = res.wrapping_add(x);
    }
    *is_err |= res >> 8;
    res
}

impl FromRadix10Checked for u8 {
    #[inline]
    fn from_radix_10_checked(mut s: &[u8]) -> Result<Self, ()> {
        let mut is_err = 0;
        let res: u64 = match s.len() {
            1 => parse_1(&mut s, &mut is_err),
            2 => parse_2(&mut s, &mut is_err),
            3 => parse_u8(&mut s, &mut is_err),
            _ => {
                strip_leading_zeros(&mut s, 3);
                if s.is_empty() || s.len() > 3 {
                    return Err(());
                }
                parse_u8(&mut s, &mut is_err)
            }
        };
        match is_err {
            0 => Ok(res as u8),
            _ => Err(()),
        }
    }
}
