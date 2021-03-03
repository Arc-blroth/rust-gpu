// Test that using push constants work.
// NOTE(eddyb) we specifically run Vulkan validation here, as the default
// validation rules are more lax and don't require a `Block` decoration
// (`#[spirv(block)]` here) on `struct ShaderConstants`.

// build-pass
// compile-flags: -Ctarget-feature=+vulkan
#![no_std]
#![feature(register_attr)]
#![register_attr(spirv)]

extern crate spirv_std;
use spirv_std::storage_class::PushConstant;

#[derive(Copy, Clone)]
#[spirv(block)]
pub struct ShaderConstants {
    pub width: u32,
    pub height: u32,
    pub time: f32,
}

#[spirv(fragment)]
pub fn main(constants: PushConstant<ShaderConstants>) {
    let _constants = *constants;
}
