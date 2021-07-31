pub mod archive;
pub mod error;
pub mod file;
pub mod source;
pub mod consts;
mod macros;

pub use archive::*;
pub use error::{Error, ZResult};
pub type Stat = zip_stat_t;

use error::ZipErrorSys;
use zipp_sys::*;

use libc::{mktime, time_t};
use std::{ffi::CStr, mem::zeroed};


pub fn version() -> &'static str {
    let ver = unsafe { CStr::from_ptr(zip_libzip_version()) };

    ver.to_str().unwrap()
}

pub fn cal_mtime(sec: i32, min: i32, hour: i32, day: i32, mon: i32, year: i32) -> time_t {
    let mut tm: libc::tm = unsafe { zeroed() };
    tm.tm_sec = sec;
    tm.tm_min = min;
    tm.tm_hour = hour;
    tm.tm_mday = day;
    tm.tm_mon = mon - 1;
    tm.tm_year = year - 1900;

    unsafe { mktime(&mut tm) }
}

pub fn compression_method_supported(method: i32, is_comp: bool) -> bool {
    unsafe { zip_compression_method_supported(method, is_comp as _) == 1 }
}

pub fn encryption_method_supported(method: u16, is_encrypt: bool) -> bool {
    unsafe { zip_encryption_method_supported(method, is_encrypt as _) == 1 }
}

