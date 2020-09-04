use crate::bindings::{
    dirent, filter_fp, opts, evil_hardcoded_ignore_files,
    scandir_baton_t, path_ignore_search, ag_asprintf, log_debug
};

use std::ffi::{ CString, CStr, OsStr };
use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use std::ptr::slice_from_raw_parts;
use std::mem::{self, MaybeUninit};

use std::os::unix::fs::DirEntryExt;

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}

pub fn str_to_c_char_ptr(s: &str) -> *mut cty::c_char {
    let c_str = CString::new(s.as_bytes()).unwrap_or_default();
    return c_str.into_raw() as *mut cty::c_char;
}

unsafe fn filename_filter(path: &str, dir: &DirEntry, baton: *mut cty::c_void) -> bool {
    // Some definitions for convencience and upcoming borrowing
    let file_n = dir.file_name();
    let filename = file_n.to_str().unwrap();
    let filename_len = filename.len();
    let extension = get_extension_from_filename(&filename);
    let mut filename_vec: Vec<char> = filename.chars().collect();

    // Some paths to always ignore
    let mut evil_hardcoded_ignore_files_rs: Vec<&str> = Vec::new();
    evil_hardcoded_ignore_files_rs.push(".");
    evil_hardcoded_ignore_files_rs.push("..");
    //evil_hardcoded_ignore_files_rs.push(cty::c_void);

    if opts.search_hidden_files == 0 && filename_vec[0] == '.' {
        return false
    }

    for file in evil_hardcoded_ignore_files_rs {
        if filename == file {
            return false
        }
    }

    let metadata = fs::symlink_metadata(Path::new(path)).unwrap();
    let file_type = metadata.file_type();
    if opts.follow_symlinks == 0 && file_type.is_symlink() {
        //log_debug("File %s ignored becaused it's a symlink", dir.file_name());    // TODO
        return false
    }

    // TODO return false if is named pipe (fifo)

    if opts.search_all_files == 1 && opts.path_to_ignore == 0 {
        return true;
    }

    if filename_vec[0] == '.' && filename_vec[1] == '/' {
        filename_vec.remove(0);
    }

    // need to be a little more unidiomatic here
    let scandir_baton = baton as *const scandir_baton_t;
    let path_start = (*scandir_baton).path_start;
    let mut ig = (*scandir_baton).ig;

    loop {
        if ig.is_null() { break; }

        // make vector, so we can use index
        let ext_len = (*ig).extensions_len as usize;
        let extensions = slice_from_raw_parts((*ig).extensions, ext_len);
        let mut extensions_vec_c_str: Vec<CString> = Vec::new();
        for i in 0..ext_len {
            let elem = (&*extensions)[i];
            let elem_c_str = CString::from_raw(elem);
            extensions_vec_c_str.push(elem_c_str);
        }

        if extension.is_some() {
            let extension_c_str = CString::new(extension.unwrap());
            if extensions_vec_c_str.contains(&extension_c_str.unwrap()) {
                return false
            }
        }
        // log_debug("file %s ignored because name matches extension %s", filename, ig->extensions[match_pos]); // TODO

        // TODO - this is still calling the C function
        if path_ignore_search(ig, path_start, filename.as_ptr() as *const cty::c_char) == 1 {
            return false
        }

        if file_type.is_dir() {
            if filename_vec[filename_len - 1] != '/' {
                let rv = path_ignore_search(ig, path_start, str_to_c_char_ptr(filename));
                if (rv > 0) {
                    return false
                }
            }
        }
        ig = (*ig).parent;
    }

    //log_debug("%s not ignored", filename);
    return true;
}

fn ag_scandir_rs(dirname: &str, names: &mut Vec<DirEntry>, baton: *mut cty::c_void) -> cty::c_int
{
    let path = Path::new(dirname);

    for entry in fs::read_dir(path).expect("unable to list") {
        let entry = entry.expect("unable to get entry");

        if unsafe { !filename_filter(dirname, &entry, baton) } {
            continue;
        }

        names.push(entry);
    }

    return names.len() as cty::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn ag_scandir(
    dirname: *const cty::c_char,
    namelist: *mut *mut *mut dirent,
    #[allow(unused_variables)] f: filter_fp, // we use our own filename_filter instead
    baton: *mut cty::c_void) -> cty::c_int
{
    let dirname_rs = CStr::from_ptr(dirname).to_str().unwrap();

    let mut names: Vec<DirEntry> = Vec::new();
    let found = ag_scandir_rs(dirname_rs, &mut names, baton);

    names.shrink_to_fit();
    assert_eq!(names.len(), names.capacity());

    for name in names {
        let name_string = name.file_name().into_string().unwrap();
        let name_vec: Vec<char> = name_string.chars().collect();
        let mut arr: [cty::c_char; 256usize] = MaybeUninit::uninit().assume_init();
        for i in 0..name_vec.len() {
            arr[i] = name_vec[i] as cty::c_char;
        }
        **namelist = &mut dirent {d_ino: name.ino(), d_off: 0, d_reclen: 0, d_type: 0, d_name: arr};
    }

    return found
}
