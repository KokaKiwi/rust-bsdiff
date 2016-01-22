use bsdiff_sys::diff as ffi;
use libc;
use std::io::Write;
use super::Result;

unsafe extern "C" fn write(stream: *mut ffi::bsdiff_stream, buffer: *const libc::c_void, size: libc::c_int) -> libc::c_int {
    use std::slice;

    let writer = (*stream).opaque as *mut &mut Write;
    let writer = &mut *writer;
    let buffer = slice::from_raw_parts(buffer as *const u8, size as usize);

    match writer.write_all(buffer) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

pub fn diff_to_writer<W: Write>(old: &[u8], new: &[u8], writer: &mut W) -> Result<()> {
    let mut writer = writer as &mut Write;
    let writer = &mut writer as *mut &mut Write;

    let mut stream = ffi::bsdiff_stream {
        opaque: writer as *mut libc::c_void,

        malloc: libc::malloc,
        free: libc::free,
        write: write,
    };
    let result = unsafe {
        ffi::bsdiff(old.as_ptr(), old.len() as libc::int64_t
                    , new.as_ptr(), new.len() as libc::int64_t
                    , &mut stream as *mut _)
    };

    if result == 0 { Ok(()) } else { Err("bsdiff error") }
}

pub fn diff_to_vec(old: &[u8], new: &[u8]) -> Result<Vec<u8>> {
    let mut dest = Vec::new();
    diff_to_writer(old, new, &mut dest).map(|_| dest)
}
