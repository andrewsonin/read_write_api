use crate::{ReadApi, RwApi, WriteApi};

impl<T, R> RwApi for T
    where
        T: ReadApi<Target=R> + WriteApi<Target=R>
{}