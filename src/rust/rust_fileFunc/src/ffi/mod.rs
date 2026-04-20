use crate::file::FileInfo;
use crate::open_file::{OpenFile, OpenError};

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr;

const SUCCESS: c_int = 0;
const ERR_FILE_NOT_FOUND: c_int = 1;

const ERR_PERMISSION_DENIED: c_int = 2;

const ERR_INVALID_PATH: c_int = 3;

const ERR_UNKOWN: c_int = -1;

#[repr(C)]
pub struct CFileInfo {
    pub file_name: *const c_char,
    pub size: u64,
    pub is_readonly: bool,
}

#[unsafe(no_mangle)]
pub extern "C" fn get_file_info(path: *const c_char) -> *mut CFileInfo {
    if path.is_null() {
        return ptr::null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(path) };
    let path_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    match FileInfo::from_path(path_str) {
        Ok(info) => {
            let file_name = CString::new(info.file_name).unwrap();
            let c_info = CFileInfo {
                file_name: file_name.into_raw(),
                size: info.size,
                is_readonly: info.is_readonly,
            };

            Box::into_raw(Box::new(c_info))
        }
        Err(_) => ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn free_file_info(ptr:*mut CFileInfo) {
    if ptr.is_null() {
        return;
    }

    unsafe {
        let info = Box::from_raw(ptr);
        if !info.file_name.is_null() {
            let _ = CString::from_raw(info.file_name as *mut c_char);
        }
    }
}

pub extern "C" fn open_a_file(path: *const c_char) -> c_int {
    if path.is_null() {
        return ERR_INVALID_PATH;
    }

    let c_str = unsafe { CStr::from_ptr(path) };
    let path_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return ERR_INVALID_PATH,
    };

    let info = match FileInfo::from_path(path_str) {
        Ok(i) => i,
        Err(_) => return ERR_FILE_NOT_FOUND,
    };

    match OpenFile::open_static(&info) {
        Ok(_) => SUCCESS,
        Err(e) => match e {
            _ => ERR_FILE_NOT_FOUND,
        }
    }
}