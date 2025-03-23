use {
    crate::cbind,
    ahash::AHasher,
    alloc::{
        boxed::Box,
        ffi::CString,
        string::{String, ToString}
    },
    core::{
        ffi::{CStr, c_char, c_int, c_void},
        hash::BuildHasherDefault,
        mem::MaybeUninit,
        ptr::{NonNull, null_mut}
    },
    indexmap::IndexMap,
    libc::{
        F_OK, PATH_MAX, access, dirname, getcwd, printf, pthread_mutex_init,
        pthread_mutex_lock, pthread_mutex_t, pthread_mutex_unlock, readlink, sched_yield,
        sprintf, strcpy, strdup, strlen, usleep
    },
    serde_json::json
};

#[allow(unused_imports)]
use crate::prelude::*;

#[no_mangle]
pub static mut MUTEX: MaybeUninit<pthread_mutex_t> = MaybeUninit::zeroed();

pub type ConfigMap = IndexMap<
    Box<CStr>,
    IndexMap<Box<CStr>, CString, BuildHasherDefault<AHasher>>,
    BuildHasherDefault<AHasher>
>;

#[no_mangle]
extern "Rust" fn example(a: String) {
    println!("ok: {a:?}");
}

#[no_mangle]
pub extern "C" fn foo_init() {
    // Init MUTEX
    unsafe { pthread_mutex_init(MUTEX.as_mut_ptr(), null_mut()) };

    // Load config
    unsafe {
        let mut buffer: *mut c_char =
            Box::into_raw(Box::new([0u8; PATH_MAX as usize])).cast();
        if readlink(c"/proc/self/exe".as_ptr(), buffer, PATH_MAX as usize - 1) < 0 {
            panic!("Couldn't get executable file's path.");
        }
        buffer = dirname(buffer);
        let exe_path = CString::from_raw(buffer).into_string().unwrap();

        let cwd = CString::from_raw(getcwd(null_mut(), 0))
            .into_string()
            .unwrap();
        println!("CWD: {cwd}");
        println!("EXE_PATH: {exe_path}");

        const CONFIG_PATH: &str = "config/app.ini";
        let mut config_path = String::new();
        let mut config_subdir = "/".to_string();
        let mut config_exists = false;

        for _ in 0..10 {
            config_path = [&exe_path, &config_subdir, CONFIG_PATH].concat();
            if access(config_path.as_ptr().cast(), F_OK) == 0 {
                config_exists = true;
                break;
            }
            config_subdir.push_str("../");
        }

        if !config_exists {
            panic!("Config file '{CONFIG_PATH}' could not be found.");
        }

        let mut config = ConfigMap::default();

        if cbind::ini_parse(
            config_path.as_ptr().cast(),
            Some(config_map_load),
            (&mut config as *mut ConfigMap).cast()
        ) != 0
        {
            panic!("Couldn't parse config file.");
        }

        println!("Config: {config:#?}");

        // let a = config[c"general"][c"boolean"]
        //     .to_string_lossy()
        //     .parse::<bool>();
    };

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

#[derive(Debug)]
#[repr(C)]
pub struct FooStruct {
    foo: Option<NonNull<c_char>>,
    bar: Option<NonNull<c_char>>
}

impl FooStruct {
    #[no_mangle]
    unsafe extern "C" fn foo_create(a: *const c_char, b: *const c_char) -> Box<Self> {
        let this = Self {
            foo: if a.is_null() { None } else { NonNull::new(strdup(a)) },
            bar: if b.is_null() { None } else { NonNull::new(strdup(b)) }
        };

        println!("Created: {this:#?}");

        this.into()
    }

    #[no_mangle]
    extern "C" fn foo_drop(self: Box<Self>) {}
}

impl Drop for FooStruct {
    fn drop(&mut self) {
        if let Some(p) = self.foo {
            let _ = unsafe { Box::from_raw(p.as_ptr()) };
        }
        if let Some(p) = self.bar {
            let _ = unsafe { Box::from_raw(p.as_ptr()) };
        }

        println!("FooStruct dropped.");
    }
}

unsafe extern "C" fn config_map_load(
    user: *mut c_void,
    section: *const c_char,
    name: *const c_char,
    value: *const c_char
) -> c_int {
    let config: &mut ConfigMap = &mut *user.cast();
    let section = CStr::from_ptr(strdup(section));
    let name = CStr::from_ptr(strdup(name));
    let value = CStr::from_ptr(value);

    if config.contains_key(section) == false {
        config.insert(section.into(), Default::default());
    }

    let mut value = value.to_str().unwrap();

    if let (Some(fc), Some(lc)) = (value.chars().next(), value.chars().last()) {
        if ['\'', '\"'].contains(&fc) && fc == lc {
            value = &value[1..value.chars().count() - 1];
        };
    }

    config[section].insert(name.into(), CString::new(value).unwrap());

    return 1;
}
