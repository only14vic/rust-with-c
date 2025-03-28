#![no_main]
#![no_std]

include!("no_std.rs");

extern crate alloc;

use {alloc::string::String, libc::EXIT_SUCCESS};

#[no_mangle]
extern "C" fn main() -> i32 {
    println!("Hello, World!");

    unsafe {
        log_init();
        foo_init();

        example("Hello!".into());
    };

    EXIT_SUCCESS
}

#[link(name = "app_nostd")]
unsafe extern "C" {
    fn foo_init();
    fn log_init();
}

/*
// Use it to link only extern "Rust" and not "C"
//
#[link(name = "app_nostd")]
unsafe extern "C" {}
*/

unsafe extern "Rust" {
    fn example(a: String);
}
