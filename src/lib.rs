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

pub fn atoi<I: FromRadix10Checked>(text: &[u8]) -> Result<I, ()> {
    I::from_radix_10_checked(text)
}

/// Helper
#[cfg(feature = "std")]
pub fn print_bytes_64(x: u64) {
    let s = format!("{:064b}", x);
    for i in 0..8 {
        print!("{} ", s[(i * 8)..(i * 8 + 8)].to_string());
        // print!("{:08b} ", b);
    }
    print!("\n");
}

#[inline]
fn parse_1(s: &mut &[u8], is_err: &mut u64) -> u64 {
    let u = s[0] as u64 ^ 0x30;
    *is_err |= (u | (u.wrapping_add(0x06))) & 0xf0;
    *s = &s[1..];
    u
}

#[inline]
fn parse_2(s: &mut &[u8], is_err: &mut u64) -> u64 {
    let mut u = unsafe { ptr::read_unaligned(s.as_ptr() as *const u16) };
    *s = &s[2..];
    u ^= 0x3030;
    *is_err |= ((u | (u.wrapping_add(0x0606))) & 0xf0f0) as u64;

    u = u.wrapping_mul(10 << 8 | 1) >> 8;
    u as u64
}

#[inline]
fn parse_4(s: &mut &[u8], is_err: &mut u64) -> u64 {
    let mut u = unsafe { ptr::read_unaligned(s.as_ptr() as *const u32) };
    *s = &s[4..];
    u ^= 0x30303030;
    *is_err |= ((u | (u.wrapping_add(0x06060606))) & 0xf0f0f0f0) as u64;

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

    u = (u.wrapping_mul(10 << 8 | 1) >> 8) & 0xff00ff00ff00ff;
    u = (u.wrapping_mul(100 << 16 | 1) >> 16) & 0xffff0000ffff;
    u = u.wrapping_mul(10000 << 32 | 1) >> 32;
    u
}

impl FromRadix10Checked for u64 {
    #[inline]
    fn from_radix_10_checked(mut s: &[u8]) -> Result<Self, ()> {
        if s.len() > 20 || s.is_empty() {
            return Err(());
        }
        let mut res: u64 = 0;
        let mut is_err = 0;

        match s.len() {
            1 => {
                res = parse_1(&mut s, &mut is_err);
            }
            2 => {
                res = parse_2(&mut s, &mut is_err);
            }
            _ => {
                while s.len() >= 8 {
                    let x = parse_8(&mut s, &mut is_err);
                    res = res.wrapping_mul(100000000);
                    res = res.wrapping_add(x);
                }
                if s.len() >= 4 {
                    let x = parse_4(&mut s, &mut is_err);
                    if (res as u128) * 10000 + x as u128 > u64::MAX as u128 {
                        return Err(());
                    }
                    res = res.wrapping_mul(10000);
                    res = res.wrapping_add(x);
                }
                if s.len() >= 2 {
                    let x = parse_2(&mut s, &mut is_err);
                    res = res.wrapping_mul(100);
                    res = res.wrapping_add(x);
                }
                if !s.is_empty() {
                    let x = parse_1(&mut s, &mut is_err);
                    res = res.wrapping_mul(10);
                    res = res.wrapping_add(x);
                }
            }
        }

        match is_err {
            0 => Ok(res),
            _ => Err(()),
        }
    }
}

impl FromRadix10Checked for u8 {
    #[inline]
    fn from_radix_10_checked(mut s: &[u8]) -> Result<Self, ()> {
        let mut is_err = 0;
        let mut res: u64;
        match s.len() {
            1 => {
                res = parse_1(&mut s, &mut is_err);
            }
            2 => {
                res = parse_2(&mut s, &mut is_err);
            }
            3 => {
                res = parse_2(&mut s, &mut is_err);
                res = res.wrapping_mul(10);
                res = res.wrapping_add(parse_1(&mut s, &mut is_err));
                is_err |= res >> 8;
            }
            _ => return Err(()),
        };

        match is_err {
            0 => Ok(res as u8),
            _ => Err(()),
        }
    }
}
