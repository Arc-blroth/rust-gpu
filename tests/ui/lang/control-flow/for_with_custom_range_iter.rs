// HACK(eddyb) test that `for` loop desugaring (with its call to `Iterator::next`
// and matching on the resulting `Option`) works, without a working `Range`
// iterator (due to the use of `mem::swap` and its block-wise implementation).

// build-pass
#![no_std]
#![feature(register_attr)]
#![register_attr(spirv)]

use spirv_std::{num_traits::Num, storage_class::Input};
use core::ops::Range;

struct RangeIter<T>(Range<T>);

impl<T: Num + Ord + Copy> Iterator for RangeIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let x = self.0.start;
        if x >= self.0.end {
            None
        } else {
            self.0.start = x + T::one();
            Some(x)
        }
    }
}

#[spirv(fragment)]
pub fn main(i: Input<i32>) {
    for _ in RangeIter(0..*i) {
    }
}

