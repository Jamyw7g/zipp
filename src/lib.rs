use std::ffi::CStr;
use std::mem::zeroed;
use std::os::raw::c_int;
use std::ptr::NonNull;

mod macros;
mod error;

pub use error::Error;


#[macro_use]
extern crate bitflags;

pub use zipp_sys::*;

bitflags! {
    pub struct OpenFlag: c_int {
        const CREATE = 1;
        const EXCL = 2;
        const CHECKCONS = 4;
        const TRUNCATE = 8;
        const RDONLY = 16;
    }
}

#[derive(Debug)]
pub struct Archive {
    inner: NonNull<zip_t>,
}

bitflags! {
    pub struct StatFlag: u32 {
        const ZIP_STAT_NAME= 1;
        const ZIP_STAT_INDEX= 2;
        const ZIP_STAT_SIZE= 4;
        const ZIP_STAT_COMP_SIZE= 8;
        const ZIP_STAT_MTIME= 16;
        const ZIP_STAT_CRC= 32;
        const ZIP_STAT_COMP_METHOD=64;
        const ZIP_STAT_ENCRYPTION_METHOD = 128;
        const ZIP_STAT_FLAGS= 256;
    }
}

impl Archive {
    pub fn open(path: &CStr) -> ZResult<Self> {
        _open(path, OpenFlag::RDONLY.bits())
    }

    pub fn create(path: &CStr) -> ZResult<Self> {
        _open(path, OpenFlag::CREATE.bits())
    }

    pub fn discard(&self) {
        unsafe { zip_discard(self.inner.as_ptr()); }
    }

    pub fn stat_name(&self, name: &CStr, flag: StatFlag) -> ZResult<zip_stat_t> {
        let mut stat = unsafe { zeroed() };
        let res = unsafe { zip_stat(self.inner.as_ptr(), name.as_ptr(), flag.bits(), &mut stat) };
        if res == -1 {
            zip_err!(self.inner.as_ptr())
        } else {
            Ok(stat)
        }
    }

    pub fn stat_index(&self, index: u64, flag: StatFlag) -> ZResult<zip_stat_t> {
        let mut stat = unsafe { zeroed() };
        let res = unsafe { zip_stat_index(self.inner.as_ptr(), index, flag.bits(), &mut stat) };
        if res == -1 {
            zip_err!(self.inner.as_ptr())
        } else {
            Ok(stat)
        }
    }
}

// Using RAII, close and save zip in default.
impl Drop for Archive {
    fn drop(&mut self) {
        let r = unsafe { zip_close(self.inner.as_ptr()) };
        debug_assert!(r == 0);
    }
}

type ZResult<T> = Result<T, Error>;

#[derive(Debug)]
pub struct OpenOptions {
    inner: OpenFlag,
}

impl OpenOptions {
    pub fn new() -> Self {
        Self {
            inner: OpenFlag::RDONLY,
        }
    }

    pub fn create(&mut self, flag: bool) -> &mut Self {
        if flag {
            self.inner.insert(OpenFlag::CREATE);
        }
        self
    }

    pub fn excl(&mut self, flag: bool) -> &mut Self {
        if flag {
            self.inner.insert(OpenFlag::EXCL);
        }
        self
    }

    pub fn truncate(&mut self, flag: bool) -> &mut Self {
        if flag {
            self.inner.insert(OpenFlag::TRUNCATE);
        }
        self
    }

    pub fn open(&self, path: &CStr) -> ZResult<Archive> {
        _open(path, self.inner.bits())
    }
}

#[inline]
fn _open(path: &CStr, flag: c_int) -> ZResult<Archive> {
    unsafe {
        let mut err = 0;
        let res = zip_open(path.as_ptr(), flag, &mut err);
        if err != 0 {
            Err(Error::from(err))
        } else {
            Ok(Archive {
                inner: NonNull::new_unchecked(res),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_archive() {
        let name = CString::new("Hello").unwrap();
        let archive = OpenOptions::new().open(&name);
        assert!(archive.is_err());

        let name = CString::new("hello.zip").unwrap();
        let archive = OpenOptions::new().open(&name);
        assert!(archive.is_ok());

        let stat = archive.unwrap().stat_index(u64::MAX, StatFlag::all());
        assert!(stat.is_err());
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
