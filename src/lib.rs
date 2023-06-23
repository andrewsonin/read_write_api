use {
    parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard},
    std::ops::{Deref, DerefMut},
};

pub trait ReadApi
{
    /// Dereference target of the return type of the
    /// [`Self::read`] method.
    type Target;

    /// [`Self::read`] return type.
    type ReadGuard<'a>: Deref<Target=Self::Target>
        where Self: 'a;

    /// [`RwLock::read`] analogue.
    fn read(&self) -> Self::ReadGuard<'_>;
}

pub trait TrivialReadApi: Deref
    where
        Self::Target: Sized {}

impl<T> ReadApi for T
    where
        Self: TrivialReadApi,
        <Self as Deref>::Target: Sized
{
    type Target = <T as Deref>::Target;
    type ReadGuard<'a> = &'a Self::Target
        where Self: 'a;

    #[inline(always)]
    fn read(&self) -> &Self::Target {
        self
    }
}

pub trait WriteApi
{
    /// Dereference target of the return type of the
    /// [`Self::write`] method.
    type Target;

    /// [`Self::write`] return type.
    type WriteGuard<'a>: DerefMut<Target=Self::Target>
        where Self: 'a;

    /// [`RwLock::write`] analogue.
    fn write(&mut self) -> Self::WriteGuard<'_>;
}

pub trait TrivialWriteApi: DerefMut
    where
        Self::Target: Sized {}

impl<T> WriteApi for T
    where
        Self: TrivialWriteApi,
        <Self as Deref>::Target: Sized
{
    type Target = <T as Deref>::Target;
    type WriteGuard<'a> = &'a mut Self::Target
        where Self: 'a;

    #[inline(always)]
    fn write(&mut self) -> &mut Self::Target {
        self
    }
}

#[doc = include_str!("../README.md")]
pub trait ReadWriteApi
{
    /// Dereference target of the return types of the
    /// [`Self::read`] and [`Self::write`] methods.
    type Target;

    /// [`Self::read`] return type.
    type ReadGuard<'a>: Deref<Target=Self::Target>
        where Self: 'a;

    /// [`Self::write`] return type.
    type WriteGuard<'a>: DerefMut<Target=Self::Target>
        where Self: 'a;

    /// [`RwLock::read`] analogue.
    fn read(&self) -> Self::ReadGuard<'_>;

    /// [`RwLock::write`] analogue.
    fn write(&mut self) -> Self::WriteGuard<'_>;
}

/// Marker trait used to tell the compiler to automatically implement [`ReadWriteApi`]
/// for trivial types and references to instances of such types.
/// See the `Examples` section of the [`ReadWriteApi`] documentation.
pub trait TrivialReadWriteApi {}

impl<T> ReadWriteApi for T
    where
        T: TrivialReadWriteApi
{
    type Target = T;

    type ReadGuard<'a> = &'a T
        where Self: 'a;

    type WriteGuard<'a> = &'a mut T
        where Self: 'a;

    #[inline(always)]
    fn read(&self) -> &Self {
        self
    }

    #[inline(always)]
    fn write(&mut self) -> &mut Self {
        self
    }
}

impl<T> ReadWriteApi for RwLock<T>
{
    type Target = T;

    type ReadGuard<'a> = RwLockReadGuard<'a, T>
        where Self: 'a;

    type WriteGuard<'a> = &'a mut T
        where Self: 'a;

    #[inline]
    fn read(&self) -> RwLockReadGuard<'_, T> {
        RwLock::read(self)
    }

    #[inline]
    fn write(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T> ReadWriteApi for &RwLock<T>
{
    type Target = T;

    type ReadGuard<'a> = RwLockReadGuard<'a, T>
        where Self: 'a;

    type WriteGuard<'a> = RwLockWriteGuard<'a, T>
        where Self: 'a;

    #[inline]
    fn read(&self) -> RwLockReadGuard<'_, T> {
        RwLock::read(self)
    }

    #[inline]
    fn write(&mut self) -> RwLockWriteGuard<'_, T> {
        RwLock::write(self)
    }
}

impl<T> ReadWriteApi for &mut RwLock<T>
{
    type Target = T;

    type ReadGuard<'a> = RwLockReadGuard<'a, T>
        where Self: 'a;

    type WriteGuard<'a> = &'a mut T
        where Self: 'a;

    #[inline]
    fn read(&self) -> RwLockReadGuard<'_, T> {
        RwLock::read(self)
    }

    #[inline]
    fn write(&mut self) -> &mut T {
        self.get_mut()
    }
}