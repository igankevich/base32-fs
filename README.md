# base32-fs

[![Crates.io Version](https://img.shields.io/crates/v/base32-fs)](https://crates.io/base32-fs/base32-fs)
[![Docs](https://docs.rs/base32-fs/badge.svg)](https://docs.rs/base32-fs)
[![dependency status](https://deps.rs/repo/github/igankevich/base32-fs/status.svg)](https://deps.rs/repo/github/igankevich/base32-fs)

This crate implements a variant of BASE32 encoding for hashes that are used as file names:
no two encoded hashes can be decoded to the same original hash unless there is a hash collision.

To achieve that the crate
- uses the same characters when encoding and decoding the data
  as opposed to the original Crockford's alphabet that permits e.g. both "a" and "A" to be decoded to `10`;
- doesn't zero-extend invalid input lengths when decoding.

Besides that the crate
- uses only lowercase letters instead of uppercase as the most common representation of hashes;
- doesn't use padding characters;
- doesn't change the sorting order of the encoded data.


## Usage


### Encode into `PathBuf`

```rust
use std::path::PathBuf;
use base32_fs::{encode, encoded_len, PathBufOutput};

let input = *b"hello";
let mut output = PathBufOutput::with_capacity(encoded_len(input.len()));
encode(&input, &mut output);
assert_eq!(PathBuf::from("d1jprv3f"), output.into_path_buf());
```


### Decode from `PathBuf`

```rust
use std::path::Path;
use base32_fs::{decode, decoded_len, PathBufInput};

let input = PathBufInput::new(Path::new("d1jprv3f"));
let mut output: Vec<u8> = Vec::new();
decode(input, &mut output);
assert_eq!(b"hello", output.as_slice());
```


### Encode into `Vec<u8>`

```rust
use base32_fs::{encode, encoded_len};

let input = *b"hello";
let mut output: Vec<u8> = Vec::with_capacity(encoded_len(input.len()));
encode(&input, &mut output);
let string = std::str::from_utf8(output.as_slice()).expect("Always a valid UTF-8 byte sequence");
assert_eq!("d1jprv3f", string);
```


### Decode from `&[u8]`

```rust
use std::path::Path;
use base32_fs::{decode, decoded_len, PathBufInput};

let input = b"d1jprv3f";
let mut output: Vec<u8> = Vec::with_capacity(decoded_len(input.len()).unwrap());
decode(input.as_slice(), &mut output);
assert_eq!(b"hello", output.as_slice());
```
