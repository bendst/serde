// Copyright 2017 Serde Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate serde2_derive;

#[derive(Serialize)]
#[serde(abc = "xyz")]
//~^^ ERROR: unknown serde container attribute `abc`
struct A {
    x: u32,
}

fn main() {}
