// Copyright 2017 Serde Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate serde2_derive;

mod remote {
    pub struct S {
        a: u8,
    }

    impl S {
        pub fn get(&self) -> u16 {
            self.a as u16
        }
    }
}

#[derive(Serialize)]
#[serde(remote = "remote::S")]
struct S {
    #[serde(getter = "remote::S::get")]
    //~^^^^ ERROR: mismatched types
    a: u8,
    //~^^^^^^ expected u8, found u16
}

fn main() {}
