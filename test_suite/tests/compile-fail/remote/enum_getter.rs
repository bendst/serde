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
    pub enum E {
        A { a: u8 },
    }
}

#[derive(Serialize)]
#[serde(remote = "remote::E")]
pub enum E {
    A {
        #[serde(getter = "get_a")]
        //~^^^^^ ERROR: #[serde(getter = "...")] is not allowed in an enum
        a: u8,
    },
}

fn main() {}
