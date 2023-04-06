//! Provider of [`RefIterMut`].

use crate::RefWrapMut;
use std::ops::DerefMut;

/// Iterator version of [`RefWrapMut`].
pub type RefIterMut<'a, T> = RefWrapMut<'a, dyn Iterator<Item = T>>;

impl<'a, T> Iterator for RefIterMut<'a, T>
where
    T: 'a,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.deref_mut().next()
    }
}
