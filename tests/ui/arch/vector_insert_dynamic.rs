// Test `OpVectorInsertDynamic`

// build-pass
#![no_std]
#![feature(register_attr)]
#![register_attr(spirv)]

use spirv_std::{arch, glam};

#[spirv(fragment)]
pub fn main() {
    let vector = glam::Vec2::new(1.0, 2.0);
    let expected = glam::Vec2::new(1.0, 3.0);
    let new_vector = unsafe { arch::vector_insert_dynamic(vector, 1, 3.0) };
    assert!(new_vector == expected);
}
