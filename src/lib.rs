#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
#[macro_use]
extern crate std;
extern crate alloc;

mod no_std;

pub mod prelude;
pub mod logger;
pub mod cbind;
pub mod foo;
