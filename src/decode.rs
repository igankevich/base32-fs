use crate::char_index;
use crate::is_valid_char;
use crate::DecodeError;
use crate::Input;
use crate::Output;

/// Decode `input` byte sequence using BASE32 encoding and write the resulting byte sequence to
/// `output`.
pub fn decode<I: Input<8>, O: Output + ?Sized>(
    mut input: I,
    output: &mut O,
) -> Result<(), DecodeError> {
    macro_rules! byte {
        (0, $a: ident, $b: ident) => {
            ($a << 3) | (($b >> 2) & 0b111)
        };
        (1, $b: ident, $c: ident, $d: ident) => {
            (($b & 0b11) << 6) | ($c << 1) | ($d >> 4)
        };
        (2, $d: ident, $e: ident) => {
            (($d & 0b1111) << 4) | ($e >> 1)
        };
        (3, $e: ident, $f: ident, $g: ident) => {
            (($e & 0b1) << 7) | ($f << 2) | ($g >> 3)
        };
        (4, $g: ident, $h: ident) => {
            (($g & 0b111) << 5) | $h
        };
    }
    while let Some(chunk) = input.next_chunk() {
        if !is_valid_chunk(chunk) {
            return Err(DecodeError);
        }
        let a = char_index(chunk[0]);
        let b = char_index(chunk[1]);
        let c = char_index(chunk[2]);
        let d = char_index(chunk[3]);
        let e = char_index(chunk[4]);
        let f = char_index(chunk[5]);
        let g = char_index(chunk[6]);
        let h = char_index(chunk[7]);
        output.push(byte!(0, a, b)); // 5 + 3 bits
        output.push(byte!(1, b, c, d)); // 2 + 5 + 1 bits
        output.push(byte!(2, d, e)); // 4 + 4 bits
        output.push(byte!(3, e, f, g)); // 1 + 5 + 2 bits
        output.push(byte!(4, g, h)); // 3 + 5 bits
    }
    let remainder = input.remainder();
    if !is_valid_remainder(remainder) {
        return Err(DecodeError);
    }
    let remaining = remainder.len();
    if remaining == 0 {
        return Ok(());
    }
    let a = char_index(remainder[0]);
    let b = remainder.get(1).copied().map(char_index).unwrap_or(0);
    output.push(byte!(0, a, b)); // 5 + 3 bits
    if remaining == 1 || remaining == 2 {
        return Ok(());
    }
    let c = remainder.get(2).copied().map(char_index).unwrap_or(0);
    let d = remainder.get(3).copied().map(char_index).unwrap_or(0);
    output.push(byte!(1, b, c, d)); // 2 + 5 + 1 bits
    if remaining == 3 || remaining == 4 {
        return Ok(());
    }
    let e = remainder.get(4).copied().map(char_index).unwrap_or(0);
    output.push(byte!(2, d, e)); // 4 + 4 bits
    if remaining == 5 {
        return Ok(());
    }
    let f = remainder.get(5).copied().map(char_index).unwrap_or(0);
    let g = remainder.get(6).copied().map(char_index).unwrap_or(0);
    output.push(byte!(3, e, f, g)); // 1 + 5 + 2 bits
    Ok(())
}

/// Returns the length of the original byte sequence for the given BASE32-encoded string length.
///
/// Returns `None` if `input_len` is invalid (i.e. was not returned by
/// [`encoded_len`](crate::encoded_len)).
#[inline]
pub const fn decoded_len(input_len: usize) -> Option<usize> {
    match remainder_decoded_len(input_len) {
        Some(remainder_len) => Some(input_len / 8 * 5 + remainder_len),
        None => None,
    }
}

#[inline]
const fn remainder_decoded_len(input_len: usize) -> Option<usize> {
    match input_len % 8 {
        0 => Some(0),
        1 => None,
        2 => Some(1),
        3 => None,
        4 => Some(2),
        5 => Some(3),
        6 => None,
        _ => Some(4),
    }
}

/// Returns `true` if the `input` is a valid BASE32-encoded string.
#[inline]
pub const fn is_valid(input: &[u8]) -> bool {
    decoded_len(input.len()).is_some() && is_valid_chunk(input)
}

#[inline]
const fn is_valid_chunk(mut input: &[u8]) -> bool {
    while let [ch, rest @ ..] = input {
        if !is_valid_char(*ch) {
            return false;
        }
        input = rest;
    }
    true
}

#[inline]
fn is_valid_remainder(input: &[u8]) -> bool {
    is_valid_chunk(input) && remainder_decoded_len(input.len()).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use arbtest::arbtest;

    use crate::CHARS;

    #[test]
    fn test_is_valid_chunk() {
        arbtest(|u| {
            let chunk: [u8; 8] = u.arbitrary()?;
            assert_eq!(
                is_valid_chunk_slow(&chunk),
                is_valid_chunk(&chunk),
                "chunk = {chunk:?}"
            );
            Ok(())
        });
    }

    fn is_valid_chunk_slow(input: &[u8]) -> bool {
        input.iter().all(|b| CHARS.contains(b))
    }
}
