use {app_nostd::prelude::*, core::ffi::CStr};

#[test]
fn test_hello_lib() {
    let ptr = hello_lib();
    let str = unsafe { CStr::from_ptr(ptr).to_str().unwrap() };
    println!("{str}");
}
