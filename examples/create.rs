use std::{convert::TryFrom, ffi::CString};

use zipp::consts::*;
use zipp::{source::Source, Archive};

fn main() {
    let zipname = CString::new("test.zip").unwrap();

    let file1 = CString::new("Cargo.toml").unwrap();
    let file2 = CString::new("README.md").unwrap();

    let archive = Archive::create(&zipname).unwrap();

    let s1 = Source::try_from(file1.as_c_str()).unwrap();
    let s2 = Source::try_from(file2.as_c_str()).unwrap();

    archive.file_add(&file1, s1, ZIP_FL_OVERWRITE).unwrap();
    archive.file_add(&file2, s2, ZIP_FL_OVERWRITE).unwrap();
}
