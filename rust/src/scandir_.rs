extern crate libc;

use crate::bindings::{ ignores, scandir_baton_t, filter_fp };
use libc::{ dirent, opendir, readdir, closedir };

use std::ffi::c_void;
use std::mem;

// pub unsafe extern "C" fn ag_scandir(
//     dirname: *const libc::c_char,
//     namelist: *mut *mut *mut dirent,
//     mut f: filter_fp,
//     baton: *mut libc::c_void)
//     -> libc::c_int
// {
//     return call_mut(dirname, namelist, f, baton);
// }

#[no_mangle]
pub unsafe extern "C" fn ag_scandir(
    dirname: *const cty::c_char,
    namelist: *mut *mut *mut dirent,
    f: filter_fp,
    baton: *mut cty::c_void) -> cty::c_int
{
    //let names_len = 32;
    //let mut results_len: cty::c_int = 0;

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

        // if f(dirname, entry, baton) == 0 {
        //     continue;
        // }

        //names[results_len as usize] = entry;
        names.push(entry);
        //results_len += 1;
    }

    closedir(dirp);
    let names_len = names.len() as cty::c_int;
    *namelist = names.into_raw_parts().0;

    return names_len;
}