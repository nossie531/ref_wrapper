mod common;

pub use common::*;
use core::cell::RefCell;
use ref_wrapper::{RefWrap, RefWrapMut};

#[test]
fn for_ref_wrap() {
    let src = RefCell::new(samples());

    let result = RefWrap::new(src.borrow(), |x| x.iter());

    assert!(result.eq(samples().iter()));
}

#[test]
fn for_ref_wrap_mut() {
    let src = RefCell::new(samples());

    let result = RefWrapMut::new(src.borrow_mut(), |x| x.iter_mut());

    result.for_each(|x| *x += 1);
    let edited = Vec::from_iter(src.borrow().iter().cloned());
    let expected = Vec::from_iter(samples().iter().map(|x| x + 1));
    assert_eq!(edited, expected);
}
