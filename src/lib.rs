extern crate bsdiff_sys;
extern crate libc;

pub mod diff;
pub mod patch;

pub type Result<T> = ::std::result::Result<T, &'static str>;

#[cfg(test)]
mod test {
    use super::{diff, patch};

    #[test]
    fn test_integrity() {
        // Make diff
        let old = [0, 1, 2, 3, 4, 5];
        let new = [0, 1, 2, 2, 3, 4, 5];

        let diff = diff::diff_to_vec(&old, &new).unwrap();

        // Apply diff
        let mut patched = [0; 7];
        patch::patch_from_slice(&old, &mut patched, &diff).unwrap();

        assert_eq!(new, patched);
    }
}
