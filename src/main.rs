#![cfg_attr(feature = "no_std", no_std)]
#![no_main]

use {
    app_nostd::{Logger, prelude::*},
    core::{ffi::CStr, hint::black_box},
    libc::EXIT_SUCCESS
};
#[cfg(feature = "no_std")]
use libc_print::std_name::*;

#[no_mangle]
extern "C" fn main() -> i32 {
    Logger::init();

    let no_std = cfg!(feature = "no_std");
    log::info!("no_std = {no_std}");

    println!("Hello, World!");

    let res = hello_lib();
    let str = unsafe { CStr::from_ptr(res).to_str().unwrap() };
    println!("{str}");

    let x: u8 = black_box(1);
    println!("x = {x}");
    dbg!(x - 1);

    EXIT_SUCCESS
}
