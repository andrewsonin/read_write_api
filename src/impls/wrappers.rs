use crate::{GuardedTarget, ReadApi, ReadApiWrapper, RwApiWrapper, RwApiWrapperOwned, WriteApi};

impl<'a, T> GuardedTarget for ReadApiWrapper<'a, T> {
    type Target = &'a T;
}

impl<'a, T> GuardedTarget for &ReadApiWrapper<'a, T> {
    type Target = &'a T;
}

impl<'a, T> GuardedTarget for &mut ReadApiWrapper<'a, T> {
    type Target = &'a T;
}

impl<'a, T> GuardedTarget for RwApiWrapper<'a, T> {
    type Target = &'a mut T;
}

impl<'a, T> GuardedTarget for &RwApiWrapper<'a, T> {
    type Target = &'a mut T;
}

impl<'a, T> GuardedTarget for &mut RwApiWrapper<'a, T> {
    type Target = &'a mut T;
}

impl<T> GuardedTarget for RwApiWrapperOwned<T> {
    type Target = T;
}

impl<T> GuardedTarget for &RwApiWrapperOwned<T> {
    type Target = T;
}

impl<T> GuardedTarget for &mut RwApiWrapperOwned<T> {
    type Target = T;
}

impl<'a, T> ReadApi for ReadApiWrapper<'a, T>
{
    type ReadGuard<'i> = &'i &'a T
        where Self: 'i;

    #[inline(always)]
    fn read(&self) -> &&'a T {
        &self.0
    }
}

impl<'a, T> ReadApi for &ReadApiWrapper<'a, T>
{
    type ReadGuard<'i> = &'i &'a T
        where Self: 'i;

    #[inline(always)]
    fn read(&self) -> &&'a T {
        &self.0
    }
}

impl<'a, T> ReadApi for &mut ReadApiWrapper<'a, T>
{
    type ReadGuard<'i> = &'i &'a T
        where Self: 'i;

    #[inline(always)]
    fn read(&self) -> &&'a T {
        &self.0
    }
}

impl<'a, T> ReadApi for RwApiWrapper<'a, T>
{
    type ReadGuard<'i> = &'i &'a mut T
        where Self: 'i;

    #[inline(always)]
    fn read(&self) -> &&'a mut T {
        &self.0
    }
}

impl<'a, T> ReadApi for &RwApiWrapper<'a, T>
{
    type ReadGuard<'i> = &'i &'a mut T
        where Self: 'i;

    #[inline(always)]
    fn read(&self) -> &&'a mut T {
        &self.0
    }
}

impl<'a, T> ReadApi for &mut RwApiWrapper<'a, T>
{
    type ReadGuard<'i> = &'i &'a mut T
        where Self: 'i;

    #[inline(always)]
    fn read(&self) -> &&'a mut T {
        &self.0
    }
}

impl<'a, T> WriteApi for RwApiWrapper<'a, T>
{
    type WriteGuard<'i> = &'i mut &'a mut T
        where Self: 'i;

    #[inline(always)]
    fn write(&mut self) -> &mut &'a mut T {
        &mut self.0
    }
}

impl<'a, T> WriteApi for &mut RwApiWrapper<'a, T>
{
    type WriteGuard<'i> = &'i mut &'a mut T
        where Self: 'i;

    #[inline(always)]
    fn write(&mut self) -> &mut &'a mut T {
        &mut self.0
    }
}

impl<T> ReadApi for RwApiWrapperOwned<T>
{
    type ReadGuard<'i> = &'i T
        where Self: 'i;

    #[inline(always)]
    fn read(&self) -> &T {
        &self.0
    }
}

impl<T> ReadApi for &RwApiWrapperOwned<T>
{
    type ReadGuard<'i> = &'i T
        where Self: 'i;

    #[inline(always)]
    fn read(&self) -> &T {
        &self.0
    }
}

impl<T> ReadApi for &mut RwApiWrapperOwned<T>
{
    type ReadGuard<'i> = &'i T
        where Self: 'i;

    #[inline(always)]
    fn read(&self) -> &T {
        &self.0
    }
}

impl<T> WriteApi for RwApiWrapperOwned<T>
{
    type WriteGuard<'i> = &'i mut T
        where Self: 'i;

    #[inline(always)]
    fn write(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> WriteApi for &mut RwApiWrapperOwned<T>
{
    type WriteGuard<'i> = &'i mut T
        where Self: 'i;

    #[inline(always)]
    fn write(&mut self) -> &mut T {
        &mut self.0
    }
}