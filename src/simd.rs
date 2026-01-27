use core::arch::x86_64::*;

#[target_feature(enable = "sse4.1")]
#[inline]
unsafe fn simd_validate_and_sub(v: __m128i, is_err: &mut u64) -> __m128i {
    let zero = _mm_set1_epi8(b'0' as i8);
    let nine = _mm_set1_epi8(b'9' as i8);

    let lt_zero = _mm_cmplt_epi8(v, zero);
    let gt_nine = _mm_cmpgt_epi8(v, nine);
    let bad = _mm_or_si128(lt_zero, gt_nine);

    *is_err |= _mm_movemask_epi8(bad) as u64;
    _mm_sub_epi8(v, zero)
}

/// Rust translation of <https://github.com/WojciechMula/toys/blob/master/conv_from_dec/parse.ssse3.cpp>.
#[target_feature(enable = "sse4.1")]
#[inline]
pub(crate) unsafe fn parse_16(s: &mut &[u8], err: &mut u64) -> u64 {
    debug_assert!(s.len() >= 16);
    let src = _mm_loadu_si128(s.as_ptr() as *const __m128i);
    *s = &s[16..];

    let v = simd_validate_and_sub(src, err);
    let mul_1_10 = _mm_setr_epi8(10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1);
    let mul_1_100 = _mm_setr_epi16(100, 1, 100, 1, 100, 1, 100, 1);
    let mul_1_10000 = _mm_setr_epi16(10000, 1, 10000, 1, 10000, 1, 10000, 1);
    let t1 = _mm_maddubs_epi16(v, mul_1_10);
    let t2 = _mm_madd_epi16(t1, mul_1_100);
    let t3 = _mm_packus_epi32(t2, t2);
    let t4 = _mm_madd_epi16(t3, mul_1_10000);

    let mut tmp = [0u8; 16];
    _mm_storeu_si128(tmp.as_mut_ptr() as *mut __m128i, t4);
    let hi = u32::from_le_bytes(tmp[0..4].try_into().unwrap()) as u64;
    let lo = u32::from_le_bytes(tmp[4..8].try_into().unwrap()) as u64;

    hi.wrapping_mul(100000000).wrapping_add(lo)
}
