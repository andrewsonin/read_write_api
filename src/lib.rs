use {
    parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard},
    std::ops::{Deref, DerefMut},
};
pub use wrappers::{ReadApiWrapper, RwApiWrapper, RwApiWrapperOwned};

mod wrappers;

#[doc = include_str!("../README.md")]
pub trait RwApi<T>: ReadApi<T> + WriteApi<T>
{}

impl<T, R> RwApi<R> for T
    where
        T: ReadApi<R> + WriteApi<R>
{}

/// Provides constant part of the [`RwApi`] interface.
pub trait ReadApi<T>
{
    /// [`Self::read`] return type.
    type ReadGuard<'a>: Deref<Target=T>
        where Self: 'a;

    /// [`RwLock::read`] analogue.
    fn read(&self) -> Self::ReadGuard<'_>;
}

/// Provides mutable part of the [`RwApi`] interface.
pub trait WriteApi<T>
{
    /// [`Self::write`] return type.
    type WriteGuard<'a>: DerefMut<Target=T>
        where Self: 'a;

    /// [`RwLock::write`] analogue.
    fn write(&mut self) -> Self::WriteGuard<'_>;
}

impl<T> ReadApi<T> for RwLock<T>
{
    type ReadGuard<'a> = RwLockReadGuard<'a, T>
        where Self: 'a;

    #[inline]
    fn read(&self) -> RwLockReadGuard<'_, T> {
        RwLock::read(self)
    }
}

impl<T> WriteApi<T> for RwLock<T>
{
    type WriteGuard<'a> = &'a mut T
        where Self: 'a;

    #[inline]
    fn write(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T> ReadApi<T> for &RwLock<T>
{
    type ReadGuard<'a> = RwLockReadGuard<'a, T>
        where Self: 'a;

    #[inline]
    fn read(&self) -> RwLockReadGuard<'_, T> {
        RwLock::read(self)
    }
}

impl<T> WriteApi<T> for &RwLock<T>
{
    type WriteGuard<'a> = RwLockWriteGuard<'a, T>
        where Self: 'a;

    #[inline]
    fn write(&mut self) -> RwLockWriteGuard<'_, T> {
        RwLock::write(self)
    }
}

impl<T> ReadApi<T> for &mut RwLock<T>
{
    type ReadGuard<'a> = RwLockReadGuard<'a, T>
        where Self: 'a;

    #[inline]
    fn read(&self) -> RwLockReadGuard<'_, T> {
        RwLock::read(self)
    }
}

impl<T> WriteApi<T> for &mut RwLock<T>
{
    type WriteGuard<'a> = &'a mut T
        where Self: 'a;

    #[inline]
    fn write(&mut self) -> &mut T {
        self.get_mut()
    }
}