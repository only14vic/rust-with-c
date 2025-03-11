#![no_main]
#![cfg_attr(feature = "no_std", no_std)]

extern crate alloc;

use {
    alloc::boxed::Box,
    app_nostd::prelude::*,
    core::{ffi::CStr, hint::black_box},
    libc::EXIT_SUCCESS
};

#[no_mangle]
extern "C" fn main() -> i32 {
    log_init();

    let no_std = cfg!(feature = "no_std");
    log::info!("no_std = {no_std}");

    println!("Hello, World!");

    for i in 0..5 {
        let ptr = hello_lib(i);
        let str = unsafe { CStr::from_ptr(ptr).to_str().unwrap() };
        println!("[{str:p}] {str}");
        let _ = unsafe { Box::from_raw(ptr) };
    }

    let x: u8 = black_box(1);
    println!("x = {x}");
    dbg!(x - 1);

    // Waits for key pressing
    // unsafe { libc::getchar() };

    EXIT_SUCCESS
}
