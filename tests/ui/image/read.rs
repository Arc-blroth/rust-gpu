// Test `OpImageRead`

// build-pass
#![no_std]
#![feature(register_attr)]
#![register_attr(spirv)]

use spirv_std::{arch, glam, storage_class::{Output, UniformConstant}, StorageImage2d};

#[spirv(fragment)]
pub fn main(image: UniformConstant<StorageImage2d>, mut output: Output<glam::Vec2>) {
    let coords = image.read(glam::IVec2::new(0, 1));
    *output = coords;
}
