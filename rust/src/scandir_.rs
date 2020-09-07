use crate::bindings::{ dirent, opendir, readdir, readdir_r, closedir,
    ignores, scandir_baton_t, filter_fp, filename_filter };

use std::ffi::{ c_void, CStr };
use std::mem;

use walkdir::WalkDir;
use walkdir::DirEntryExt;

pub unsafe fn char_ptr_to_string(s: *const libc::c_char) -> String {
    return String::from(CStr::from_ptr(s).to_str().unwrap());
}

#[no_mangle]
pub unsafe extern "C" fn ag_scandir(
    dirname: *const cty::c_char,
    namelist: *mut *mut *mut dirent,
    f: filter_fp,
    baton: *mut cty::c_void) -> cty::c_int
{
    // let mut names: Vec<*mut dirent> = Vec::new();
    let mut result = 0;

    for entry in WalkDir::new(char_ptr_to_string(dirname)).into_iter().filter_map(|e| e.ok()) {
        //println!("{}", entry.path().display());
        let file_name = entry.file_name();
        //println!("file name rust:{}", file_name.to_str().unwrap());

        let mut i8_arr: [i8; 256] = [0; 256];
        let file_name_bytes = file_name.to_str().unwrap().as_bytes();
        for i in 0..file_name_bytes.len() {
            //println!("bytes [{}]: {}", i, file_name_bytes[i]);
            i8_arr[i] = file_name_bytes[i] as i8;

        }

        let file_type = entry.file_type();
        println!("file_type: {}", file_type.is_dir());
        let mut e = dirent {d_ino: entry.ino(), d_off: 0, d_reclen: 0, d_type: 0, d_name: i8_arr};
        if filename_filter(dirname, &e, baton) != 0 {
            **namelist = &mut dirent {d_ino: entry.ino(), d_off: 0, d_reclen: 0, d_type: 0, d_name: i8_arr};
            result += 1;
        }
        // TODO - couple with ag_scandir function's argument
        // if !names.contains(&e) && filename_filter(dirname, e, baton) != 0 {
        //     names.push(e);
        // }
    }

    // let result = names.len() as cty::c_int;
    // names.shrink_to_fit();
    // assert_eq!(names.len(), names.capacity());
    // *namelist = names.into_raw_parts().0;

    return result
}
