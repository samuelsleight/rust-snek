//////////////////////////////////////////////////////////////////////////////
//  File: rust-snek/snek/unix.rs
//////////////////////////////////////////////////////////////////////////////
//  Copyright 2016 Samuel Sleight
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//////////////////////////////////////////////////////////////////////////////

#![cfg(unix)]

extern crate libc;

use ::Error;

use std::path::Path;
use std::ffi::{CStr, CString};
use libc::{c_char, c_int, c_void};

extern {
    fn dlopen(path: *mut c_char, mode: c_int) -> *mut c_void;
    fn dlclose(handle: *mut c_void);
    fn dlsym(handle: *mut c_void, symbol: *mut c_char) -> *mut c_void;
    fn dlerror() -> *mut c_char;
}

pub fn load_library<P>(path: P) -> Result<*mut c_void, Error> where P: AsRef<Path> {
    let path_string = CString::new(path.as_ref().to_string_lossy().as_ref()).unwrap();
    let result = unsafe { dlopen(path_string.into_raw() as *mut c_char, 1) };

    if result == (0 as *mut libc::c_void) {
        let error = unsafe { CStr::from_ptr(dlerror()).to_string_lossy().into_owned() };
        Err(Error::LibraryLoadError(error))
    } else {
        Ok(result)
    }
}

pub fn load_symbol(handle: *mut c_void, symbol: &str) -> Result<*mut c_void, Error> {
    let string = CString::new(symbol).unwrap();
    let result = unsafe { dlsym(handle, string.into_raw()) };

    if result == (0 as *mut libc::c_void) {
        let error = unsafe { CStr::from_ptr(dlerror()).to_string_lossy().into_owned() };
        Err(Error::SymbolLoadError(error))
    } else {
        Ok(result)
    }
}

pub fn drop_library(handle: *mut c_void) {
    unsafe { dlclose(handle) }
}
