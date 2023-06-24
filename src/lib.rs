use std::ops::{Deref, DerefMut};

pub use wrappers::{ReadApiWrapper, RwApiWrapper, RwApiWrapperOwned};

mod impls;
mod wrappers;

#[doc = include_str!("../README.md")]
pub trait RwApi: ReadApi + WriteApi {}

/// Provides a constant part of the [`RwApi`] interface.
pub trait ReadApi: GuardedTarget
{
    /// [`Self::read`] return type.
    type ReadGuard<'a>: Deref<Target=Self::Target>
        where Self: 'a;

    /// [`RwLock::read`](parking_lot::RwLock::read) analogue.
    fn read(&self) -> Self::ReadGuard<'_>;
}

/// Provides a mutable part of the [`RwApi`] interface.
pub trait WriteApi: GuardedTarget
{
    /// [`Self::write`] return type.
    type WriteGuard<'a>: DerefMut<Target=Self::Target>
        where Self: 'a;

    /// [`RwLock::write`](parking_lot::RwLock::write) analogue.
    fn write(&mut self) -> Self::WriteGuard<'_>;
}

/// Provides a single dereferencing target type for [`ReadApi`], [`WriteApi`] and [`RwApi`].
pub trait GuardedTarget
{
    /// Dereference target of the read and write guards.
    type Target;
}