ref_wrapper
===

Wrapper of dynamically borrowed data.

*The author of this crate is not good at English.*  
*Forgive me if the document is hard to read.*

This crate have mainly below items.

|Name         | Summary                           |
|-------------|-----------------------------------|
|`RefWrap`    | Wrapper of `Ref`.                 |
|`RefWrapMut` | Wrapper of `RefMut`.              |
|`RefIter`    | Iterator version of `RefWrap`.    |
|`RefIterMut` | Iterator version of `RefWrapMut`. |

## Examples

Normal use case.

```rust
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

```rust
let src = RefCell::new(vec![1, 2, 3]);
let iter = RefIter::new(src.borrow(), |x| Box::new(x.iter()));
assert_eq!(iter.sum::<i32>(), 6);
```

## Frequent careless mistakes

When using synonyms for iterators, be careful not to specify the wrong type parameter.
For example, the result type of `Vec<T>::iter(&self)` method is `Iter<'a, T>`, which
when wrapped becomes `RefIter<'a, &T>`. Note the change from `T` to `&T`. The reason
for this is that the type `Iter<'a, T>` implements the trait `Iterator<Item = &'a T>`,
and the wrapping target is latter. Although somewhat cumbersome, this allows for the
case where an Iterator's Item is not a reference.

## What's new.

Ver 0.1.2

* Some internal function is inlined.
