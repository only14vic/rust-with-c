extern crate alloc;

#[allow(unused_imports)]
#[cfg(feature = "no_std")]
use libc_print::std_name::*;
use {alloc::boxed::Box, core::ffi::c_char, libc::sprintf};

// Manual bind C function
//
// #[link(name = "ircclient")]
// unsafe extern "C" {
//     pub fn irc_create_session(callbacks: *const libc::c_void)
//              -> *const libc::c_void;
// }

#[no_mangle]
pub extern "C" fn hello_lib() -> *const c_char {
    let buffer: *mut c_char = Box::into_raw(Box::new([0u8; 100])) as *mut i8;

    unsafe {
        sprintf(
            buffer,
            c"Привет из \"%s\"".as_ptr(),
            concat!(module_path!(), "\0").as_ptr()
        );
    }

    //dbg!(unsafe { CStr::from_ptr(buffer) });

    return buffer;
}

#[no_mangle]
pub extern "C" fn lib_foo_callback(a: i32) -> *const c_char {
    let buffer: *mut c_char = Box::into_raw(Box::new([0u8; 100])) as *mut i8;

    unsafe {
        sprintf(buffer, c"a=%d".as_ptr(), a);
    }

    return buffer;
}

pub type FooCallback = unsafe extern "C" fn(i32) -> *const c_char;
