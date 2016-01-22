use libc;

#[repr(C)]
pub struct bsdiff_stream {
    pub opaque: *mut libc::c_void,

    pub malloc: unsafe extern "C" fn(size: libc::size_t) -> *mut libc::c_void,
    pub free: unsafe extern "C" fn(ptr: *mut libc::c_void),
    pub write: unsafe extern "C" fn(stream: *mut bsdiff_stream, buffer: *const libc::c_void, size: libc::c_int) -> libc::c_int,
}

extern "C" {
    pub fn bsdiff(old: *const libc::uint8_t, oldsize: libc::int64_t
                , new: *const libc::uint8_t, newsize: libc::int64_t
                , stream: *mut bsdiff_stream) -> libc::c_int;
}
