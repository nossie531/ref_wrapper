//! Provider of [`RefWrapMut`].

use crate::reset_mut_lifetime;
use core::any::Any;
use core::cell::RefMut;
use core::ops::{Deref, DerefMut};

/// Wrapper of [`RefMut`].
pub struct RefWrapMut<'a, T> {
    _src: RefMut<'a, dyn Any>,
    value: T,
}

impl<'a, T> RefWrapMut<'a, T>
where
    T: 'a,
{
    /// Create a new value.
    pub fn new<S, F>(mut src: RefMut<'a, S>, f: F) -> Self
    where
        S: Any,
        F: FnOnce(&'a mut S) -> T,
    {
        let src_ref = unsafe { reset_mut_lifetime(&mut src) };
        Self {
            _src: src,
            value: f(src_ref),
        }
    }
}

impl<T> Deref for RefWrapMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for RefWrapMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
