use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use lazy_static::lazy_static;

pub trait HashableRegistry: Send {
    fn clone_box(&self) -> Box<dyn HashableRegistry>;
}

impl<T: 'static + HashableRegistry + Clone> HashableRegistry for T {
    fn clone_box(&self) -> Box<dyn HashableRegistry> {
        Box::new(self.clone())
    }
}

pub struct Registry {
    map: Arc<Mutex<HashMap<String, Box<dyn HashableRegistry>>>>,
}

lazy_static! {
    static ref REGISTRY: Registry = {
        Registry {
            map: Arc::new(Mutex::new(HashMap::new())),
        }
    };
}

impl Registry {
    pub fn insert<T: HashableRegistry + 'static>(key: String, value: T) {
        let mut map = REGISTRY.map.lock().unwrap();
        map.insert(key, Box::new(value));
    }

    pub fn get<T: 'static + Send + HashableRegistry + Clone>(
        key: &str,
    ) -> Option<Box<dyn HashableRegistry>> {
        let map = REGISTRY.map.lock().unwrap();
        map.get(key).clone().map(|v| v.clone_box())
    }

    pub fn clear(&self) {
        let mut map = self.map.lock().unwrap();
        map.clear();
    }
}
