use std::{
    any::{type_name, Any},
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

    pub fn get<T>(&self) -> Arc<Box<T>>
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

    pub fn set<J>(
        &self,
        func: impl Fn(&Registry) -> Box<dyn Any + Send + Sync + 'static> + Send + Sync + 'static,
    ) where
        J: 'static,
    {
        let s = type_name::<J>().to_owned();
        let mut map = self.type_map.lock().unwrap();

        map.insert(s, Arc::new(Box::new(func)));
    }

    pub fn clear(&self) {
        let mut map = self.map.lock().unwrap();
        map.clear();
    }
}
