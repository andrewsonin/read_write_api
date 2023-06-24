use {
    crate::{GuardedTarget, ReadApi, WriteApi},
    parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};

impl<T> GuardedTarget for RwLock<T> {
    type Target = T;
}

impl<T> GuardedTarget for &RwLock<T> {
    type Target = T;
}

impl<T> GuardedTarget for &mut RwLock<T> {
    type Target = T;
}

impl<T> ReadApi for RwLock<T>
{
    type ReadGuard<'a> = RwLockReadGuard<'a, T>
        where Self: 'a;

    #[inline]
    fn read(&self) -> RwLockReadGuard<'_, T> {
        RwLock::read(self)
    }
}

impl<T> WriteApi for RwLock<T>
{
    type WriteGuard<'a> = &'a mut T
        where Self: 'a;

    #[inline]
    fn write(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T> ReadApi for &RwLock<T>
{
    type ReadGuard<'a> = RwLockReadGuard<'a, T>
        where Self: 'a;

    #[inline]
    fn read(&self) -> RwLockReadGuard<'_, T> {
        RwLock::read(self)
    }
}

impl<T> WriteApi for &RwLock<T>
{
    type WriteGuard<'a> = RwLockWriteGuard<'a, T>
        where Self: 'a;

    #[inline]
    fn write(&mut self) -> RwLockWriteGuard<'_, T> {
        RwLock::write(self)
    }
}

impl<T> ReadApi for &mut RwLock<T>
{
    type ReadGuard<'a> = RwLockReadGuard<'a, T>
        where Self: 'a;

    #[inline]
    fn read(&self) -> RwLockReadGuard<'_, T> {
        RwLock::read(self)
    }
}

impl<T> WriteApi for &mut RwLock<T>
{
    type WriteGuard<'a> = &'a mut T
        where Self: 'a;

    #[inline]
    fn write(&mut self) -> &mut T {
        self.get_mut()
    }
}