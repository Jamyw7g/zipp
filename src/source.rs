use std::{
    convert::TryFrom,
    ffi::CStr,
    io::{self, ErrorKind, Read, Seek, SeekFrom, Write},
    marker::PhantomData,
    mem::ManuallyDrop,
};

use zipp_sys::*;

use crate::*;

/// Reference type, increase reference count when clone, descrease when drop
#[derive(Debug)]
pub struct Source<T> {
    inner: *mut zip_source_t,
    _phantom_data: PhantomData<T>,
}

impl<T> Source<T> {
    pub fn from_ptr(ptr: *mut zip_source_t) -> Self {
        Self {
            inner: ptr,
            _phantom_data: PhantomData,
        }
    }

    /// concume Self into raw pointer
    pub fn into_raw(self) -> *mut zip_source_t {
        let me = ManuallyDrop::new(self);
        me.inner
    }

    pub fn as_ptr(&self) -> *mut zip_source_t {
        self.inner
    }

    /// open zip_source for reading
    pub fn open_read(&self) -> ZResult<ReadHalf<T>> {
        unsafe {
            let errno = zip_source_open(self.inner);
            if errno == -1 {
                let error = zip_source_error(self.inner);
                Err(ZipErrorSys::from(&mut *error).into())
            } else {
                Ok(ReadHalf {
                    inner: self.inner,
                    _phantom_data: PhantomData,
                })
            }
        }
    }

    /// prepare zip source for writing
    ///
    /// offset: preserves the first offset bytes of the original file.
    /// This is done efficiently, and writes to source won't overwrite the original data until drop.
    pub fn open_write(&self, offset: Option<u64>) -> ZResult<WriteHalf<T>> {
        let offset = offset.unwrap_or(0);
        unsafe {
            let errno = zip_source_begin_write_cloning(self.inner, offset);
            if errno == -1 {
                let error = zip_source_error(self.inner);
                Err(ZipErrorSys::from(&mut *error).into())
            } else {
                Ok(WriteHalf {
                    inner: self.inner,
                    _phantom_data: PhantomData,
                })
            }
        }
    }
}

impl<T> Clone for Source<T> {
    fn clone(&self) -> Self {
        unsafe {
            zip_source_keep(self.inner);
        }
        Self {
            inner: self.inner,
            _phantom_data: PhantomData,
        }
    }
}

impl<T> Drop for Source<T> {
    fn drop(&mut self) {
        unsafe {
            zip_source_free(self.inner);
        }
    }
}

/// Type marker, mark Source was open from a file.
#[doc(hidden)]
#[derive(Debug)]
pub struct File;

impl TryFrom<&CStr> for Source<File> {
    type Error = Error;

    fn try_from(name: &CStr) -> Result<Self, Self::Error> {
        let mut error = ZipErrorSys::default();
        let inner = unsafe { zip_source_file_create(name.as_ptr(), 0, 0, &mut *error) };
        if inner.is_null() {
            Err(error.into())
        } else {
            Ok(Self {
                inner,
                _phantom_data: PhantomData,
            })
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for Source<&'a [u8]> {
    type Error = Error;

    fn try_from(buf: &'a [u8]) -> Result<Self, Self::Error> {
        let mut error = ZipErrorSys::default();
        let inner = unsafe {
            zip_source_buffer_create(buf.as_ptr().cast(), buf.len() as _, 0, &mut *error)
        };
        if inner.is_null() {
            Err(error.into())
        } else {
            Ok(Self {
                inner,
                _phantom_data: PhantomData,
            })
        }
    }
}

/// A reference of Source which can be read
#[derive(Debug)]
pub struct ReadHalf<'a, T: 'a> {
    inner: *mut zip_source_t,
    _phantom_data: PhantomData<&'a Source<T>>,
}

impl<'a, T> ReadHalf<'a, T> {
    // explicit close reading
    pub fn close(self) -> ZResult<()> {
        let me = ManuallyDrop::new(self);
        unsafe {
            let errno = zip_source_close(me.inner);
            if errno == -1 {
                let error = zip_source_error(me.inner);
                Err(ZipErrorSys::from(&mut *error).into())
            } else {
                Ok(())
            }
        }
    }
}

impl<'a, T> Read for ReadHalf<'a, T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        unsafe {
            let res = zip_source_read(self.inner, buf.as_mut_ptr().cast(), buf.len() as _);
            if res == -1 {
                let error = zip_source_error(self.inner);
                let error: Error = ZipErrorSys::from(&mut *error).into();
                Err(io::Error::new(ErrorKind::Other, error))
            } else {
                Ok(res as _)
            }
        }
    }
}

impl<'a, T> Seek for ReadHalf<'a, T> {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        unsafe {
            let errno = match pos {
                SeekFrom::Start(p) => zip_source_seek(self.inner, p as _, SEEK_SET),
                SeekFrom::Current(p) => zip_source_seek(self.inner, p as _, SEEK_CUR),
                SeekFrom::End(p) => zip_source_seek(self.inner, p as _, SEEK_END),
            };
            if errno == -1 {
                let error = zip_source_error(self.inner);
                let error: Error = ZipErrorSys::from(&mut *error).into();
                Err(io::Error::new(ErrorKind::Other, error))
            } else {
                Ok(zip_source_tell(self.inner) as _)
            }
        }
    }
}

impl<'a, T> Drop for ReadHalf<'a, T> {
    fn drop(&mut self) {
        unsafe {
            let r = zip_source_close(self.inner);
            debug_assert_eq!(r, 0);
        }
    }
}

/// A reference of Source which can be write
#[derive(Debug)]
pub struct WriteHalf<'a, T: 'a> {
    inner: *mut zip_source_t,
    _phantom_data: PhantomData<&'a Source<T>>,
}

impl<'a, T> WriteHalf<'a, T> {
    // explicit commit write
    pub fn commit(self) -> ZResult<()> {
        let me = ManuallyDrop::new(self);
        unsafe {
            let errno = zip_source_commit_write(me.inner);
            if errno == -1 {
                let error = zip_source_error(me.inner);
                Err(ZipErrorSys::from(&mut *error).into())
            } else {
                Ok(())
            }
        }
    }
}

impl<'a, T> Write for WriteHalf<'a, T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unsafe {
            let res = zip_source_write(self.inner, buf.as_ptr().cast(), buf.len() as _);
            if res == -1 {
                let error = zip_source_error(self.inner);
                let error: Error = ZipErrorSys::from(&mut *error).into();
                Err(io::Error::new(ErrorKind::Other, error))
            } else {
                Ok(res as _)
            }
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<'a, T> Seek for WriteHalf<'a, T> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        unsafe {
            let errno = match pos {
                SeekFrom::Start(p) => zip_source_seek_write(self.inner, p as _, SEEK_SET),
                SeekFrom::Current(p) => zip_source_seek_write(self.inner, p as _, SEEK_CUR),
                SeekFrom::End(p) => zip_source_seek_write(self.inner, p as _, SEEK_END),
            };
            if errno == -1 {
                let error = zip_source_error(self.inner);
                let error: Error = ZipErrorSys::from(&mut *error).into();
                Err(io::Error::new(ErrorKind::Other, error))
            } else {
                Ok(zip_source_tell_write(self.inner) as _)
            }
        }
    }
}

impl<'a, T> Drop for WriteHalf<'a, T> {
    fn drop(&mut self) {
        unsafe {
            let r = zip_source_commit_write(self.inner);
            debug_assert_eq!(r, 0);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::consts::{ZIP_FL_ENC_UTF_8, ZIP_FL_OVERWRITE};

    use super::*;
    use std::ffi::CString;

    #[test]
    fn read() {
        let filename = CString::new("Cargo.toml").unwrap();
        let source = Source::try_from(filename.as_c_str()).unwrap();
        let mut reader = source.open_read().unwrap();

        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).unwrap();
        drop(reader);

        let zipname = CString::new("tests/test.zip").unwrap();
        let archive = OpenOptions::new().create(true).open(&zipname).unwrap();
        archive
            .file_add(&filename, source, ZIP_FL_ENC_UTF_8 | ZIP_FL_OVERWRITE)
            .unwrap();

        println!("{:?}", buffer);
    }
}
