use libc;

#[repr(C)]
pub struct bspatch_stream {
    pub opaque: *mut libc::c_void,

    pub read: unsafe extern "C" fn(stream: *mut bspatch_stream, buffer: *mut libc::c_void, length: libc::c_int) -> libc::c_int,
}

extern "C" {
    pub fn bspatch(old: *const libc::uint8_t, oldsize: libc::int64_t
                 , new: *mut libc::uint8_t, newsize: libc::int64_t
                 , stream: *mut bspatch_stream) -> libc::c_int;
}
