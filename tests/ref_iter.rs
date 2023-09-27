mod common;

pub use common::*;
use ref_wrapper::RefIter;
use std::cell::RefCell;

#[test]
fn with_normal() {
    let src = RefCell::new(samples());

    let result = RefIter::new(src.borrow(), |x| Box::new(x.iter()));

    assert!(result.eq(samples().iter()));
}
