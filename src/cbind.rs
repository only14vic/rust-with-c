#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/include/bindings.rs"));

// Manual bind C function
//
// #[link(name = "ircclient")]
// unsafe extern "C" {
//     pub fn irc_create_session(callbacks: *const libc::c_void)
//              -> *const libc::c_void;
// }
