// Test `OpVectorExtractDynamic`

// build-pass
#![no_std]
#![feature(register_attr)]
#![register_attr(spirv)]

use spirv_std::{arch, glam};

#[spirv(fragment)]
pub fn main() {
    let vector = glam::Vec2::new(1.0, 2.0);
    let element = unsafe { arch::vector_extract_dynamic(vector, 1) };
    assert!(2.0 == element);
}
