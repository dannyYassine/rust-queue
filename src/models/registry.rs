use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, Mutex},
};

use lazy_static::lazy_static;

type RegistryData = Arc<Mutex<HashMap<String, Box<dyn HashableRegistry + 'static>>>>;

pub trait HashableRegistry: Send + Any {}

pub struct Registry {
    map: RegistryData,
}

lazy_static! {
    static ref REGISTRY: Registry = {
        Registry {
            map: Arc::new(Mutex::new(HashMap::new())),
        }
    };
}

impl Registry {
    pub fn get_instance() -> &'static Self {
        &REGISTRY
    }

    pub fn set<T: HashableRegistry + 'static>(&self, key: &str, value: T) {
        let mut map = self.map.lock().unwrap();
        map.insert(key.to_string(), Box::new(value));
    }

    pub fn get<T: HashableRegistry, U>(&self, key: &str, func: Box<dyn Fn(Option<&T>) -> U>) -> U {
        let map_clone = Arc::clone(&self.map);
        let map = map_clone.lock().unwrap();

        return func(downcast_ref::<T>(map.get(key).unwrap()));
    }

    pub fn clear(&self) {
        let mut map = self.map.lock().unwrap();
        map.clear();
    }
}

fn downcast_ref<T: 'static>(any: &dyn Any) -> Option<&T> {
    if any.type_id() == TypeId::of::<T>() {
        unsafe { Some(&*(any as *const dyn Any as *const T)) }
    } else {
        None
    }
}
