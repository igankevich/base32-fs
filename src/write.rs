pub trait Write {
    fn push(&mut self, ch: u8);
}

impl<T: From<u8>> Write for &mut [T] {
    fn push(&mut self, ch: u8) {
        let (a, b) = core::mem::take(self).split_at_mut(1);
        a[0] = ch.into();
        *self = b;
    }
}

#[cfg(feature = "alloc")]
impl<T: From<u8>> Write for alloc::vec::Vec<T> {
    fn push(&mut self, ch: u8) {
        alloc::vec::Vec::<T>::push(self, ch.into());
    }
}
