//! Provide functions for reset reference lifetimes.

use core::any::Any;
use core::ops::{Deref, DerefMut};

/// Reset immutable reference lifetime.
///
/// # Safety
///
/// Be careful when handling the return value. Access to a referenced
/// target over its lifetime will result in undefined behavior.
#[inline(always)]
pub unsafe fn reset_ref_lifetime<'a, S, T>(src: &S) -> &'a T
where
    S: Deref<Target = T>,
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
pub unsafe fn reset_mut_lifetime<'a, S, T>(src: &mut S) -> &'a mut T
where
    S: DerefMut<Target = T>,
    T: Any,
{
    let src_ptr = src.deref_mut() as *mut T;
    unsafe { &mut *src_ptr }
}
