use {app_nostd::prelude::*, core::ffi::CStr};

#[test]
fn test_hello_lib() {
    unsafe {
        println!("{}", CStr::from_ptr(hello_lib()).to_str().unwrap());
    }
}
