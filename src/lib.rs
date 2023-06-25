use std::ops::{Deref, DerefMut};

pub use wrappers::{ReadApiWrapper, RwApiWrapper, RwApiWrapperOwned};

mod impls;
mod wrappers;

/// Provides an interface to unify single-threaded code and RwLocks-based code.
///
/// A nice side effect of this trait is manifested in the activation
/// of the borrow-checker to check the absence of deadlocks introduced in the current scope.
/// This is because `Self::write` can now only be called on a mutable,
/// but not constant, reference.
///
/// # Example
///
/// ```rust
/// use parking_lot::RwLock;
/// use read_write_api::{RwApi, RwApiWrapper, RwApiWrapperOwned};
///
/// fn do_something(mut x: impl RwApi<Target=u64>) -> u64 {
///     if *x.read() == 1 {
///         *x.write() = 2;
///         *x.read()
///     } else {
///         *x.read()
///     }
/// }
///
/// assert_eq!(do_something(RwApiWrapperOwned(1)), 2);
/// assert_eq!(do_something(RwApiWrapperOwned(3)), 3);
/// assert_eq!(do_something(&mut RwApiWrapperOwned(1)), 2);
/// assert_eq!(do_something(&mut RwApiWrapperOwned(3)), 3);
///
/// assert_eq!(do_something(RwLock::new(1)), 2);
/// assert_eq!(do_something(RwLock::new(3)), 3);
/// assert_eq!(do_something(&RwLock::new(1)), 2);
/// assert_eq!(do_something(&RwLock::new(3)), 3);
/// assert_eq!(do_something(&mut RwLock::new(1)), 2);
/// assert_eq!(do_something(&mut RwLock::new(3)), 3);
///
/// fn do_something_ref<'a>(mut x: impl RwApi<Target=&'a mut u64>) -> u64 {
///     if **x.read() == 1 {
///         **x.write() = 2;
///         **x.read()
///     } else {
///         **x.read()
///     }
/// }
///
/// assert_eq!(do_something_ref(RwApiWrapper(&mut 1)), 2);
/// assert_eq!(do_something_ref(RwApiWrapper(&mut 3)), 3);
/// assert_eq!(do_something_ref(&mut RwApiWrapper(&mut 1)), 2);
/// assert_eq!(do_something_ref(&mut RwApiWrapper(&mut 3)), 3);
///
/// assert_eq!(do_something_ref(RwLock::new(&mut 1)), 2);
/// assert_eq!(do_something_ref(RwLock::new(&mut 3)), 3);
/// assert_eq!(do_something_ref(&RwLock::new(&mut 1)), 2);
/// assert_eq!(do_something_ref(&RwLock::new(&mut 3)), 3);
/// assert_eq!(do_something_ref(&mut RwLock::new(&mut 1)), 2);
/// assert_eq!(do_something_ref(&mut RwLock::new(&mut 3)), 3);
/// ```
pub trait RwApi: ReadApi + WriteApi {}

/// Provides a constant part of the [`RwApi`] interface.
///
/// # Example
///
/// ```rust
/// use std::ops::Deref;
/// use parking_lot::RwLock;
/// use read_write_api::{ReadApi, RwApi, RwApiWrapper, RwApiWrapperOwned};
///
/// fn do_something(mut x: impl RwApi<Target=u64>) -> u64 {
///     if *x.read() == 1 {
///         *x.write() = 2;
///         read_x(&x)
///     } else {
///         read_x(&x)
///     }
/// }
///
/// fn read_x(x: &impl ReadApi<Target=u64>) -> u64 {
///     *x.read()
/// }
///
/// assert_eq!(do_something(RwApiWrapperOwned(1)), 2);
/// assert_eq!(do_something(RwApiWrapperOwned(3)), 3);
/// assert_eq!(do_something(&mut RwApiWrapperOwned(1)), 2);
/// assert_eq!(do_something(&mut RwApiWrapperOwned(3)), 3);
///
/// assert_eq!(do_something(RwLock::new(1)), 2);
/// assert_eq!(do_something(RwLock::new(3)), 3);
/// assert_eq!(do_something(&RwLock::new(1)), 2);
/// assert_eq!(do_something(&RwLock::new(3)), 3);
/// assert_eq!(do_something(&mut RwLock::new(1)), 2);
/// assert_eq!(do_something(&mut RwLock::new(3)), 3);
///
/// fn do_something_ref<'a>(mut x: impl RwApi<Target=&'a mut u64>) -> u64 {
///     if **x.read() == 1 {
///         **x.write() = 2;
///         read_x_ref(&x)
///     } else {
///         read_x_ref(&x)
///     }
/// }
///
/// fn read_x_ref<T>(x: &impl ReadApi<Target=T>) -> u64
///     where
///         T: Deref<Target=u64>
/// {
///     **x.read()
/// }
///
/// assert_eq!(do_something_ref(RwApiWrapper(&mut 1)), 2);
/// assert_eq!(do_something_ref(RwApiWrapper(&mut 3)), 3);
/// assert_eq!(do_something_ref(&mut RwApiWrapper(&mut 1)), 2);
/// assert_eq!(do_something_ref(&mut RwApiWrapper(&mut 3)), 3);
///
/// assert_eq!(do_something_ref(RwLock::new(&mut 1)), 2);
/// assert_eq!(do_something_ref(RwLock::new(&mut 3)), 3);
/// assert_eq!(do_something_ref(&RwLock::new(&mut 1)), 2);
/// assert_eq!(do_something_ref(&RwLock::new(&mut 3)), 3);
/// assert_eq!(do_something_ref(&mut RwLock::new(&mut 1)), 2);
/// assert_eq!(do_something_ref(&mut RwLock::new(&mut 3)), 3);
/// ```
pub trait ReadApi: GuardedTarget
{
    /// [`Self::read`] return type.
    type ReadGuard<'a>: Deref<Target=Self::Target>
        where Self: 'a;

    /// [`RwLock::read`](parking_lot::RwLock::read) analogue.
    fn read(&self) -> Self::ReadGuard<'_>;
}

/// Provides a mutable part of the [`RwApi`] interface.
///
/// # Example
///
/// ```rust
/// use std::ops::Deref;
/// use parking_lot::RwLock;
/// use read_write_api::{RwApi, RwApiWrapper, RwApiWrapperOwned, WriteApi};
///
/// fn do_something(mut x: impl RwApi<Target=u64>) -> u64 {
///     if *x.read() == 1 {
///         *write_x(&mut x) = 2;
///         *x.read()
///     } else {
///         *x.read()
///     }
/// }
///
/// fn write_x<T: WriteApi>(x: &mut T) -> T::WriteGuard<'_> {
///     x.write()
/// }
///
/// assert_eq!(do_something(RwApiWrapperOwned(1)), 2);
/// assert_eq!(do_something(RwApiWrapperOwned(3)), 3);
/// assert_eq!(do_something(&mut RwApiWrapperOwned(1)), 2);
/// assert_eq!(do_something(&mut RwApiWrapperOwned(3)), 3);
///
/// assert_eq!(do_something(RwLock::new(1)), 2);
/// assert_eq!(do_something(RwLock::new(3)), 3);
/// assert_eq!(do_something(&RwLock::new(1)), 2);
/// assert_eq!(do_something(&RwLock::new(3)), 3);
/// assert_eq!(do_something(&mut RwLock::new(1)), 2);
/// assert_eq!(do_something(&mut RwLock::new(3)), 3);
///
/// fn do_something_ref<'a>(mut x: impl RwApi<Target=&'a mut u64>) -> u64 {
///     if **x.read() == 1 {
///         **write_x(&mut x) = 2;
///         **x.read()
///     } else {
///         **x.read()
///     }
/// }
///
/// assert_eq!(do_something_ref(RwApiWrapper(&mut 1)), 2);
/// assert_eq!(do_something_ref(RwApiWrapper(&mut 3)), 3);
/// assert_eq!(do_something_ref(&mut RwApiWrapper(&mut 1)), 2);
/// assert_eq!(do_something_ref(&mut RwApiWrapper(&mut 3)), 3);
///
/// assert_eq!(do_something_ref(RwLock::new(&mut 1)), 2);
/// assert_eq!(do_something_ref(RwLock::new(&mut 3)), 3);
/// assert_eq!(do_something_ref(&RwLock::new(&mut 1)), 2);
/// assert_eq!(do_something_ref(&RwLock::new(&mut 3)), 3);
/// assert_eq!(do_something_ref(&mut RwLock::new(&mut 1)), 2);
/// assert_eq!(do_something_ref(&mut RwLock::new(&mut 3)), 3);
/// ```
pub trait WriteApi: GuardedTarget
{
    /// [`Self::write`] return type.
    type WriteGuard<'a>: DerefMut<Target=Self::Target>
        where Self: 'a;

    /// [`RwLock::write`](parking_lot::RwLock::write) analogue.
    fn write(&mut self) -> Self::WriteGuard<'_>;
}

/// Provides a single dereferencing target type for
/// the [`ReadApi`], [`WriteApi`] and [`RwApi`] traits.
///
/// # Example
///
/// ```rust
/// use read_write_api::{GuardedTarget, ReadApi, RwApi, WriteApi};
///
/// struct A(u64);
///
/// impl GuardedTarget for A {
///     type Target = u64;
/// }
///
/// impl ReadApi for A
/// {
///     type ReadGuard<'a>  = &'a u64
///         where Self: 'a;
///
///     fn read(&self) -> Self::ReadGuard<'_> {
///         &self.0
///     }
/// }
///
/// impl WriteApi for A
/// {
///     type WriteGuard<'a> = &'a mut u64
///         where Self: 'a;
///
///     fn write(&mut self) -> Self::WriteGuard<'_> {
///         &mut self.0
///     }
/// }
///
/// fn accept_read_write(_: impl RwApi<Target=u64>) {}
///
/// accept_read_write(A(1))
/// ```
pub trait GuardedTarget
{
    /// Dereference target of the read and write guards.
    type Target;
}

#[cfg(doctest)]
mod test_readme
{
    macro_rules! external_doc_test {
        ($x:expr) => {
            #[doc = $x]
            extern {}
        };
    }

    external_doc_test!(include_str!("../README.md"));
}