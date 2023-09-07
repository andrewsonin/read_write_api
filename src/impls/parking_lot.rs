use {
    crate::{
        DowngradableWriteApi,
        DowngradableWriteGuard,
        GuardedTarget,
        ReadApi,
        UpgradableReadApi,
        UpgradableReadGuard,
        WriteApi,
    },
    parking_lot::{RwLock, RwLockReadGuard, RwLockUpgradableReadGuard, RwLockWriteGuard},
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

impl<T> UpgradableReadApi for RwLock<T>
{
    type UpgradableReadGuard<'a> = &'a mut T
        where Self: 'a;

    #[inline]
    fn upgradable_read(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T> DowngradableWriteApi for RwLock<T>
{
    type DowngradableWriteGuard<'a> = &'a mut T
        where Self: 'a;

    #[inline]
    fn downgradable_write(&mut self) -> &mut T {
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

impl<T> UpgradableReadApi for &RwLock<T>
{
    type UpgradableReadGuard<'a> = RwLockUpgradableReadGuard<'a, T>
        where Self: 'a;

    #[inline]
    fn upgradable_read(&mut self) -> RwLockUpgradableReadGuard<'_, T> {
        RwLock::upgradable_read(self)
    }
}

impl<T> DowngradableWriteApi for &RwLock<T>
{
    type DowngradableWriteGuard<'a> = RwLockWriteGuard<'a, T>
        where Self: 'a;

    #[inline]
    fn downgradable_write(&mut self) -> RwLockWriteGuard<'_, T> {
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

impl<T> UpgradableReadApi for &mut RwLock<T>
{
    type UpgradableReadGuard<'a> = RwLockUpgradableReadGuard<'a, T>
        where Self: 'a;

    #[inline]
    fn upgradable_read(&mut self) -> RwLockUpgradableReadGuard<'_, T> {
        RwLock::upgradable_read(self)
    }
}

impl<T> DowngradableWriteApi for &mut RwLock<T>
{
    type DowngradableWriteGuard<'a> = &'a mut T
        where Self: 'a;

    #[inline]
    fn downgradable_write(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<'a, T> UpgradableReadGuard for RwLockUpgradableReadGuard<'a, T>
{
    type UpgradeResult = RwLockWriteGuard<'a, T>;
    type UpgradeToDowngradableResult = RwLockWriteGuard<'a, T>;

    #[inline]
    fn upgrade(self) -> RwLockWriteGuard<'a, T> {
        RwLockUpgradableReadGuard::upgrade(self)
    }

    #[inline]
    fn upgrade_to_downgradable(self) -> RwLockWriteGuard<'a, T> {
        RwLockUpgradableReadGuard::upgrade(self)
    }
}

impl<'a, T> DowngradableWriteGuard for RwLockWriteGuard<'a, T>
{
    type DowngradeResult = RwLockReadGuard<'a, T>;
    type DowngradeToUpgradableResult = RwLockUpgradableReadGuard<'a, T>;

    #[inline]
    fn downgrade(self) -> RwLockReadGuard<'a, T> {
        RwLockWriteGuard::downgrade(self)
    }

    #[inline]
    fn downgrade_to_upgradable(self) -> RwLockUpgradableReadGuard<'a, T> {
        RwLockWriteGuard::downgrade_to_upgradable(self)
    }
}

impl<'a, T: ?Sized> UpgradableReadGuard for &'a mut T
{
    type UpgradeResult = Self;
    type UpgradeToDowngradableResult = Self;

    #[inline(always)]
    fn upgrade(self) -> Self {
        self
    }

    #[inline(always)]
    fn upgrade_to_downgradable(self) -> Self {
        self
    }
}

impl<'a, T: ?Sized> DowngradableWriteGuard for &'a mut T
{
    type DowngradeResult = Self;
    type DowngradeToUpgradableResult = Self;

    #[inline(always)]
    fn downgrade(self) -> Self {
        self
    }

    #[inline(always)]
    fn downgrade_to_upgradable(self) -> Self {
        self
    }
}