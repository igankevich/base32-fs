// Crockford's base32.
pub(crate) const CHARS: [u8; 32] = *b"0123456789abcdefghjkmnpqrstvwxyz";

#[inline]
pub(crate) const fn char_index(ch: u8) -> u8 {
    // Here we choose a range based on the value of the 6th bit.
    let i = (ch >> 6) & 1;
    if i == 0 {
        // Range '0'..='9'.
        ch - b'0'
    } else {
        // Range 'a'..='z'.
        let correction = match ch {
            ..b'i' => 23,
            b'i'..b'l' => 24,
            b'l'..b'o' => 25,
            b'o'..b'u' => 26,
            b'u'.. => 27,
        };
        (ch & 0b111_111) - correction
    }
}

#[inline]
pub(crate) const fn is_valid_char(b: u8) -> bool {
    if !b.is_ascii_digit() && !b.is_ascii_lowercase() {
        return false;
    }
    if b == b'i' || b == b'l' || b == b'o' || b == b'u' {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::catch_unwind;

    #[test]
    fn test_indices() {
        for (i, ch) in CHARS.iter().copied().enumerate() {
            let result = catch_unwind(|| assert_eq!(i, char_index(ch) as usize));
            assert!(
                result.is_ok(),
                "Panic context: i={i}, ch={:?}",
                char::from(ch)
            );
        }
    }
}
