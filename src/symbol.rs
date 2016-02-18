//////////////////////////////////////////////////////////////////////////////
//  File: rust-snek/symbol.rs
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

use std::ptr;
use std::marker::PhantomData;
use libc::c_void;

/// This provides an interface around a symbol loaded from a
/// dynamic library. This should not be constructed manually,
/// but returned from [`Snek::symbol`](struct.Snek.html#method.symbol)
/// or used internally via the [`snek!`](macro.snek!.html) macro.
#[derive(Debug)]
pub struct Symbol<'a> {
    symbol: *mut c_void,

    _life: PhantomData<&'a c_void>
}

impl<'a> Symbol<'a> {
    /// Construct a new `Symbol` wrapping a symbol. This should not be used
    /// manually, however is public to allow access from the 
    /// [`snek!`](macro.snek!.html) macro.
    pub fn new(symbol: *mut c_void) -> Symbol<'a> {
        Symbol {
            symbol: symbol,

            _life: PhantomData
        }
    }

    /// Use the symbol as if it was a certain type. There is no way of checking
    /// that the symbol is of the specified type, so this function should be used
    /// with care.
    ///
    /// # Safety
    /// When calling this function, ensure the type of the symbol is actually the
    /// type you say it is.
    ///
    /// # Example
    /// ```
    /// # extern crate libc;
    /// # extern crate snek;
    /// # use snek::Snek;
    /// # use libc::c_int;
    /// # fn main() {
    /// # match Snek::load("libexample.so") {
    /// #    Ok(snek) => match snek.symbol("add") {
    /// #        Ok(symbol) => {
    /// let result: c_int =  unsafe { symbol.with(|add: extern fn(c_int, c_int) -> c_int| add(3, 7)) };
    /// #       },
    /// #       _ => ()
    /// #   },
    /// #   _ => ()
    /// # }
    /// # }
    pub unsafe fn with<F, T, U>(&self, f: F) -> U where F: Fn(T) -> U {
        let value = ptr::read(&self.symbol as *const _ as *const T);
        f(value)
    }
}


