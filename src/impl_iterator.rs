//! [`Iterator`] implementations for [`RefWrap`] and [`RefWrapMut`].

use crate::{RefWrap, RefWrapMut};
use core::ops::DerefMut;

impl<'a, I, T> Iterator for RefWrap<'a, I>
where
    I: Iterator<Item = T> + 'a,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.deref_mut().next()
    }
}

impl<'a, I, T> Iterator for RefWrapMut<'a, I>
where
    I: Iterator<Item = T> + 'a,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.deref_mut().next()
    }
}
