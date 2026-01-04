use proptest::prelude::*;
mod common;
use cetane::*;
use common::*;

fn is_valid_int<I: FromRadix10Checked, V: itoa::Integer>(x: V) -> bool {
    let mut buffer = itoa::Buffer::new();
    let s = buffer.format(x);
    is_valid::<I>(s.as_bytes())
}

fn is_valid<I: FromRadix10Checked>(s: &[u8]) -> bool {
    let ours = atoi::<u64>(s).ok();
    let std = correct_parse(s);
    ours == std
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(65536))]

    #[test]
    fn valid_u8(x in any::<u8>()) {
        prop_assert!(is_valid_int::<u8, u8>(x));
        prop_assert!(is_valid_int::<u64, u8>(x));
    }

    #[test]
    fn valid_u16(x in any::<u16>()) {
        prop_assert!(is_valid_int::<u8, u16>(x));
        prop_assert!(is_valid_int::<u64, u16>(x));
    }

    #[test]
    fn valid_u32(x in any::<u32>()) {
        prop_assert!(is_valid_int::<u8, u32>(x));
        prop_assert!(is_valid_int::<u64, u32>(x));
    }

    #[test]
    fn valid_u64(x in any::<u64>()) {
        prop_assert!(is_valid_int::<u8, u64>(x));
        prop_assert!(is_valid_int::<u64, u64>(x));
    }

    #[test]
    fn valid_u128(x in any::<u128>()) {
        prop_assert!(is_valid_int::<u8, u128>(x));
        prop_assert!(is_valid_int::<u64, u128>(x));
    }

    #[test]
    fn arb_string(s in any::<String>()) {
        prop_assert!(is_valid::<u8>(s.as_bytes()));
        prop_assert!(is_valid::<u64>(s.as_bytes()));
    }

    #[test]
    fn arb_bytes(s in any::<Vec<u8>>()) {
        prop_assert!(is_valid::<u8>(&s));
        prop_assert!(is_valid::<u64>(&s));
    }
}
