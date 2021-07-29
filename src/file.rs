use std::{
    io::{self, Read, Seek, SeekFrom},
    marker::PhantomData,
};
use zipp_sys::*;

use crate::*;

#[derive(Debug)]
pub struct File<'a> {
    inner: *mut zip_file_t,
    _phantom_data: PhantomData<&'a Archive>,
}

impl File<'_> {
    pub fn from_ptr(inner: *mut zip_file_t) -> Self {
        Self {
            inner,
            _phantom_data: PhantomData,
        }
    }
    fn error(&self) -> ZipErrorSys<&mut zip_error_t> {
        unsafe {
            let error = zip_file_get_error(self.inner);
            (&mut *error).into()
        }
    }
}

impl Drop for File<'_> {
    fn drop(&mut self) {
        unsafe {
            if !self.inner.is_null() {
                zip_fclose(self.inner);
            }
        }
    }
}

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

    use super::*;

    #[test]
    fn read() {
        let name = CString::new("hello.zip").unwrap();
        let archive = Archive::open(&name).unwrap();
        let mut file = archive.iter_file(ZIP_FL_COMPRESSED).next().unwrap();
        let mut buffer = Vec::new();
        let size = file.read_to_end(&mut buffer).unwrap();
        assert_eq!(size, 5);
        assert_eq!(buffer, b"hell\n");
    }

    #[test]
    fn seek() {
        let name = CString::new("hello.zip").unwrap();
        let archive = Archive::open(&name).unwrap();
        let mut file = archive.iter_file(ZIP_FL_COMPRESSED).next().unwrap();
        let n = file.seek(SeekFrom::Start(2)).unwrap();
        assert_eq!(n, 2);
        let n = file.seek(SeekFrom::Current(2)).unwrap();
        assert_eq!(n, 4);
        let n = file.seek(SeekFrom::End(0)).unwrap();
        assert_eq!(n, 5);
    }
}
