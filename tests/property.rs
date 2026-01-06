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

#[cfg(miri)]
fn proptest_config() -> ProptestConfig {
    ProptestConfig {
        failure_persistence: None,
        cases: 4,
        ..Default::default()
    }
}

#[cfg(not(miri))]
fn proptest_config() -> ProptestConfig {
    ProptestConfig::with_cases(65536)
}

proptest! {
    #![proptest_config(proptest_config())]

    #[test]
    fn valid_u16(x in any::<u16>()) {
        prop_assert!(is_valid_int::<u8, u16>(x));
    }

    #[test]
    fn valid_u32(x in any::<u32>()) {
        prop_assert!(is_valid_int::<u8, u32>(x));
        prop_assert!(is_valid_int::<u32, u32>(x));
        prop_assert!(is_valid_int::<u64, u32>(x));
    }

    #[test]
    fn valid_u64(x in any::<u64>()) {
        prop_assert!(is_valid_int::<u8, u64>(x));
        prop_assert!(is_valid_int::<u32, u64>(x));
        prop_assert!(is_valid_int::<u64, u64>(x));
    }

    #[test]
    fn valid_u128(x in any::<u128>()) {
        prop_assert!(is_valid_int::<u8, u128>(x));
        prop_assert!(is_valid_int::<u32, u128>(x));
        prop_assert!(is_valid_int::<u64, u128>(x));
    }

    #[test]
    fn arb_string(s in any::<String>()) {
        prop_assert!(is_valid::<u8>(s.as_bytes()));
        prop_assert!(is_valid::<u32>(s.as_bytes()));
        prop_assert!(is_valid::<u64>(s.as_bytes()));
    }

    #[test]
    fn arb_bytes(s in any::<Vec<u8>>()) {
        prop_assert!(is_valid::<u8>(&s));
        prop_assert!(is_valid::<u32>(&s));
        prop_assert!(is_valid::<u64>(&s));
    }
}
