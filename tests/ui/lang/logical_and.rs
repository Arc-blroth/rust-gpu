// Test using `&&` operator.

// build-pass
#![no_std]
#![feature(register_attr)]
#![register_attr(spirv)]

extern crate spirv_std;

fn f(x: bool, y: bool) -> bool {
    x && y
}

#[spirv(fragment)]
pub fn main() {
    f(false, true);
}
