mod common;
use common::*;

use fast_atoi::*;

use std::cmp::PartialEq;
use std::fmt::Debug;
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neg() {
        assert_eq!(atoi::<i32>(b"-12"), Ok(-12));
        assert_eq!(atoi::<i32>(b"12"), Ok(12));
        assert_eq!(atoi::<i32>(i32::MAX.to_string().as_bytes()), Ok(i32::MAX));
        assert_eq!(atoi::<i32>(i32::MIN.to_string().as_bytes()), Ok(i32::MIN));
    }

    #[test]
    fn test_empty() {
        assert!(atoi::<u8>(b"").is_err());
        assert!(atoi::<u16>(b"").is_err());
        assert!(atoi::<u32>(b"").is_err());
        assert!(atoi::<u64>(b"").is_err());
        assert!(atoi::<u128>(b"").is_err());

        assert!(atoi::<i8>(b"").is_err());
        assert!(atoi::<i16>(b"").is_err());
        assert!(atoi::<i32>(b"").is_err());
        assert!(atoi::<i64>(b"").is_err());
        assert!(atoi::<i128>(b"").is_err());
    }

    #[test]
    fn simple() {
        assert_eq!(atoi::<u64>(b"7852"), Ok(7852));
    }

    #[test]
    fn simple_error() {
        assert!(atoi::<u8>(b"1234a").is_err());
        assert!(atoi::<u16>(b"1234a").is_err());
        assert!(atoi::<u32>(b"1234a").is_err());
        assert!(atoi::<u64>(b"1234a").is_err());
        assert!(atoi::<u128>(b"1234a").is_err());

        assert!(atoi::<i8>(b"1234a").is_err());
        assert!(atoi::<i16>(b"1234a").is_err());
        assert!(atoi::<i32>(b"1234a").is_err());
        assert!(atoi::<i64>(b"1234a").is_err());
        assert!(atoi::<i128>(b"1234a").is_err());
    }

    #[test]
    fn zeros() {
        assert_eq!(atoi::<u8>(b"0000"), Ok(0));
        assert_eq!(atoi::<u8>(b"0001"), Ok(1));

        assert_eq!(atoi::<u16>(b"00000000"), Ok(0));
        assert_eq!(atoi::<u16>(b"00000001"), Ok(1));

        assert_eq!(atoi::<u32>(b"000000000000000000000000"), Ok(0));
        assert_eq!(atoi::<u32>(b"000000000000000000000001"), Ok(1));

        assert_eq!(atoi::<u64>(b"000000000000000000000000"), Ok(0));
        assert_eq!(atoi::<u64>(b"000000000000000000000001"), Ok(1));

        assert_eq!(
            atoi::<u128>(b"000000000000000000000000000000000000000000000000"),
            Ok(0)
        );
        assert_eq!(
            atoi::<u128>(b"000000000000000000000000000000000000000000000001"),
            Ok(1)
        );

        assert_eq!(atoi::<i8>(b"000000000000000000000000"), Ok(0));
        assert_eq!(atoi::<i8>(b"000000000000000000000001"), Ok(1));
        assert_eq!(atoi::<i8>(b"-000000000000000000000000"), Ok(0));
        assert_eq!(atoi::<i8>(b"-000000000000000000000001"), Ok(-1));

        assert_eq!(atoi::<i16>(b"000000000000000000000000"), Ok(0));
        assert_eq!(atoi::<i16>(b"000000000000000000000001"), Ok(1));
        assert_eq!(atoi::<i16>(b"-000000000000000000000000"), Ok(0));
        assert_eq!(atoi::<i16>(b"-000000000000000000000001"), Ok(-1));

        assert_eq!(atoi::<i32>(b"000000000000000000000000"), Ok(0));
        assert_eq!(atoi::<i32>(b"000000000000000000000001"), Ok(1));
        assert_eq!(atoi::<i32>(b"-000000000000000000000000"), Ok(0));
        assert_eq!(atoi::<i32>(b"-000000000000000000000001"), Ok(-1));

        assert_eq!(
            atoi::<i64>(b"00000000000000000000000000000000000000000000000"),
            Ok(0)
        );
        assert_eq!(
            atoi::<i64>(b"000000000000000000000000000000000000000000000001"),
            Ok(1)
        );
        assert_eq!(
            atoi::<i64>(b"-00000000000000000000000000000000000000000000000"),
            Ok(0)
        );
        assert_eq!(
            atoi::<i64>(b"-000000000000000000000000000000000000000000000001"),
            Ok(-1)
        );

        assert_eq!(
            atoi::<i128>(b"00000000000000000000000000000000000000000000000"),
            Ok(0)
        );
        assert_eq!(
            atoi::<i128>(b"000000000000000000000000000000000000000000000001"),
            Ok(1)
        );
        assert_eq!(
            atoi::<i128>(b"-00000000000000000000000000000000000000000000000"),
            Ok(0)
        );
        assert_eq!(
            atoi::<i128>(b"-000000000000000000000000000000000000000000000001"),
            Ok(-1)
        );
    }

    macro_rules! all_parse_valid_num {
        ($min:literal, $max:literal, $type:ty) => {{
            let mut buf = [42u8; 64];
            for x in $min..=$max {
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
    fn test_exhaustive_valid_u128() {
        all_parse_valid_num!(0, 16777216, u128);
    }

    #[test]
    fn test_exhaustive_valid_u64() {
        all_parse_valid_num!(0, 16777216, u64);
    }

    #[test]
    fn test_exhaustive_valid_u32() {
        all_parse_valid_num!(0, 16777216, u32);
    }

    #[test]
    fn test_exhaustive_valid_u16() {
        all_parse_valid_num!(0, 65535, u16);
    }

    #[test]
    fn test_exhaustive_valid_u8() {
        all_parse_valid_num!(0, 255, u8);
    }

    #[test]
    fn test_exhaustive_valid_i8() {
        all_parse_valid_num!(-128, 127, i8);
    }

    #[test]
    fn test_exhaustive_valid_i16() {
        all_parse_valid_num!(-32768, 32767, i16);
    }

    #[test]
    fn test_exhaustive_valid_i32() {
        all_parse_valid_num!(-65535, 65535, i32);
    }

    #[test]
    fn test_exhaustive_valid_i64() {
        all_parse_valid_num!(-65535, 65535, i64);
    }

    #[test]
    fn test_exhaustive_valid_i128() {
        all_parse_valid_num!(-65535, 65535, i128);
    }

    fn all_parse_byte<I: FromRadix10Checked + PartialEq + Debug + FromStr>() {
        let mut buf = [0u8; 8];
        for i in 0..=1 {
            let base = unsafe { buf.as_ptr().add(i) };
            for b1 in 0..=255 {
                buf[i] = b1;
                let s1 = unsafe { std::slice::from_raw_parts(base, 1) };
                assert_eq!(atoi::<I>(s1).ok(), correct_parse(s1));
                for b2 in 0..=255 {
                    buf[i + 1] = b2;
                    let s2 = unsafe { std::slice::from_raw_parts(base, 2) };
                    assert_eq!(atoi::<I>(s2).ok(), correct_parse(s2));
                    for b3 in 0..=255 {
                        buf[i + 2] = b3;
                        let s3 = unsafe { std::slice::from_raw_parts(base, 3) };
                        assert_eq!(atoi::<I>(s3).ok(), correct_parse(s3));
                        for b4 in 0..=255 {
                            buf[i + 3] = b4;
                            let s4 = unsafe { std::slice::from_raw_parts(base, 4) };
                            assert_eq!(atoi::<I>(s4).ok(), correct_parse(s4));
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_exhaustive_u128() {
        all_parse_byte::<u128>();
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
    fn test_exhaustive_u16() {
        all_parse_byte::<u16>();
    }

    #[test]
    fn test_exhaustive_u8() {
        all_parse_byte::<u8>();
    }

    #[test]
    fn test_exhaustive_i8() {
        all_parse_byte::<i8>();
    }

    #[test]
    fn test_exhaustive_i16() {
        all_parse_byte::<i16>();
    }

    #[test]
    fn test_exhaustive_i32() {
        all_parse_byte::<i32>();
    }

    #[test]
    fn test_exhaustive_i64() {
        all_parse_byte::<i64>();
    }

    #[test]
    fn test_exhaustive_i128() {
        all_parse_byte::<i128>();
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
        println!("{:?}", b);
        assert_eq!(atoi::<I>(&b).ok(), correct_parse(&b), "{:?}", b);
        for i in 0..b.len() {
            b[i] += 1;
            assert_eq!(atoi::<I>(&b).ok(), correct_parse(&b), "{:?}", b);
            b[i] -= 1;
        }
    }

    #[test]
    fn test_large() {
        assert_large_correct::<u8>(u8::MAX);
        assert_large_correct::<u16>(u16::MAX);
        assert_large_correct::<u32>(u32::MAX);
        assert_large_correct::<u64>(u64::MAX);
        assert_large_correct::<u128>(u128::MAX);

        assert_large_correct::<i8>(i8::MIN);
        assert_large_correct::<i8>(i8::MAX);

        assert_large_correct::<i16>(i16::MIN);
        assert_large_correct::<i16>(i16::MAX);

        assert_large_correct::<i32>(i32::MIN);
        assert_large_correct::<i32>(i32::MAX);

        assert_large_correct::<i64>(i64::MIN);
        assert_large_correct::<i64>(i64::MAX);

        assert_large_correct::<i128>(i128::MIN);
        assert_large_correct::<i128>(i128::MAX);
    }
}
