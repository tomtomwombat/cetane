use std::str::FromStr;

// Below are valid u8's for std, but not for this crate:
// - "+0"
pub fn correct_parse<I: FromStr>(data: &[u8]) -> Option<I> {
    for b in data.iter() {
        if !(b'0'..=b'9').contains(&b) {
            return None;
        }
    }
    str::from_utf8(data).map(|s| s.parse().ok()).ok().flatten()
}
