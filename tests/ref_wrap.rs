mod for_test;

pub use for_test::*;

use core::cell::RefCell;
use core::ops::{Deref, DerefMut};
use drop_tracer::DropTracer;
use ref_wrapper::RefWrap;

#[test]
fn deref() {
    let src = RefCell::new(samples());
    let target = RefWrap::new(src.borrow(), |x| VecStat::new(x));

    let result = target.deref();

    assert_eq!(*result.base(), samples());
    assert_eq!(result.summary(), samples().iter().sum::<i32>());
    assert!(src.try_borrow().is_ok());
    assert!(src.try_borrow_mut().is_err());
}

#[test]
fn deref_mut() {
    let src = RefCell::new(samples());
    let mut target = RefWrap::new(src.borrow(), |x| VecStat::new(x));

    let result = target.deref_mut();

    result.more(42);
    assert_eq!(result.summary(), samples().iter().sum::<i32>() + 42);
    assert!(src.try_borrow().is_ok());
    assert!(src.try_borrow_mut().is_err());
}

#[test]
fn test_drop_count() {
    DropTracer::test_drop(|tracer| {
        let src = RefCell::new(tracer.new_item());
        RefWrap::new(src.borrow(), |_| "dummy");
    });
}

pub struct VecStat<'a> {
    base: &'a Vec<i32>,
    more: i32,
}

impl<'a> VecStat<'a> {
    pub fn new(base: &'a Vec<i32>) -> Self {
        Self { base, more: 0 }
    }

    pub fn base(&'a self) -> &'a Vec<i32> {
        self.base
    }

    pub fn more(&mut self, value: i32) {
        self.more = value
    }

    pub fn summary(&self) -> i32 {
        self.base.iter().sum::<i32>() + self.more
    }
}
