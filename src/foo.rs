use {
    alloc::boxed::Box,
    core::{
        ffi::{CStr, c_char, c_void},
        mem::MaybeUninit,
        ptr::null_mut
    },
    libc::{
        printf, pthread_mutex_init, pthread_mutex_lock, pthread_mutex_t,
        pthread_mutex_unlock, sched_yield, sprintf, strcpy, strlen, usleep
    },
    serde_json::json
};

#[allow(unused_imports)]
use crate::prelude::*;

#[no_mangle]
pub static mut MUTEX: MaybeUninit<pthread_mutex_t> = MaybeUninit::zeroed();

#[allow(dead_code)]
#[repr(C)]
struct FooStruct {
    foo: *mut c_char
}

#[no_mangle]
pub extern "C" fn foo_init() {
    unsafe { pthread_mutex_init(MUTEX.as_mut_ptr(), null_mut()) };

    println!("JSON: {}", json!("Hello JSON!"));
}

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

        unsafe {
            pthread_mutex_lock(MUTEX.as_mut_ptr());
            // 2x faster than println!()
            printf(
                c"[%p] %s (strlen=%ld)\n".as_ptr(),
                ptr,
                str.as_ptr(),
                strlen(ptr)
            );
            pthread_mutex_unlock(MUTEX.as_mut_ptr());

            let _ = Box::from_raw(ptr);

            sched_yield();
            usleep(1);
        }
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
