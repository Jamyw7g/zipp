use std::mem::zeroed;
use crate::Error;
use zipp_sys::{zip_get_error, zip_file_get_error};

#[macro_export]
macro_rules! file_err {
    ($fp_ptr:expr) => {
        unsafe {
            let error = zip_file_get_error($fp_ptr);
            Err(Error::from((&*error).zip_err))
        }
    };
}

#[macro_export]
macro_rules! zip_err {
    ($zip_ptr:expr) => {
        unsafe {
            let error = zip_get_error($zip_ptr);
            Err(Error::from((&*error).zip_err))
        }
    };
}
