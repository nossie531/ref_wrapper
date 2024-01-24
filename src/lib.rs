/*! Wrapper of dynamically borrowed data.

*The author of this crate is not good at English.*
*Forgive me if the document is hard to read.*

This crate provide wrappers for value generated from dynamic borrowing types.

| Wrapper        | Target             |
|----------------|--------------------|
| [`RefWrap`]    | [`Ref`][Ref]       |
| [`RefWrapMut`] | [`RefMut`][RefMut] |

For example, this makes it possible to generate iterators from dynamicaly
borrowed values. Note that the original borrowing is consumed and taken into
the wrapper type. Therefore, when the wrapper is dropped, the original
borrowing is also dropped.

[Ref]: core::cell::Ref
[RefMut]: core::cell::RefMut

# Examples

Normal use case.

```
# use ref_wrapper::{RefWrap};
# use core::cell::{Ref, RefCell};
# use core::ops::Deref;
#
let src = RefCell::new(vec![1, 2, 3]);
let target = RefWrap::new(src.borrow(), |x| VecStat(x));
assert_eq!(target.summary(), 6);

pub struct VecStat<'a>(&'a Vec<i32>);
impl<'a> VecStat<'a> {
    pub fn summary(&self) -> i32 {
        self.0.iter().sum::<i32>()
    }
}
```

Iterator use case.

```
# use ref_wrapper::RefWrap;
# use core::cell::{Ref, RefCell};
#
let src = RefCell::new(vec![1, 2, 3]);
let iter = RefWrap::new(src.borrow(), |x| x.iter());
assert_eq!(iter.sum::<i32>(), 6);
```
*/

#![no_std]

mod impl_iterator;
mod ref_wrap;
mod ref_wrap_mut;
mod reset_lifetime;

pub use ref_wrap::*;
pub use ref_wrap_mut::*;
use reset_lifetime::*;
