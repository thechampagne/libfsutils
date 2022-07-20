/*
* Copyright (c) 2022 XXIV
* 
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
* 
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
* 
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::ffi::CString;
use std::ffi::CStr;
use std::vec::Vec;
use fs_utils::check::is_folder_empty;
use fs_utils::copy::copy_directory;
use fs_utils::copy::destination_directory;
use fs_utils::read::head;
use fs_utils::read::head_to_string;
use fs_utils::read::head_to_string_with_message;
use fs_utils::remove::cleanup_folder;

#[repr(C)]
union fs_utils_t {
    buffer: *mut c_char,
    error: *mut c_char,
}

/// Checks if the given folder is empty.
///
/// Example:
/// * *
/// int main()
/// {
///   int is_empty;
///   if (fs_utils_is_folder_empty(".", &is_empty) != 0)
///   {
///     printf("Something went wrong\n");
///     return -1;
///   }
///   printf("Is folder empty: %d\n", is_empty);
///   return 0;
/// }
/// * *
///
/// @param path
/// @return 0 on success and non zero value on failure
#[no_mangle]
unsafe extern "C" fn fs_utils_is_folder_empty(path: *const c_char, is_empty: *mut c_int) -> c_int {
    if path.is_null() {
        return -1;
    }
    let str = match CStr::from_ptr(path).to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };
    match is_folder_empty(str) {
        Ok(v) => {
            if v {
                *is_empty = 1
            } else {
                *is_empty = 0
            }
            0
        }
        Err(_) => -1,
    }
}

/// Copies the contents of the source directory to the given destination directory.
/// In destination_dir, a new subdirectory with the basename of the source_dir will be created.
/// It will not perform the copy operation if the effective destination directory does already exist.
///
/// Example:
/// * *
/// int main()
/// {
///   fs_utils_t fs;
///   int res;
///   if ((res = fs_utils_copy_directory(&fs,"src", "dest")) != 0)
///   {
///     if (res == 1)
///     {
///       printf("Something went wrong: %s", fs.error);
///       return -1;
///     }
///     else
///     {
///       printf("Something went wrong");
///       return -1;
///     }
///   }
///   
///   printf("Path: %s\n", fs.buffer);
///   fs_utils_clean(&fs);
///   return 0;
/// }
/// * *
///
/// @param fs_utils pointer to fs_utils_t
/// @param source_dir
/// @param destination_dir
/// @return 0 on success and non zero value on failure
#[no_mangle]
unsafe extern "C" fn fs_utils_copy_directory(
    fs_utils: *mut fs_utils_t,
    source_dir: *const c_char,
    destination_dir: *const c_char,
) -> c_int {
    if source_dir.is_null() || destination_dir.is_null() {
        match CString::new("source_dir or destination_dir is null") {
            Ok(s) => {
                (*fs_utils).error = s.into_raw();
                return 1;
            }
            Err(_) => {
                (*fs_utils).error = std::ptr::null_mut();
                return -1;
            }
        }
    }
    let sdir = match CStr::from_ptr(source_dir).to_str() {
        Ok(s) => s,
        Err(_) => match CString::new("UTF-8 validation failed in source_dir") {
            Ok(s) => {
                (*fs_utils).error = s.into_raw();
                return 1;
            }
            Err(_) => {
                (*fs_utils).error = std::ptr::null_mut();
                return -1;
            }
        },
    };
    let ddir = match CStr::from_ptr(destination_dir).to_str() {
        Ok(s) => s,
        Err(_) => match CString::new("UTF-8 validation failed in destination_dir") {
            Ok(s) => {
                (*fs_utils).error = s.into_raw();
                return 1;
            }
            Err(_) => {
                (*fs_utils).error = std::ptr::null_mut();
                return -1;
            }
        },
    };
    match copy_directory(sdir, ddir) {
        Ok(v) => match CString::new(v.to_string_lossy().into_owned()) {
            Ok(s) => {
                (*fs_utils).buffer = s.into_raw();
                0
            }
            Err(err) => match CString::new(err.to_string()) {
                Ok(s) => {
                    (*fs_utils).error = s.into_raw();
                    1
                }
                Err(_) => {
                    (*fs_utils).error = std::ptr::null_mut();
                    -1
                }
            },
        },
        Err(_) => -1,
    }
}

/// Example:
/// * *
/// int main()
/// {
///   char* res = fs_utils_destination_directory("src", "dest");
///   if (res == NULL)
///   {
///       printf("Something went wrong");
///       return -1;
///   }
///   
///   printf("Destination directory: %s\n", res);
///   fs_utils_free(res);
///   return 0;
/// }
/// * *
///
/// @param source_dir
/// @param destination_dir
/// @return the computed destination directory, given a source directory.
#[no_mangle]
unsafe extern "C" fn fs_utils_destination_directory(
    source_dir: *const c_char,
    destination_dir: *const c_char,
) -> *mut c_char {
    if source_dir.is_null() || destination_dir.is_null() {
        return std::ptr::null_mut();
    }
    let sdir = match CStr::from_ptr(source_dir).to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let ddir = match CStr::from_ptr(destination_dir).to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let res = destination_directory(sdir, ddir);
    match CString::new(res.to_string_lossy().into_owned()) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Reads the first N bytes from a file.
/// It is equivalent to head -c limit *nix utility.
///
/// Example:
/// * *
/// int main()
/// {
///   size_t length;
///   uint8_t* res = fs_utils_head("path", 10, &length);
///   if (res == NULL)
///   {
///       printf("Something went wrong");
///       return -1;
///   }
///   for (size_t i = 0; i < length; i++)
///   {
///      printf("%c", res[i]);
///   }
///   fs_utils_free_array(res, length);
///   return 0;
/// }
/// * *
///
/// @param path
/// @param limit
/// @param length
/// @return array
#[no_mangle]
unsafe extern "C" fn fs_utils_head(
    path: *const c_char,
    limit: usize,
    length: *mut usize,
) -> *mut u8 {
    if path.is_null() {
        return std::ptr::null_mut();
    }
    let str = match CStr::from_ptr(path).to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    match head(str, limit) {
        Ok(mut v) => {
            v.shrink_to_fit();
            let ptr: *mut u8 = v.as_mut_ptr();
            *length = v.len();
            std::mem::forget(v);
            ptr
        }
        Err(_) => return std::ptr::null_mut(),
    }
}

/// Reads the first N bytes from a file and return them as a string.
/// It assumes that the file is encoded with UTF-8, so any invalid UTF-8
/// sequences will be replaced with U+FFFD REPLACEMENT CHARACTER, which looks like this: �.
///
/// Example:
/// * *
/// int main()
/// {
///   char* res = fs_utils_head_to_string("path", 10);
///   if (res == NULL)
///   {
///       printf("Something went wrong");
///       return -1;
///   }
///   printf("%s", res);
///   fs_utils_free(res);
///   return 0;
/// }
/// * *
///
/// @param path
/// @param limit
/// @return dynamic string
#[no_mangle]
unsafe extern "C" fn fs_utils_head_to_string(path: *const c_char, limit: usize) -> *mut c_char {
    if path.is_null() {
        return std::ptr::null_mut();
    }
    let str = match CStr::from_ptr(path).to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let res = match head_to_string(str, limit) {
        Ok(v) => v,
        Err(_) => return std::ptr::null_mut(),
    };
    match CString::new(res) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Reads the first N bytes from a file and return them as a string.
/// If the file size is greater than N bytes, the truncation message will be put at the end of the String.
/// It assumes that the file is encoded with UTF-8, so any invalid UTF-8
/// sequences will be replaced with U+FFFD REPLACEMENT CHARACTER, which looks like this: �.
///
/// Example:
/// * *
/// int main()
/// {
///   char* res = fs_utils_head_to_string_with_message("path", 10, "Error");
///   if (res == NULL)
///   {
///       printf("Something went wrong");
///       return -1;
///   }
///   printf("%s", res);
///   fs_utils_free(res);
///   return 0;
/// }
/// * *
///
/// @param path
/// @param limit
/// @param truncation_message
/// @return dynamic string
#[no_mangle]
unsafe extern "C" fn fs_utils_head_to_string_with_message(
    path: *const c_char,
    limit: usize,
    truncation_message: *const c_char,
) -> *mut c_char {
    if path.is_null() || truncation_message.is_null() {
        return std::ptr::null_mut();
    }
    let str = match CStr::from_ptr(path).to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let msg = match CStr::from_ptr(truncation_message).to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let res = match head_to_string_with_message(str, limit, msg) {
        Ok(v) => v,
        Err(_) => return std::ptr::null_mut(),
    };
    match CString::new(res) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Cleans up the contents (files and folders) of the given folder while keeping the folder itself.
/// It is useful if you don't want to loose the permissions set on the folder
/// or if you only have enough permissions to manipulate with the contents of the given folder
/// but not the folder itself.
///
/// Example:
/// * *
/// int main()
/// {
///   if (fs_utils_cleanup_folder("folder_path") != 0)
///   {
///       printf("Something went wrong");
///       return -1;
///   }
///   return 0;
/// }
/// * *
///
/// @param folder_path
/// @return 0 on success and non zero value on failure
#[no_mangle]
unsafe extern "C" fn fs_utils_cleanup_folder(folder_path: *const c_char) -> c_int {
    if folder_path.is_null() {
        return -1;
    }
    let str = match CStr::from_ptr(folder_path).to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };
    match cleanup_folder(str) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

/// function to free the memory after using fs_utils functions
///
/// @param ptr string returned from fs_utils functions
#[no_mangle]
unsafe extern "C" fn fs_utils_free(ptr: *mut c_char) {
  if !ptr.is_null() {
    _ = CString::from_raw(ptr);
  }
}

/// function to free the memory after using fs_utils functions
///
/// @param ptr array returned from fs_utils functions
/// @param length array length
#[no_mangle]
unsafe extern "C" fn fs_utils_free_array(ptr: *mut u8, length: usize) {
  if !ptr.is_null() {
    _ = Vec::from_raw_parts(ptr, length, length);
  }
}

/// function to free the memory after using fs_utils functions
///
/// @param ptr pointer to fs_utils_t
#[no_mangle]
unsafe extern "C" fn fs_utils_clean(ptr: *mut fs_utils_t) {
  if !ptr.is_null() {
    if !(*ptr).buffer.is_null() {
      _ = CString::from_raw((*ptr).buffer);
    }
  }
}