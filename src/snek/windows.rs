//////////////////////////////////////////////////////////////////////////////
//  File: rust-snek/snek/windows.rs
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

#![cfg(windows)]

use ::Error;

use std::ptr;
use std::slice;
use std::path::Path;
use std::ffi::CString;
use libc::c_void;
use winapi::{self, HRESULT, DWORD, HMODULE};
use kernel32;

pub fn load_library<P>(path: P) -> Result<*mut c_void, Error> where P: AsRef<Path> {
    let path_string = CString::new(path.as_ref().to_string_lossy().as_ref()).unwrap();
    let module = unsafe { kernel32::LoadLibraryA(path_string.as_ptr()) };
    
    if module.is_null() {
        let error = last_error_string().unwrap_or_else(|| "Unknown Error".into());
        Err(Error::LibraryLoadError(error))
    } else {
        Ok(module as *mut c_void)
    }
}

pub fn load_symbol(handle: *mut c_void, symbol: &str) -> Result<*mut c_void, Error> {
    let module = handle as HMODULE;
    let string = CString::new(symbol).unwrap();
    let result = unsafe { kernel32::GetProcAddress(module, string.as_ptr()) };
    
    if result.is_null() {
        let error = last_error_string().unwrap_or_else(|| "Unknown Error".into());
        Err(Error::SymbolLoadError(error))
    } else {
        Ok(result as *mut c_void)
    }
}

pub fn drop_library(handle: *mut c_void) {
    unsafe { kernel32::FreeLibrary(handle as HMODULE) };
}

fn hresult_from_win32(win32: DWORD) -> HRESULT {
    if win32 as HRESULT <= 0 {
        win32 as HRESULT
    } else {
        ((win32 & 0x0000FFFF) | ((winapi::FACILITY_WIN32 as DWORD) << 16) | 0x80000000) as HRESULT
    }
}

fn hresult_to_string(hr: HRESULT) -> Option<String> {
    unsafe {
        let mut buffer: *mut u8 = ptr::null_mut();
        let num_chars = kernel32::FormatMessageA(
            winapi::FORMAT_MESSAGE_ALLOCATE_BUFFER |
            winapi::FORMAT_MESSAGE_FROM_SYSTEM |
            winapi::FORMAT_MESSAGE_IGNORE_INSERTS,
            ptr::null_mut(),
            hr as DWORD,
            0, // unknown lang-id, use default
            (&mut buffer) as *mut *mut u8 as *mut i8,
            0, // minimum buffer size
            ptr::null_mut(),
        );
        if num_chars == 0 {
            return None;
        }
        
        let bytes = slice::from_raw_parts(buffer, num_chars as usize);
        let message = String::from_utf8_lossy(bytes).into_owned();
        kernel32::LocalFree(buffer as *mut _);
        
        Some(message)
    }
}

fn last_error_hr() -> HRESULT {
    hresult_from_win32(unsafe { kernel32::GetLastError() })
}

fn last_error_string() -> Option<String> {
    hresult_to_string(last_error_hr())
}
