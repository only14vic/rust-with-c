use {
    crate::prelude::*,
    alloc::boxed::Box,
    core::ffi::{CStr, c_char, c_void},
    libc::{printf, sprintf, strcpy, strlen, usleep}
};

#[no_mangle]
pub extern "C" fn hello_lib(a: i32) -> *mut c_char {
    let buffer: *mut c_char = Box::into_raw(Box::new([0u8; 100])).cast();
    unsafe {
        sprintf(
            buffer,
            c"Привет из \"%s\" (a=%d)".as_ptr(),
            concat!(module_path!(), "\0").as_ptr(),
            a
        )
    };

    log::trace!("hello_lib(): [{buffer:p}]");
    return buffer;
}

#[no_mangle]
pub extern "C" fn hello_lib_pthread(arg: *mut c_void) -> *mut c_void {
    let value = unsafe { CStr::from_ptr(arg.cast()).to_string_lossy() };
    println!("Thread argument: {value:?}",);

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

        unsafe { usleep(100) };
    }

    unsafe { strcpy(arg.cast(), c"Data from Thread.".as_ptr()) };

    return arg;
}

#[no_mangle]
pub extern "C" fn lib_foo_callback(a: i32) -> *mut c_char {
    let buffer: *mut c_char = Box::into_raw(Box::new([0_i8; 100])).cast();
    unsafe { sprintf(buffer, c"Foo callback: a=%d".as_ptr(), a) };

    log::trace!("lib_foo_callback(): [{buffer:p}]");
    return buffer;
}

pub type FooCallback = unsafe extern "C" fn(i32) -> *mut c_char;
