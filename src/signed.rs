use crate::core::*;
use crate::FromRadix10Checked;

#[inline]
fn parse_i8(s: &mut &[u8], is_err: &mut u64, sign: i8) -> i8 {
    let mut res: u64 = 0;
    maybe_parse_2(s, is_err, &mut res);
    maybe_parse_1(s, is_err, &mut res);
    *is_err |= match sign {
        -1 => res > 128,
        _ => res > 127,
    } as u64;
    let sign_mask = sign >> 7; // 0 or all-ones
    (res as i8 ^ sign_mask).wrapping_add(sign_mask & 1)
}

#[inline]
fn parse_i16(s: &mut &[u8], is_err: &mut u64, sign: i16) -> i16 {
    let mut res: u64 = 0;
    maybe_parse_4(s, is_err, &mut res);
    maybe_parse_2(s, is_err, &mut res);
    maybe_parse_1(s, is_err, &mut res);
    *is_err |= match sign {
        -1 => res > 32_768,
        _ => res > 32_767,
    } as u64;
    let sign_mask = sign >> 15; // 0 or all-ones
    (res as i16 ^ sign_mask).wrapping_add(sign_mask & 1)
}

#[inline]
fn parse_i32(s: &mut &[u8], is_err: &mut u64, sign: i32) -> i32 {
    if s.len() == 3 {
        return sign * parse_3(s, is_err) as i32;
    }
    if s.len() == 4 {
        return sign * parse_4(s, is_err) as i32;
    }
    if s.len() == 5 {
        return sign * parse_5(s, is_err) as i32;
    }
    let mut res: u64 = 0;
    maybe_parse_8(s, is_err, &mut res);
    maybe_parse_4(s, is_err, &mut res);
    maybe_parse_2(s, is_err, &mut res);
    maybe_parse_1(s, is_err, &mut res);
    *is_err |= match sign {
        -1 => res > 2147483648,
        _ => res > 2147483647,
    } as u64;

    let sign_mask = sign >> 31; // 0 or all-ones
    (res as i32 ^ sign_mask).wrapping_add(sign_mask & 1)
}

#[inline]
fn parse_i64(s: &mut &[u8], is_err: &mut u64, sign: i64) -> i64 {
    if s.len() == 3 {
        return sign * parse_3(s, is_err) as i64;
    }
    if s.len() == 4 {
        return sign * parse_4(s, is_err) as i64;
    }
    if s.len() == 5 {
        return sign * parse_5(s, is_err) as i64;
    }
    let mut res: u64 = 0;
    maybe_parse_8(s, is_err, &mut res);
    maybe_parse_8(s, is_err, &mut res);
    maybe_parse_4(s, is_err, &mut res);
    maybe_parse_2(s, is_err, &mut res);
    maybe_parse_1(s, is_err, &mut res);
    *is_err |= match sign {
        -1 => res > 9_223_372_036_854_775_808,
        _ => res > 9_223_372_036_854_775_807,
    } as u64;

    let sign_mask = sign >> 63; // 0 or all-ones
    (res as i64 ^ sign_mask).wrapping_add(sign_mask & 1)
}

#[inline]
fn parse_i128(s: &mut &[u8], is_err: &mut u64, sign: i128) -> i128 {
    if s.len() == 3 {
        return sign * parse_3(s, is_err) as i128;
    }
    if s.len() == 4 {
        return sign * parse_4(s, is_err) as i128;
    }
    if s.len() == 5 {
        return sign * parse_5(s, is_err) as i128;
    }
    let mut res = parse_up_to_38(s, is_err);
    if !s.is_empty() {
        let x = parse_1(s, is_err);
        *is_err |= match sign {
            -1 => {
                (res > 17014118346046923173168730371588410572)
                    || (res >= 17014118346046923173168730371588410572 && x > 8)
            }
            _ => {
                (res > 17014118346046923173168730371588410572)
                    || (res >= 17014118346046923173168730371588410572 && x > 7)
            }
        } as u64;
        res = res.wrapping_mul(10);
        res = res.wrapping_add(x as u128);
    }
    let sign_mask = sign >> 127; // 0 or all-ones
    (res as i128 ^ sign_mask).wrapping_add(sign_mask & 1)
}

macro_rules! impl_signed_radix_10 {
    ($type:ty, $max_digits:literal, $parse_n:ident) => {
        impl FromRadix10Checked for $type {
            #[inline]
            fn from_radix_10_checked(mut s: &[u8]) -> Result<Self, ()> {
                let mut sign = 1;
                if !s.is_empty() {
                    if s[0] == b'-' {
                        sign = -1;
                        s = &s[1..];
                    } else if s[0] == b'+' {
                        parse_plus_sign(&mut s);
                    }
                }
                let mut is_err = 0;
                let res = match s.len() {
                    1 => sign * parse_1(&mut s, &mut is_err) as $type,
                    2 => sign * parse_2(&mut s, &mut is_err) as $type,
                    // 3 if $max_digits > 3 => sign * parse_3(&mut s, &mut is_err) as $type,
                    3..=$max_digits => $parse_n(&mut s, &mut is_err, sign),
                    _ => {
                        strip_leading_zeros(&mut s, $max_digits);
                        if s.is_empty() || s.len() > $max_digits {
                            return error::<Self>();
                        }
                        $parse_n(&mut s, &mut is_err, sign)
                    }
                };
                match is_err {
                    0 => Ok(res),
                    _ => error::<Self>(),
                }
            }
        }
    };
}

impl_signed_radix_10!(i8, 3, parse_i8);
impl_signed_radix_10!(i16, 5, parse_i16);
impl_signed_radix_10!(i32, 10, parse_i32);
impl_signed_radix_10!(i64, 19, parse_i64);
impl_signed_radix_10!(i128, 39, parse_i128);
