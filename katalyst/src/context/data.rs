use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct ContextData {
    store: HashMap<TypeId, Arc<Any + Send + Sync>>,
}

impl ContextData {
    pub fn get<T: Any + Send + Sync>(&self) -> Option<Arc<T>> {
        let id = TypeId::of::<T>();
        println!("id: {:?}", id);
        let result = self.store.get(&id)?;
        println!("retrieved");
        let dc = result.downcast_ref::<Arc<T>>()?;
        println!("casted");
        Some(dc.clone())
    }

    pub fn set<T: Any + Send + Sync>(&mut self, item: T) {
        let id = TypeId::of::<T>();
        println!("id: {:?}", id);
        self.store.insert(id, Arc::new(item));
    }
}
