#[cold]
pub(crate) fn parse_plus_sign(s: &mut &[u8]) {
    *s = &s[1..];
}

#[inline]
pub(crate) fn strip_leading_zeros(s: &mut &[u8], until: usize) {
    while s.len() > until && s[0] == b'0' {
        *s = &s[1..];
    }
}

#[cold]
pub(crate) fn error<T>() -> Result<T, ()> {
    Err(())
}

macro_rules! impl_read {
    ($func_name:ident, $t:ty) => {
        #[inline(always)]
        fn $func_name(s: &mut &[u8]) -> $t {
            let size = core::mem::size_of::<$t>();
            debug_assert!(s.len() >= size);
            let u = <$t>::from_le_bytes(s[..size].try_into().unwrap());
            *s = &s[size..];
            <$t>::from_le(u)
        }
    };
}

impl_read!(read_u8, u8);
impl_read!(read_u16, u16);
impl_read!(read_u32, u32);
impl_read!(read_u64, u64);

/// Parses exactly 1 byte into the `u64`.
/// If there's an error, `is_err` is set to a non-zero value.
#[inline]
pub fn parse_1(s: &mut &[u8], is_err: &mut u64) -> u64 {
    let mut u = read_u8(s) as u64;
    u ^= 0x30;
    *is_err |= (u | u.wrapping_add(0x06)) & 0xf0;
    u
}

/// Parses exactly 2 bytes into the `u64`.
/// If there's an error, `is_err` is set to a non-zero value.
#[inline]
pub fn parse_2(s: &mut &[u8], is_err: &mut u64) -> u64 {
    let mut u = read_u16(s);
    u ^= 0x3030;
    *is_err |= ((u | u.wrapping_add(0x0606)) & 0xf0f0) as u64;
    u = u.wrapping_mul(10 << 8 | 1) >> 8;
    u as u64
}

#[inline(always)]
fn swar_4(mut u: u32, err: &mut u64) -> u32 {
    *err |= ((u | u.wrapping_add(0x06060606)) & 0xf0f0f0f0) as u64;
    u = (u.wrapping_mul(10 << 8 | 1) >> 8) & 0xff00ff;
    u = u.wrapping_mul(100 << 16 | 1) >> 16;
    u
}

/// Parses exactly 4 bytes into the `u64`.
/// If there's an error, `is_err` is set to a non-zero value.
#[inline]
pub fn parse_4(s: &mut &[u8], err: &mut u64) -> u64 {
    let mut u = read_u32(s);
    u ^= 0x30303030;
    swar_4(u, err) as u64
}

#[inline(always)]
fn swar_8(mut u: u64, err: &mut u64) -> u64 {
    *err |= (u | u.wrapping_add(0x0606060606060606)) & 0xf0f0f0f0f0f0f0f0;
    // 10 * d7 + d6, 10 * d5 + d4, 10 * d3 + d2, 10 * d1 + d0
    u = (u.wrapping_mul(10 << 8 | 1) >> 8) & 0xff00ff00ff00ff;
    // 100 * (10 * d7 + d6) + 1 * (10 * d5 + d4), 100 * (10 * d3 + d2) + 1 * (10 * d1 + d0)
    u = (u.wrapping_mul(100 << 16 | 1) >> 16) & 0xffff0000ffff;
    // 10000 * (100 * (10 * d7 + d6) + 1 * (10 * d5 + d4)) + 1 * (100 * (10 * d3 + d2) + 1 * (10 * d1 + d0))
    u = u.wrapping_mul(10000 << 32 | 1) >> 32;
    u
}

/// Parses exactly 8 bytes into the `u64`.
/// If there's an error, `is_err` is set to a non-zero value.
#[inline]
pub fn parse_8(s: &mut &[u8], err: &mut u64) -> u64 {
    let mut u = read_u64(s);
    u ^= 0x3030303030303030;
    swar_8(u, err)
}

#[inline]
pub fn parse_4_to_8(s: &mut &[u8], err: &mut u64, len: usize) -> u64 {
    let mut u = u32::from_le(unsafe { core::ptr::read_unaligned(s.as_ptr() as *const u32) });
    *s = &s[len - 4..];
    u ^= 0x30303030;
    let offset = 8 - len;
    u = u << (offset << 3);
    let hi = swar_4(u, err) as u64;
    fold_4(s, err, hi)
}

#[inline]
pub fn parse_8_to_16(s: &mut &[u8], err: &mut u64, len: usize) -> u64 {
    let mut u = u64::from_le(unsafe { core::ptr::read_unaligned(s.as_ptr() as *const u64) });
    *s = &s[len - 8..];
    u ^= 0x3030303030303030;
    let offset = 16 - len;
    u = u << (offset << 3);
    let hi = swar_8(u, err);
    fold_8(s, err, hi)
}

macro_rules! impl_fold {
    ($func_name:ident, $parse_n:ident, $radix_pow:literal) => {
        #[inline(always)]
        pub(crate) fn $func_name(s: &mut &[u8], err: &mut u64, hi: u64) -> u64 {
            let x = $parse_n(s, err);
            hi.wrapping_mul($radix_pow).wrapping_add(x)
        }
    };
}

impl_fold!(fold_1, parse_1, 10);
impl_fold!(fold_2, parse_2, 100);
impl_fold!(fold_4, parse_4, 10000);
impl_fold!(fold_8, parse_8, 100000000);

#[inline]
pub(crate) fn parse_3(s: &mut &[u8], err: &mut u64) -> u64 {
    let hi = parse_2(s, err);
    fold_1(s, err, hi)
}

#[inline]
pub(crate) fn parse_5(s: &mut &[u8], err: &mut u64) -> u64 {
    let hi = parse_4(s, err);
    fold_1(s, err, hi)
}

#[inline(always)]
pub(crate) fn maybe_parse_8(s: &mut &[u8], err: &mut u64, res: &mut u64) {
    if s.len() >= 8 {
        *res = fold_8(s, err, *res);
    }
}
#[inline(always)]
pub(crate) fn maybe_parse_4(s: &mut &[u8], err: &mut u64, res: &mut u64) {
    if s.len() >= 4 {
        *res = fold_4(s, err, *res);
    }
}
#[inline(always)]
pub(crate) fn maybe_parse_2(s: &mut &[u8], err: &mut u64, res: &mut u64) {
    if s.len() >= 2 {
        *res = fold_2(s, err, *res);
    }
}
#[inline(always)]
pub(crate) fn maybe_parse_1(s: &mut &[u8], err: &mut u64, res: &mut u64) {
    if !s.is_empty() {
        *res = fold_1(s, err, *res);
    }
}

#[inline]
pub(crate) fn parse_up_to_38(s: &mut &[u8], err: &mut u64) -> u128 {
    let mut res: u128 = 0;
    if s.len() >= 16 {
        res = parse_16(s, err) as u128;
    }
    if s.len() >= 16 {
        let x = parse_16(s, err) as u128;
        res = res.wrapping_mul(10000000000000000);
        res = res.wrapping_add(x);
    } else if s.len() >= 8 {
        let x = parse_8(s, err) as u128;
        res = res.wrapping_mul(100000000);
        res = res.wrapping_add(x);
    }
    if s.len() >= 4 {
        let x = parse_4(s, err) as u128;
        res = res.wrapping_mul(10000);
        res = res.wrapping_add(x);
    }
    if s.len() >= 2 {
        let x = parse_2(s, err) as u128;
        res = res.wrapping_mul(100);
        res = res.wrapping_add(x);
    }
    res
}

/// Parses exactly 16 bytes into the `u128`.
/// If there's an error, `is_err` is set to a non-zero value.
#[inline(always)]
pub fn parse_16(s: &mut &[u8], err: &mut u64) -> u64 {
    #[cfg(all(target_arch = "x86_64", not(miri)))]
    {
        if std::is_x86_feature_detected!("sse4.1") {
            unsafe {
                return crate::simd::parse_16(s, err);
            }
        }
    }

    crate::fallback::parse_16(s, err)
}
