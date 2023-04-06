//! Provider of [`RefIter`].

use crate::RefWrap;
use std::ops::DerefMut;

/// Iterator version of [`RefWrap`].
pub type RefIter<'a, T> = RefWrap<'a, dyn Iterator<Item = T>>;

impl<'a, T> Iterator for RefIter<'a, T>
where
    T: 'a,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.deref_mut().next()
    }
}
