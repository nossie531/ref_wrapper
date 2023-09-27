/*! Wrapper of dynamically borrowed data.

*The author of this crate is not good at English.*
*Forgive me if the document is hard to read.*

This Allows access to dynamically borrowed data ([`Ref`][Ref] or [`RefMut`][RefMut])
with a given wrapper type. For example, this makes it possible to generate iterators
for handling the borrowed data. Note that the original borrowing is consumed and
taken into the wrapper type. Therefore, when the wrapper is dropped, the original
borrowing is also dropped.

[Ref]: std::cell::Ref
[RefMut]: std::cell::RefMut

# Examples

Normal use case.

```
# use ref_wrapper::{RefWrap};
# use std::cell::{Ref, RefCell};
# use std::ops::Deref;
#
let src = RefCell::new(vec![1, 2, 3]);
let target = RefWrap::new(src.borrow(), |x| Box::new(VecStat(x)));
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
# use ref_wrapper::{RefIter, RefWrap};
# use std::cell::{Ref, RefCell};
#
let src = RefCell::new(vec![1, 2, 3]);
let iter = RefIter::new(src.borrow(), |x| Box::new(x.iter()));
assert_eq!(iter.sum::<i32>(), 6);
```
*/

mod ref_iter;
mod ref_iter_mut;
mod ref_wrap;
mod ref_wrap_mut;
mod reset_lifetime;

pub use ref_iter::RefIter;
pub use ref_iter_mut::RefIterMut;
pub use ref_wrap::RefWrap;
pub use ref_wrap_mut::RefWrapMut;

use reset_lifetime::reset_mut_lifetime;
use reset_lifetime::reset_ref_lifetime;
