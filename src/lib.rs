#![cfg_attr(feature = "no_std", no_std)]

#[cfg(not(feature = "no_std"))]
#[macro_use]
extern crate std;
extern crate alloc;

mod no_std;

pub mod prelude;
pub mod logger;
pub mod irc;
pub mod foo;
