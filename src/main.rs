#![no_main]
#![cfg_attr(feature = "no_std", no_std)]

use {
    app_nostd::prelude::*,
    core::{
        ffi::{CStr, c_void},
        hint::black_box
    },
    libc::{EXIT_SUCCESS, free}
};

#[no_mangle]
extern "C" fn main() -> i32 {
    log_init();

    let no_std = cfg!(feature = "no_std");
    log::info!("no_std = {no_std}");

    println!("Hello, World!");

    for i in 0..5 {
        let res = hello_lib(i);
        let str = unsafe { CStr::from_ptr(res).to_str().unwrap() };
        println!("[{str:p}] {str}");
        unsafe { free(res as *mut c_void) };
    }

    let x: u8 = black_box(1);
    println!("x = {x}");
    dbg!(x - 1);

    EXIT_SUCCESS
}
