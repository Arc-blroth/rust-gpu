// Tests using a vector like pointer at `const` time.

// build-pass
#![no_std]
#![feature(register_attr, ptr_internals)]
#![register_attr(spirv)]

use spirv_std as _;

use core::ptr::Unique;
const VEC_LIKE: (Unique<usize>, usize, usize) = (Unique::<usize>::dangling(), 0, 0);

pub fn assign_vec_like() {
    let _vec_like = VEC_LIKE;
}

#[spirv(fragment)]
pub fn main() {}
