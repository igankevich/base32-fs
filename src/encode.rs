use crate::Output;
use crate::CHARS;

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

/// Encode `input` byte sequence using BASE32 encoding and write the resulting byte sequence to
/// `output`.
pub fn encode<O: Output + ?Sized>(input: &[u8], output: &mut O) {
    macro_rules! byte {
        (0, $a: ident) => {
            CHARS[($a >> 3) as usize]
        };
        (1, $a: ident, $b: ident) => {
            CHARS[((($a & 0b111) << 2) | ($b >> 6)) as usize]
        };
        (2, $b: ident) => {
            CHARS[(($b >> 1) & 0b11111) as usize]
        };
        (3, $b: ident, $c: ident) => {
            CHARS[((($b & 0b1) << 4) | ($c >> 4)) as usize]
        };
        (4, $c: ident, $d: ident) => {
            CHARS[((($c & 0b1111) << 1) | ($d >> 7)) as usize]
        };
        (5, $d: ident) => {
            CHARS[(($d >> 2) & 0b11111) as usize]
        };
        (6, $d: ident, $e: ident) => {
            CHARS[((($d & 0b11) << 3) | ($e >> 5)) as usize]
        };
        (7, $e: ident) => {
            CHARS[($e & 0b11111) as usize]
        };
    }
    let mut chunks = input.chunks_exact(5);
    for chunk in chunks.by_ref() {
        let a = chunk[0];
        let b = chunk[1];
        let c = chunk[2];
        let d = chunk[3];
        let e = chunk[4];
        output.push(byte!(0, a)); // 5 bits
        output.push(byte!(1, a, b)); // 3 + 2 bits
        output.push(byte!(2, b)); // 5 bits
        output.push(byte!(3, b, c)); // 1 + 4 bits
        output.push(byte!(4, c, d)); // 4 + 1 bits
        output.push(byte!(5, d)); // 5 bits
        output.push(byte!(6, d, e)); // 2 + 3 bits
        output.push(byte!(7, e)); // 5 bits
    }
    let remainder = chunks.remainder();
    let remaining = remainder.len();
    if remaining == 0 {
        return;
    }
    let a = remainder[0];
    output.push(byte!(0, a)); // 5 bits
    let b = remainder.get(1).copied().unwrap_or(0);
    output.push(byte!(1, a, b)); // 3 + 2 bits
    if remaining == 1 {
        return;
    }
    let c = remainder.get(2).copied().unwrap_or(0);
    output.push(byte!(2, b)); // 5 bits
    output.push(byte!(3, b, c)); // 1 + 4 bits
    if remaining == 2 {
        return;
    }
    let d = remainder.get(3).copied().unwrap_or(0);
    output.push(byte!(4, c, d)); // 4 + 1 bits
    if remaining == 3 {
        return;
    }
    let e = remainder.get(4).copied().unwrap_or(0);
    output.push(byte!(5, d)); // 5 bits
    output.push(byte!(6, d, e)); // 2 + 3 bits
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use alloc::vec::Vec;
    use arbtest::arbtest;

    use crate::decode;
    use crate::decoded_len;

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
    fn test_encoded_len_no_panic() {
        let _enc_len = encoded_len(MAX_INPUT_LEN);
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
        encode(&input, &mut &mut output[..]);
        let mut decoded = [b'_'; 5];
        decode(output.as_slice(), &mut &mut decoded[..]).unwrap();
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
            let mut encoded = Vec::with_capacity(encoded_len(input.len()));
            encode(&input, &mut encoded);
            assert!(
                !encoded.contains(&b'_'),
                "input = {:?}, encoded = {:?}",
                input,
                core::str::from_utf8(&encoded)
            );
            let mut decoded: Vec<u8> = Vec::with_capacity(decoded_len(encoded.len()).unwrap());
            decode(encoded.as_slice(), &mut decoded).unwrap();
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
            let mut encoded = Vec::with_capacity(encoded_len(input.len()));
            encode(&input, &mut encoded);
            assert!(
                !encoded.contains(&b'_'),
                "input = {:?}, encoded = {:?}",
                input,
                core::str::from_utf8(&encoded)
            );
            let mut decoded: Vec<u8> = Vec::with_capacity(decoded_len(encoded.len()).unwrap());
            decode(encoded.as_slice(), &mut decoded).unwrap();
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
            let mut encoded = Vec::with_capacity(encoded_len(input.len()));
            encode(&input, &mut encoded);
            assert!(
                !encoded.contains(&b'_'),
                "input = {:?}, encoded = {:?}",
                input,
                core::str::from_utf8(&encoded)
            );
            let mut decoded: Vec<u8> = Vec::with_capacity(decoded_len(encoded.len()).unwrap());
            decode(encoded.as_slice(), &mut decoded).unwrap();
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
            let mut decoded: Vec<u8> = Vec::with_capacity(output_len);
            decode(input.as_slice(), &mut decoded).unwrap();
            Ok(())
        });
    }

    #[test]
    fn test_decode_zeroes() {
        arbtest(|u| {
            let input_len: usize = u.arbitrary_len::<u8>()?;
            let input = vec![0_u8; input_len];
            let mut encoded = Vec::with_capacity(encoded_len(input.len()));
            encode(&input, &mut encoded);
            assert!(
                !encoded.contains(&b'_'),
                "input = {:?}, encoded = {:?}",
                input,
                core::str::from_utf8(&encoded)
            );
            let mut decoded: Vec<u8> = Vec::with_capacity(decoded_len(encoded.len()).unwrap());
            decode(encoded.as_slice(), &mut decoded).unwrap();
            assert_eq!(
                input,
                decoded,
                "input = {input:?}, encoded = {:?}, decoded = {decoded:?}",
                core::str::from_utf8(&encoded),
            );
            Ok(())
        });
    }

    #[test]
    fn test_hashes() {
        arbtest(|u| {
            let mut hashes: Vec<[u8; 32]> = u.arbitrary()?;
            hashes.sort_unstable();
            hashes.dedup();
            let mut strings = Vec::with_capacity(hashes.len());
            for hash in hashes.iter() {
                let mut hash_string = [0_u8; encoded_len(32)];
                encode(&hash[..], &mut &mut hash_string[..]);
                strings.push(hash_string);
            }
            strings.sort_unstable();
            strings.dedup();
            assert_eq!(hashes.len(), strings.len());
            let mut actual_hashes = Vec::with_capacity(hashes.len());
            for string in strings.iter() {
                let mut actual_hash = [0_u8; 32];
                decode(&string[..], &mut &mut actual_hash[..]).unwrap();
                actual_hashes.push(actual_hash);
            }
            actual_hashes.sort_unstable();
            assert_eq!(hashes, actual_hashes);
            Ok(())
        });
    }

    #[test]
    fn test_sorting() {
        arbtest(|u| {
            let hash_len: usize = u.arbitrary_len::<u8>()?;
            let mut hashes = [vec![0_u8; hash_len], vec![0_u8; hash_len]];
            for i in 0..hash_len {
                hashes[0][i] = u.arbitrary()?;
                hashes[1][i] = u.arbitrary()?;
            }
            let expected = hashes[0].cmp(&hashes[1]);
            let mut encoded = [
                vec![0_u8; encoded_len(hash_len)],
                vec![0_u8; encoded_len(hash_len)],
            ];
            encode(&hashes[0][..], &mut &mut encoded[0][..]);
            encode(&hashes[1][..], &mut &mut encoded[1][..]);
            let actual = encoded[0].cmp(&encoded[1]);
            assert_eq!(
                expected,
                actual,
                "expected = {expected:?}, actual = {actual:?}, raw = {:?} {:?}, encoded = {} {}",
                hashes[0],
                hashes[1],
                core::str::from_utf8(&encoded[0]).unwrap(),
                core::str::from_utf8(&encoded[1]).unwrap(),
            );
            Ok(())
        });
    }
}
