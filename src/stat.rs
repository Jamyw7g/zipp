use std::ffi::CStr;

use zipp_sys::*;

#[derive(Debug, Clone)]
pub struct Stat {
    pub valid: u64,
    pub name: String,
    pub index: u64,
    pub size: u64,
    pub comp_size: u64,
    pub mtime: time_t,
    pub crc: u32,
    pub comp_method: u16,
    pub encryption_method: u16,
    pub flags: u32,
}

impl From<zip_stat_t> for Stat {
    fn from(stat: zip_stat_t) -> Self {
        let zip_stat_t {
            valid,
            name,
            index,
            size,
            comp_size,
            mtime,
            crc,
            comp_method,
            encryption_method,
            flags,
        } = stat;
        let name = unsafe { CStr::from_ptr(name).to_string_lossy().to_string() };
        Stat {
            valid,
            name,
            index,
            size,
            comp_size,
            mtime,
            crc,
            comp_method,
            encryption_method,
            flags,
        }
    }
}
