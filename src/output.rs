/// An output of [`encode`](crate::encode) or [`decode`](crate::decode).
///
/// This is a simpler alternative to [`Write`](std::io::Write).
pub trait Output {
    /// Output one byte.
    fn push(&mut self, ch: u8);
}

impl<T: From<u8>> Output for &mut [T] {
    fn push(&mut self, ch: u8) {
        // TODO split_at_mut_unchecked?
        let (a, b) = core::mem::take(self).split_at_mut(1);
        a[0] = ch.into();
        *self = b;
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<T: From<u8>> Output for alloc::vec::Vec<T> {
    fn push(&mut self, ch: u8) {
        alloc::vec::Vec::<T>::push(self, ch.into());
    }
}
