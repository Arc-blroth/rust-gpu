// Test creating unitialized memory.

// ignore
#![no_std]
#![feature(register_attr, asm, ptr_internals)]
#![register_attr(spirv)]

use spirv_std as _;

//use core::mem::MaybeUninit;
//const MAYBEI32: MaybeUninit<&i32> = MaybeUninit::<&i32>::uninit();
//
//pub fn create_uninit_and_write() {
//    unsafe {
//        asm! {
//            "OpCapability Addresses"
//        }
//    }
//
//    let mut maybei32 = MAYBEI32;
//    unsafe { maybei32.as_mut_ptr().write(&0); }
//    let _maybei32 = unsafe { maybei32.assume_init() };
//}

#[spirv(fragment)]
pub fn main() {}
