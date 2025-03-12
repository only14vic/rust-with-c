#![no_main]
#![cfg_attr(feature = "no_std", no_std)]

extern crate alloc;

use {
    alloc::string::String,
    app_nostd::prelude::*,
    core::{
        ffi::CStr,
        hint::black_box,
        ptr::{null, null_mut}
    },
    libc::{EXIT_SUCCESS, malloc_stats, pthread_create, pthread_join, pthread_t, usleep}
};

#[no_mangle]
extern "C" fn main() -> i32 {
    log_init();

    let no_std = cfg!(feature = "no_std");
    log::info!("no_std = {no_std}");

    println!("Hello, World!");

    let mut thread: pthread_t = 0;
    let mut value = String::with_capacity(100);
    value.push_str("Data from Main.\0");
    let value_ptr = value.as_mut_ptr();

    let ret = unsafe {
        pthread_create(&mut thread, null(), hello_lib_pthread, value_ptr.cast());

        for _ in 0..5 {
            println!("Main thread");
            usleep(100);
        }

        pthread_join(thread, null_mut())
    };

    let value_str = unsafe { CStr::from_ptr(value_ptr.cast()).to_string_lossy() };
    println!("Thread return: {ret}\nThread value: {value_str:?}",);

    let x: u8 = black_box(1);
    println!("x = {x}");
    dbg!(x - 1);

    unsafe { malloc_stats() };

    // Waits for key pressing
    // unsafe { libc::getchar() };

    EXIT_SUCCESS
}
