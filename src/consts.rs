
pub const ZIP_CREATE: i32 = 1;
pub const ZIP_EXCL: i32 = 2;
pub const ZIP_CHECKCONS: i32 = 4;
pub const ZIP_TRUNCATE: i32 = 8;
pub const ZIP_RDONLY: i32 = 16;


pub const ZIP_ER_OK: i32 = 0;
pub const ZIP_ER_MULTIDISK: i32 = 1;
pub const ZIP_ER_RENAME: i32 = 2;
pub const ZIP_ER_CLOSE: i32 = 3;
pub const ZIP_ER_SEEK: i32 = 4;
pub const ZIP_ER_READ: i32 = 5;
pub const ZIP_ER_WRITE: i32 = 6;
pub const ZIP_ER_CRC: i32 = 7;
pub const ZIP_ER_ZIPCLOSED: i32 = 8;
pub const ZIP_ER_NOENT: i32 = 9;
pub const ZIP_ER_EXISTS: i32 = 10;
pub const ZIP_ER_OPEN: i32 = 11;
pub const ZIP_ER_TMPOPEN: i32 = 12;
pub const ZIP_ER_ZLIB: i32 = 13;
pub const ZIP_ER_MEMORY: i32 = 14;
pub const ZIP_ER_CHANGED: i32 = 15;
pub const ZIP_ER_COMPNOTSUPP: i32 = 16;
pub const ZIP_ER_EOF: i32 = 17;
pub const ZIP_ER_INVAL: i32 = 18;
pub const ZIP_ER_NOZIP: i32 = 19;
pub const ZIP_ER_INTERNAL: i32 = 20;
pub const ZIP_ER_INCONS: i32 = 21;
pub const ZIP_ER_REMOVE: i32 = 22;
pub const ZIP_ER_DELETED: i32 = 23;
pub const ZIP_ER_ENCRNOTSUPP: i32 = 24;
pub const ZIP_ER_RDONLY: i32 = 25;
pub const ZIP_ER_NOPASSWD: i32 = 26;
pub const ZIP_ER_WRONGPASSWD: i32 = 27;
pub const ZIP_ER_OPNOTSUPP: i32 = 28;
pub const ZIP_ER_INUSE: i32 = 29;
pub const ZIP_ER_TELL: i32 = 30;
pub const ZIP_ER_COMPRESSED_DATA: i32 = 31;
pub const ZIP_ER_CANCELLED: i32 = 32;


pub const ZIP_ET_NONE: i32 = 0;
pub const ZIP_ET_SYS: i32 = 1;
pub const ZIP_ET_ZLIB: i32 = 2;
pub const ZIP_ET_LIBZIP: i32 = 3;


pub const ZIP_STAT_NAME: u32 = 1;
pub const ZIP_STAT_INDEX: u32 = 2;
pub const ZIP_STAT_SIZE: u32 = 4;
pub const ZIP_STAT_COMP_SIZE: u32 = 8;
pub const ZIP_STAT_MTIME: u32 = 16;
pub const ZIP_STAT_CRC: u32 = 32;
pub const ZIP_STAT_COMP_METHOD: u32 = 64;
pub const ZIP_STAT_ENCRYPTION_METHOD: u32 = 128;
pub const ZIP_STAT_FLAGS: u32 = 256;
pub const ZIP_STAT_ALL: u32 = 511;


pub const ZIP_FL_NOCASE: u32 = 1;
pub const ZIP_FL_NODIR: u32 = 2;
pub const ZIP_FL_COMPRESSED: u32 = 4;
pub const ZIP_FL_UNCHANGED: u32 = 8;
pub const ZIP_FL_RECOMPRESS: u32 = 16;
pub const ZIP_FL_ENCRYPTED: u32 = 32;
pub const ZIP_FL_ENC_GUESS: u32 = 0;
pub const ZIP_FL_ENC_RAW: u32 = 64;
pub const ZIP_FL_ENC_STRICT: u32 = 128;
pub const ZIP_FL_LOCAL: u32 = 256;
pub const ZIP_FL_CENTRAL: u32 = 512;
pub const ZIP_FL_ENC_UTF_8: u32 = 2048;
pub const ZIP_FL_ENC_CP437: u32 = 4096;
pub const ZIP_FL_OVERWRITE: u32 = 8192;


pub const ZIP_AFL_RDONLY: u32 = 2;
pub const ZIP_EXTRA_FIELD_ALL: u32 = 65535;
pub const ZIP_EXTRA_FIELD_NEW: u32 = 65535;


pub const ZIP_CM_DEFAULT: i32 = -1;
pub const ZIP_CM_STORE: i32 = 0;
pub const ZIP_CM_SHRINK: i32 = 1;
pub const ZIP_CM_REDUCE_1: i32 = 2;
pub const ZIP_CM_REDUCE_2: i32 = 3;
pub const ZIP_CM_REDUCE_3: i32 = 4;
pub const ZIP_CM_REDUCE_4: i32 = 5;
pub const ZIP_CM_IMPLODE: i32 = 6;
pub const ZIP_CM_DEFLATE: i32 = 8;
pub const ZIP_CM_DEFLATE64: i32 = 9;
pub const ZIP_CM_PKWARE_IMPLODE: i32 = 10;
pub const ZIP_CM_BZIP2: i32 = 12;
pub const ZIP_CM_LZMA: i32 = 14;
pub const ZIP_CM_TERSE: i32 = 18;
pub const ZIP_CM_LZ77: i32 = 19;
pub const ZIP_CM_LZMA2: i32 = 33;
pub const ZIP_CM_ZSTD: i32 = 93;
pub const ZIP_CM_XZ: i32 = 95;
pub const ZIP_CM_JPEG: i32 = 96;
pub const ZIP_CM_WAVPACK: i32 = 97;
pub const ZIP_CM_PPMD: i32 = 98;


pub const ZIP_EM_NONE: u16 = 0;
pub const ZIP_EM_TRAD_PKWARE: u16 = 1;
pub const ZIP_EM_AES_128: u16 = 257;
pub const ZIP_EM_AES_192: u16 = 258;
pub const ZIP_EM_AES_256: u16 = 259;
pub const ZIP_EM_UNKNOWN: u16 = 65535;
