Provides an interface to unify single-threaded code and RwLocks-based code.

A nice side effect of this trait is manifested in the activation
of the borrow-checker to check the absence of deadlocks introduced in the current scope.
This is because `Self::write` can now only be called on a mutable,
but not constant, reference.

# Example

```rust
use parking_lot::RwLock;
use read_write_api::{RwApi, RwApiWrapper, RwApiWrapperOwned};

fn do_something(mut x: impl RwApi<Target=u64>) -> u64 {
    if *x.read() == 1 {
        *x.write() = 2;
        *x.read()
    } else {
        *x.read()
    }
}

assert_eq!(do_something(RwApiWrapperOwned(1)), 2);
assert_eq!(do_something(RwApiWrapperOwned(3)), 3);
assert_eq!(do_something(&mut RwApiWrapperOwned(1)), 2);
assert_eq!(do_something(&mut RwApiWrapperOwned(3)), 3);

assert_eq!(do_something(RwLock::new(1)), 2);
assert_eq!(do_something(RwLock::new(3)), 3);
assert_eq!(do_something(&RwLock::new(1)), 2);
assert_eq!(do_something(&RwLock::new(3)), 3);
assert_eq!(do_something(&mut RwLock::new(1)), 2);
assert_eq!(do_something(&mut RwLock::new(3)), 3);

fn do_something_ref<'a>(mut x: impl RwApi<Target=&'a mut u64>) -> u64 {
    if **x.read() == 1 {
        **x.write() = 2;
        **x.read()
    } else {
        **x.read()
    }
}

assert_eq!(do_something_ref(RwApiWrapper(&mut 1)), 2);
assert_eq!(do_something_ref(RwApiWrapper(&mut 3)), 3);
assert_eq!(do_something_ref(&mut RwApiWrapper(&mut 1)), 2);
assert_eq!(do_something_ref(&mut RwApiWrapper(&mut 3)), 3);

assert_eq!(do_something_ref(RwLock::new(&mut 1)), 2);
assert_eq!(do_something_ref(RwLock::new(&mut 3)), 3);
assert_eq!(do_something_ref(&RwLock::new(&mut 1)), 2);
assert_eq!(do_something_ref(&RwLock::new(&mut 3)), 3);
assert_eq!(do_something_ref(&mut RwLock::new(&mut 1)), 2);
assert_eq!(do_something_ref(&mut RwLock::new(&mut 3)), 3);
```