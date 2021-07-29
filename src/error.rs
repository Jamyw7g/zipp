use std::borrow::{Borrow, BorrowMut};
use std::ffi::CStr;
use std::fmt::Display;
use std::mem::zeroed;
use std::ops::{Deref, DerefMut};
use std::os::raw::c_int;
use zipp_sys::*;

pub type ZResult<T> = Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    sys: Option<SysError>,
    zip: Option<ZipError>,
    msg: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for Error {}

#[allow(dead_code)]
#[derive(Debug)]
pub enum SysError {
    Sys(c_int),
    Zlib(c_int),
    Unknown(c_int),
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum ZipError {
    Multidisk,
    Rename,
    Close,
    Seek,
    Read,
    Write,
    Crc,
    Zipclosed,
    Noent,
    Exists,
    Open,
    Tmpopen,
    Zlib,
    Memory,
    Changed,
    Compnotsupp,
    Eof,
    Inval,
    Nozip,
    Internal,
    Incons,
    Remove,
    Deleted,
    Encrnotsupp,
    Rdonly,
    Nopasswd,
    Wrongpasswd,
    Opnotsupp,
    Inuse,
    Tell,
    CompressedData,
    Cancelled,
}

#[derive(Debug)]
pub(crate) struct ZipErrorSys<E: Borrow<zip_error_t> + BorrowMut<zip_error_t>> {
    error: E,
    owned: bool,
}

impl<E> Drop for ZipErrorSys<E>
where
    E: Borrow<zip_error_t> + BorrowMut<zip_error_t>,
{
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                zip_error_fini(self.error.borrow_mut());
            }
        }
    }
}

impl From<c_int> for ZipErrorSys<zip_error_t> {
    fn from(code: c_int) -> Self {
        unsafe {
            let mut error = zeroed();
            zip_error_init_with_code(&mut error, code);
            Self { error, owned: true }
        }
    }
}

impl<'a> From<&'a mut zip_error_t> for ZipErrorSys<&'a mut zip_error_t> {
    fn from(error: &'a mut zip_error_t) -> Self {
        Self {
            error,
            owned: false,
        }
    }
}

impl Default for ZipErrorSys<zip_error_t> {
    fn default() -> Self {
        unsafe {
            let mut error = zeroed();
            zip_error_init(&mut error);
            Self { error, owned: true }
        }
    }
}

impl<E> Deref for ZipErrorSys<E>
where
    E: Borrow<zip_error_t> + BorrowMut<zip_error_t>,
{
    type Target = zip_error_t;
    fn deref(&self) -> &Self::Target {
        self.error.borrow()
    }
}

impl<E> DerefMut for ZipErrorSys<E>
where
    E: Borrow<zip_error_t> + BorrowMut<zip_error_t>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.error.borrow_mut()
    }
}

impl<E> ZipErrorSys<E>
where
    E: Borrow<zip_error_t> + BorrowMut<zip_error_t>,
{
    pub fn system(&self) -> Option<SysError> {
        let typ = unsafe { zip_error_system_type(self.error.borrow()) } as u32;
        match typ {
            ZIP_ET_NONE | ZIP_ET_LIBZIP => None,
            _ => {
                let code = unsafe { zip_error_code_system(self.error.borrow()) };
                Some(match typ {
                    ZIP_ET_SYS => SysError::Sys(code),
                    ZIP_ET_ZLIB => SysError::Zlib(code),
                    _ => SysError::Unknown(code),
                })
            }
        }
    }

    pub fn zip(&self) -> Option<ZipError> {
        let code = unsafe { zip_error_code_zip(self.error.borrow()) } as u32;
        Some(match code {
            ZIP_ER_OK => return None,
            ZIP_ER_MULTIDISK => ZipError::Multidisk,
            ZIP_ER_RENAME => ZipError::Rename,
            ZIP_ER_CLOSE => ZipError::Close,
            ZIP_ER_SEEK => ZipError::Seek,
            ZIP_ER_READ => ZipError::Read,
            ZIP_ER_WRITE => ZipError::Write,
            ZIP_ER_CRC => ZipError::Crc,
            ZIP_ER_ZIPCLOSED => ZipError::Zipclosed,
            ZIP_ER_NOENT => ZipError::Noent,
            ZIP_ER_EXISTS => ZipError::Exists,
            ZIP_ER_OPEN => ZipError::Open,
            ZIP_ER_TMPOPEN => ZipError::Tmpopen,
            ZIP_ER_ZLIB => ZipError::Zlib,
            ZIP_ER_MEMORY => ZipError::Memory,
            ZIP_ER_CHANGED => ZipError::Changed,
            ZIP_ER_COMPNOTSUPP => ZipError::Compnotsupp,
            ZIP_ER_EOF => ZipError::Eof,
            ZIP_ER_INVAL => ZipError::Inval,
            ZIP_ER_NOZIP => ZipError::Nozip,
            ZIP_ER_INTERNAL => ZipError::Internal,
            ZIP_ER_INCONS => ZipError::Incons,
            ZIP_ER_REMOVE => ZipError::Remove,
            ZIP_ER_DELETED => ZipError::Deleted,
            ZIP_ER_ENCRNOTSUPP => ZipError::Encrnotsupp,
            ZIP_ER_RDONLY => ZipError::Rdonly,
            ZIP_ER_NOPASSWD => ZipError::Nopasswd,
            ZIP_ER_WRONGPASSWD => ZipError::Wrongpasswd,
            ZIP_ER_OPNOTSUPP => ZipError::Opnotsupp,
            ZIP_ER_INUSE => ZipError::Inuse,
            ZIP_ER_TELL => ZipError::Tell,
            ZIP_ER_COMPRESSED_DATA => ZipError::CompressedData,
            ZIP_ER_CANCELLED => ZipError::Cancelled,
            _ => unreachable!(),
        })
    }

    pub fn message(&mut self) -> String {
        unsafe {
            let ptr = zip_error_strerror(self.error.borrow_mut());
            if ptr.is_null() {
                return String::from("Unknown error message");
            }
            let c_str = CStr::from_ptr(ptr);
            c_str.to_string_lossy().to_string()
        }
    }
}

impl<E> From<ZipErrorSys<E>> for Error
where
    E: Borrow<zip_error_t> + BorrowMut<zip_error_t>,
{
    fn from(mut error: ZipErrorSys<E>) -> Self {
        let sys = error.system();
        let zip = error.zip();
        let msg = error.message();
        Self { sys, zip, msg }
    }
}
