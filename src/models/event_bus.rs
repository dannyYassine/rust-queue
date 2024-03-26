use std::{
    any::{type_name, Any},
    collections::HashMap,
    sync::{Arc, Mutex},
};

use lazy_static::lazy_static;

pub type EventListener = Box<dyn Fn(Box<&dyn Any>) + Send>;

#[derive(Default)]
pub struct EventBus {
    listeners: HashMap<String, Vec<EventListener>>,
}

impl EventBus {
    pub fn has_key(&self, key: &String) -> bool {
        return self.listeners.contains_key(key);
    }
    pub fn has<E>(&self) -> bool {
        let s = type_name::<E>().to_owned();
        let key = s.split("::").last().unwrap_or_default().to_owned();

        return self.listeners.contains_key(&key);
    }
    pub fn listen_with_key(&mut self, key: &String, func: impl Fn(Box<&dyn Any>) + 'static + Send) {
        self.listeners
            .entry(key.to_owned())
            .or_insert_with(Vec::new)
            .push(Box::new(move |event: Box<&dyn Any>| {
                func(event);
            }));
    }
    pub fn listen<E>(&mut self, func: impl Fn(&E) + 'static + Send)
    where
        E: 'static + Send,
    {
        let s = type_name::<E>().to_owned();
        let key = s.split("::").last().unwrap_or_default().to_owned();

        self.listeners
            .entry(key)
            .or_insert_with(Vec::new)
            .push(Box::new(move |event: Box<&dyn Any>| {
                let e = event.downcast_ref::<E>().unwrap();
                func(e);
            }));
    }
    pub fn emit<E>(&self, event: &E)
    where
        E: 'static + Send,
    {
        let s = type_name::<E>().to_owned();
        let key = s.split("::").last().unwrap_or_default().to_owned();

        if let Some(listeners) = self.listeners.get(&key) {
            for listener in listeners {
                listener(Box::new(event));
            }
        }
    }

    pub fn clear(&mut self) {
        self.listeners.clear();
    }
}

pub struct SharedEventBus {
    pub event_bus: Arc<Mutex<EventBus>>,
}

lazy_static! {
    static ref SHARED_EVENT_BUS: SharedEventBus = {
        SharedEventBus {
            event_bus: Arc::new(Mutex::new(EventBus {
                listeners: HashMap::new(),
            })),
        }
    };
}

impl SharedEventBus {
    pub fn get_instance() -> &'static Self {
        &SHARED_EVENT_BUS
    }

    pub fn emit<E>(event: &E)
    where
        E: 'static + Send,
    {
        let event_bus = &SHARED_EVENT_BUS.event_bus.lock().unwrap();
        event_bus.emit(event);
    }
}
