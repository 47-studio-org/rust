//@ build-pass (FIXME(62277): could be check-pass?)

use std::ops::*;

#[derive(Copy, Clone)]
struct R(RangeToInclusive<usize>);


fn main() {}
