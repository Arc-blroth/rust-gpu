// Test that calling `panic!` works.

// build-pass
#![no_std]
#![feature(register_attr)]
#![register_attr(spirv)]

extern crate spirv_std;

#[spirv(fragment)]
pub fn main() {
    panic!("aaa");
}
