// Copyright 2018 Serde Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate serde2_derive;

#[derive(Serialize)]
struct Foo {
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    //~^^^ ERROR: #[serde(flatten] can not be combined with #[serde(skip_serializing_if = "...")]
    other: Option<Other>,
}

#[derive(Serialize)]
struct Other {
    x: u32,
}

fn main() {}
