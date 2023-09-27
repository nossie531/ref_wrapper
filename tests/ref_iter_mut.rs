mod common;

pub use common::*;
use ref_wrapper::RefIterMut;
use std::cell::RefCell;

#[test]
fn with_normal() {
    let src = RefCell::new(samples());

    let result = RefIterMut::new(src.borrow_mut(), |x| Box::new(x.iter_mut()));

    result.for_each(|x| *x += 1);
    let edited = Vec::from_iter(src.borrow().iter().cloned());
    let expected = Vec::from_iter(samples().iter().map(|x| x + 1));
    assert_eq!(edited, expected);
}
