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
