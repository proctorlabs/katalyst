use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
struct Container<T: Any + Send + Sync> {
    data: Arc<T>,
}

impl<T: Any + Send + Sync> Container<T> {
    fn new(obj: T) -> Self {
        Container {
            data: Arc::new(obj),
        }
    }

    fn get(&self) -> Arc<T> {
        self.data.clone()
    }
}

#[derive(Debug, Default)]
pub struct ContextData {
    store: HashMap<TypeId, Box<Any + Send + Sync>>,
}

impl ContextData {
    pub fn get<T: Any + Send + Sync>(&self) -> Option<Arc<T>> {
        let id = TypeId::of::<T>();
        let result = self.store.get(&id)?;
        let dc = result.downcast_ref::<Container<T>>()?;
        Some(dc.get())
    }

    pub fn set<T: Any + Send + Sync>(&mut self, item: T) {
        let id = TypeId::of::<T>();
        self.store.insert(id, Box::new(Container::new(item)));
    }
}
