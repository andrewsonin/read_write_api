#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
/// [`ReadApi`](crate::ReadApi) wrapper for constant references.
///
/// # Example
///
/// ```rust
/// use read_write_api::{ReadApi, ReadApiWrapper};
/// use parking_lot::RwLock;
///
/// fn read<T: ReadApi>(x: &T) -> T::ReadGuard<'_> {
///     x.read()
/// }
///
/// let _ = read(&ReadApiWrapper(&1));
/// let _ = read(&RwLock::new(2));
/// ```
pub struct ReadApiWrapper<'a, T: ?Sized>(
    /// Wrapped reference.
    pub &'a T
);

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
/// [`RwApi`](crate::RwApi) wrapper for mutable references.
///
/// # Example
///
/// See the [`RwApi`](crate::RwApi) docs for usage examples.
pub struct RwApiWrapper<'a, T: ?Sized>(
    /// Wrapped reference.
    pub &'a mut T
);

#[derive(Default, Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
/// [`RwApi`](crate::RwApi) owning wrapper for solitary objects.
///
/// # Example
///
/// See the [`RwApi`](crate::RwApi) docs for usage examples.
pub struct RwApiWrapperOwned<T>(
    /// Wrapped owned object.
    pub T
);