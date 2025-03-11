use {
    app_nostd::prelude::*,
    core::ffi::{CStr, c_char}
};

#[test]
fn test_hello_lib() {
    let mut last_ptr: *const c_char = core::ptr::null();

    for i in 0..5 {
        let ptr = hello_lib(i);

        if last_ptr.is_null() {
            last_ptr = ptr;
        }
        assert_eq!(ptr as *const c_char, last_ptr);

        let str = unsafe { CStr::from_ptr(ptr).to_string_lossy() };
        println!("[{ptr:p}]: {str}");

        let _ = unsafe { Box::from_raw(ptr) };
        // OR you may use libc::free()
        //unsafe { libc::free(ptr as *mut c_void) };
    }
}
