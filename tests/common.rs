use std::str::FromStr;

// Below are valid u8's for std, but not for this crate:
// - "+0"
pub fn correct_parse<I: FromStr>(data: &[u8]) -> Option<I> {
    /*
    if !data.is_empty() && data[0] == b'+' {
        return None;
    }
    */
    if data.iter().all(|&b| b.is_ascii_digit()) {
        unsafe { str::from_utf8_unchecked(data) }.parse().ok()
    } else {
        return None;
    }
}
