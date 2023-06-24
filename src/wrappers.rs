use crate::{ReadApi, WriteApi};

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

impl<'a, T> ReadApi<T> for ReadApiWrapper<'a, T>
{
    type ReadGuard<'i> = &'a T
        where Self: 'i;

    #[inline(always)]
    fn read(&self) -> &'a T {
        self.0
    }
}

impl<'a, T> ReadApi<T> for &ReadApiWrapper<'a, T>
{
    type ReadGuard<'i> = &'a T
        where Self: 'i;

    #[inline(always)]
    fn read(&self) -> &'a T {
        self.0
    }
}

impl<'a, T> ReadApi<T> for &mut ReadApiWrapper<'a, T>
{
    type ReadGuard<'i> = &'a T
        where Self: 'i;

    #[inline(always)]
    fn read(&self) -> &'a T {
        self.0
    }
}

impl<'a, T> ReadApi<&'a T> for ReadApiWrapper<'a, T>
{
    type ReadGuard<'i> = &'i &'a T
        where Self: 'i;

    #[inline(always)]
    fn read(&self) -> &&'a T {
        &self.0
    }
}

impl<'a, T> ReadApi<&'a T> for &ReadApiWrapper<'a, T>
{
    type ReadGuard<'i> = &'i &'a T
        where Self: 'i;

    #[inline(always)]
    fn read(&self) -> &&'a T {
        &self.0
    }
}

impl<'a, T> ReadApi<&'a T> for &mut ReadApiWrapper<'a, T>
{
    type ReadGuard<'i> = &'i &'a T
        where Self: 'i;

    #[inline(always)]
    fn read(&self) -> &&'a T {
        &self.0
    }
}

impl<'a, T> ReadApi<T> for RwApiWrapper<'a, T>
{
    type ReadGuard<'i> = &'i T
        where Self: 'i;

    #[inline(always)]
    fn read(&self) -> &T {
        self.0
    }
}

impl<'a, T> ReadApi<T> for &RwApiWrapper<'a, T>
{
    type ReadGuard<'i> = &'i T
        where Self: 'i;

    #[inline(always)]
    fn read(&self) -> &T {
        self.0
    }
}

impl<'a, T> ReadApi<T> for &mut RwApiWrapper<'a, T>
{
    type ReadGuard<'i> = &'i T
        where Self: 'i;

    #[inline(always)]
    fn read(&self) -> &T {
        self.0
    }
}

impl<'a, T> ReadApi<&'a mut T> for RwApiWrapper<'a, T>
{
    type ReadGuard<'i> = &'i &'a mut T
        where Self: 'i;

    #[inline(always)]
    fn read(&self) -> &&'a mut T {
        &self.0
    }
}

impl<'a, T> ReadApi<&'a mut T> for &RwApiWrapper<'a, T>
{
    type ReadGuard<'i> = &'i &'a mut T
        where Self: 'i;

    #[inline(always)]
    fn read(&self) -> &&'a mut T {
        &self.0
    }
}

impl<'a, T> ReadApi<&'a mut T> for &mut RwApiWrapper<'a, T>
{
    type ReadGuard<'i> = &'i &'a mut T
        where Self: 'i;

    #[inline(always)]
    fn read(&self) -> &&'a mut T {
        &self.0
    }
}

impl<'a, T> WriteApi<T> for RwApiWrapper<'a, T>
{
    type WriteGuard<'i> = &'i mut T
        where Self: 'i;

    #[inline(always)]
    fn write(&mut self) -> &mut T {
        self.0
    }
}

impl<'a, T> WriteApi<T> for &mut RwApiWrapper<'a, T>
{
    type WriteGuard<'i> = &'i mut T
        where Self: 'i;

    #[inline(always)]
    fn write(&mut self) -> &mut T {
        self.0
    }
}

impl<'a, T> WriteApi<&'a mut T> for RwApiWrapper<'a, T>
{
    type WriteGuard<'i> = &'i mut &'a mut T
        where Self: 'i;

    #[inline(always)]
    fn write(&mut self) -> &mut &'a mut T {
        &mut self.0
    }
}

impl<'a, T> WriteApi<&'a mut T> for &mut RwApiWrapper<'a, T>
{
    type WriteGuard<'i> = &'i mut &'a mut T
        where Self: 'i;

    #[inline(always)]
    fn write(&mut self) -> &mut &'a mut T {
        &mut self.0
    }
}

impl<T> ReadApi<T> for RwApiWrapperOwned<T>
{
    type ReadGuard<'i> = &'i T
        where Self: 'i;

    #[inline(always)]
    fn read(&self) -> &T {
        &self.0
    }
}

impl<T> ReadApi<T> for &RwApiWrapperOwned<T>
{
    type ReadGuard<'i> = &'i T
        where Self: 'i;

    #[inline(always)]
    fn read(&self) -> &T {
        &self.0
    }
}

impl<T> ReadApi<T> for &mut RwApiWrapperOwned<T>
{
    type ReadGuard<'i> = &'i T
        where Self: 'i;

    #[inline(always)]
    fn read(&self) -> &T {
        &self.0
    }
}

impl<T> WriteApi<T> for RwApiWrapperOwned<T>
{
    type WriteGuard<'i> = &'i mut T
        where Self: 'i;

    #[inline(always)]
    fn write(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> WriteApi<T> for &mut RwApiWrapperOwned<T>
{
    type WriteGuard<'i> = &'i mut T
        where Self: 'i;

    #[inline(always)]
    fn write(&mut self) -> &mut T {
        &mut self.0
    }
}