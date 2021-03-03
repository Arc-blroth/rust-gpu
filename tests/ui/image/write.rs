// Test `OpImageWrite`

// build-pass
#![no_std]
#![feature(register_attr)]
#![register_attr(spirv)]

use spirv_std::{arch, glam, storage_class::{Input, Output, UniformConstant}, StorageImage2d};

#[spirv(fragment)]
pub fn main(input: Input<glam::Vec2>, image: UniformConstant<StorageImage2d>) {
    let texels = *input;
    unsafe {
        image.write(glam::UVec2::new(0, 1), texels);
    }
}
