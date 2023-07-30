//! Provide functions for reset reference lifetimes.

use std::any::Any;
use std::ops::{Deref, DerefMut};

/// Reset immutable reference lifetime.
///
/// # Safety
///
/// Be careful when handling the return value. Access to a referenced
/// target over its lifetime will result in undefined behavior.
#[inline(always)]
pub unsafe fn reset_ref_lifetime<'a, R, T>(src: &R) -> &'a T
where
    R: Deref<Target = T>,
    T: Any,
{
    let src_ptr = src.deref() as *const T;
    unsafe { &*src_ptr }
}

/// Reset mutable reference lifetime.
///
/// # Safety
///
/// Be careful when handling the return value. Access to a referenced
/// target over its lifetime will result in undefined behavior.
#[inline(always)]
pub unsafe fn reset_mut_lifetime<'a, R, T>(src: &mut R) -> &'a mut T
where
    R: DerefMut<Target = T>,
    T: Any,
{
    let src_ptr = src.deref_mut() as *mut T;
    unsafe { &mut *src_ptr }
}
