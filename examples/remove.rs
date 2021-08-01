use std::ffi::CString;

use zipp::{consts::ZIP_FL_NOCASE, OpenOptions};

fn main() {
    let zipname = CString::new("test.zip").unwrap();
    let archive = OpenOptions::new().open(&zipname).unwrap();

    let filename = CString::new("Cargo.lock").unwrap();
    let index = archive.get_index(&filename, ZIP_FL_NOCASE).unwrap();
    archive.file_delete(index).unwrap();
}
