use {
    parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard},
    std::ops::{Deref, DerefMut},
};
pub use auto_impl::{AutoReadApi, AutoWriteApi};

mod auto_impl;

#[doc = include_str!("../README.md")]
pub trait ReadWriteApi<T>: ReadApi<Target=T> + WriteApi<Target=T>
{}

impl<T, R> ReadWriteApi<R> for T
    where
        T: ReadApi<Target=R> + WriteApi<Target=R>
{}

/// Provides constant part of the [`ReadWriteApi`] interface.
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

/// Provides mutable part of the [`ReadWriteApi`] interface.
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

impl<T> ReadApi for RwLock<T>
{
    type Target = T;
    type ReadGuard<'a> = RwLockReadGuard<'a, T>
        where Self: 'a;

    #[inline]
    fn read(&self) -> RwLockReadGuard<'_, T> {
        RwLock::read(self)
    }
}

impl<T> WriteApi for RwLock<T>
{
    type Target = T;
    type WriteGuard<'a> = &'a mut T
        where Self: 'a;

    #[inline]
    fn write(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T> ReadApi for &RwLock<T>
{
    type Target = T;
    type ReadGuard<'a> = RwLockReadGuard<'a, T>
        where Self: 'a;

    #[inline]
    fn read(&self) -> RwLockReadGuard<'_, T> {
        RwLock::read(self)
    }
}

impl<T> WriteApi for &RwLock<T>
{
    type Target = T;
    type WriteGuard<'a> = RwLockWriteGuard<'a, T>
        where Self: 'a;

    #[inline]
    fn write(&mut self) -> RwLockWriteGuard<'_, T> {
        RwLock::write(self)
    }
}

impl<T> ReadApi for &mut RwLock<T>
{
    type Target = T;
    type ReadGuard<'a> = RwLockReadGuard<'a, T>
        where Self: 'a;

    #[inline]
    fn read(&self) -> RwLockReadGuard<'_, T> {
        RwLock::read(self)
    }
}

impl<T> WriteApi for &mut RwLock<T>
{
    type Target = T;
    type WriteGuard<'a> = &'a mut T
        where Self: 'a;

    #[inline]
    fn write(&mut self) -> &mut T {
        self.get_mut()
    }
}