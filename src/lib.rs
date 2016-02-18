//////////////////////////////////////////////////////////////////////////////
//  File: rust-snek/lib.rs
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

//! This library provides a convenient interface for loading dynamic libraries
//! at runtime, and for retrieving symbols from them. The recommended use is
//! via the [`snek!`](macro.snek!.html) macro, which defines a structure which
//! loads symbols on construction and has methods for each one, however it is
//! also possible to manually load libraries and symbols using a [`Snek`](struct.Snek.html)
//! instance.
//!
//! # Safety
//! There is no way of verifying the type of loaded symbols, so both methods of
//! using them assume that the given type is correct - this library should be used
//! very carefully. Also consider it unstable at the moment, I mostly have no idea
//! what I am doing.
//!
//! # Example
//! ```
//! #[macro_use] extern crate snek;
//! extern crate libc;
//!
//! use libc::c_int;
//!
//! snek! {
//!     Example {
//!         hello: () -> (),
//!         add: (x: c_int, y: c_int) -> c_int
//!     }
//! }
//!
//! fn main() {
//!     if let Ok(example) = Example::load("libexample.so") {
//!         example.hello();
//!         println!("2 + 4 = {}", example.add(2, 4));
//!     }
//! }

extern crate libc;

pub use snek::{Snek, load_library, load_symbol, drop_library};
pub use symbol::Symbol;

mod snek;
mod symbol;

/// This enum stores information about the error returned when loading a library
/// or symbol fails. On unix platforms, it hold the result of `dlerror()`.
#[derive(Debug)]
pub enum Error {
    LibraryLoadError(String),
    SymbolLoadError(String)
}

/// This macro is used to generate a struct that wraps a dynamic library with
/// generated loading code. Each defined function will be loaded as a symbol
/// from the library when an instance of the struct is constructed, and can be
/// called via functions of the same name attached to the struct.
///
/// As with [`Symbol::with`](struct.Symbol.html#method.with), there is no way
/// of verifying the types of the functions so care should be taken to ensure
/// they are correct.
///
/// In the same way as a [`Snek`](struct.Snek.html) instance, when an instance
/// of a struct defined by this macro is dropped, the library is unloaded.
///
/// # Example
/// This example loads the same function as given in the [`Snek`](struct.Snek.html)
/// usage example:
///
/// ```
/// # #[macro_use] extern crate snek;
/// # extern crate libc;
/// # use libc::c_int;
/// snek! {
///     Example {
///         add: (x: c_int, y: c_int) -> c_int
///     }
/// }
///
/// fn main() {
///     if let Ok(example) = Example::load("libexample.so") {
///         println!("{}", example.add(3, 7))
///     }
/// }
/// ```
///
/// Additional functions can be loaded by simply adding them in the macro usage:
///
/// ```
/// # #[macro_use] extern crate snek;
/// # extern crate libc;
/// # use libc::c_int;
/// snek! {
///     Example {
///         add: (x: c_int, y: c_int) -> c_int,
///         hello: () -> ()
///     }
/// }
/// # fn main () {}
/// ```
#[macro_export]
macro_rules! snek {
    ($sname:ident { 
        $($symbol:ident : ($($pn: ident : $pt:ty),*) -> $ot:ty),*
    }) => {
        pub struct $sname<'a> {
            handle: *mut libc::c_void,
            $($symbol: snek::Symbol<'a>),*
        }

        impl<'a> $sname<'a> {
            pub fn load<P>(path: P) -> Result<$sname<'a>, snek::Error> where P: AsRef<std::path::Path> {
                let handle = match snek::load_library(path) {
                    Ok(result) => result,
                    Err(err) => return Err(err)
                };

                Ok($sname {
                    handle: handle,
                    $($symbol: match snek::load_symbol(handle, stringify!($symbol)) {
                        Ok(result) => snek::Symbol::new(result),
                        Err(err) => return Err(err)
                    }),*
                })
            }

            $(pub fn $symbol(&self, $($pn: $pt),*) -> $ot {
                self.$symbol.with(|f: extern fn($($pt),*) -> $ot| f($($pn),*))
            })*
        }

        impl<'a> Drop for $sname<'a> {
            fn drop(&mut self) {
                snek::drop_library(self.handle)
            }
        }
    }
}
