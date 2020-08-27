use crate::bindings::{ dirent, opendir, readdir, readdir_r, closedir,
    ignores, scandir_baton_t, filter_fp, filename_filter };

use std::ffi::c_void;
use std::mem;

#[no_mangle]
pub unsafe extern "C" fn ag_scandir(
    dirname: *const cty::c_char,
    namelist: *mut *mut *mut dirent,
    f: filter_fp,
    baton: *mut cty::c_void) -> cty::c_int
{
    let mut dirp = opendir(dirname);
    if dirp.is_null() || mem::size_of::<*mut dirent>() == 0 {
        closedir(dirp);
        return -1
    }

    let mut names: Vec<*mut dirent> = Vec::new();
    loop {
        let entry = readdir(dirp);
        if entry.is_null() {
            //mem::forget(entry);
            break;
        }

        // TODO - couple with ag_scandir function's argument
        if filename_filter(dirname, entry, baton) == 0 {
            continue;
        }

        if (*entry).d_reclen == 0 {
            //mem::forget(entry);
            return -1
        }

        names.push(entry);
    }

    if !dirp.is_null() {
        closedir(dirp);
    }
    names.shrink_to_fit();
    assert_eq!(names.len(), names.capacity());
    let len = names.len() as cty::c_int;
    *namelist = names.into_raw_parts().0;

    return len;
}
