//! Provider of [`RefWrap`].

use crate::reset_ref_lifetime;
use core::any::Any;
use core::cell::Ref;
use core::ops::{Deref, DerefMut};

/// Wrapper of [`Ref`].
pub struct RefWrap<'a, T> {
    _src: Ref<'a, dyn Any>,
    value: T,
}

impl<'a, T> RefWrap<'a, T> {
    /// Create a new value.
    pub fn new<S, F>(src: Ref<'a, S>, f: F) -> Self
    where
        S: Any,
        F: FnOnce(&'a S) -> T,
    {
        let src_ref = unsafe { reset_ref_lifetime(&src) };
        Self {
            _src: src,
            value: f(src_ref),
        }
    }
}

impl<T> Deref for RefWrap<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for RefWrap<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
