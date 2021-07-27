use std::os::raw::c_int;

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Multi-disk zip archives not supported.")]
    Multidisk,
    #[error("Renaming temporary file failed.")]
    Rename,
    #[error("Closing zip archive failed.")]
    Close,
    #[error("Seek error.")]
    Seek,
    #[error("Read error.")]
    Read,
    #[error("Write error.")]
    Write,
    #[error("CRC error.")]
    Crc,
    #[error("Containing zip archive was closed.")]
    Zipclosed,
    #[error("No such file.")]
    Noent,
    #[error("File already exists.")]
    Exists,
    #[error("Can't open file.")]
    Open,
    #[error("Failure to create temporary file.")]
    Tmpopen,
    #[error("Zlib error.")]
    Zlib,
    #[error("Malloc failure.")]
    Memory,
    #[error("Entry has been changed.")]
    Changed,
    #[error("Compression method not supported.")]
    Compnotsupp,
    #[error("Premature end of file.")]
    Eof,
    #[error("Invalid argument.")]
    Inval,
    #[error("Not a zip archive.")]
    Nozip,
    #[error("Internal error.")]
    Internal,
    #[error("Zip archive inconsistent.")]
    Incons,
    #[error("Can't remove file.")]
    Remove,
    #[error("Entry has been deleted.")]
    Deleted,
    #[error("Encryption method not supported.")]
    Encrnotsupp,
    #[error("Read-only archive.")]
    Rdonly,
    #[error("No password provided.")]
    Nopasswd,
    #[error("Wrong password provided.")]
    Wrongpasswd,
    #[error("Operation not supported.")]
    Opnotsupp,
    #[error("Resource still in use.")]
    Inuse,
    #[error("Tell error.")]
    Tell,
    #[error("Compressed data invalid.")]
    CompressedData,
    #[error("Cancelled")]
    Cancelled,
}


impl From<c_int> for Error {
    fn from(zip_err: c_int) -> Self {
        match zip_err {
            1 => Self::Multidisk,
            2 => Self::Rename,
            3 => Self::Close,
            4 => Self::Seek,
            5 => Self::Read,
            6 => Self::Write,
            7 => Self::Crc,
            8 => Self::Zipclosed,
            9 => Self::Noent,
            10 => Self::Exists,
            11 => Self::Open,
            12 => Self::Tmpopen,
            13 => Self::Zlib,
            14 => Self::Memory,
            15 => Self::Changed,
            16 => Self::Compnotsupp,
            17 => Self::Eof,
            18 => Self::Inval,
            19 => Self::Nozip,
            20 => Self::Internal,
            21 => Self::Incons,
            22 => Self::Remove,
            23 => Self::Deleted,
            24 => Self::Encrnotsupp,
            25 => Self::Rdonly,
            26 => Self::Nopasswd,
            27 => Self::Wrongpasswd,
            28 => Self::Opnotsupp,
            29 => Self::Inuse,
            30 => Self::Tell,
            31 => Self::CompressedData,
            32 => Self::Cancelled,
            _ => unreachable!(),
        }
    }
}
