use std::any::Any;
use std::sync::Arc;

pub trait Locatable: Send + Sync + Any {}
impl<T> Locatable for T where T: Send + Sync + Any {}

#[derive(Debug)]
pub struct ArcObject<T: Locatable> {
    obj: Arc<T>,
}

impl<T: Locatable> ArcObject<T> {
    pub fn new(obj: T) -> Self {
        ArcObject { obj: Arc::new(obj) }
    }

    pub fn clone(&self) -> Arc<T> {
        self.obj.clone()
    }
}
