mod for_test;

pub use for_test::*;

use core::cell::RefCell;
use core::ops::{Deref, DerefMut};
use drop_tracer::DropTracer;
use ref_wrapper::RefWrapMut;

#[test]
fn deref() {
    let src = RefCell::new(samples());
    let target = RefWrapMut::new(src.borrow_mut(), |x| VecEdit::new(x));

    let result = target.deref();

    assert_eq!(*result.base(), samples());
    assert!(src.try_borrow().is_err());
    assert!(src.try_borrow_mut().is_err());
}

#[test]
fn deref_mut() {
    let src = RefCell::new(samples());
    let mut target = RefWrapMut::new(src.borrow_mut(), |x| VecEdit::new(x));

    let result = target.deref_mut();

    result.add(1);
    let expected_base = Vec::from_iter(samples().iter().map(|x| x + 1));
    assert_eq!(*target.base(), expected_base);
    assert!(src.try_borrow().is_err());
    assert!(src.try_borrow_mut().is_err());
}

#[test]
fn test_drop_count() {
    DropTracer::test_drop(|tracer| {
        let src = RefCell::new(tracer.new_item());
        RefWrapMut::new(src.borrow_mut(), |_| "dummy");
    });
}

pub struct VecEdit<'a> {
    base: &'a mut Vec<i32>,
}

impl<'a> VecEdit<'a> {
    pub fn new(base: &'a mut Vec<i32>) -> Self {
        Self { base }
    }

    pub fn base(&'a self) -> &'a Vec<i32> {
        self.base
    }

    pub fn add(&mut self, value: i32) {
        self.base.iter_mut().for_each(|x| *x += value);
    }
}
