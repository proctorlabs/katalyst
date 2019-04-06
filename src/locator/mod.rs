mod container;

use container::ArcObject;
pub use container::Locatable;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct Locator {
    services: HashMap<TypeId, Box<Any + Sync + Send>>,
}

impl Locator {
    pub fn locate<T: Locatable>(&self) -> Option<Arc<T>> {
        let id = TypeId::of::<T>();
        let result = self.services.get(&id)?;
        let dc = result.downcast_ref::<ArcObject<T>>()?;
        Some(dc.clone())
    }

    pub fn register<T: Locatable>(&mut self, item: T) {
        let id = TypeId::of::<T>();
        self.services.insert(id, Box::new(ArcObject::new(item)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct LocatableThing {
        contents: &'static str,
    }

    #[test]
    fn register_in_locator_can_retrieve() {
        let mut locator = Locator::default();
        locator.register::<LocatableThing>(LocatableThing { contents: "Hello!" });
        let thing = locator.locate::<LocatableThing>().unwrap();
        assert_eq!("Hello!", thing.contents);
    }

    #[test]
    fn returns_none_if_not_found() {
        let locator = Locator::default();
        let thing = locator.locate::<LocatableThing>();
        assert!(thing.is_none());
    }
}
