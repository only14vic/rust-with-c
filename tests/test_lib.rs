use {
    app_nostd::prelude::*,
    core::{
        ffi::{CStr, c_char, c_void},
        ptr::addr_of
    },
    libc::free
};

#[test]
fn test_hello_lib() {
    let mut last_ptr: *const c_char = core::ptr::null();

    for i in 0..5 {
        let ptr = hello_lib(i);

        if last_ptr.is_null() {
            last_ptr = ptr;
        }
        assert_eq!(addr_of!(*ptr), addr_of!(*last_ptr));

        let str = unsafe { CStr::from_ptr(ptr).to_str().unwrap() };
        println!("[{str:p}]: {str}");

        unsafe { free(ptr as *mut c_void) };
    }
}
