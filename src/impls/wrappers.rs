use crate::{
    DowngradableWriteApi,
    GuardedTarget,
    ReadApi,
    ReadApiWrapper,
    RwApiWrapper,
    RwApiWrapperOwned,
    UpgradableReadApi,
    WriteApi,
};

impl<'a, T: ?Sized> GuardedTarget for ReadApiWrapper<'a, T> {
    type Target = &'a T;
}

impl<'a, T: ?Sized> GuardedTarget for &ReadApiWrapper<'a, T> {
    type Target = &'a T;
}

impl<'a, T: ?Sized> GuardedTarget for &mut ReadApiWrapper<'a, T> {
    type Target = &'a T;
}

impl<'a, T: ?Sized> GuardedTarget for RwApiWrapper<'a, T> {
    type Target = &'a mut T;
}

impl<'a, T: ?Sized> GuardedTarget for &RwApiWrapper<'a, T> {
    type Target = &'a mut T;
}

impl<'a, T: ?Sized> GuardedTarget for &mut RwApiWrapper<'a, T> {
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

impl<'a, T: ?Sized> ReadApi for ReadApiWrapper<'a, T>
{
    type ReadGuard<'i> = &'i &'a T
        where Self: 'i;

    #[inline(always)]
    fn read(&self) -> &&'a T {
        &self.0
    }
}

impl<'a, T: ?Sized> ReadApi for &ReadApiWrapper<'a, T>
{
    type ReadGuard<'i> = &'i &'a T
        where Self: 'i;

    #[inline(always)]
    fn read(&self) -> &&'a T {
        &self.0
    }
}

impl<'a, T: ?Sized> ReadApi for &mut ReadApiWrapper<'a, T>
{
    type ReadGuard<'i> = &'i &'a T
        where Self: 'i;

    #[inline(always)]
    fn read(&self) -> &&'a T {
        &self.0
    }
}

impl<'a, T: ?Sized> ReadApi for RwApiWrapper<'a, T>
{
    type ReadGuard<'i> = &'i &'a mut T
        where Self: 'i;

    #[inline(always)]
    fn read(&self) -> &&'a mut T {
        &self.0
    }
}

impl<'a, T: ?Sized> ReadApi for &RwApiWrapper<'a, T>
{
    type ReadGuard<'i> = &'i &'a mut T
        where Self: 'i;

    #[inline(always)]
    fn read(&self) -> &&'a mut T {
        &self.0
    }
}

impl<'a, T: ?Sized> ReadApi for &mut RwApiWrapper<'a, T>
{
    type ReadGuard<'i> = &'i &'a mut T
        where Self: 'i;

    #[inline(always)]
    fn read(&self) -> &&'a mut T {
        &self.0
    }
}

impl<'a, T: ?Sized> UpgradableReadApi for RwApiWrapper<'a, T>
{
    type UpgradableReadGuard<'i> = &'i mut &'a mut T
        where Self: 'i;

    #[inline(always)]
    fn upgradable_read(&mut self) -> &mut &'a mut T {
        &mut self.0
    }
}

impl<'a, T: ?Sized> UpgradableReadApi for &mut RwApiWrapper<'a, T>
{
    type UpgradableReadGuard<'i> = &'i mut &'a mut T
        where Self: 'i;

    #[inline(always)]
    fn upgradable_read(&mut self) -> &mut &'a mut T {
        &mut self.0
    }
}

impl<'a, T: ?Sized> WriteApi for RwApiWrapper<'a, T>
{
    type WriteGuard<'i> = &'i mut &'a mut T
        where Self: 'i;

    #[inline(always)]
    fn write(&mut self) -> &mut &'a mut T {
        &mut self.0
    }
}

impl<'a, T: ?Sized> WriteApi for &mut RwApiWrapper<'a, T>
{
    type WriteGuard<'i> = &'i mut &'a mut T
        where Self: 'i;

    #[inline(always)]
    fn write(&mut self) -> &mut &'a mut T {
        &mut self.0
    }
}

impl<'a, T: ?Sized> DowngradableWriteApi for RwApiWrapper<'a, T>
{
    type DowngradableWriteGuard<'i> = &'i mut &'a mut T
        where Self: 'i;

    #[inline(always)]
    fn downgradable_write(&mut self) -> &mut &'a mut T {
        &mut self.0
    }
}

impl<'a, T: ?Sized> DowngradableWriteApi for &mut RwApiWrapper<'a, T>
{
    type DowngradableWriteGuard<'i> = &'i mut &'a mut T
        where Self: 'i;

    #[inline(always)]
    fn downgradable_write(&mut self) -> &mut &'a mut T {
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

impl<T> UpgradableReadApi for RwApiWrapperOwned<T>
{
    type UpgradableReadGuard<'i> = &'i mut T
        where Self: 'i;

    #[inline(always)]
    fn upgradable_read(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> UpgradableReadApi for &mut RwApiWrapperOwned<T>
{
    type UpgradableReadGuard<'i> = &'i mut T
        where Self: 'i;

    #[inline(always)]
    fn upgradable_read(&mut self) -> &mut T {
        &mut self.0
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

impl<T> DowngradableWriteApi for RwApiWrapperOwned<T>
{
    type DowngradableWriteGuard<'i> = &'i mut T
        where Self: 'i;

    #[inline(always)]
    fn downgradable_write(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> DowngradableWriteApi for &mut RwApiWrapperOwned<T>
{
    type DowngradableWriteGuard<'i> = &'i mut T
        where Self: 'i;

    #[inline(always)]
    fn downgradable_write(&mut self) -> &mut T {
        &mut self.0
    }
}