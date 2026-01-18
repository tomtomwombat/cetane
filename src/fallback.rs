use crate::core::{fold_8, parse_8};

#[inline]
pub(crate) fn parse_16(s: &mut &[u8], err: &mut u64) -> u64 {
    let hi = parse_8(s, err);
    fold_8(s, err, hi);
}
