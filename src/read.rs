use core::iter::FusedIterator;

/// An input for [`decode`](crate::decode).
pub trait Input<const N: usize> {
    /// Get the next chunk of size `N` from the input.
    ///
    /// Returns `None` if the input doesn't have sufficient number of bytes.
    fn next_chunk(&mut self) -> Option<&[u8]>;

    /// Get the remainder of the input.
    ///
    /// Should only be called after [`next_chunk`](Self::next_chunk) returns `None`.
    fn remainder(&self) -> &[u8];
}

impl<const N: usize> Input<N> for &[u8] {
    fn next_chunk(&mut self) -> Option<&[u8]> {
        let (chunk, rest) = self.split_at_checked(N)?;
        *self = rest;
        Some(chunk)
    }

    fn remainder(&self) -> &[u8] {
        self
    }
}

/// The implementation of the [`Input`](crate::Input) for iterators.
///
/// Useful to decode BASE32 values directly from [`EncodeWide`](std::os::windows::ffi::EncodeWide)
/// on Windows (via [`encode_wide`](std::os::windows::ffi::OsStrExt::encode_wide).
pub struct ReadIter<I, T, const N: usize>
where
    I: Iterator<Item = T> + FusedIterator,
    T: Into<u8>,
{
    iter: I,
    chunk: [u8; N],
    remainder_len: usize,
}

impl<I, T, const N: usize> ReadIter<I, T, N>
where
    I: Iterator<Item = T> + FusedIterator,
    T: Into<u8>,
{
    /// Create new input from the supplied iterator.
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            chunk: [0; N],
            remainder_len: 0,
        }
    }
}

impl<I, T, const N: usize> Input<N> for ReadIter<I, T, N>
where
    I: Iterator<Item = T> + FusedIterator,
    T: Into<u8>,
{
    fn next_chunk(&mut self) -> Option<&[u8]> {
        for i in 0..N {
            match self.iter.next() {
                Some(x) => self.chunk[i] = x.into(),
                None => {
                    self.remainder_len = i;
                    return None;
                }
            }
        }
        Some(&self.chunk[..])
    }

    fn remainder(&self) -> &[u8] {
        &self.chunk[..self.remainder_len]
    }
}
