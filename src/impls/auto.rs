use crate::{DowngradableWriteApi, ReadApi, RwApi, UpgradableReadApi, WriteApi};

impl<T: ?Sized, R: ?Sized> RwApi for T
    where
        Self: ReadApi<Target=R>
        + WriteApi<Target=R>
        + DowngradableWriteApi<Target=R>
        + UpgradableReadApi<Target=R>
{}