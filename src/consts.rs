//! const variable flags

/// `OpenFlag` Create the archive if it does not exist.
pub const ZIP_CREATE: i32 = 1;
/// `OpenFlag` Error if archive already exists.
pub const ZIP_EXCL: i32 = 2;
/// `OpenFlag` Perform additional stricter consistency checks on the archive,
/// and error if they fail.
pub const ZIP_CHECKCONS: i32 = 4;
/// `OpenFlag` If archive exists, ignore its current contents. In other words,
/// handle it the same way as an empty archive.
pub const ZIP_TRUNCATE: i32 = 8;
/// `OpenFlag` Open archive in read-only mode.
pub const ZIP_RDONLY: i32 = 16;

/// `Error` No error.
pub const ZIP_ER_OK: i32 = 0;
/// `Error` Multi-disk zip archives not supported.
pub const ZIP_ER_MULTIDISK: i32 = 1;
/// `Error` Renaming temporary file failed.
pub const ZIP_ER_RENAME: i32 = 2;
/// `Error` Closing zip archive failed.
pub const ZIP_ER_CLOSE: i32 = 3;
/// `Error` Seek error.
pub const ZIP_ER_SEEK: i32 = 4;
/// `Error` Read error.
pub const ZIP_ER_READ: i32 = 5;
/// `Error` Write error.
pub const ZIP_ER_WRITE: i32 = 6;
/// `Error` CRC error.
pub const ZIP_ER_CRC: i32 = 7;
/// `Error` Containing zip archive was closed.
pub const ZIP_ER_ZIPCLOSED: i32 = 8;
/// `Error` No such file.
pub const ZIP_ER_NOENT: i32 = 9;
/// `Error` File already exists.
pub const ZIP_ER_EXISTS: i32 = 10;
/// `Error` Can't open file.
pub const ZIP_ER_OPEN: i32 = 11;
/// `Error` Failure to create temporary file.
pub const ZIP_ER_TMPOPEN: i32 = 12;
/// `Error` Zlib error.
pub const ZIP_ER_ZLIB: i32 = 13;
/// `Error` Malloc failure.
pub const ZIP_ER_MEMORY: i32 = 14;
/// `Error` Entry has been changed.
pub const ZIP_ER_CHANGED: i32 = 15;
/// `Error` Compression method not supported.
pub const ZIP_ER_COMPNOTSUPP: i32 = 16;
/// `Error` Premature end of file.
pub const ZIP_ER_EOF: i32 = 17;
/// `Error` Invalid argument.
pub const ZIP_ER_INVAL: i32 = 18;
/// `Error` Not a zip archive.
pub const ZIP_ER_NOZIP: i32 = 19;
/// `Error` Internal error.
pub const ZIP_ER_INTERNAL: i32 = 20;
/// `Error` Zip archive inconsistent.
pub const ZIP_ER_INCONS: i32 = 21;
/// `Error` Can't remove file.
pub const ZIP_ER_REMOVE: i32 = 22;
/// `Error` Entry has been deleted.
pub const ZIP_ER_DELETED: i32 = 23;
/// `Error` Encryption method not supported.
pub const ZIP_ER_ENCRNOTSUPP: i32 = 24;
/// `Error` Read-only archive.
pub const ZIP_ER_RDONLY: i32 = 25;
/// `Error` No password provided.
pub const ZIP_ER_NOPASSWD: i32 = 26;
/// `Error` Wrong password provided.
pub const ZIP_ER_WRONGPASSWD: i32 = 27;
/// `Error` Operation not supported.
pub const ZIP_ER_OPNOTSUPP: i32 = 28;
/// `Error` Resource still in use.
pub const ZIP_ER_INUSE: i32 = 29;
/// `Error` Tell error.
pub const ZIP_ER_TELL: i32 = 30;
/// `Error` Compressed data invalid.
pub const ZIP_ER_COMPRESSED_DATA: i32 = 31;
/// `Error` Operation cancelled.
pub const ZIP_ER_CANCELLED: i32 = 32;

/// sys_err unused
pub const ZIP_ET_NONE: i32 = 0;
/// sys_err is errno
pub const ZIP_ET_SYS: i32 = 1;
/// sys_err is zlib error code
pub const ZIP_ET_ZLIB: i32 = 2;
/// sys_err is libzip error code
pub const ZIP_ET_LIBZIP: i32 = 3;

/// `Stat` get stat name
pub const ZIP_STAT_NAME: u32 = 1;
/// `Stat` get stat index
pub const ZIP_STAT_INDEX: u32 = 2;
/// `Stat` get stat size
pub const ZIP_STAT_SIZE: u32 = 4;
/// `Stat` get stat comp_size
pub const ZIP_STAT_COMP_SIZE: u32 = 8;
/// `Stat` get stat mtime
pub const ZIP_STAT_MTIME: u32 = 16;
/// `Stat` get stat crc
pub const ZIP_STAT_CRC: u32 = 32;
/// `Stat` get stat comp_method
pub const ZIP_STAT_COMP_METHOD: u32 = 64;
/// `Stat` get stat encryption_method
pub const ZIP_STAT_ENCRYPTION_METHOD: u32 = 128;
/// `Stat` get stat flags
pub const ZIP_STAT_FLAGS: u32 = 256;
/// `Stat` get stat all field
pub const ZIP_STAT_ALL: u32 = 511;

/// ignore case on name lookup
pub const ZIP_FL_NOCASE: u32 = 1;
/// ignore directory component
pub const ZIP_FL_NODIR: u32 = 2;
/// read compressed data
pub const ZIP_FL_COMPRESSED: u32 = 4;
/// use original data, ignoring changes
pub const ZIP_FL_UNCHANGED: u32 = 8;
/// force recompression of data
pub const ZIP_FL_RECOMPRESS: u32 = 16;
/// read encrypted data (implies ZIP_FL_COMPRESSED)
pub const ZIP_FL_ENCRYPTED: u32 = 32;
/// guess string encoding (is default)
pub const ZIP_FL_ENC_GUESS: u32 = 0;
/// get unmodified string
pub const ZIP_FL_ENC_RAW: u32 = 64;
/// follow specification strictly
pub const ZIP_FL_ENC_STRICT: u32 = 128;
/// in local header
pub const ZIP_FL_LOCAL: u32 = 256;
/// in central directory
pub const ZIP_FL_CENTRAL: u32 = 512;
/// string is UTF-8 encoded
pub const ZIP_FL_ENC_UTF_8: u32 = 2048;
/// string is CP437 encoded
pub const ZIP_FL_ENC_CP437: u32 = 4096;
/// if file with name exists, overwrite (replace) it
pub const ZIP_FL_OVERWRITE: u32 = 8192;

pub const ZIP_AFL_RDONLY: u32 = 2;
pub const ZIP_EXTRA_FIELD_ALL: u32 = 65535;
pub const ZIP_EXTRA_FIELD_NEW: u32 = 65535;

/// `Compression` better of deflate or store
pub const ZIP_CM_DEFAULT: i32 = -1;
/// `Compression` stored (uncompressed)
pub const ZIP_CM_STORE: i32 = 0;
/// `Compression` shrunk
pub const ZIP_CM_SHRINK: i32 = 1;
/// `Compression` reduced with factor 1
pub const ZIP_CM_REDUCE_1: i32 = 2;
/// `Compression` reduced with factor 2
pub const ZIP_CM_REDUCE_2: i32 = 3;
/// `Compression` reduced with factor 3
pub const ZIP_CM_REDUCE_3: i32 = 4;
/// `Compression` reduced with factor 4
pub const ZIP_CM_REDUCE_4: i32 = 5;
/// `Compression` imploded
pub const ZIP_CM_IMPLODE: i32 = 6;

/// `Compression` deflated
pub const ZIP_CM_DEFLATE: i32 = 8;
/// `Compression` deflate64
pub const ZIP_CM_DEFLATE64: i32 = 9;
/// `Compression` PKWARE imploding
pub const ZIP_CM_PKWARE_IMPLODE: i32 = 10;
/// `Compression` compressed using BZIP2 algorithm
pub const ZIP_CM_BZIP2: i32 = 12;
/// `Compression` LZMA (EFS)
pub const ZIP_CM_LZMA: i32 = 14;
/// `Compression` compressed using IBM TERSE
pub const ZIP_CM_TERSE: i32 = 18;
/// `Compression` IBM LZ77 z Architecture
pub const ZIP_CM_LZ77: i32 = 19;
/// `Compression`
pub const ZIP_CM_LZMA2: i32 = 33;
/// `Compression` Zstandard compressed data
pub const ZIP_CM_ZSTD: i32 = 93;
/// `Compression` XZ compressed data
pub const ZIP_CM_XZ: i32 = 95;
/// `Compression` Compressed Jpeg data
pub const ZIP_CM_JPEG: i32 = 96;
/// `Compression` WavPack compressed data
pub const ZIP_CM_WAVPACK: i32 = 97;
/// `Compression` PPMd version I, Rev 1
pub const ZIP_CM_PPMD: i32 = 98;

/// `Encryption` not encrypted
pub const ZIP_EM_NONE: u16 = 0;
/// `Encryption` traditional PKWARE encryption
pub const ZIP_EM_TRAD_PKWARE: u16 = 1;
/// `Encryption` AES-128
pub const ZIP_EM_AES_128: u16 = 257;
/// `Encryption` AES-192
pub const ZIP_EM_AES_192: u16 = 258;
/// `Encryption` AES-256
pub const ZIP_EM_AES_256: u16 = 259;
/// `Encryption` unknown algorithm
pub const ZIP_EM_UNKNOWN: u16 = 65535;
