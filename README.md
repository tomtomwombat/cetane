# fast-atoi
[![Github](https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/tomtomwombat/fast-atoi)

fast-atoi is a SIMD-accelerated integer and digit parsing library for Rust. It matches the behavior of std parsing while achieving state-of-the-art performance on common inputs. fast-atoi also exposes low-level building blocks for constructing custom numeric parsers. fast-atoi is up to 3x faster than existing Rust parsers and particularly excels at unpredictable length inputs.

# Usage
```rust
use fast_atoi::*;

let _ = atoi::<u64>(b"42").unwrap();
let _ = atoi::<i8>(b"-42").unwrap();
let _ = atoi::<u32>(b"00042").unwrap();
let _ = atoi::<u128>(b"+42").unwrap();

// Ignoring the pesky '+' is faster
let _ = atoi_no_plus::<u64>(b"42").unwrap();
assert!(atoi_no_plus::<u64>(b"+42").is_err());

let _: u64 = "42".parse_radix10().unwrap();
let _: u64 = b"42".parse_radix10().unwrap();
```
```rust
use fast_atoi::{parse_4, parse_2};

fn my_really_fast_6_digit_parser(mut src: &[u8]) -> Result<u64, ()> {
      assert_eq!(src.len(), 6);
      let mut is_err = 0;
      let left = parse_4(&mut src, &mut is_err);
      let right = parse_2(&mut src, &mut is_err);
      if is_err > 0 {
            return Err(());
      }
      Ok(left.wrapping_mul(100).wrapping_add(right))
}

assert_eq!(my_really_fast_6_digit_parser(b"123456"), Ok(123456));
```

## SIMD Support

fast-atoi automatically uses SIMD on x86_64 (SSE2) if available. No configuration or feature flags are required. For maximum performance, users may compile with:
```ignore
RUSTFLAGS="-C target-cpu=native"
```
This is optional and not required for correctness. If SIMD support is not detected, a fallback is automatically used instead.

# Performance
Benchmark source and more results: https://github.com/tomtomwombat/atoi-benchmark.
- AMD Ryzen 9 5900X 12-Core Processor             (3.70 GHz)
- 64-bit operating system, x64-based processor

<img width="1920" height="967" alt="u64parsen" src="https://github.com/user-attachments/assets/8e95aaea-e4ef-4296-9108-2ffe453c7bd8" />
<img width="1920" height="967" alt="u64parse1n" src="https://github.com/user-attachments/assets/708956c2-5d4b-4ef0-aa5d-65f30b8526ce" />
<img width="1920" height="967" alt="i64parsen" src="https://github.com/user-attachments/assets/54a70214-1af4-4f31-b807-4ca3be74b19d" />
<img width="1920" height="967" alt="i64parse1n" src="https://github.com/user-attachments/assets/0a0d34a0-d01d-4a9a-aeec-493cac2b9b0b" />



# Should I Use This?
Yes. fast-atoi's behavior is of 1-1 parity with std. At worst, fast-atoi matches the performance of the next fastest parser. At best (for larger inputs) fast-atoi is 2-3x faster. fast-atoi is extensively tested:
- exhaustive testing for correct inputs
- exhaustive testing for all 4-byte combinations at different alignments
- no unsafe code (except for SIMD)
- miri for undefined behavior
- extensive property testing
- performance tested on a variety of input patterns

# To Do
Below are some ideas for features. Create an issue if you have a use-case for any.
- General radix
- Unchecked parsing
- Parsing aligned data
- Parsing buffered data (i.e. input has trailing buffer)
- AVX and NEON support

# References
- https://lemire.me/blog/2023/11/28/parsing-8-bit-integers-quickly/
- https://lemire.me/blog/2018/10/03/quickly-parsing-eight-digits/
- https://github.com/WojciechMula/toys/blob/master/conv_from_dec/parse.ssse3.cpp

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
