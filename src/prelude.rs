#[cfg(not(feature = "std"))]
pub use libc_print::std_name::*;

pub use crate::{foo::*, logger::*};
