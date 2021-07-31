use std::ffi::CString;

use zipp::Archive;
use zipp::consts::*;

fn main() {
    let zipname = CString::new("test.zip").unwrap();
    let archive = Archive::open(&zipname).unwrap();

    archive
        .iter_stat(ZIP_STAT_ALL)
        .for_each(|item| println!("{:#?}", item));
}
