#![cfg_attr(feature = "no_std", no_std)]

/// Можно использовать std если отлючить feature = "no_std"
#[cfg(not(feature = "no_std"))]
#[allow(unused_imports)]
#[macro_use]
extern crate std;

pub mod prelude;
pub mod irc;

mod no_std_core;
