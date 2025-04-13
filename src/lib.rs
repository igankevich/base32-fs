#![no_std]

#[cfg(any(feature = "alloc", test))]
extern crate alloc;

#[cfg(any(feature = "std", test))]
extern crate std;

mod encode;
mod write;

pub use self::encode::*;
pub use self::write::*;
