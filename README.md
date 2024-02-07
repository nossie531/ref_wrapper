ref_wrapper
===

Wrapper of dynamically borrowed data.

*The author of this crate is not good at English.*  
*Forgive me if the document is hard to read.*

This crate provide wrappers for value generated from dynamic borrowing types.

| Wrapper     | Target   |
|-------------|----------|
|`RefWrap`    | `Ref`    |
|`RefWrapMut` | `RefMut` |

## Examples

Normal use case.

```rust
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

```rust
let src = RefCell::new(vec![1, 2, 3]);
let iter = RefWrap::new(src.borrow(), |x| x.iter());
assert_eq!(iter.sum::<i32>(), 6);
```

## What's new.

v0.2.1

* Minor refactoring.

v0.2.0

* Crate is now `no_std`.
* `new` method callback argument returns no longer need to wrapped in `Box`.
* `RefWrap` and `RefWrapMut` implement `Iterator` in certain cases.
* Instead of the above, `RefIter` and `RefIterMut` have been removed.

v0.1.3-0.1.5

* Minor refactoring.

v0.1.2

* Some internal function is inlined.
