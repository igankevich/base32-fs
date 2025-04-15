#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(any(feature = "alloc", test))]
extern crate alloc;

#[cfg(any(feature = "std", test))]
extern crate std;

mod alphabet;
mod decode;
mod encode;
mod error;
mod read;
mod write;

pub(crate) use self::alphabet::*;
pub use self::decode::*;
pub use self::encode::*;
pub use self::error::*;
pub use self::read::*;
pub use self::write::*;
