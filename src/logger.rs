extern crate alloc;

use {
    core::{ffi::CStr, str::FromStr},
    libc::getenv,
    log::{LevelFilter, Log}
};
#[cfg(feature = "no_std")]
use libc_print::std_name::*;

static LOGGER: Logger = Logger;

pub struct Logger;

impl Logger {
    pub fn init() {
        log::set_logger(&LOGGER).unwrap();

        let level: LevelFilter = unsafe {
            match getenv(c"LOG_LEVEL".as_ptr()) {
                level if level.is_null() == false => {
                    LevelFilter::from_str(
                        CStr::from_ptr(level)
                            .to_string_lossy()
                            .trim_end_matches("=")
                    )
                    .unwrap()
                },
                _ => {
                    if cfg!(debug_assertions) {
                        LevelFilter::Debug
                    } else {
                        LevelFilter::Info
                    }
                },
            }
        };

        log::set_max_level(level);
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            println!("[{}] {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
