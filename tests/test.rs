use for_test::*;
use ref_wrapper::{RefIter, RefIterMut, RefWrap, RefWrapMut};
use std::cell::RefCell;

#[test]
fn ref_wrap() {
    with_deref();
    with_deref_mut();
    with_borrow_shared();
    with_borrow_collision();

    fn with_deref() {
        let src = RefCell::new(samples());
        let result = RefWrap::new(src.borrow(), |x| Box::new(summary(x)));
        assert_eq!(*result, summary(&samples()));
    }

    fn with_deref_mut() {
        let src = RefCell::new(samples());
        let mut result = RefWrap::new(src.borrow(), |x| Box::new(summary(x)));
        *result += 1;
        assert_eq!(*result, summary(&samples()) + 1);
    }

    fn with_borrow_shared() {
        let src = RefCell::new(samples());
        let _keep_borrow = RefWrap::new(src.borrow(), |_| Box::new(()));
        assert!(src.try_borrow().is_ok());
    }

    fn with_borrow_collision() {
        let src = RefCell::new(samples());
        let _keep_borrow = RefWrap::new(src.borrow(), |_| Box::new(()));
        assert!(src.try_borrow_mut().is_err());
    }
}

#[test]
fn ref_wrap_mut() {
    with_deref();
    with_deref_mut();
    with_borrow_collision();

    fn with_deref() {
        let src = RefCell::new(samples());
        let result = RefWrapMut::new(src.borrow_mut(), |x| Box::new(VecEditer::new(x)));
        assert_eq!(result.summary(), summary(&samples()));
    }

    fn with_deref_mut() {
        let src = RefCell::new(samples());
        let mut result = RefWrapMut::new(src.borrow_mut(), |x| Box::new(VecEditer::new(x)));
        result.edit(increment_all);
        drop(result);
        let edited = src.borrow().clone();
        let expected = edit(&mut samples(), increment_all).clone();
        assert_eq!(edited, expected)
    }

    fn with_borrow_collision() {
        let src = RefCell::new(samples());
        let _keep_borrow = RefWrapMut::new(src.borrow_mut(), |_| Box::new(()));
        assert!(src.try_borrow_mut().is_err());
    }
}

#[test]
fn ref_iter() {
    with_normal();
    with_methods_on_self_sized();

    fn with_normal() {
        let src = RefCell::new(samples());
        let iter = RefIter::new(src.borrow(), |x| Box::new(x.iter()));
        assert!(iter.eq(samples().iter()));
    }

    fn with_methods_on_self_sized() {
        let src = RefCell::new(samples());
        let iter = RefIter::new(src.borrow(), |x| Box::new(x.iter()));
        assert_eq!(iter.count(), samples().len());
    }
}

#[test]
fn ref_iter_mut() {
    with_normal();
    with_methods_on_self_sized();

    fn with_normal() {
        let src = RefCell::new(samples());
        let iter = RefIterMut::new(src.borrow_mut(), |x| Box::new(x.iter_mut()));
        iter.for_each(increment);
        let edited = src.borrow().iter().cloned().collect::<Vec<_>>();
        let expected = edit(&mut samples(), increment_all).clone();
        assert_eq!(edited, expected);
    }

    fn with_methods_on_self_sized() {
        let src = RefCell::new(samples());
        let iter = RefIterMut::new(src.borrow_mut(), |x| Box::new(x.iter_mut()));
        assert_eq!(iter.count(), samples().len());
    }
}

mod for_test {
    pub fn samples() -> Vec<i32> {
        vec![1, 2, 3]
    }

    pub fn summary(vec: &Vec<i32>) -> i32 {
        vec.iter().sum::<i32>()
    }

    pub fn increment(x: &mut i32) {
        *x += 1;
    }

    pub fn increment_all(vec: &mut Vec<i32>) {
        vec.iter_mut().for_each(increment);
    }

    pub fn edit<F>(target: &mut Vec<i32>, f: F) -> &mut Vec<i32>
    where
        F: FnOnce(&mut Vec<i32>),
    {
        f(target);
        target
    }

    pub struct VecEditer<'a> {
        base: &'a mut Vec<i32>,
    }

    impl<'a> VecEditer<'a> {
        pub fn new(base: &'a mut Vec<i32>) -> Self {
            Self { base }
        }

        pub fn summary(&self) -> i32 {
            summary(self.base)
        }

        pub fn edit<F>(&mut self, f: F)
        where
            F: FnOnce(&mut Vec<i32>),
        {
            edit(&mut self.base, f);
        }
    }
}
