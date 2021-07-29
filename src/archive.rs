use std::ffi::CStr;
use std::marker::PhantomData;
use std::mem::{ManuallyDrop, zeroed};
use std::os::raw::c_int;
use std::ptr::{NonNull, null};

use zipp_sys::*;

use crate::file::File;
use crate::source::Source;
use crate::stat::Stat;
use crate::*;

#[derive(Debug)]
pub struct Archive {
    inner: NonNull<zip_t>,
}

impl Archive {
    pub fn open(path: &CStr) -> ZResult<Self> {
        _open(path, OpenFlag::RDONLY.bits())
    }

    pub fn from_source<T>(source: Source<T>, flag: i32) -> ZResult<Self> {
        let mut error = ZipErrorSys::default();
        let res = unsafe { zip_open_from_source(source.into_raw(), flag, &mut *error) };
        if res.is_null() {
            Err(error.into())
        } else {
            unsafe {
                Ok(Self {
                    inner: NonNull::new_unchecked(res),
                })
            }
        }
    }

    pub fn create(path: &CStr) -> ZResult<Self> {
        _open(path, OpenFlag::CREATE.bits())
    }

    pub fn discard(self) {
        let me = ManuallyDrop::new(self);
        unsafe {
            zip_discard(me.inner.as_ptr());
        }
    }

    pub fn close(self) -> ZResult<()> {
        let me = ManuallyDrop::new(self);
        let errno = unsafe { zip_close(me.inner.as_ptr()) };
        if errno == -1 {
            zip_err!(me.inner.as_ptr())
        } else {
            Ok(())
        }
    }

    pub fn set_default_password(&self, password: &CStr) -> ZResult<()> {
        let errno = unsafe { zip_set_default_password(self.inner.as_ptr(), password.as_ptr()) };
        if errno == -1 {
            zip_err!(self.inner.as_ptr())
        } else {
            Ok(())
        }
    }

    pub fn open_file(&self, name: &CStr, flag: u32, passwd: Option<&CStr>) -> ZResult<File> {
        let passwd = if passwd.is_none() { null() } else { passwd.unwrap().as_ptr() };
        let res = unsafe { zip_fopen_encrypted(self.inner.as_ptr(), name.as_ptr(), flag, passwd) };
        if res.is_null() {
            zip_err!(self.inner.as_ptr())
        } else {
            Ok(File::from_ptr(res))
        }
    }

    pub fn open_file_index(&self, index: u64, flag: u32, passwd: Option<&CStr>) -> ZResult<File> {
        let passwd = if passwd.is_none() { null() } else { passwd.unwrap().as_ptr() };
        let res = unsafe { zip_fopen_index_encrypted(self.inner.as_ptr(), index, flag, passwd) };
        if res.is_null() {
            zip_err!(self.inner.as_ptr())
        } else {
            Ok(File::from_ptr(res))
        }
    }

    pub fn file_rename(&self, index: u64, name: &CStr, flag: ZipFlag) -> ZResult<()> {
        let errno = unsafe { zip_file_rename(self.inner.as_ptr(), index, name.as_ptr(), flag.bits()) };
        if errno == -1 {
            zip_err!(self.inner.as_ptr())
        } else {
            Ok(())
        }
    }

    pub fn file_add<T>(&self, name: &CStr, source: Source<T>, flag: u32) -> ZResult<u64> {
        let res =
            unsafe { zip_file_add(self.inner.as_ptr(), name.as_ptr(), source.into_raw(), flag) };
        if res == -1 {
            zip_err!(self.inner.as_ptr())
        } else {
            Ok(res as _)
        }
    }

    pub fn file_replace<T>(&self, index: u64, source: Source<T>, flag: u32) -> ZResult<()> {
        let res = unsafe { zip_file_replace(self.inner.as_ptr(), index, source.into_raw(), flag) };
        if res == -1 {
            zip_err!(self.inner.as_ptr())
        } else {
            Ok(())
        }
    }

    pub fn stat_name(&self, name: &CStr, flag: StatFlag) -> ZResult<Stat> {
        let mut stat = unsafe { zeroed() };
        let res = unsafe {
            zip_stat_init(&mut stat);
            zip_stat(self.inner.as_ptr(), name.as_ptr(), flag.bits(), &mut stat)
        };
        if res == -1 {
            zip_err!(self.inner.as_ptr())
        } else {
            Ok(Stat::from(stat))
        }
    }

    pub fn get_name(&self, index: u64, flag: ZipFlag) -> &[u8] {
        unsafe { CStr::from_ptr(zip_get_name(self.inner.as_ptr(), index, flag.bits())).to_bytes() }
    }

    pub fn num_entries(&self, flag: ZipFlag) -> u64 {
        // Safety: because of self.inner is `NonNull` pointer
        unsafe { zip_get_num_entries(self.inner.as_ptr(), flag.bits()) as _ }
    }

    pub fn source_file(&self, name: &CStr, start: u64, len: i64) -> ZResult<Source<&Self>> {
        let ptr = unsafe { zip_source_file(self.inner.as_ptr(), name.as_ptr(), start, len) };
        if ptr.is_null() {
            zip_err!(self.inner.as_ptr())
        } else {
            Ok(Source::from_ptr(ptr))
        }
    }

    pub fn dir_add(&self, name: &CStr, flag: ZipFlag) -> ZResult<u64> {
        let res = unsafe {
            zip_dir_add(self.inner.as_ptr(), name.as_ptr(), flag.bits())
        };
        if res == -1 {
            zip_err!(self.inner.as_ptr())
        } else {
            Ok( res as _ )
        }
    }

    pub fn stat_index(&self, index: u64, flag: StatFlag) -> ZResult<Stat> {
        let mut stat = unsafe { zeroed() };
        let res = unsafe {
            zip_stat_init(&mut stat);
            zip_stat_index(self.inner.as_ptr(), index, flag.bits(), &mut stat)
        };
        if res == -1 {
            zip_err!(self.inner.as_ptr())
        } else {
            Ok(Stat::from(stat))
        }
    }

    pub fn iter_file(&self, flag: u32) -> IterFile {
        IterFile {
            ptr: self.inner.as_ptr(),
            pos: 0,
            flag,
            _phantom_data: PhantomData,
        }
    }

    pub fn iter_stat(&self, flag: StatFlag) -> IterStat {
        IterStat {
            ptr: self.inner.as_ptr(),
            pos: 0,
            flag,
            _phantom_data: PhantomData,
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

#[derive(Debug)]
pub struct IterFile<'a> {
    ptr: *mut zip_t,
    pos: u64,
    flag: u32,
    _phantom_data: PhantomData<&'a Archive>,
}

impl<'a> Iterator for IterFile<'a> {
    type Item = File<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let res = unsafe { zip_fopen_index(self.ptr, self.pos, self.flag) };
        if res.is_null() {
            None
        } else {
            self.pos += 1;
            Some(File::from_ptr(res))
        }
    }
}

#[derive(Debug)]
pub struct IterStat<'a> {
    ptr: *mut zip_t,
    pos: u64,
    flag: StatFlag,
    _phantom_data: PhantomData<&'a Archive>,
}

impl<'a> Iterator for IterStat<'a> {
    type Item = Stat;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let mut stat = zeroed();
            zip_stat_init(&mut stat);
            let errno = zip_stat_index(self.ptr, self.pos, self.flag.bits(), &mut stat);
            if errno == -1 {
                None
            } else {
                self.pos += 1;
                Some(Stat::from(stat))
            }
        }
    }
}

bitflags! {
    pub struct OpenFlag: c_int {
        const CREATE = 1;
        const EXCL = 2;
        const CHECKCONS = 4;
        const TRUNCATE = 8;
        const RDONLY = 16;
    }
}

bitflags! {
    pub struct StatFlag: u32 {
        const NAME= 1;
        const INDEX= 2;
        const SIZE= 4;
        const COMP_SIZE= 8;
        const MTIME= 16;
        const CRC= 32;
        const COMP_METHOD=64;
        const ENCRYPTION_METHOD = 128;
        const FLAGS= 256;
    }
}

bitflags! {
    pub struct ZipFlag: u32 {
        const FL_NOCASE= 1;
        const FL_NODIR= 2;
        const FL_COMPRESSED= 4;
        const FL_UNCHANGED= 8;
        const FL_RECOMPRESS= 16;
        const FL_ENCRYPTED= 32;
        const FL_ENC_GUESS= 0;
        const FL_ENC_RAW= 64;
        const FL_ENC_STRICT= 128;
        const FL_LOCAL= 256;
        const FL_CENTRAL= 512;
        const FL_ENC_UTF_8= 2048;
        const FL_ENC_CP437= 4096;
        const FL_OVERWRITE= 8192;
    }
}

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
        let mut errno = 0;
        let res = zip_open(path.as_ptr(), flag, &mut errno);
        if res.is_null() {
            Err(ZipErrorSys::from(errno).into())
        } else {
            Ok(Archive {
                inner: NonNull::new_unchecked(res),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use super::*;

    #[test]
    fn iter() {
        let name = CString::new("hello.zip").unwrap();
        let archive = Archive::open(&name).unwrap();

        let stats: Vec<_> = archive.iter_stat(StatFlag::all()).collect();

        let files: Vec<_> = archive.iter_file(ZIP_FL_COMPRESSED).collect();

        assert_eq!(stats.len(), 1);
        assert_eq!(files.len(), 1);

        println!("{:?}", stats);
        println!("{:?}", files);
    }

    #[test]
    fn archive() {
        let name = CString::new("Hello").unwrap();
        let archive = OpenOptions::new().open(&name);
        assert!(archive.is_err());

        let name = CString::new("hello.zip").unwrap();
        let archive = OpenOptions::new().open(&name);
        assert!(archive.is_ok());

        let stat = archive.unwrap().stat_index(u64::MAX, StatFlag::all());
        assert!(stat.is_err());
    }
}
