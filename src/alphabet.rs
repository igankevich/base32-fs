// Crockford's base32.
pub(crate) const CHARS: [u8; 32] = *b"0123456789abcdefghjkmnpqrstvwxyz";

#[inline]
pub(crate) const fn char_index(ch: u8) -> u8 {
    // This is an attempt at compressing the range '1'..='z' to a smaller range that fits into a
    // cache line (64 bytes). To achieve that we nullify 6th bit and choose a range based on the
    // value of this bit.
    let i = (ch >> 6) & 1;
    let j = (ch & 0b0011_1111) - 33;
    INDICES[i as usize][j as usize]
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

const INDICES: [[u8; 26]; 2] = [
    [
        NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
        NA,
    ],
    [
        10, 11, 12, 13, 14, 15, 16, 17, NA, 18, 19, NA, 20, 21, NA, 22, 23, 24, 25, 26, NA, 27, 28,
        29, 30, 31,
    ],
];

const NA: u8 = 32;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indices() {
        for (i, ch) in CHARS.iter().enumerate() {
            assert_eq!(i, char_index(*ch) as usize);
        }
    }
}
