use libc::time_t;

pub const SEEK_SET: i32 = 0;
pub const SEEK_CUR: i32 = 1;
pub const SEEK_END: i32 = 2;


pub const ZIP_OPSYS_DOS: u32 = 0;
pub const ZIP_OPSYS_AMIGA: u32 = 1;
pub const ZIP_OPSYS_OPENVMS: u32 = 2;
pub const ZIP_OPSYS_UNIX: u32 = 3;
pub const ZIP_OPSYS_VM_CMS: u32 = 4;
pub const ZIP_OPSYS_ATARI_ST: u32 = 5;
pub const ZIP_OPSYS_OS_2: u32 = 6;
pub const ZIP_OPSYS_MACINTOSH: u32 = 7;
pub const ZIP_OPSYS_Z_SYSTEM: u32 = 8;
pub const ZIP_OPSYS_CPM: u32 = 9;
pub const ZIP_OPSYS_WINDOWS_NTFS: u32 = 10;
pub const ZIP_OPSYS_MVS: u32 = 11;
pub const ZIP_OPSYS_VSE: u32 = 12;
pub const ZIP_OPSYS_ACORN_RISC: u32 = 13;
pub const ZIP_OPSYS_VFAT: u32 = 14;
pub const ZIP_OPSYS_ALTERNATE_MVS: u32 = 15;
pub const ZIP_OPSYS_BEOS: u32 = 16;
pub const ZIP_OPSYS_TANDEM: u32 = 17;
pub const ZIP_OPSYS_OS_400: u32 = 18;
pub const ZIP_OPSYS_OS_X: u32 = 19;
pub const ZIP_OPSYS_DEFAULT: u32 = 3;


pub const ZIP_FILE_ATTRIBUTES_HOST_SYSTEM: u32 = 1;
pub const ZIP_FILE_ATTRIBUTES_ASCII: u32 = 2;
pub const ZIP_FILE_ATTRIBUTES_VERSION_NEEDED: u32 = 4;
pub const ZIP_FILE_ATTRIBUTES_EXTERNAL_FILE_ATTRIBUTES: u32 = 8;
pub const ZIP_FILE_ATTRIBUTES_GENERAL_PURPOSE_BIT_FLAGS: u32 = 16;


pub type zip_int8_t = i8;
pub type zip_uint8_t = u8;
pub type zip_int16_t = i16;
pub type zip_uint16_t = u16;
pub type zip_int32_t = i32;
pub type zip_uint32_t = u32;
pub type zip_int64_t = i64;
pub type zip_uint64_t = u64;


pub const ZIP_SOURCE_OPEN: zip_source_cmd = 0;
pub const ZIP_SOURCE_READ: zip_source_cmd = 1;
pub const ZIP_SOURCE_CLOSE: zip_source_cmd = 2;
pub const ZIP_SOURCE_STAT: zip_source_cmd = 3;
pub const ZIP_SOURCE_ERROR: zip_source_cmd = 4;
pub const ZIP_SOURCE_FREE: zip_source_cmd = 5;
pub const ZIP_SOURCE_SEEK: zip_source_cmd = 6;
pub const ZIP_SOURCE_TELL: zip_source_cmd = 7;
pub const ZIP_SOURCE_BEGIN_WRITE: zip_source_cmd = 8;
pub const ZIP_SOURCE_COMMIT_WRITE: zip_source_cmd = 9;
pub const ZIP_SOURCE_ROLLBACK_WRITE: zip_source_cmd = 10;
pub const ZIP_SOURCE_WRITE: zip_source_cmd = 11;
pub const ZIP_SOURCE_SEEK_WRITE: zip_source_cmd = 12;
pub const ZIP_SOURCE_TELL_WRITE: zip_source_cmd = 13;
pub const ZIP_SOURCE_SUPPORTS: zip_source_cmd = 14;
pub const ZIP_SOURCE_REMOVE: zip_source_cmd = 15;
pub const ZIP_SOURCE_RESERVED_1: zip_source_cmd = 16;
pub const ZIP_SOURCE_BEGIN_WRITE_CLONING: zip_source_cmd = 17;
pub const ZIP_SOURCE_ACCEPT_EMPTY: zip_source_cmd = 18;
pub const ZIP_SOURCE_GET_FILE_ATTRIBUTES: zip_source_cmd = 19;
pub type zip_source_cmd = libc::c_uint;
pub use self::zip_source_cmd as zip_source_cmd_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zip_source_args_seek {
    pub offset: zip_int64_t,
    pub whence: libc::c_int,
}
pub type zip_source_args_seek_t = zip_source_args_seek;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zip_error {
    pub zip_err: libc::c_int,
    pub sys_err: libc::c_int,
    pub str_: *mut libc::c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zip_stat {
    pub valid: zip_uint64_t,
    pub name: *const libc::c_char,
    pub index: zip_uint64_t,
    pub size: zip_uint64_t,
    pub comp_size: zip_uint64_t,
    pub mtime: time_t,
    pub crc: zip_uint32_t,
    pub comp_method: zip_uint16_t,
    pub encryption_method: zip_uint16_t,
    pub flags: zip_uint32_t,
}

impl std::fmt::Debug for zip_stat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stat")
            .field("valid", &self.valid)
            .field("name", unsafe { &std::ffi::CStr::from_ptr(self.name) })
            .field("index", &self.index)
            .field("size", &self.size)
            .field("comp_size", &self.comp_size)
            .field("mtime", &self.mtime)
            .field("crc", &self.crc)
            .field("comp_method", &self.comp_method)
            .field("encryption_method", &self.encryption_method)
            .field("flags", &self.flags)
            .finish()
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zip_buffer_fragment {
    pub data: *mut zip_uint8_t,
    pub length: zip_uint64_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zip_file_attributes {
    pub valid: zip_uint64_t,
    pub version: zip_uint8_t,
    pub host_system: zip_uint8_t,
    pub ascii: zip_uint8_t,
    pub version_needed: zip_uint8_t,
    pub external_file_attributes: zip_uint32_t,
    pub general_purpose_bit_flags: zip_uint16_t,
    pub general_purpose_bit_mask: zip_uint16_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zip {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zip_file {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zip_source {
    _unused: [u8; 0],
}
pub type zip_t = zip;
pub type zip_error_t = zip_error;
pub type zip_file_t = zip_file;
pub type zip_file_attributes_t = zip_file_attributes;
pub type zip_source_t = zip_source;
pub type zip_stat_t = zip_stat;
pub type zip_buffer_fragment_t = zip_buffer_fragment;
pub type zip_flags_t = zip_uint32_t;
pub type zip_source_callback = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut libc::c_void,
        arg2: *mut libc::c_void,
        arg3: zip_uint64_t,
        arg4: zip_source_cmd_t,
    ) -> zip_int64_t,
>;
pub type zip_progress_callback = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut zip_t, arg2: f64, arg3: *mut libc::c_void),
>;
pub type zip_cancel_callback = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut zip_t,
        arg2: *mut libc::c_void,
    ) -> libc::c_int,
>;
pub type zip_progress_callback_t = ::std::option::Option<unsafe extern "C" fn(arg1: f64)>;
extern "C" {
    pub fn zip_register_progress_callback(arg1: *mut zip_t, arg2: zip_progress_callback_t);
    pub fn zip_add(
        arg1: *mut zip_t,
        arg2: *const libc::c_char,
        arg3: *mut zip_source_t,
    ) -> zip_int64_t;
    pub fn zip_add_dir(arg1: *mut zip_t, arg2: *const libc::c_char) -> zip_int64_t;
    pub fn zip_get_file_comment(
        arg1: *mut zip_t,
        arg2: zip_uint64_t,
        arg3: *mut libc::c_int,
        arg4: libc::c_int,
    ) -> *const libc::c_char;
    pub fn zip_get_num_files(arg1: *mut zip_t) -> libc::c_int;
    pub fn zip_rename(
        arg1: *mut zip_t,
        arg2: zip_uint64_t,
        arg3: *const libc::c_char,
    ) -> libc::c_int;
    pub fn zip_replace(
        arg1: *mut zip_t,
        arg2: zip_uint64_t,
        arg3: *mut zip_source_t,
    ) -> libc::c_int;
    pub fn zip_set_file_comment(
        arg1: *mut zip_t,
        arg2: zip_uint64_t,
        arg3: *const libc::c_char,
        arg4: libc::c_int,
    ) -> libc::c_int;
    pub fn zip_error_get_sys_type(arg1: libc::c_int) -> libc::c_int;
    pub fn zip_error_get(
        arg1: *mut zip_t,
        arg2: *mut libc::c_int,
        arg3: *mut libc::c_int,
    );
    pub fn zip_error_to_str(
        arg1: *mut libc::c_char,
        arg2: zip_uint64_t,
        arg3: libc::c_int,
        arg4: libc::c_int,
    ) -> libc::c_int;
    pub fn zip_file_error_get(
        arg1: *mut zip_file_t,
        arg2: *mut libc::c_int,
        arg3: *mut libc::c_int,
    );
    pub fn zip_close(arg1: *mut zip_t) -> libc::c_int;
    pub fn zip_delete(arg1: *mut zip_t, arg2: zip_uint64_t) -> libc::c_int;
    pub fn zip_dir_add(
        arg1: *mut zip_t,
        arg2: *const libc::c_char,
        arg3: zip_flags_t,
    ) -> zip_int64_t;
    pub fn zip_discard(arg1: *mut zip_t);
    pub fn zip_get_error(arg1: *mut zip_t) -> *mut zip_error_t;
    pub fn zip_error_clear(arg1: *mut zip_t);
    pub fn zip_error_code_zip(arg1: *const zip_error_t) -> libc::c_int;
    pub fn zip_error_code_system(arg1: *const zip_error_t) -> libc::c_int;
    pub fn zip_error_fini(arg1: *mut zip_error_t);
    pub fn zip_error_init(arg1: *mut zip_error_t);
    pub fn zip_error_init_with_code(arg1: *mut zip_error_t, arg2: libc::c_int);
    pub fn zip_error_set(
        arg1: *mut zip_error_t,
        arg2: libc::c_int,
        arg3: libc::c_int,
    );
    pub fn zip_error_strerror(arg1: *mut zip_error_t) -> *const libc::c_char;
    pub fn zip_error_system_type(arg1: *const zip_error_t) -> libc::c_int;
    pub fn zip_error_to_data(
        arg1: *const zip_error_t,
        arg2: *mut libc::c_void,
        arg3: zip_uint64_t,
    ) -> zip_int64_t;
    pub fn zip_fclose(arg1: *mut zip_file_t) -> libc::c_int;
    pub fn zip_fdopen(
        arg1: libc::c_int,
        arg2: libc::c_int,
        arg3: *mut libc::c_int,
    ) -> *mut zip_t;
    pub fn zip_file_add(
        arg1: *mut zip_t,
        arg2: *const libc::c_char,
        arg3: *mut zip_source_t,
        arg4: zip_flags_t,
    ) -> zip_int64_t;
    pub fn zip_file_attributes_init(arg1: *mut zip_file_attributes_t);
    pub fn zip_file_error_clear(arg1: *mut zip_file_t);
    pub fn zip_file_extra_field_delete(
        arg1: *mut zip_t,
        arg2: zip_uint64_t,
        arg3: zip_uint16_t,
        arg4: zip_flags_t,
    ) -> libc::c_int;
    pub fn zip_file_extra_field_delete_by_id(
        arg1: *mut zip_t,
        arg2: zip_uint64_t,
        arg3: zip_uint16_t,
        arg4: zip_uint16_t,
        arg5: zip_flags_t,
    ) -> libc::c_int;
    pub fn zip_file_extra_field_set(
        arg1: *mut zip_t,
        arg2: zip_uint64_t,
        arg3: zip_uint16_t,
        arg4: zip_uint16_t,
        arg5: *const zip_uint8_t,
        arg6: zip_uint16_t,
        arg7: zip_flags_t,
    ) -> libc::c_int;
    pub fn zip_file_extra_fields_count(
        arg1: *mut zip_t,
        arg2: zip_uint64_t,
        arg3: zip_flags_t,
    ) -> zip_int16_t;
    pub fn zip_file_extra_fields_count_by_id(
        arg1: *mut zip_t,
        arg2: zip_uint64_t,
        arg3: zip_uint16_t,
        arg4: zip_flags_t,
    ) -> zip_int16_t;
    pub fn zip_file_extra_field_get(
        arg1: *mut zip_t,
        arg2: zip_uint64_t,
        arg3: zip_uint16_t,
        arg4: *mut zip_uint16_t,
        arg5: *mut zip_uint16_t,
        arg6: zip_flags_t,
    ) -> *const zip_uint8_t;
    pub fn zip_file_extra_field_get_by_id(
        arg1: *mut zip_t,
        arg2: zip_uint64_t,
        arg3: zip_uint16_t,
        arg4: zip_uint16_t,
        arg5: *mut zip_uint16_t,
        arg6: zip_flags_t,
    ) -> *const zip_uint8_t;
    pub fn zip_file_get_comment(
        arg1: *mut zip_t,
        arg2: zip_uint64_t,
        arg3: *mut zip_uint32_t,
        arg4: zip_flags_t,
    ) -> *const libc::c_char;
    pub fn zip_file_get_error(arg1: *mut zip_file_t) -> *mut zip_error_t;
    pub fn zip_file_get_external_attributes(
        arg1: *mut zip_t,
        arg2: zip_uint64_t,
        arg3: zip_flags_t,
        arg4: *mut zip_uint8_t,
        arg5: *mut zip_uint32_t,
    ) -> libc::c_int;
    pub fn zip_file_rename(
        arg1: *mut zip_t,
        arg2: zip_uint64_t,
        arg3: *const libc::c_char,
        arg4: zip_flags_t,
    ) -> libc::c_int;
    pub fn zip_file_replace(
        arg1: *mut zip_t,
        arg2: zip_uint64_t,
        arg3: *mut zip_source_t,
        arg4: zip_flags_t,
    ) -> libc::c_int;
    pub fn zip_file_set_comment(
        arg1: *mut zip_t,
        arg2: zip_uint64_t,
        arg3: *const libc::c_char,
        arg4: zip_uint16_t,
        arg5: zip_flags_t,
    ) -> libc::c_int;
    pub fn zip_file_set_dostime(
        arg1: *mut zip_t,
        arg2: zip_uint64_t,
        arg3: zip_uint16_t,
        arg4: zip_uint16_t,
        arg5: zip_flags_t,
    ) -> libc::c_int;
    pub fn zip_file_set_encryption(
        arg1: *mut zip_t,
        arg2: zip_uint64_t,
        arg3: zip_uint16_t,
        arg4: *const libc::c_char,
    ) -> libc::c_int;
    pub fn zip_file_set_external_attributes(
        arg1: *mut zip_t,
        arg2: zip_uint64_t,
        arg3: zip_flags_t,
        arg4: zip_uint8_t,
        arg5: zip_uint32_t,
    ) -> libc::c_int;
    pub fn zip_file_set_mtime(
        arg1: *mut zip_t,
        arg2: zip_uint64_t,
        arg3: time_t,
        arg4: zip_flags_t,
    ) -> libc::c_int;
    pub fn zip_file_strerror(arg1: *mut zip_file_t) -> *const libc::c_char;
    pub fn zip_fopen(
        arg1: *mut zip_t,
        arg2: *const libc::c_char,
        arg3: zip_flags_t,
    ) -> *mut zip_file_t;
    pub fn zip_fopen_encrypted(
        arg1: *mut zip_t,
        arg2: *const libc::c_char,
        arg3: zip_flags_t,
        arg4: *const libc::c_char,
    ) -> *mut zip_file_t;
    pub fn zip_fopen_index(
        arg1: *mut zip_t,
        arg2: zip_uint64_t,
        arg3: zip_flags_t,
    ) -> *mut zip_file_t;
    pub fn zip_fopen_index_encrypted(
        arg1: *mut zip_t,
        arg2: zip_uint64_t,
        arg3: zip_flags_t,
        arg4: *const libc::c_char,
    ) -> *mut zip_file_t;
    pub fn zip_fread(
        arg1: *mut zip_file_t,
        arg2: *mut libc::c_void,
        arg3: zip_uint64_t,
    ) -> zip_int64_t;
    pub fn zip_fseek(
        arg1: *mut zip_file_t,
        arg2: zip_int64_t,
        arg3: libc::c_int,
    ) -> zip_int8_t;
    pub fn zip_ftell(arg1: *mut zip_file_t) -> zip_int64_t;
    pub fn zip_get_archive_comment(
        arg1: *mut zip_t,
        arg2: *mut libc::c_int,
        arg3: zip_flags_t,
    ) -> *const libc::c_char;
    pub fn zip_get_archive_flag(
        arg1: *mut zip_t,
        arg2: zip_flags_t,
        arg3: zip_flags_t,
    ) -> libc::c_int;
    pub fn zip_get_name(
        arg1: *mut zip_t,
        arg2: zip_uint64_t,
        arg3: zip_flags_t,
    ) -> *const libc::c_char;
    pub fn zip_get_num_entries(arg1: *mut zip_t, arg2: zip_flags_t) -> zip_int64_t;
    pub fn zip_libzip_version() -> *const libc::c_char;
    pub fn zip_name_locate(
        arg1: *mut zip_t,
        arg2: *const libc::c_char,
        arg3: zip_flags_t,
    ) -> zip_int64_t;
    pub fn zip_open(
        arg1: *const libc::c_char,
        arg2: libc::c_int,
        arg3: *mut libc::c_int,
    ) -> *mut zip_t;
    pub fn zip_open_from_source(
        arg1: *mut zip_source_t,
        arg2: libc::c_int,
        arg3: *mut zip_error_t,
    ) -> *mut zip_t;
    pub fn zip_register_progress_callback_with_state(
        arg1: *mut zip_t,
        arg2: f64,
        arg3: zip_progress_callback,
        arg4: ::std::option::Option<unsafe extern "C" fn(arg1: *mut libc::c_void)>,
        arg5: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn zip_register_cancel_callback_with_state(
        arg1: *mut zip_t,
        arg2: zip_cancel_callback,
        arg3: ::std::option::Option<unsafe extern "C" fn(arg1: *mut libc::c_void)>,
        arg4: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn zip_set_archive_comment(
        arg1: *mut zip_t,
        arg2: *const libc::c_char,
        arg3: zip_uint16_t,
    ) -> libc::c_int;
    pub fn zip_set_archive_flag(
        arg1: *mut zip_t,
        arg2: zip_flags_t,
        arg3: libc::c_int,
    ) -> libc::c_int;
    pub fn zip_set_default_password(
        arg1: *mut zip_t,
        arg2: *const libc::c_char,
    ) -> libc::c_int;
    pub fn zip_set_file_compression(
        arg1: *mut zip_t,
        arg2: zip_uint64_t,
        arg3: zip_int32_t,
        arg4: zip_uint32_t,
    ) -> libc::c_int;
    pub fn zip_source_begin_write(arg1: *mut zip_source_t) -> libc::c_int;
    pub fn zip_source_begin_write_cloning(
        arg1: *mut zip_source_t,
        arg2: zip_uint64_t,
    ) -> libc::c_int;
    pub fn zip_source_buffer(
        arg1: *mut zip_t,
        arg2: *const libc::c_void,
        arg3: zip_uint64_t,
        arg4: libc::c_int,
    ) -> *mut zip_source_t;
    pub fn zip_source_buffer_create(
        arg1: *const libc::c_void,
        arg2: zip_uint64_t,
        arg3: libc::c_int,
        arg4: *mut zip_error_t,
    ) -> *mut zip_source_t;
    pub fn zip_source_buffer_fragment(
        arg1: *mut zip_t,
        arg2: *const zip_buffer_fragment_t,
        arg3: zip_uint64_t,
        arg4: libc::c_int,
    ) -> *mut zip_source_t;
    pub fn zip_source_buffer_fragment_create(
        arg1: *const zip_buffer_fragment_t,
        arg2: zip_uint64_t,
        arg3: libc::c_int,
        arg4: *mut zip_error_t,
    ) -> *mut zip_source_t;
    pub fn zip_source_close(arg1: *mut zip_source_t) -> libc::c_int;
    pub fn zip_source_commit_write(arg1: *mut zip_source_t) -> libc::c_int;
    pub fn zip_source_error(arg1: *mut zip_source_t) -> *mut zip_error_t;
    pub fn zip_source_file(
        arg1: *mut zip_t,
        arg2: *const libc::c_char,
        arg3: zip_uint64_t,
        arg4: zip_int64_t,
    ) -> *mut zip_source_t;
    pub fn zip_source_file_create(
        arg1: *const libc::c_char,
        arg2: zip_uint64_t,
        arg3: zip_int64_t,
        arg4: *mut zip_error_t,
    ) -> *mut zip_source_t;
    pub fn zip_source_free(arg1: *mut zip_source_t);
    pub fn zip_source_function(
        arg1: *mut zip_t,
        arg2: zip_source_callback,
        arg3: *mut libc::c_void,
    ) -> *mut zip_source_t;
    pub fn zip_source_function_create(
        arg1: zip_source_callback,
        arg2: *mut libc::c_void,
        arg3: *mut zip_error_t,
    ) -> *mut zip_source_t;
    pub fn zip_source_get_file_attributes(
        arg1: *mut zip_source_t,
        arg2: *mut zip_file_attributes_t,
    ) -> libc::c_int;
    pub fn zip_source_is_deleted(arg1: *mut zip_source_t) -> libc::c_int;
    pub fn zip_source_keep(arg1: *mut zip_source_t);
    pub fn zip_source_make_command_bitmap(arg1: zip_source_cmd_t, ...) -> zip_int64_t;
    pub fn zip_source_open(arg1: *mut zip_source_t) -> libc::c_int;
    pub fn zip_source_read(
        arg1: *mut zip_source_t,
        arg2: *mut libc::c_void,
        arg3: zip_uint64_t,
    ) -> zip_int64_t;
    pub fn zip_source_rollback_write(arg1: *mut zip_source_t);
    pub fn zip_source_seek(
        arg1: *mut zip_source_t,
        arg2: zip_int64_t,
        arg3: libc::c_int,
    ) -> libc::c_int;
    pub fn zip_source_seek_compute_offset(
        arg1: zip_uint64_t,
        arg2: zip_uint64_t,
        arg3: *mut libc::c_void,
        arg4: zip_uint64_t,
        arg5: *mut zip_error_t,
    ) -> zip_int64_t;
    pub fn zip_source_seek_write(
        arg1: *mut zip_source_t,
        arg2: zip_int64_t,
        arg3: libc::c_int,
    ) -> libc::c_int;
    pub fn zip_source_stat(arg1: *mut zip_source_t, arg2: *mut zip_stat_t)
        -> libc::c_int;
    pub fn zip_source_tell(arg1: *mut zip_source_t) -> zip_int64_t;
    pub fn zip_source_tell_write(arg1: *mut zip_source_t) -> zip_int64_t;
    pub fn zip_source_window_create(
        arg1: *mut zip_source_t,
        arg2: zip_uint64_t,
        arg3: zip_int64_t,
        arg4: *mut zip_error_t,
    ) -> *mut zip_source_t;
    pub fn zip_source_write(
        arg1: *mut zip_source_t,
        arg2: *const libc::c_void,
        arg3: zip_uint64_t,
    ) -> zip_int64_t;
    pub fn zip_source_zip(
        arg1: *mut zip_t,
        arg2: *mut zip_t,
        arg3: zip_uint64_t,
        arg4: zip_flags_t,
        arg5: zip_uint64_t,
        arg6: zip_int64_t,
    ) -> *mut zip_source_t;
    pub fn zip_source_zip_create(
        arg1: *mut zip_t,
        arg2: zip_uint64_t,
        arg3: zip_flags_t,
        arg4: zip_uint64_t,
        arg5: zip_int64_t,
        arg6: *mut zip_error_t,
    ) -> *mut zip_source_t;
    pub fn zip_stat(
        arg1: *mut zip_t,
        arg2: *const libc::c_char,
        arg3: zip_flags_t,
        arg4: *mut zip_stat_t,
    ) -> libc::c_int;
    pub fn zip_stat_index(
        arg1: *mut zip_t,
        arg2: zip_uint64_t,
        arg3: zip_flags_t,
        arg4: *mut zip_stat_t,
    ) -> libc::c_int;
    pub fn zip_stat_init(arg1: *mut zip_stat_t);
    pub fn zip_strerror(arg1: *mut zip_t) -> *const libc::c_char;
    pub fn zip_unchange(arg1: *mut zip_t, arg2: zip_uint64_t) -> libc::c_int;
    pub fn zip_unchange_all(arg1: *mut zip_t) -> libc::c_int;
    pub fn zip_unchange_archive(arg1: *mut zip_t) -> libc::c_int;
    pub fn zip_compression_method_supported(
        method: zip_int32_t,
        compress: libc::c_int,
    ) -> libc::c_int;
    pub fn zip_encryption_method_supported(
        method: zip_uint16_t,
        encode: libc::c_int,
    ) -> libc::c_int;
}
