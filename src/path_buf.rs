use crate::Input;
use crate::Output;

use alloc::vec::Vec;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;

/// An implementation of [`Output`](crate::Output) for file system paths.
///
/// Works on both Unix-like and Windows platforms.
pub struct PathBufOutput {
    #[cfg(windows)]
    bytes: Vec<u16>,
    #[cfg(unix)]
    bytes: Vec<u8>,
}

impl PathBufOutput {
    /// Transform into [`PathBuf`](std::path::PathBuf).
    pub fn into_path_buf(self) -> PathBuf {
        #[cfg(unix)]
        let buf = OsString::from_vec(self.bytes).into();
        #[cfg(windows)]
        let buf = OsString::from_wide(&self.bytes[..]).into();
        buf
    }

    /// Create path buffer from the supplied standard one.
    ///
    /// This method involves memory allocation only on Windows platforms.
    pub fn from_path_buf(path: PathBuf) -> Self {
        #[cfg(unix)]
        let bytes = path.into_os_string().into_vec();
        #[cfg(windows)]
        let bytes = path.as_os_str().encode_wide().collect();
        Self { bytes }
    }

    /// Create path buffer from the supplied path.
    ///
    /// This method involves memory allocation only on Unix platforms.
    pub fn from_path(path: &Path) -> Self {
        #[cfg(unix)]
        let bytes = path.as_os_str().as_bytes().to_vec();
        #[cfg(windows)]
        let bytes = path.as_os_str().encode_wide().collect();
        Self { bytes }
    }

    /// Create empty path buffer.
    pub const fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    /// Create empty path buffer with desired capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            bytes: Vec::with_capacity(capacity),
        }
    }
}

impl From<PathBufOutput> for PathBuf {
    fn from(other: PathBufOutput) -> Self {
        other.into_path_buf()
    }
}

impl From<PathBuf> for PathBufOutput {
    fn from(other: PathBuf) -> Self {
        Self::from_path_buf(other)
    }
}

impl Output for PathBufOutput {
    fn push(&mut self, ch: u8) {
        #[cfg(windows)]
        let ch = ch.into();
        self.bytes.push(ch)
    }
}

/// An implementation of [`Input`](crate::Input) for file system paths.
///
/// Works on both Unix-like and Windows platforms.
pub struct PathBufInput<'a> {
    #[cfg(unix)]
    inner: &'a [u8],
    #[cfg(windows)]
    inner: WideCharIter<'a>,
}

impl<'a> PathBufInput<'a> {
    /// Create new input from the supplied path.
    pub fn new(path: &'a Path) -> Self {
        #[cfg(unix)]
        let inner = path.as_os_str().as_bytes();
        #[cfg(windows)]
        let inner = WideCharIter::new(path.as_os_str().encode_wide());
        Self { inner }
    }
}

impl<'a> From<&'a Path> for PathBufInput<'a> {
    fn from(other: &'a Path) -> Self {
        Self::new(other)
    }
}

impl Input<8> for PathBufInput<'_> {
    fn next_chunk(&mut self) -> Option<&[u8]> {
        Input::<8>::next_chunk(&mut self.inner)
    }

    fn remainder(&self) -> &[u8] {
        Input::<8>::remainder(&self.inner)
    }
}

#[cfg(unix)]
mod unix {
    pub use std::os::unix::ffi::OsStrExt;
    pub use std::os::unix::ffi::OsStringExt;
}

#[cfg(unix)]
use self::unix::*;

#[cfg(windows)]
mod windows {
    use crate::Input;

    pub use std::os::windows::ffi::EncodeWide;
    pub use std::os::windows::ffi::OsStrExt;
    pub use std::os::windows::ffi::OsStringExt;

    pub struct WideCharIter<'a> {
        iter: EncodeWide<'a>,
        chunk: [u8; 8],
        remainder_len: usize,
    }

    impl<'a> WideCharIter<'a> {
        pub fn new(iter: EncodeWide<'a>) -> Self {
            Self {
                iter,
                chunk: [0; 8],
                remainder_len: 0,
            }
        }
    }

    impl<'a> Input<8> for WideCharIter<'a> {
        fn next_chunk(&mut self) -> Option<&[u8]> {
            for i in 0..8 {
                match self.iter.next() {
                    // Map to an invalid character to trigger an error.
                    Some(x) => self.chunk[i] = x.try_into().unwrap_or(u8::MAX),
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
}

#[cfg(windows)]
use self::windows::*;
