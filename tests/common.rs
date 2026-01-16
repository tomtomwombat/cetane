#[cold]
pub(crate) fn cold<T>(x: T) -> T {
    x
}

// Faster version of `std::str::FromStr(std::from_utf8(data))` for integers.
pub fn correct_parse<I: std::str::FromStr>(data: &[u8]) -> Option<I> {
    if data.is_empty() {
        return cold(None);
    }
    let z = match data[0] {
        b'-' | b'+' => 1,
        _ => 0,
    };
    if data.len() == z {
        return cold(None);
    }
    let valid_digits = data[z..].iter().all(|&b| b.is_ascii_digit());
    if valid_digits {
        unsafe { str::from_utf8_unchecked(data) }.parse().ok()
    } else {
        return None;
    }
}
