use std::{
    any::{type_name, Any, TypeId},
    collections::HashMap,
    sync::{Arc, Mutex},
};

use lazy_static::lazy_static;

// Type alias for the closure used in the JobMap
type JobClosure = Box<dyn Fn(&Registry) -> Box<dyn Any + Send + Sync + 'static> + Send + Sync>;
type TypeRegistry = Arc<Mutex<HashMap<String, Arc<JobClosure>>>>;

type RegistryData = Arc<Mutex<HashMap<String, Box<dyn Any + Send + Sync + 'static>>>>;

pub struct Registry {
    map: RegistryData,
    type_map: TypeRegistry,
}

lazy_static! {
    static ref REGISTRY: Registry = {
        Registry {
            map: Arc::new(Mutex::new(HashMap::new())),
            type_map: Arc::new(Mutex::new(HashMap::new())),
        }
    };
}

impl Registry {
    pub fn get_instance() -> &'static Self {
        &REGISTRY
    }

    pub fn set<T: Any + Send + Sync + 'static>(&self, key: &str, value: T) {
        let mut map = self.map.lock().unwrap();
        map.insert(key.to_string(), Box::new(value));
    }

    pub fn get<T: Any + Send + Sync>(&self, key: &str) -> Option<T>
    where
        T: Clone,
    {
        let map_clone = Arc::clone(&self.map);
        let map = map_clone.lock().unwrap();

        return downcast_ref::<T>(map.get(key).unwrap()).cloned();
    }

    pub fn get_type<T>(&self) -> Arc<Box<T>>
    where
        T: Clone + Any + Send + Sync,
    {
        let value = {
            let s = type_name::<T>().to_owned();
            let map = self.type_map.lock().unwrap();
            let func = map.get(&s).unwrap().clone();

            drop(map); // to unlock lock

            func(self)
        };

        let typed_value = value.downcast::<T>().unwrap();

        Arc::new(typed_value)
    }

    pub fn register<J>(&self, func: JobClosure) {
        let s = type_name::<J>().to_owned();
        let mut map = self.type_map.lock().unwrap();

        map.insert(s, Arc::new(func));
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
