//! Provider of [`RefWrapMut`].

use crate::reset_mut_lifetime;
use std::any::Any;
use std::cell::RefMut;
use std::ops::{Deref, DerefMut};

/// Wrapper of [`RefMut`].
pub struct RefWrapMut<'a, T>
where
    T: 'a + ?Sized,
{
    _base: RefMut<'a, dyn Any>,
    value: Box<T>,
}

impl<'a, T> RefWrapMut<'a, T>
where
    T: 'a + ?Sized,
{
    /// Create a new value.
    pub fn new<S, F>(mut base: RefMut<'a, S>, f: F) -> Self
    where
        S: Any,
        F: FnOnce(&'a mut S) -> Box<T>,
    {
        let base_ref = unsafe { reset_mut_lifetime(&mut base) };
        Self {
            _base: base,
            value: f(base_ref),
        }
    }
}

impl<T> Deref for RefWrapMut<'_, T>
where
    T: ?Sized,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for RefWrapMut<'_, T>
where
    T: ?Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
