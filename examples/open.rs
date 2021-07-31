use std::ffi::CString;

use zipp::{Archive, StatFlag};

fn main() {
    let zipname = CString::new("test.zip").unwrap();
    let archive = Archive::open(&zipname).unwrap();

    archive
        .iter_stat(StatFlag::all())
        .for_each(|item| println!("{:#?}", item));
}
