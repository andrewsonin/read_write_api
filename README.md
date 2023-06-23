Provides an interface to unify single-threaded code and RwLocks-based code.

A nice side effect of this trait is manifested in the activation
of the borrow-checker to check the absence of deadlocks introduced in the current scope.
This is because `Self::write` can now only be called on a mutable,
but not constant, reference.

# Example
```rust
use read_write_api::{ReadWriteApi, TrivialReadWriteApi};
use parking_lot::RwLock;

struct A(u64);

fn do_something(mut x: impl ReadWriteApi<Target=A>) -> u64 {
    if x.read().0 == 1 {
        x.write().0 = 2;
        x.read().0
    } else {
        x.read().0
    }
}

impl TrivialReadWriteApi for A {}

assert_eq!(do_something(A(1)), 2);
assert_eq!(do_something(A(3)), 3);
assert_eq!(do_something(&RwLock::new(A(1))), 2);
assert_eq!(do_something(&RwLock::new(A(3))), 3);
assert_eq!(do_something(RwLock::new(A(1))), 2);
assert_eq!(do_something(RwLock::new(A(3))), 3);

fn do_something_ref<'a>(mut x: impl ReadWriteApi<Target=&'a mut A>) -> u64 {
    if x.read().0 == 1 {
        x.write().0 = 2;
        x.read().0
    } else {
        x.read().0
    }
}

impl TrivialReadWriteApi for &mut A {}

assert_eq!(do_something_ref(&mut A(1)), 2);
assert_eq!(do_something_ref(&mut A(3)), 3);
assert_eq!(do_something_ref(&RwLock::new(&mut A(1))), 2);
assert_eq!(do_something_ref(&RwLock::new(&mut A(3))), 3);
assert_eq!(do_something_ref(RwLock::new(&mut A(1))), 2);
assert_eq!(do_something_ref(RwLock::new(&mut A(3))), 3)
```