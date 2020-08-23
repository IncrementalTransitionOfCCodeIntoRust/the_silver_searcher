use crate::bindings::{ dirent, opendir, readdir, readdir_r, closedir,
    ignores, scandir_baton_t, filter_fp, filename_filter };

use std::ffi::c_void;
use std::mem;

#[no_mangle]
pub unsafe extern "C" fn ag_scandir(
    dirname: *const cty::c_char,
    namelist: *mut *mut *mut dirent,
    f: filter_fp,
    baton: *mut libc::c_void) -> cty::c_int
{
    let dirp = opendir(dirname);
    if dirp.is_null() {
        closedir(dirp);
        return -1
    }

    if mem::size_of::<*const dirent>() == 0 {
        closedir(dirp);
        return -1
    }

    let mut names = Vec::new();
    loop {
        let mut entry = readdir(dirp);
        if entry.is_null() { break; }

        // TODO - check this one out, couple with this function's argument, get it to work ...
        if filename_filter(dirname, entry, baton) == 0 {
            continue;
        }

        names.push(entry);
    }

    closedir(dirp);
    names.shrink_to_fit();
    assert!(names.len() == names.capacity());
    let names_len = names.len() as cty::c_int;
    //*namelist = names.into_raw_parts().0;
    *namelist = names.as_mut_ptr();

    return names_len;
}