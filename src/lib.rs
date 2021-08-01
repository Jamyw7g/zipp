//! zipp provides a libzip's Rust bindings.
//! It make you easy using libzip in Rust project.

/// Wrap zip_archive_t
pub mod archive;
/// Some const-variable flag
pub mod consts;
/// Provide a error convertor, convert c-api error to Rust
pub mod error;
/// Wrap zip_file_t
pub mod file;
/// Internal macros wrap
mod macros;
/// Wrap zip_source_t
pub mod source;

pub use archive::*;
pub use error::{Error, ZResult};

/// The alias of zip_stat_t
pub type Stat = zip_stat_t;

use error::ZipErrorSys;
use zipp_sys::*;

use libc::{mktime, time_t};
use std::{ffi::CStr, mem::zeroed};

/// show libzip version
pub fn version() -> &'static str {
    let ver = unsafe { CStr::from_ptr(zip_libzip_version()) };

    ver.to_str().unwrap()
}

/// convert human time to time_t
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

/// check any (de)compression method supported
pub fn compression_method_supported(method: i32, is_comp: bool) -> bool {
    unsafe { zip_compression_method_supported(method, is_comp as _) == 1 }
}

/// check any (de)encryption method supported
pub fn encryption_method_supported(method: u16, is_encrypt: bool) -> bool {
    unsafe { zip_encryption_method_supported(method, is_encrypt as _) == 1 }
}
