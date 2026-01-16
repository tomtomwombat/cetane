use crate::core::*;
use crate::{FromRadix10Checked, FromRadix10CheckedNoPlus};

#[inline]
fn parse_u128(s: &mut &[u8], is_err: &mut u64) -> u128 {
    if s.len() == 3 {
        return parse_3(s, is_err) as u128;
    }
    if s.len() == 4 {
        return parse_4(s, is_err) as u128;
    }
    if s.len() == 5 {
        return parse_5(s, is_err) as u128;
    }
    let mut res = parse_up_to_38(s, is_err);
    if !s.is_empty() {
        let x = parse_1(s, is_err);
        // TODO: can check this in 2 or 4 branch instead (whichever is less common)
        *is_err |= (res > 34028236692093846346337460743176821145) as u64;
        *is_err |= (res >= 34028236692093846346337460743176821145 && x > 5) as u64;
        res = res.wrapping_mul(10);
        res = res.wrapping_add(x as u128);
    }
    res
}

#[inline]
fn parse_u64(s: &mut &[u8], is_err: &mut u64) -> u64 {
    if s.len() == 3 {
        return parse_3(s, is_err);
    }
    if s.len() == 4 {
        return parse_4(s, is_err);
    }
    if s.len() == 5 {
        return parse_5(s, is_err);
    }
    let mut res: u64 = 0;
    maybe_parse_8(s, is_err, &mut res);
    maybe_parse_8(s, is_err, &mut res);
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
    maybe_parse_2(s, is_err, &mut res);
    maybe_parse_1(s, is_err, &mut res);
    res
}

#[inline]
fn parse_u32(s: &mut &[u8], is_err: &mut u64) -> u64 {
    if s.len() == 3 {
        return parse_3(s, is_err);
    }
    if s.len() == 4 {
        return parse_4(s, is_err);
    }
    if s.len() == 5 {
        return parse_5(s, is_err);
    }
    let mut res: u64 = 0;
    maybe_parse_8(s, is_err, &mut res);
    maybe_parse_4(s, is_err, &mut res);
    maybe_parse_2(s, is_err, &mut res);
    maybe_parse_1(s, is_err, &mut res);
    *is_err |= res >> 32;
    res
}

#[inline]
fn parse_u16(s: &mut &[u8], is_err: &mut u64) -> u64 {
    let mut res: u64 = 0;
    maybe_parse_4(s, is_err, &mut res);
    maybe_parse_2(s, is_err, &mut res);
    maybe_parse_1(s, is_err, &mut res);
    *is_err |= res >> 16;
    res
}

#[inline]
fn parse_u8(s: &mut &[u8], is_err: &mut u64) -> u64 {
    let mut res = 0;
    maybe_parse_2(s, is_err, &mut res);
    maybe_parse_1(s, is_err, &mut res);
    *is_err |= res >> 8;
    res
}

macro_rules! impl_unsigned_radix_10 {
    ($type:ty, $core:ty, $max_digits:literal, $parse_n:ident) => {
        impl FromRadix10CheckedNoPlus for $type {
            #[inline]
            fn from_radix_10_checked_no_plus(mut s: &[u8]) -> Result<Self, ()> {
                let mut is_err = 0;
                let res = match s.len() {
                    1 => parse_1(&mut s, &mut is_err) as $core,
                    2 => parse_2(&mut s, &mut is_err) as $core,
                    // 3 if $max_digits > 3 => parse_3(&mut s, &mut is_err) as $core,
                    3..=$max_digits => $parse_n(&mut s, &mut is_err),
                    _ => {
                        strip_leading_zeros(&mut s, $max_digits);
                        if s.is_empty() || s.len() > $max_digits {
                            return error::<Self>();
                        }
                        $parse_n(&mut s, &mut is_err)
                    }
                };
                match is_err {
                    0 => Ok(res as $type),
                    _ => error::<Self>(),
                }
            }
        }

        impl FromRadix10Checked for $type {
            #[inline]
            fn from_radix_10_checked(mut s: &[u8]) -> Result<Self, ()> {
                if !s.is_empty() && s[0] == b'+' {
                    parse_plus_sign(&mut s);
                }
                Self::from_radix_10_checked_no_plus(s)
            }
        }
    };
}

impl_unsigned_radix_10!(u128, u128, 39, parse_u128);
impl_unsigned_radix_10!(u64, u64, 20, parse_u64);
impl_unsigned_radix_10!(u32, u64, 10, parse_u32);
impl_unsigned_radix_10!(u16, u64, 5, parse_u16);
impl_unsigned_radix_10!(u8, u64, 3, parse_u8);
