use bsdiff_sys::patch as ffi;
use libc;
use std::io::Read;
use super::Result;

unsafe extern "C" fn read(stream: *mut ffi::bspatch_stream, buffer: *mut libc::c_void, length: libc::c_int) -> libc::c_int {
    use std::slice;

    let reader = (*stream).opaque as *mut &mut Read;
    let reader = &mut *reader;
    let buffer = slice::from_raw_parts_mut(buffer as *mut u8, length as usize);

    match reader.read_exact(buffer) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

pub fn patch_from_reader<R: Read>(old: &[u8], new: &mut [u8], reader: &mut R) -> Result<()> {
    let mut reader = reader as &mut Read;
    let reader = &mut reader as *mut &mut Read;

    let mut stream = ffi::bspatch_stream {
        opaque: reader as *mut libc::c_void,

        read: read,
    };
    let result = unsafe {
        ffi::bspatch(old.as_ptr(), old.len() as libc::int64_t
                   , new.as_mut_ptr(), new.len() as libc::int64_t
                   , &mut stream as *mut _)
    };

    if result == 0 { Ok(()) } else { Err("bspatch error") }
}

pub fn patch_from_slice(old: &[u8], new: &mut [u8], mut diff: &[u8]) -> Result<()> {
    patch_from_reader(old, new, &mut diff)
}
