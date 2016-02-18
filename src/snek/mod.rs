//////////////////////////////////////////////////////////////////////////////
//  File: rust-snek/snek/mod.rs
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

extern crate libc;

use ::{Error, Symbol};

use std::path::Path;
use libc::c_void;

#[cfg(unix)]
pub use self::unix::{load_library, load_symbol, drop_library};

#[cfg(windows)]
pub use self::windows::{load_library, load_symbol, drop_library};

mod unix;
mod windows;

/// This provides an interface for manually loading a dynamic library and
/// symbols from it. While this exists, it is more recommended to use the 
/// [`snek!`](macro.snek!.html) macro to generate a wrapper for a library 
/// automatically.
///
/// A `Snek` instance contains nothing but a handle to the loaded library,
/// and provides a single method for loading symbols from the library. When
/// the instance is dropped, it unloads the library, so the lifetime of
/// any loaded symbols is tied to the lifetime of the `Snek` instance.
///
/// For more information about using the loaded symbols see the 
/// [`Symbol`](struct.Symbol.html)  documentation.
///
/// # Example
/// ```
/// # extern crate libc;
/// # extern crate snek;
/// # use snek::Snek;
/// # use libc::c_int;
/// # fn main() {
/// match Snek::load("libexample.so") {
///     Ok(snek) => match snek.symbol("add") {
///         Ok(symbol) => println!("{}", unsafe { symbol.with(
///             |add: extern fn(c_int, c_int) -> c_int| add(3, 7)
///         ) }),
///
///         _ => ()
///     },
///
///     _ => ()
/// }
/// # }
/// ```
#[derive(Debug)]
pub struct Snek {
    handle: *mut c_void
}

impl Snek {
    /// Attempt to load a dynamic library from the given path, returning a `Snek`
    /// instance wrapping the handle. 
    ///
    /// If the load fails, this will return [`Error::LibraryLoadError`](enum.Error.html)
    pub fn load<P>(path: P) -> Result<Snek, Error> where P: AsRef<Path> {
        load_library(path).map(|result| Snek { handle: result })
    }

    /// Attempt to load a symbol from the dynamic library, returning a 
    /// [`Symbol`](struct.Symbol.html) instance wrapping it.
    ///
    /// If the load fails, this will return [`Error::SymbolLoadError`](enum.Error.html)
    pub fn symbol<'a>(&'a self, symbol: &str) -> Result<Symbol<'a>, Error> {
        load_symbol(self.handle, symbol).map(|result| Symbol::new(result))
    }
}

impl Drop for Snek {
    fn drop(&mut self) {
        drop_library(self.handle)
    }
}

