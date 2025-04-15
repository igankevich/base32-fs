/// BASE32 decode error.
///
/// Either the length is wrong or some characters are invalid.
#[derive(Debug)]
pub struct DecodeError;

#[cfg(feature = "std")]
impl std::error::Error for DecodeError {}

impl core::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str("BASE32 decode error")
    }
}
