// Test that bounds checking causes panics.

// build-pass
#![no_std]
#![feature(register_attr)]
#![register_attr(spirv)]

extern crate spirv_std;

fn array_bounds_check(x: [u32; 4], i: usize) -> u32 {
    x[i]
}

#[spirv(fragment)]
pub fn main() {
    array_bounds_check([0, 1, 2, 3], 5);
}
