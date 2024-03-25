use std::{
    any::{type_name, Any},
    collections::HashMap,
};

pub type EventListener = Box<dyn Fn(Box<&dyn Any>)>;

#[derive(Default)]
pub struct EventBus {
    listeners: HashMap<String, Vec<EventListener>>,
}

impl EventBus {
    pub fn listen<E>(&mut self, func: impl Fn(&E) + 'static)
    where
        E: 'static,
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
        E: 'static,
    {
        let s = type_name::<E>().to_owned();
        let key = s.split("::").last().unwrap_or_default().to_owned();

        let listeners = self.listeners.get(&key).unwrap();
        for listener in listeners {
            listener(Box::new(event));
        }
    }
}
