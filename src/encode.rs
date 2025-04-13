#![allow(missing_docs)]
#![allow(unused)]

/// Maximum number of bytes that can be encoded as BASE32.
pub const MAX_INPUT_LEN: usize = usize::MAX / 8 * 5 + 4;

/// Returns the length of the BASE32-encoded string for the given input length.
///
/// Panics if `input_len` is greater than [`MAX_INPUT_LEN`](crate::MAX_INPUT_LEN).
pub const fn encoded_len(input_len: usize) -> usize {
    if input_len > MAX_INPUT_LEN {
        panic!("The input is too large");
    }
    input_len / 5 * 8
        + match input_len % 5 {
            0 => 0,
            1 => 2,
            2 => 4,
            3 => 5,
            _ => 7,
        }
}

/// Returns the length of the original byte sequence for the given BASE32-encoded string length.
///
/// Returns `None` if `input_len` is invalid (i.e. was not returned by
/// [`encoded_len`](crate::encoded_len)).
pub const fn decoded_len(input_len: usize) -> Option<usize> {
    Some(
        input_len / 8 * 5
            + match input_len % 8 {
                0 => 0,
                1 => return None,
                2 => 1,
                3 => return None,
                4 => 2,
                5 => 3,
                6 => return None,
                _ => 4,
            },
    )
}

/// Returns `true` if the `input` is a valid BASE32-encoded string.
pub fn is_valid_base32(input: &[u8]) -> bool {
    decoded_len(input.len()).is_some() && input.iter().all(|b| CHARS.contains(b))
}

pub fn encode_into(input: &[u8], output: &mut [u8]) {
    let input_len = input.len();
    let output_len = output.len();
    if encoded_len(input_len) > output.len() {
        panic!("Output slice is too small");
    }
    let aligned_input_len = input_len / 5 * 5;
    let aligned_output_len = output_len / 8 * 8;
    let input_iter = (0..aligned_input_len).step_by(5);
    let output_iter = (0..aligned_output_len).step_by(8);
    for (i, j) in output_iter.zip(input_iter) {
        let a = input[j];
        let b = input[j + 1];
        let c = input[j + 2];
        let d = input[j + 3];
        let e = input[j + 4];
        output[i] = CHARS[(a & 0b11111) as usize]; // 5 bits
        output[i + 1] = CHARS[((a >> 5) | ((b & 0b11) << 3)) as usize]; // 3 + 2 bits
        output[i + 2] = CHARS[((b >> 2) & 0b11111) as usize]; // 5 bits
        output[i + 3] = CHARS[((b >> 7) | ((c & 0b1111) << 1)) as usize]; // 1 + 4 bits
        output[i + 4] = CHARS[((c >> 4) | ((d & 0b1) << 4)) as usize]; // 4 + 1 bits
        output[i + 5] = CHARS[((d >> 1) & 0b11111) as usize]; // 5 bits
        output[i + 6] = CHARS[((d >> 6) | ((e & 0b111) << 2)) as usize]; // 2 + 3 bits
        output[i + 7] = CHARS[(e >> 3) as usize]; // 5 bits
    }
    let remaining = input_len - aligned_input_len;
    if remaining == 0 {
        return;
    }
    let i = aligned_output_len;
    let j = aligned_input_len;
    let a = input[j];
    output[i] = CHARS[(a & 0b11111) as usize]; // 5 bits
    let b = input.get(j + 1).copied().unwrap_or(0);
    output[i + 1] = CHARS[((a >> 5) | ((b & 0b11) << 3)) as usize]; // 3 + 2 bits
    if remaining == 1 {
        return;
    }
    let c = input.get(j + 2).copied().unwrap_or(0);
    output[i + 2] = CHARS[((b >> 2) & 0b11111) as usize]; // 5 bits
    output[i + 3] = CHARS[((b >> 7) | ((c & 0b1111) << 1)) as usize]; // 1 + 4 bits
    if remaining == 2 {
        return;
    }
    let d = input.get(j + 3).copied().unwrap_or(0);
    output[i + 4] = CHARS[((c >> 4) | ((d & 0b1) << 4)) as usize]; // 4 + 1 bits
    if remaining == 3 {
        return;
    }
    let e = input.get(j + 4).copied().unwrap_or(0);
    output[i + 5] = CHARS[((d >> 1) & 0b11111) as usize]; // 5 bits
    output[i + 6] = CHARS[((d >> 6) | ((e & 0b111) << 2)) as usize]; // 2 + 3 bits
}

pub fn decode_into(input: &[u8], output: &mut [u8]) -> Result<(), DecodeError> {
    let input_len = input.len();
    let output_len = output.len();
    if let Some(l) = decoded_len(input_len) {
        if l > output.len() {
            return Err(DecodeError::OutputTooSmall);
        }
    } else {
        return Err(DecodeError::InvalidLen);
    }
    if !input.iter().all(|b| CHARS.contains(b)) {
        return Err(DecodeError::InvalidChar);
    }
    let aligned_input_len = input_len / 8 * 8;
    let aligned_output_len = output_len / 5 * 5;
    let input_iter = (0..aligned_input_len).step_by(8);
    let output_iter = (0..aligned_output_len).step_by(5);
    for (i, j) in output_iter.zip(input_iter) {
        let a = char_index(input[j]);
        let b = char_index(input[j + 1]);
        let c = char_index(input[j + 2]);
        let d = char_index(input[j + 3]);
        let e = char_index(input[j + 4]);
        let f = char_index(input[j + 5]);
        let g = char_index(input[j + 6]);
        let h = char_index(input[j + 7]);
        output[i] = a | ((b & 0b111) << 5); // 5 + 3 bits
        output[i + 1] = (b >> 3) | (c << 2) | ((d & 0b1) << 7); // 2 + 5 + 1 bits
        output[i + 2] = (d >> 1) | ((e & 0b1111) << 4); // 4 + 4 bits
        output[i + 3] = (e >> 4) | (f << 1) | ((g & 0b11) << 6); // 1 + 5 + 2 bits
        output[i + 4] = (g >> 2) | (h << 3); // 3 + 5 bits
    }
    let remaining = input_len - aligned_input_len;
    if remaining == 0 {
        return Ok(());
    }
    let i = aligned_output_len;
    let j = aligned_input_len;
    let a = char_index(input[j]);
    let b = input.get(j + 1).copied().map(char_index).unwrap_or(0);
    output[i] = a | ((b & 0b111) << 5); // 5 + 3 bits
    if remaining == 1 || remaining == 2 {
        return Ok(());
    }
    let c = input.get(j + 2).copied().map(char_index).unwrap_or(0);
    let d = input.get(j + 3).copied().map(char_index).unwrap_or(0);
    output[i + 1] = (b >> 3) | (c << 2) | ((d & 0b1) << 7); // 2 + 5 + 1 bits
    if remaining == 3 || remaining == 4 {
        return Ok(());
    }
    let e = input.get(j + 4).copied().map(char_index).unwrap_or(0);
    output[i + 2] = (d >> 1) | ((e & 0b1111) << 4); // 4 + 4 bits
    if remaining == 5 {
        return Ok(());
    }
    let f = input.get(j + 5).copied().map(char_index).unwrap_or(0);
    let g = input.get(j + 6).copied().map(char_index).unwrap_or(0);
    output[i + 3] = (e >> 4) | (f << 1) | ((g & 0b11) << 6); // 1 + 5 + 2 bits
    Ok(())
}

#[derive(Debug)]
pub enum DecodeError {
    OutputTooSmall,
    InvalidLen,
    InvalidChar,
}

const fn char_index(ch: u8) -> u8 {
    let i = (ch >> 6) & 1;
    let j = (ch & 0b0011_1111) - 33;
    INDICES[i as usize][j as usize]
}

// Crockford's base32.
const CHARS: [u8; 32] = *b"0123456789abcdefghjkmnpqrstvwxyz";

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
    use alloc::vec;
    use alloc::vec::Vec;
    use arbtest::arbtest;

    #[test]
    fn test_indices() {
        for (i, ch) in CHARS.iter().enumerate() {
            assert_eq!(i, char_index(*ch) as usize);
        }
    }

    #[test]
    fn test_encoded_len() {
        arbtest(|u| {
            let input_len = u.int_in_range(0..=usize::MAX / 8)?;
            let enc_len = encoded_len(input_len);
            let dec_len = decoded_len(enc_len).unwrap();
            assert_eq!(input_len, dec_len);
            Ok(())
        });
    }

    #[test]
    #[should_panic]
    fn test_encoded_len_panic() {
        let _enc_len = encoded_len(MAX_INPUT_LEN + 1);
    }

    #[test]
    fn test_encode() {
        let input = *b"hello";
        let mut output = [b'_'; encoded_len(5)];
        encode_into(&input, &mut output[..]);
        let mut decoded = [b'_'; 5];
        decode_into(&output, &mut decoded).unwrap();
        assert_eq!(input, decoded);
    }

    #[test]
    fn test_len_divisible_by_5() {
        arbtest(|u| {
            let input_len: usize = u.arbitrary_len::<u8>()? * 5;
            let mut input = Vec::with_capacity(input_len);
            for _ in 0..input_len {
                input.push(u.arbitrary()?);
            }
            let mut encoded = vec![b'_'; encoded_len(input.len())];
            encode_into(&input, &mut encoded);
            assert!(
                !encoded.contains(&b'_'),
                "input = {:?}, encoded = {:?}",
                input,
                core::str::from_utf8(&encoded)
            );
            let mut decoded = vec![0_u8; decoded_len(encoded.len()).unwrap()];
            decode_into(&encoded, &mut decoded).unwrap();
            assert_eq!(input, decoded);
            Ok(())
        });
    }

    #[test]
    fn test_len_non_divisible_by_5() {
        arbtest(|u| {
            let input_len: usize = u.int_in_range(0..=4)?;
            let mut input = Vec::with_capacity(input_len);
            for _ in 0..input_len {
                input.push(u.arbitrary()?);
            }
            let mut encoded = vec![b'_'; encoded_len(input.len())];
            encode_into(&input, &mut encoded);
            assert!(
                !encoded.contains(&b'_'),
                "input = {:?}, encoded = {:?}",
                input,
                core::str::from_utf8(&encoded)
            );
            let mut decoded = vec![0_u8; decoded_len(encoded.len()).unwrap()];
            decode_into(&encoded, &mut decoded).unwrap();
            assert_eq!(
                input,
                decoded,
                "input = {input:?}, encoded = {:?}, decoded = {decoded:?}",
                core::str::from_utf8(&encoded)
            );
            Ok(())
        });
    }

    #[test]
    fn test_any_len() {
        arbtest(|u| {
            let input: Vec<u8> = u.arbitrary()?;
            let mut encoded = vec![b'_'; encoded_len(input.len())];
            encode_into(&input, &mut encoded);
            assert!(
                !encoded.contains(&b'_'),
                "input = {:?}, encoded = {:?}",
                input,
                core::str::from_utf8(&encoded)
            );
            let mut decoded = vec![0_u8; decoded_len(encoded.len()).unwrap()];
            decode_into(&encoded, &mut decoded).unwrap();
            assert_eq!(
                input,
                decoded,
                "input = {input:?}, encoded = {:?}, decoded = {decoded:?}",
                core::str::from_utf8(&encoded)
            );
            Ok(())
        });
    }

    #[test]
    fn test_decode() {
        arbtest(|u| {
            let input_len: usize = u.arbitrary_len::<u8>()?;
            let Some(output_len) = decoded_len(input_len) else {
                return Ok(());
            };
            let mut input = Vec::with_capacity(input_len);
            for _ in 0..input_len {
                input.push(*u.choose(&CHARS)?);
            }
            let mut decoded = vec![0_u8; output_len];
            decode_into(&input, &mut decoded).unwrap();
            Ok(())
        });
    }

    #[test]
    fn test_decode_zeroes() {
        arbtest(|u| {
            let input_len: usize = u.arbitrary_len::<u8>()?;
            let input = vec![0_u8; input_len];
            let mut encoded = vec![b'_'; encoded_len(input.len())];
            encode_into(&input, &mut encoded);
            assert!(
                !encoded.contains(&b'_'),
                "input = {:?}, encoded = {:?}",
                input,
                core::str::from_utf8(&encoded)
            );
            let mut decoded = vec![0_u8; decoded_len(encoded.len()).unwrap()];
            decode_into(&encoded, &mut decoded).unwrap();
            assert_eq!(
                input,
                decoded,
                "input = {input:?}, encoded = {:?}, decoded = {decoded:?}",
                core::str::from_utf8(&encoded),
            );
            Ok(())
        });
    }
}
