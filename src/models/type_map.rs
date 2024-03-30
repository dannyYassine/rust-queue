use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct TypeMap(HashMap<TypeId, Box<dyn Any>>);

impl TypeMap {
    pub fn new() -> Self {
        TypeMap(HashMap::new())
    }
    pub fn set<T: Any + 'static>(&mut self, t: T) {
        self.0.insert(TypeId::of::<T>(), Box::new(t));
    }
    pub fn has<T: Any + 'static>(&self) -> bool {
        self.0.contains_key(&TypeId::of::<T>())
    }

    pub fn get_mut<T: Any + 'static>(&mut self) -> Option<&mut T> {
        self.0
            .get_mut(&TypeId::of::<T>())
            .map(|t| t.downcast_mut::<T>().unwrap())
    }
}
