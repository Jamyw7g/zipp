use std::{convert::TryFrom, ffi::CString};

use zipp::{source::Source, Archive, ZipFlag};

fn main() {
    let zipname = CString::new("test.zip").unwrap();

    let file1 = CString::new("Cargo.toml").unwrap();
    let file2 = CString::new("README.md").unwrap();

    let archive = Archive::create(&zipname).unwrap();

    let s1 = Source::try_from(file1.as_c_str()).unwrap();
    let s2 = Source::try_from(file2.as_c_str()).unwrap();

    archive.file_add(&file1, s1, ZipFlag::FL_OVERWRITE).unwrap();
    archive.file_add(&file2, s2, ZipFlag::FL_OVERWRITE).unwrap();
}
