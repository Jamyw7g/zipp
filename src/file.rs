use std::{
    io::{self, Read, Seek, SeekFrom},
    marker::PhantomData,
};
use zipp_sys::*;

use crate::*;

/// File contain the file item in archive.
#[derive(Debug)]
pub struct File<'a> {
    inner: *mut zip_file_t,
    _phantom_data: PhantomData<&'a Archive>,
}

impl File<'_> {
    /// Using raw pointer construct File
    pub fn from_ptr(inner: *mut zip_file_t) -> Self {
        Self {
            inner,
            _phantom_data: PhantomData,
        }
    }

    /// Get File error
    fn error(&self) -> ZipErrorSys<&mut zip_error_t> {
        unsafe {
            let error = zip_file_get_error(self.inner);
            (&mut *error).into()
        }
    }
}

/// Close File when it drop
impl Drop for File<'_> {
    fn drop(&mut self) {
        unsafe {
            if !self.inner.is_null() {
                zip_fclose(self.inner);
            }
        }
    }
}

/// Implement Read trait that can use std-io to read
impl Read for File<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let res = unsafe { zip_fread(self.inner, buf.as_mut_ptr().cast(), buf.len() as _) };
        if res == -1 {
            let error: Error = self.error().into();
            Err(io::Error::new(io::ErrorKind::Other, error))
        } else {
            Ok(res as _)
        }
    }
}

/// Implement Seek trait that can use std-io to seek
impl Seek for File<'_> {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        let res = unsafe {
            match pos {
                SeekFrom::Start(p) => zip_fseek(self.inner, p as _, SEEK_SET),
                SeekFrom::Current(p) => zip_fseek(self.inner, p as _, SEEK_CUR),
                SeekFrom::End(p) => zip_fseek(self.inner, p as _, SEEK_END),
            }
        };
        if res == -1 {
            let error: Error = self.error().into();
            Err(io::Error::new(io::ErrorKind::Other, error))
        } else {
            unsafe { Ok(zip_ftell(self.inner) as _) }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use crate::consts::ZIP_FL_UNCHANGED;

    use super::*;

    #[test]
    fn read() {
        let name = CString::new("tests/test.zip").unwrap();
        let archive = Archive::open(&name).unwrap();
        let filename = CString::new("test/test_file1").unwrap();
        let mut file = archive
            .open_file(&filename, ZIP_FL_UNCHANGED, None)
            .unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        println!("data = {:?}", buffer);
    }

    #[test]
    fn seek() {
        let name = CString::new("tests/test.zip").unwrap();
        let _archive = Archive::open(&name).unwrap();
    }
}
