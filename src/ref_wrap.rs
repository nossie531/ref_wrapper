//! Provider of [`RefWrap`].

use crate::reset_ref_lifetime;
use std::any::Any;
use std::cell::Ref;
use std::ops::{Deref, DerefMut};

/// Wrapper of [`Ref`].
pub struct RefWrap<'a, T>
where
    T: 'a + ?Sized,
{
    _base: Ref<'a, dyn Any>,
    value: Box<T>,
}

impl<'a, T> RefWrap<'a, T>
where
    T: 'a + ?Sized,
{
    /// Create a new value.
    pub fn new<S, F>(base: Ref<'a, S>, f: F) -> Self
    where
        S: Any,
        F: FnOnce(&'a S) -> Box<T>,
    {
        let base_ref = unsafe { reset_ref_lifetime(&base) };
        Self {
            _base: base,
            value: f(base_ref),
        }
    }
}

impl<T> Deref for RefWrap<'_, T>
where
    T: ?Sized,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for RefWrap<'_, T>
where
    T: ?Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
