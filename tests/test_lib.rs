use {
    app_nostd::prelude::*,
    core::{
        ffi::{CStr, c_char},
        ptr::addr_of
    }
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

        let _ = unsafe { Box::from_raw(ptr) };
        // OR you may use libc::free()
        //unsafe { free(ptr as *mut c_void) };
    }
}
