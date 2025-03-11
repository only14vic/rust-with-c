use {alloc::boxed::Box, core::ffi::c_char, libc::sprintf};

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
