#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
/// [`ReadApi`] wrapper for constant references.
pub struct ReadApiWrapper<'a, T>(
    /// Wrapped reference.
    pub &'a T
);

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
/// [`RwApi`](crate::RwApi) wrapper for mutable references.
pub struct RwApiWrapper<'a, T>(
    /// Wrapped reference.
    pub &'a mut T
);

#[derive(Default, Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
/// [`RwApi`](crate::RwApi) owning wrapper for solitary objects.
pub struct RwApiWrapperOwned<T>(
    /// Wrapped owned object.
    pub T
);