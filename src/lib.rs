#![no_std]

#[cfg(test)]
extern crate std;

#[cfg(test)]
extern crate alloc;

mod encode;

pub use self::encode::*;
