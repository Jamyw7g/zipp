mod archive;
mod error;
mod file;
mod macros;
mod source;
mod stat;

use std::ffi::CStr;

pub use archive::*;
use error::ZipErrorSys;
pub use error::{Error, ZResult};
use zipp_sys::zip_libzip_version;

#[macro_use]
extern crate bitflags;


pub fn version() -> &'static str {
    let ver = unsafe { CStr::from_ptr(zip_libzip_version()) };
    
    ver.to_str().unwrap()
}


#[cfg(test)]
mod tests {

    #[test]
    fn version() {
        println!("{}", super::version());
    }
}
