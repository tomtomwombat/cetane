# cetane
cetane is a high-performance integer and digit parsing library for Rust. It matches the behavior of std parsing while achieving state-of-the-art performance on common inputs. cetane also exposes low-level building blocks for constructing custom numeric parsers. cetane is 1-3x faster than existing Rust parsers.

# Usage
```rust
use cetane::*;

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
use cetane::{parse_4, parse_2};

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

# Performance
Benchmark source and more results: https://github.com/tomtomwombat/atoi-benchmark.
- Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (2.59 GHz)
- 64-bit operating system, x64-based processor
<img width="1920" height="967" alt="exact" src="https://github.com/user-attachments/assets/221e0898-e0bc-4cfc-b516-7ea8a482a59e" />

<img width="1920" height="967" alt="range" src="https://github.com/user-attachments/assets/dd6ad148-6671-439d-b7b2-2aa07d4c6aa7" />

# Should I Use This?
Yes. cetane's behavior is of 1-1 parity with std. At worst, cetane matches the performance of the next fastest parser. At best (for larger inputs) cetane is 2-3x faster. cetane is extensivly tested:
- exhaustive testing for correct inputs
- exhaustive testing for all 4-byte combinations at different alignements
- miri for undefined behavior
- extensive property testing

# To Do
Below are some ideas for future functionality. Create an issue if you have a use-case for any.
- General radix
- Unchecked parsing
- Parsing aligned data
- Parsing buffered data (i.e. input has trailing buffer)

# How it works
This crate is an extension of the 8-bit int parser explained in https://lemire.me/blog/2023/11/28/parsing-8-bit-integers-quickly/. The algorithm adds divide and conquer to Lemire's SWAR (SIMD within a register) techniques to parse varying width unsigned integers from decimal bytes. 

cetane's integer parsers are built from composing 5 core parsing functions, `parse_1`, `parse_2`, `parse_4`, `parse_8`, `parse_16`. Each of these functions parse numbers from the range 0 to 9, 99, 9999, 99999999, and 9999999999999999 respectively:
1. Read the bytes from the input directly into an uint.
2. Convert each byte (digit) to the decimal representation (e.g. b'0' -> 0)
3. Validate that all the bytes are digits.
4. Dot product the bytes with their respective magnitudes in a series of log2(n) SWAR steps.

As an example, here's a walkthrough of `parse_4` applied to `s = b"7852"`:
```rust,ignore
fn parse_4(s: &mut &[u8], is_err: &mut u64) -> u64 {
    let mut u = unsafe { ptr::read_unaligned(s.as_ptr() as *const u32) };
    *s = &s[4..];
    u ^= 0x30303030;
    *is_err |= ((u | (u.wrapping_add(0x06060606))) & 0xf0f0f0f0) as u64;

    u = (u.wrapping_mul(10 << 8 | 1) >> 8) & 0xff00ff;
    u = u.wrapping_mul(100 << 16 | 1) >> 16;
    u as u64
}
```
#### 1.
```rust,ignore
let mut u = unsafe { ptr::read_unaligned(s.as_ptr() as *const u32) };
*s = &s[4..];
```
Read 4 bytes of `s` into a litte endian `u32`, `u`, and advance the `s` pointer by 4. The first byte in `s` is the least significant byte in `u`.

```ignore
s:
      '7'      '8'      '5'      '2'
00110111 00111000 00110101 00110010
       │
       └──────────────────────────┐
u:                                ▼
      '2'      '5'      '8'      '7'
00110010 00110101 00111000 00110111
```

#### 2.
```rust,ignore
u ^= 0x30303030;
```
Convert each character to the decimal value. Since `'0'` is `48` or `0x30` in hex, subtract `0x30` from each byte in parrell.
Since '0' = 48 = 00110000, all binary representations up '9' all are 0011xxxx, XOR strips those bytes leaving the decimal value.
XOR typically has fewer CPU cycles than subtraction.
```ignore
'2' = 50 '5' = 55 '8' = 56 '7' = 55
00110010 00110101 00111000 00110111
-
00110000 00110000 00110000 00110000
=
       2        5        8        7
00000010 00000101 00001000 00000111
```

#### 3.
```rust,ignore
*is_err |= ((u | (u.wrapping_add(0x06060606))) & 0xf0f0f0f0) as u64;
```
Check if all four digits are valid in parallel. The goal is to map only the 10 valid digits to 0-15 range. Any other non-valid digits will be outside this range. If there are any bits outside of that that range, the digit is not valid.


First, adding 6 to all bytes moves valid all bytes to the 6-15 (00000110 - 00001111) range: 
```ignore
00000010 00000101 00001000 00000111
+
00000110 00000110 00000110 00000110
=
00001000 00001011 00001110 00001101
```

ORing with the decimal representation ensures that any orginal bits outside of the 15 range get put back (and don't disapear from the wrapping add).
```ignore
00000010 00000101 00001000 00000111
|
00001000 00001011 00001110 00001101
=
00001010 00001111 00001110 00001111
```
Masking detects any values larger than 15.
```ignore
00001010 00001111 00001110 00001111
&
11110000 11110000 11110000 11110000
=
00000000 00000000 00000000 00000000
```
Since no bytes are outside the range, the result is 0, so there an errors in processing these 4 bytes.
This information is folded into `is_err`, which is != 0 if any byte is not a valid character.

#### 4.
```rust,ignore
u = (u.wrapping_mul(10 << 8 | 1) >> 8) & 0xff00ff;
u = u.wrapping_mul(100 << 16 | 1) >> 16;
```
We have the digits, but they are seperated in their own byte buckets.

Our number can be expressed as `1000 * d3 + 100 * d2 + 10 * d1 + d0`. In our example `d3 = 7, d2 = 8, d1 = 8, d0 = 2`. This can be expressed as `100 * (10 * d3 + d2) + 1 * (10 * d1 + d0)`.

We'll complete these operations in two steps:
1. Get (10 * 7 + 8) = 78 and (5 * 10 + 2) = 52
2. Compute 100 * 78 + 1 * 52

First, get (10 * 7 + 8) and (5 * 10 + 2):
```rust,ignore
u = (u.wrapping_mul(10 << 8 | 1) >> 8) & 0xff00ff;
```

```ignore
       2        5        8        7
00000010 00000101 00001000 00000111
*
                        10        1
00000000 00000000 00001010 00000001
=
      52       85       78        7
00110100 01010101 01001110 00000111
|        |        |        |
|        |        |        ▼
|        |        ▼        7 * 1 (junk)
|        ▼        8 * 1 + 7 * 10
▼        5 * 1 + 8 * 10 (junk)
2 * 1 + 5 * 10
```
```ignore
00110100 01010101 01001110 00000111
>> 8 && ff00ff
=
               52                78
00000000 00110100 00000000 01001110
```


Then 100 * 78 + 1 * 52:
```rust,ignore
u = u.wrapping_mul(100 << 16 | 1) >> 16;
```
```ignore
               52                78
00000000 00110100 00000000 01001110
*
              100                 1
00000000 01100100 00000000 00000001
=
00011110 10101100 00000000 01001110
└─┬─────────────┘          |
  |                        ▼
  |                        78 * 1 (junk)
  ▼        
52 * 1 + 78 * 100
```
Then we just have to right shift by 16 to get the final result, 7852.

This algorithm can be extended to more digits. For example the crux of `parse_8` is,
```ignore
u = (u.wrapping_mul(10 << 8 | 1) >> 8) & 0xff00ff00ff00ff; ─────────────────────────────────────┐   
u = (u.wrapping_mul(100 << 16 | 1) >> 16) & 0xffff0000ffff; ─────────────────────────────┐      │   
u = u.wrapping_mul(10000 << 32 | 1) >> 32; ────────────────────────────────────┐         │      │   
                                                                               ▼         │      │   
                               ┌──────────────────────────────────────────────────┐      ▼      │     
                     ┌─────────┴──────────┐                            ┌──────────┴─────────┐   ▼        
                ┌────┴─────┐         ┌────┴─────┐                 ┌────┴─────┐         ┌────┴─────┐ 
10000 * (100 * (10 * d7 + d6) + 1 * (10 * d5 + d4)) + 1 * (100 * (10 * d3 + d2) + 1 * (10 * d1 + d0)
```
`parse_16` would look like
```rust,ignore
u = (u.wrapping_mul(10 << 8 | 1) >> 8) & 0x00ff00ff00ff00ff00ff00ff00ff00ff;
u = (u.wrapping_mul(100 << 16 | 1) >> 16) & 0x0000ffff0000ffff0000ffff0000ffff;
u = (u.wrapping_mul(10000 << 32 | 1) >> 32) & 0x00000000ffffffff00000000ffffffff;
u = u.wrapping_mul(100000000 << 64 | 1) >> 64;
```
