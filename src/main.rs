#![no_main]
#![cfg_attr(feature = "no_std", no_std)]

extern crate alloc;

use {
    alloc::boxed::Box,
    app_nostd::prelude::*,
    core::{ffi::CStr, hint::black_box},
    libc::{EXIT_SUCCESS, malloc_stats, printf, strlen}
};

#[no_mangle]
extern "C" fn main() -> i32 {
    log_init();

    let no_std = cfg!(feature = "no_std");
    log::info!("no_std = {no_std}");

    println!("Hello, World!");

    for i in 0..5 {
        let ptr = hello_lib(i);
        let str = unsafe { CStr::from_ptr(ptr) };

        assert_eq!(ptr.cast_const(), str.as_ptr());

        // Too slowly
        /*
        println!(
            "[{ptr:p}] {} (strlen={})",
            str.to_string_lossy(),
            str.count_bytes()
        );
        */

        // 2x faster than println!()
        unsafe {
            printf(
                c"[%p] %s (strlen=%ld)\n".as_ptr(),
                ptr,
                str.as_ptr(),
                strlen(ptr)
            )
        };

        let _ = unsafe { Box::from_raw(ptr) };
    }

    let x: u8 = black_box(1);
    println!("x = {x}");
    dbg!(x - 1);

    unsafe { malloc_stats() };

    // Waits for key pressing
    // unsafe { libc::getchar() };

    EXIT_SUCCESS
}
