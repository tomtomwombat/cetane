use proptest::prelude::*;
mod common;
use common::*;
use rip_atoi::*;

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
    ProptestConfig::with_cases(65536 << 6)
}

proptest! {
    #![proptest_config(proptest_config())]

    #[test]
    fn valid_i32(x in any::<i32>()) {
        prop_assert!(is_valid_int::<u8, i32>(x));
        prop_assert!(is_valid_int::<u16, i32>(x));
        prop_assert!(is_valid_int::<u32, i32>(x));
        prop_assert!(is_valid_int::<u64, i32>(x));
        prop_assert!(is_valid_int::<u128, i32>(x));

        prop_assert!(is_valid_int::<i8, i32>(x));
        prop_assert!(is_valid_int::<i16, i32>(x));
        prop_assert!(is_valid_int::<i32, i32>(x));
        prop_assert!(is_valid_int::<i64, i32>(x));
        prop_assert!(is_valid_int::<i128, i32>(x));
    }

    #[test]
    fn valid_i128(x in any::<i128>()) {
        prop_assert!(is_valid_int::<u8, i128>(x));
        prop_assert!(is_valid_int::<u16, i128>(x));
        prop_assert!(is_valid_int::<u32, i128>(x));
        prop_assert!(is_valid_int::<u64, i128>(x));
        prop_assert!(is_valid_int::<u128, i128>(x));

        prop_assert!(is_valid_int::<i8, i128>(x));
        prop_assert!(is_valid_int::<i16, i128>(x));
        prop_assert!(is_valid_int::<i32, i128>(x));
        prop_assert!(is_valid_int::<i64, i128>(x));
        prop_assert!(is_valid_int::<i128, i128>(x));
    }

    #[test]
    fn valid_u32(x in any::<u32>()) {
        prop_assert!(is_valid_int::<u8, u32>(x));
        prop_assert!(is_valid_int::<u16, u32>(x));
        prop_assert!(is_valid_int::<u32, u32>(x));
        prop_assert!(is_valid_int::<u64, u32>(x));
        prop_assert!(is_valid_int::<u128, u32>(x));

        prop_assert!(is_valid_int::<i8, u32>(x));
        prop_assert!(is_valid_int::<i16, u32>(x));
        prop_assert!(is_valid_int::<i32, u32>(x));
        prop_assert!(is_valid_int::<i64, u32>(x));
        prop_assert!(is_valid_int::<i128, u32>(x));
    }

    #[test]
    fn valid_u128(x in any::<u128>()) {
        prop_assert!(is_valid_int::<u8, u128>(x));
        prop_assert!(is_valid_int::<u16, u128>(x));
        prop_assert!(is_valid_int::<u32, u128>(x));
        prop_assert!(is_valid_int::<u64, u128>(x));
        prop_assert!(is_valid_int::<u128, u128>(x));

        prop_assert!(is_valid_int::<i8, u128>(x));
        prop_assert!(is_valid_int::<i16, u128>(x));
        prop_assert!(is_valid_int::<i32, u128>(x));
        prop_assert!(is_valid_int::<i64, u128>(x));
        prop_assert!(is_valid_int::<i128, u128>(x));
    }

    #[test]
    fn arb_string(s in any::<String>()) {
        prop_assert!(is_valid::<u8>(s.as_bytes()));
        prop_assert!(is_valid::<u16>(s.as_bytes()));
        prop_assert!(is_valid::<u32>(s.as_bytes()));
        prop_assert!(is_valid::<u64>(s.as_bytes()));
        prop_assert!(is_valid::<u128>(s.as_bytes()));

        prop_assert!(is_valid::<i8>(s.as_bytes()));
        prop_assert!(is_valid::<i16>(s.as_bytes()));
        prop_assert!(is_valid::<i32>(s.as_bytes()));
        prop_assert!(is_valid::<i64>(s.as_bytes()));
        prop_assert!(is_valid::<i128>(s.as_bytes()));
    }

    #[test]
    fn arb_bytes(s in any::<Vec<u8>>()) {
        prop_assert!(is_valid::<u8>(&s));
        prop_assert!(is_valid::<u16>(&s));
        prop_assert!(is_valid::<u32>(&s));
        prop_assert!(is_valid::<u64>(&s));
        prop_assert!(is_valid::<u128>(&s));

        prop_assert!(is_valid::<i8>(&s));
        prop_assert!(is_valid::<i16>(&s));
        prop_assert!(is_valid::<i32>(&s));
        prop_assert!(is_valid::<i64>(&s));
        prop_assert!(is_valid::<i128>(&s));
    }
}
