use crate::bindings::{
    filename_filter, opendir, readdir, readdir_r, closedir,
    ignores, scandir_baton_t, filter_fp, dirent
};

use std::ffi::{ c_void, CStr, CString };
use std::mem;
use std::io;
use std::str;
use std::path::{ Path, PathBuf };
use same_file::is_same_file;

pub fn str_to_c_char_ptr(s: &str) -> *mut libc::c_char {
    let c_str = CString::new(s.as_bytes()).unwrap_or_default();
    return c_str.into_raw() as *mut libc::c_char;
}

fn contains_loop<P: AsRef<Path>>(path: P) -> io::Result<Option<(PathBuf, PathBuf)>> {
    let path = path.as_ref();
    let mut path_buf = path.to_path_buf();
    while path_buf.pop() {
        if is_same_file(&path_buf, path)? {
            return Ok(Some((path_buf, path.to_path_buf())));
        } else if let Some(looped_paths) = contains_loop(&path_buf)? {
            return Ok(Some(looped_paths));
        }
    }
    return Ok(None);
}

fn to_u8(c: &[i8; 256]) -> [u8; 256] {
    let mut u8_arr: [u8; 256] = [0; 256];
    for i in 0..256 {
        let elem = c[i] as u8;
        u8_arr[i] = elem + 128;
    }

    return u8_arr
}

#[no_mangle]
pub unsafe extern "C" fn ag_scandir(
    dirname: *const cty::c_char,
    namelist: *mut *mut *mut dirent,
    #[allow(unused_variables)] f: filter_fp,
    baton: *mut cty::c_void) -> cty::c_int
{
    let dirp = opendir(dirname);
    if dirp.is_null() || mem::size_of::<*mut dirent>() == 0 {
        closedir(dirp);
        return -1
    }

    let mut names: Vec<*mut dirent> = Vec::new();
    loop {
        let entry = readdir(dirp);
        if entry.is_null() {
            break;
        }

        // TODO - couple with ag_scandir function's argument
        if filename_filter(dirname, entry, baton) == 0 {
            continue;
        }

        const size: usize = 256;
        let d_name: [cty::c_char; size] = (*entry).d_name;
        let d_name_u8: [u8; size] = to_u8(&d_name);   
        let d_name_str = str::from_utf8_unchecked(&d_name_u8[..]);
        let opp = contains_loop(Path::new(d_name_str));

        if !names.contains(&entry) {// &&  opp.unwrap_or_default() == None {
            names.push(entry);
        }
    }

    if !dirp.is_null() {
        closedir(dirp);
    }

    let found = names.len() as cty::c_int;
    names.shrink_to_fit();
    assert_eq!(names.len(), names.capacity());
    *namelist = names.into_raw_parts().0;

    return found;
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::bindings::filename_filter;

//     #[test]
//     fn huhu_works() {
//         let d : *mut *mut *mut dirent = unsafe { mem::MaybeUninit::uninit().assume_init() };
//         let s: scandir_baton_t = unsafe { mem::MaybeUninit::uninit().assume_init() };
//         let f: std::option::Option<unsafe extern "C" fn(*const i8, *const dirent, *mut libc::c_void) -> i32> = unsafe { mem::MaybeUninit::uninit().assume_init() };
//         let v: *mut libc::c_void = unsafe { mem::MaybeUninit::uninit().assume_init() };
//         assert_eq!(unsafe { ag_scandir(str_to_c_char_ptr("."), d, f, v) }, 0);
//     }
// }

