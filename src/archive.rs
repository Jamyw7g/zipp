use std::ffi::CStr;
use std::marker::PhantomData;
use std::mem::{zeroed, ManuallyDrop};
use std::os::raw::c_int;
use std::ptr::{null, null_mut, NonNull};

use zipp_sys::*;

use crate::file::File;
use crate::source::Source;
use crate::consts::*;
use crate::*;

#[derive(Debug)]
pub struct Archive {
    inner: NonNull<zip_t>,
}

impl Archive {
    pub fn open(path: &CStr) -> ZResult<Self> {
        _open(path, ZIP_RDONLY)
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
        _open(path, ZIP_CREATE)
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

    pub fn unchanged(&self, index: u64) -> ZResult<()> {
        let errno = unsafe { zip_unchange(self.inner.as_ptr(), index) };
        if errno == -1 {
            zip_err!(self.inner.as_ptr())
        } else {
            Ok(())
        }
    }

    pub fn unchanged_archive(&self) -> ZResult<()> {
        let errno = unsafe { zip_unchange_archive(self.inner.as_ptr()) };
        if errno == -1 {
            zip_err!(self.inner.as_ptr())
        } else {
            Ok(())
        }
    }

    pub fn unchanged_all(&self) -> ZResult<()> {
        let errno = unsafe { zip_unchange_all(self.inner.as_ptr()) };
        if errno == -1 {
            zip_err!(self.inner.as_ptr())
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

    pub fn set_file_compression(&self, index: u64, comp: i32, comp_flag: u32) -> ZResult<()> {
        let errno =
            unsafe { zip_set_file_compression(self.inner.as_ptr(), index, comp, comp_flag) };
        if errno == -1 {
            zip_err!(self.inner.as_ptr())
        } else {
            Ok(())
        }
    }

    pub fn file_set_encryption(&self, index: u64, method: u16, passwd: &CStr) -> ZResult<()> {
        let errno = unsafe {
            zip_file_set_encryption(self.inner.as_ptr(), index, method, passwd.as_ptr())
        };
        if errno == -1 {
            zip_err!(self.inner.as_ptr())
        } else {
            Ok(())
        }
    }

    pub fn set_comment(&self, comment: &CStr) -> ZResult<()> {
        let len = comment.to_bytes().len();
        let len = if len > u16::MAX as usize {
            u16::MAX
        } else {
            len as u16
        };
        let errno = unsafe { zip_set_archive_comment(self.inner.as_ptr(), comment.as_ptr(), len) };
        if errno == -1 {
            zip_err!(self.inner.as_ptr())
        } else {
            Ok(())
        }
    }

    pub fn get_comment(&self, flag: u32) -> ZResult<&CStr> {
        let res = unsafe { zip_get_archive_comment(self.inner.as_ptr(), null_mut(), flag) };
        if res.is_null() {
            zip_err!(self.inner.as_ptr())
        } else {
            unsafe { Ok(CStr::from_ptr(res)) }
        }
    }

    pub fn open_file(&self, name: &CStr, flag: u32, passwd: Option<&CStr>) -> ZResult<File> {
        let passwd = if passwd.is_none() {
            null()
        } else {
            passwd.unwrap().as_ptr()
        };
        let res =
            unsafe { zip_fopen_encrypted(self.inner.as_ptr(), name.as_ptr(), flag, passwd) };
        if res.is_null() {
            zip_err!(self.inner.as_ptr())
        } else {
            Ok(File::from_ptr(res))
        }
    }

    pub fn open_file_index(
        &self,
        index: u64,
        flag: u32,
        passwd: Option<&CStr>,
    ) -> ZResult<File> {
        let passwd = if passwd.is_none() {
            null()
        } else {
            passwd.unwrap().as_ptr()
        };
        let res =
            unsafe { zip_fopen_index_encrypted(self.inner.as_ptr(), index, flag, passwd) };
        if res.is_null() {
            zip_err!(self.inner.as_ptr())
        } else {
            Ok(File::from_ptr(res))
        }
    }

    pub fn file_rename(&self, index: u64, name: &CStr, flag: u32) -> ZResult<()> {
        let errno =
            unsafe { zip_file_rename(self.inner.as_ptr(), index, name.as_ptr(), flag) };
        if errno == -1 {
            zip_err!(self.inner.as_ptr())
        } else {
            Ok(())
        }
    }

    pub fn file_add<T>(&self, name: &CStr, source: Source<T>, flag: u32) -> ZResult<u64> {
        let res = unsafe {
            zip_file_add(
                self.inner.as_ptr(),
                name.as_ptr(),
                source.into_raw(),
                flag,
            )
        };
        if res == -1 {
            zip_err!(self.inner.as_ptr())
        } else {
            Ok(res as _)
        }
    }

    pub fn file_replace<T>(&self, index: u64, source: Source<T>, flag: u32) -> ZResult<()> {
        let res =
            unsafe { zip_file_replace(self.inner.as_ptr(), index, source.into_raw(), flag) };
        if res == -1 {
            zip_err!(self.inner.as_ptr())
        } else {
            Ok(())
        }
    }

    pub fn file_delete(&self, index: u64) -> ZResult<()> {
        let errno = unsafe { zip_delete(self.inner.as_ptr(), index) };
        if errno == -1 {
            zip_err!(self.inner.as_ptr())
        } else {
            Ok(())
        }
    }

    pub fn file_get_comment(&self, index: u64, flag: u32) -> ZResult<&CStr> {
        let res =
            unsafe { zip_file_get_comment(self.inner.as_ptr(), index, null_mut(), flag) };
        if res.is_null() {
            zip_err!(self.inner.as_ptr())
        } else {
            unsafe { Ok(CStr::from_ptr(res)) }
        }
    }

    pub fn stat_name(&self, name: &CStr, flag: u32) -> ZResult<Stat> {
        let mut stat = unsafe { zeroed() };
        let res = unsafe {
            zip_stat_init(&mut stat);
            zip_stat(self.inner.as_ptr(), name.as_ptr(), flag, &mut stat)
        };
        if res == -1 {
            zip_err!(self.inner.as_ptr())
        } else {
            Ok(stat)
        }
    }

    pub fn get_name(&self, index: u64, flag: u32) -> &CStr {
        unsafe { CStr::from_ptr(zip_get_name(self.inner.as_ptr(), index, flag)) }
    }

    pub fn get_index(&self, name: &CStr, flag: u32) -> ZResult<u64> {
        let res = unsafe { zip_name_locate(self.inner.as_ptr(), name.as_ptr(), flag) };
        if res == -1 {
            zip_err!(self.inner.as_ptr())
        } else {
            Ok( res as _ )
        }
    }

    pub fn num_entries(&self, flag: u32) -> u64 {
        // Safety: because of self.inner is `NonNull` pointer
        unsafe { zip_get_num_entries(self.inner.as_ptr(), flag) as _ }
    }

    /*
    pub fn source_file(&self, name: &CStr, start: u64, len: i64) -> ZResult<Source<&Self>> {
        let ptr = unsafe { zip_source_file(self.inner.as_ptr(), name.as_ptr(), start, len) };
        if ptr.is_null() {
            zip_err!(self.inner.as_ptr())
        } else {
            Ok(Source::from_ptr(ptr))
        }
    }
    */

    pub fn dir_add(&self, name: &CStr, flag: u32) -> ZResult<u64> {
        let res = unsafe { zip_dir_add(self.inner.as_ptr(), name.as_ptr(), flag) };
        if res == -1 {
            zip_err!(self.inner.as_ptr())
        } else {
            Ok(res as _)
        }
    }

    pub fn stat_index(&self, index: u64, flag: u32) -> ZResult<Stat> {
        let mut stat = unsafe { zeroed() };
        let res = unsafe {
            zip_stat_init(&mut stat);
            zip_stat_index(self.inner.as_ptr(), index, flag, &mut stat)
        };
        if res == -1 {
            zip_err!(self.inner.as_ptr())
        } else {
            Ok(stat)
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

    pub fn iter_stat(&self, flag: u32) -> IterStat {
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
    flag: u32,
    _phantom_data: PhantomData<&'a Archive>,
}

impl<'a> Iterator for IterStat<'a> {
    type Item = Stat;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let mut stat = zeroed();
            zip_stat_init(&mut stat);
            let errno = zip_stat_index(self.ptr, self.pos, self.flag, &mut stat);
            if errno == -1 {
                None
            } else {
                self.pos += 1;
                Some(stat)
            }
        }
    }
}

#[derive(Debug)]
pub struct OpenOptions {
    inner: c_int,
}

impl OpenOptions {
    pub fn new() -> Self {
        Self {
            inner: 0,
        }
    }

    pub fn rdonly(&mut self, flag: bool) -> &mut Self {
        if flag {
            self.inner |= ZIP_RDONLY;
        }
        self
    }

    pub fn create(&mut self, flag: bool) -> &mut Self {
        if flag {
            self.inner |= ZIP_CREATE;
        }
        self
    }

    pub fn excl(&mut self, flag: bool) -> &mut Self {
        if flag {
            self.inner |= ZIP_EXCL;
        }
        self
    }

    pub fn truncate(&mut self, flag: bool) -> &mut Self {
        if flag {
            self.inner |= ZIP_TRUNCATE;
        }
        self
    }

    pub fn open(&self, path: &CStr) -> ZResult<Archive> {
        _open(path, self.inner)
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
        let name = CString::new("tests/test.zip").unwrap();
        let archive = Archive::open(&name).unwrap();

        let stats: Vec<_> = archive.iter_stat(ZIP_STAT_CRC).collect();

        let files: Vec<_> = archive.iter_file(ZIP_FL_COMPRESSED).collect();

        println!("{:?}", stats);
        println!("{:?}", files);
    }

    #[test]
    fn archive() {
        let name = CString::new("Hello").unwrap();
        let archive = OpenOptions::new().open(&name);
        assert!(archive.is_err());

        let name = CString::new("tests/test.zip").unwrap();
        let archive = OpenOptions::new().open(&name);
        assert!(archive.is_ok());

        let stat = archive.unwrap().stat_index(u64::MAX, ZIP_STAT_SIZE);
        assert!(stat.is_err());
    }
}