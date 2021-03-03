// Simple single entrypoint function test.

// build-pass

#![no_std]
#![feature(register_attr)]
#![register_attr(spirv)]

use spirv_std as _;

#[spirv(fragment)]
pub fn main() {
}
