use parking_lot::{Mutex, MutexGuard};
use std::sync::Arc;

#[derive(Debug)]
pub struct LockedResource<T>(Arc<Mutex<T>>);

#[derive(Debug)]
pub struct Resource<'a, T>(MutexGuard<'a, T>);

impl<'a, T> std::ops::Deref for Resource<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Clone for LockedResource<T> {
    fn clone(&self) -> Self {
        LockedResource(self.0.clone())
    }
}

impl<T> LockedResource<T> {
    pub fn new(res: T) -> LockedResource<T> {
        LockedResource(Arc::new(Mutex::new(res)))
    }

    pub fn set(&self, new: T) -> T {
        let res: &mut T = &mut self.0.lock();
        std::mem::replace(res, new)
    }

    pub fn take(&self) -> T
    where
        T: Default,
    {
        self.set(T::default())
    }

    pub fn get(&self) -> Resource<T> {
        Resource(self.0.lock())
    }
}
