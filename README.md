Provides an interface to unify single-threaded code and RwLocks-based code.

A nice side effect of this trait is manifested in the activation
of the borrow-checker to check the absence of deadlocks introduced in the current scope.
This is because `Self::write` can now only be called on a mutable,
but not constant, reference.

# Example

```rust
use parking_lot::RwLock;
use read_write_api::{ReadWriteApi, AutoReadApi, AutoWriteApi};

struct A(u64);

fn do_something(mut x: impl ReadWriteApi<u64>) -> u64 {
    if *x.read() == 1 {
        *x.write() = 2;
        *x.read()
    } else {
        *x.read()
    }
}

impl AutoReadApi for A {
    type Target = u64;

    #[inline]
    fn get(&self) -> &Self::Target {
        &self.0
    }
}

impl AutoWriteApi for A {
    type Target = u64;

    #[inline]
    fn get_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

assert_eq!(do_something(&mut A(1)), 2);
assert_eq!(do_something(&mut A(3)), 3);
assert_eq!(do_something(&RwLock::new(1)), 2);
assert_eq!(do_something(&RwLock::new(3)), 3);
assert_eq!(do_something(RwLock::new(1)), 2);
assert_eq!(do_something(RwLock::new(3)), 3);

struct B<'a>(&'a mut u64);

fn do_something_ref<'a>(mut x: impl ReadWriteApi<&'a mut u64>) -> u64 {
    if **x.read() == 1 {
        **x.write() = 2;
        **x.read()
    } else {
        **x.read()
    }
}

impl<'a> AutoReadApi for B<'a> {
    type Target = &'a mut u64;

    #[inline]
    fn get(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> AutoWriteApi for B<'a> {
    type Target = &'a mut u64;

    #[inline]
    fn get_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

assert_eq!(do_something_ref(&mut B(&mut 1)), 2);
assert_eq!(do_something_ref(&mut B(&mut 3)), 3);
assert_eq!(do_something_ref(&RwLock::new(&mut 1)), 2);
assert_eq!(do_something_ref(&RwLock::new(&mut 3)), 3);
assert_eq!(do_something_ref(RwLock::new(&mut 1)), 2);
assert_eq!(do_something_ref(RwLock::new(&mut 3)), 3)
```