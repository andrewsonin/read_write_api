use crate::{ReadApi, WriteApi};

impl<T> ReadApi for &T
    where
        T: AutoReadApi
{
    type Target = <T as AutoReadApi>::Target;
    type ReadGuard<'a> = &'a <T as AutoReadApi>::Target
        where Self: 'a;

    #[inline(always)]
    fn read(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> ReadApi for &mut T
    where
        T: AutoReadApi
{
    type Target = <T as AutoReadApi>::Target;
    type ReadGuard<'a> = &'a <T as AutoReadApi>::Target
        where Self: 'a;

    #[inline(always)]
    fn read(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> WriteApi for &mut T
    where
        T: AutoWriteApi
{
    type Target = <T as AutoWriteApi>::Target;
    type WriteGuard<'a> = &'a mut <T as AutoWriteApi>::Target
        where Self: 'a;

    #[inline(always)]
    fn write(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

/// Marker trait used to tell the compiler to automatically implement [`ReadApi`]
/// for references to instances of the type.
/// See the `Examples` section of the [`ReadWriteApi`](crate::ReadWriteApi) documentation.
pub trait AutoReadApi {
    /// Type to be used as a read guard.
    type Target;
    /// Returns read guard for [`Self`].
    fn get(&self) -> &Self::Target;
}

/// Marker trait used to tell the compiler to automatically implement [`WriteApi`]
/// for references to instances of the type.
/// See the `Examples` section of the [`ReadWriteApi`](crate::ReadWriteApi) documentation.
pub trait AutoWriteApi {
    /// Resulting write guard for the type.
    type Target;
    /// Returns write guard for [`Self`].
    fn get_mut(&mut self) -> &mut Self::Target;
}