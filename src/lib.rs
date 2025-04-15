#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]

#[cfg(any(feature = "alloc", test))]
extern crate alloc;

#[cfg(any(feature = "std", test))]
extern crate std;

mod alphabet;
mod decode;
mod encode;
mod error;
mod input;
mod output;
#[cfg(all(feature = "std", any(unix, windows)))]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
mod path_buf;

pub(crate) use self::alphabet::*;
pub use self::decode::*;
pub use self::encode::*;
pub use self::error::*;
pub use self::input::*;
pub use self::output::*;
#[cfg(all(feature = "std", any(unix, windows)))]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use self::path_buf::*;
