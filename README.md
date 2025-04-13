# basis32

This crate implements modified BASE32 encoding that is useful for using encoded file hashes as file names:
no two encoded file hashes can be decoded to the same original hash unless there is a hash collision.

To achieve that the crate
- uses the same characters when encoding and decoding the data
  as opposed to the original Crockford's alphabet that permits e.g. both "a" and "A" to be decoded to `10`;
- doesn't zero-extend invalid input lengths when decoding.

Besides that the crate
- uses only lowercase letters instead of uppercase as the most common representation of hashes;
- doesn't use padding characters;
- doesn't change the sorting order of the encoded data.
