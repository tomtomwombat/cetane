mod common;
use common::*;

use cetane::*;

use std::cmp::PartialEq;
use std::fmt::Debug;
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert!(atoi::<u8>(b"").is_err());
        assert!(atoi::<u32>(b"").is_err());
        assert!(atoi::<u64>(b"").is_err());
    }

    #[test]
    fn simple() {
        assert_eq!(atoi::<u64>(b"7852"), Ok(7852));
    }

    #[test]
    fn test1() {
        assert_eq!(atoi::<u8>(b"97"), Ok(97));
        assert_eq!(atoi::<u32>(b"97"), Ok(97));
        assert_eq!(atoi::<u64>(b"97"), Ok(97));
    }

    #[test]
    fn test2() {
        assert!(atoi::<u8>(b"1234a").is_err());
        assert!(atoi::<u32>(b"1234a").is_err());
        assert!(atoi::<u64>(b"1234a").is_err());
    }

    #[test]
    fn zeros() {
        assert_eq!(atoi::<u8>(b"0000"), Ok(0));
        assert_eq!(atoi::<u8>(b"0001"), Ok(1));

        assert_eq!(atoi::<u32>(b"000000000000000000000000"), Ok(0));
        assert_eq!(atoi::<u32>(b"000000000000000000000001"), Ok(1));

        assert_eq!(atoi::<u64>(b"000000000000000000000000"), Ok(0));
        assert_eq!(atoi::<u64>(b"000000000000000000000001"), Ok(1));
    }

    macro_rules! all_parse_valid_num {
        ($max:literal, $type:ty) => {{
            let mut buf = [42u8; 64];
            for x in 0..=$max {
                let mut buffer = itoa::Buffer::new();
                let s = buffer.format(x);
                let l = s.as_bytes().len();
                for i in 0..=7 {
                    buf[i..i + l].copy_from_slice(s.as_bytes());
                    assert_eq!(atoi::<$type>(&buf[i..i + l]), Ok(x));
                }
            }
        }};
    }

    #[test]
    fn test_exhaustive_valid_u64() {
        all_parse_valid_num!(16777216, u64);
    }

    #[test]
    fn test_exhaustive_valid_u32() {
        all_parse_valid_num!(16777216, u32);
    }

    #[test]
    fn test_exhaustive_valid_u8() {
        all_parse_valid_num!(255, u8);
    }

    fn all_parse_byte<I: FromRadix10Checked + PartialEq + Debug + FromStr>() {
        let mut buf = [0u8; 8];
        for i in 0..=3 {
            for b1 in 0..=255 {
                buf[i] = b1;
                assert_eq!(
                    atoi::<I>(&buf[i..i + 1]).ok(),
                    correct_parse(&buf[i..i + 1])
                );
                for b2 in 0..=255 {
                    buf[i + 1] = b2;
                    assert_eq!(
                        atoi::<I>(&buf[i..i + 2]).ok(),
                        correct_parse(&buf[i..i + 2])
                    );
                    for b3 in 0..=255 {
                        buf[i + 2] = b3;
                        assert_eq!(
                            atoi::<I>(&buf[i..i + 3]).ok(),
                            correct_parse(&buf[i..i + 3])
                        );
                        for b4 in 0..=255 {
                            buf[i + 3] = b4;
                            assert_eq!(
                                atoi::<I>(&buf[i..i + 4]).ok(),
                                correct_parse(&buf[i..i + 4])
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_exhaustive_u64() {
        all_parse_byte::<u64>();
    }

    #[test]
    fn test_exhaustive_u32() {
        all_parse_byte::<u32>();
    }

    #[test]
    fn test_exhaustive_u8() {
        all_parse_byte::<u8>();
    }

    #[test]
    fn reg() {
        assert!(atoi::<u64>(b"a ").is_err());
        assert_eq!(
            atoi::<u64>(b"10000000000000000001"),
            Ok(10000000000000000001)
        );
        assert!(atoi::<u64>(b"20000000000000000000").is_err());
        assert!(atoi::<u64>(":c  A\u{1a7f}Aa𞸤𝒮0".as_bytes()).is_err());
    }

    fn assert_large_correct<I: FromRadix10Checked + PartialEq + Debug + FromStr + ToString>(
        max: I,
    ) {
        let mut b = max.to_string().as_bytes().to_vec();
        for i in 0..b.len() {
            b[i] += 1;
            assert_eq!(atoi::<I>(&b).ok(), correct_parse(&b));
            b[i] -= 1;
        }
    }

    #[test]
    fn test_large() {
        assert_large_correct::<u8>(u8::MAX);
        assert_large_correct::<u32>(u32::MAX);
        assert_large_correct::<u64>(u64::MAX);
    }
}
