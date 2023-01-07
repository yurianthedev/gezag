use std::{cell::Ref, error::Error, ops::Deref};

use crate::entities::{Index, Key};

pub trait Add<T> {
    type S: 'static;
    type E: Error + Send + Sync + 'static;

    fn add(&self, res: T) -> Result<Self::S, Self::E>;
}

pub trait Remove<T> {
    type K: Key<T> + 'static;
    type S: 'static;
    type E: Error + Send + Sync + 'static;

    fn remove(&self, key: Self::K) -> Result<Self::S, Self::E>;
}

pub trait List<T> {
    type S: Deref<Target = [T]> + 'static;
    type E: Error + Send + Sync + 'static;

    fn list(&self) -> Result<Self::S, Self::E>;
}

pub trait ListBy<T> {
    type I: Index<T>;
    type S: Deref<Target = [T]> + 'static;
    type E: Error + Send + Sync + 'static;

    fn list_by(&self, index: &Self::I) -> Result<Self::S, Self::E>;
}
