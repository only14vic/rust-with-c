#![no_main]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
#[macro_use]
extern crate std;
extern crate alloc;
extern crate app_nostd;

use {
    alloc::string::String,
    app_nostd::prelude::*,
    core::{ffi::CStr, hint::black_box, mem::zeroed, ptr::null_mut},
    libc::{
        EXIT_SUCCESS, getpid, malloc_stats, pthread_attr_destroy, pthread_attr_init,
        pthread_attr_setstacksize, pthread_attr_t, pthread_create, pthread_join,
        pthread_mutex_lock, pthread_mutex_unlock, pthread_t, sched_yield, usleep
    }
};

extern "Rust" {
    fn example(a: String);
}

#[no_mangle]
extern "C" fn main() -> i32 {
    log_init();
    foo_init();

    unsafe { example("Rust".into()) };

    #[cfg(not(feature = "std"))]
    {
        use alloc::vec;
        let a = rustix::process::getcwd(vec![]);
        println!("Rustix getcwd: {a:#?}");
    }

    let no_std = cfg!(not(feature = "std"));
    log::info!("no_std = {no_std}");

    println!("Hello, World!");
    println!("PID: {}", unsafe { getpid() });

    //unsafe { libc::getchar() };

    #[cfg(feature = "std")]
    {
        println!("PWD stdlib: {:?}", std::env::current_dir().unwrap());

        //unsafe { libc::getchar() };

        let _ = std::thread::spawn(|| {
            let v = std::fs::read_to_string(".env").unwrap();
            println!("{v}");
        })
        .join();
    }

    //unsafe { libc::getchar() };

    let mut attr: pthread_attr_t = unsafe { zeroed() };
    unsafe {
        pthread_attr_init(&mut attr);
        pthread_attr_setstacksize(&mut attr, 24 * 1024);
    }
    let mut thread: pthread_t = 0;
    let mut value = String::with_capacity(100);
    value.push_str("Data from Main.\0");
    let value_ptr = value.as_mut_ptr();

    let ret = unsafe {
        pthread_create(&mut thread, &attr, hello_lib_pthread, value_ptr.cast());

        for _ in 0..5 {
            pthread_mutex_lock(MUTEX.as_mut_ptr());
            println!("Main thread");
            pthread_mutex_unlock(MUTEX.as_mut_ptr());

            sched_yield();
            usleep(1);
        }

        let res = pthread_join(thread, null_mut());
        pthread_attr_destroy(&mut attr);

        res
    };

    let value_str = unsafe { CStr::from_ptr(value_ptr.cast()).to_string_lossy() };
    println!("Thread return: {ret}\nThread value: {value_str:?}",);

    let x: u8 = black_box(1);
    println!("x = {x}");
    dbg!(x - 1);

    unsafe {
        malloc_stats();
        println!("PID: {}", getpid());
    }

    // Waits for key pressing
    //unsafe { getchar() };

    EXIT_SUCCESS
}
