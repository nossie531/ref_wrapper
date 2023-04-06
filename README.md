ref_wrapper
===

Wrapper of dynamically borrowed data.

*The author of this crate is not good at English.*  
*Forgive me if the document is hard to read.*

This crate have mainly below items.

|Name         | Summary                                                       |
|-------------|---------------------------------------------------------------|
|`RefWrap`    | Wrapper of `Ref` which can read content from base reference.  |
|`RefWrapMut` | Wrapper of `RefMut` which can read/write from base reference. |
|`RefIter`    | Type synonim (iterator version of `Ref`).                     |
|`RefIterMut` | Type synonim (iterator version of `RefMut`).                  |

## Examples

Normal use case.

```rust
let vec = RefCell::new(vec![1, 2, 3]);
let sum = extract_summary(vec.borrow());
assert_eq!(*sum, 6);

fn extract_summary(src: Ref<Vec<i32>>) -> RefWrap<i32> {
    RefWrap::new(src, |x| Box::new(x.iter().sum::<i32>()))
}
```

Special use case for iterator.

```rust
let vec = RefCell::new(vec![1, 2, 3]);
let mut iter = extract_iter(vec.borrow());
assert_eq!(*iter.next().unwrap(), 1);
assert_eq!(*iter.next().unwrap(), 2);
assert_eq!(*iter.next().unwrap(), 3);

fn extract_iter(src: Ref<Vec<i32>>) -> RefIter<&i32> {
    RefIter::new(src, |x| Box::new(x.iter()))
}
```

## Frequent careless mistakes.

When using synonyms for iterators, be careful not to specify the wrong type parameter.
For example, the result type of `Vec<T>::iter(&self)` method is `Iter<'a, T>`, which
when wrapped becomes `RefIter<'a, &T>`. Note the change from `T` to `&T`. The reason
for this is that the type `Iter<'a, T>` implements the trait `Iterator<Item = &'a T>`,
and the wrapping target is latter. Although somewhat cumbersome, this allows for the
case where an Iterator's Item is not a reference.
