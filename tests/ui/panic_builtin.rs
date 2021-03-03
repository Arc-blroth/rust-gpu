// Test panics coming from the Rust language such as `1 / 0`.

// build-pass
#![no_std]
#![feature(register_attr)]
#![register_attr(spirv)]

extern crate spirv_std;

fn int_div(x: usize) -> usize {
    1 / x
}

#[spirv(fragment)]
pub fn main() {
    int_div(0);
}
