use std::{
    any::{type_name, Any},
    collections::HashMap,
    sync::{Arc, Mutex},
};

use super::event_bus::SharedEventBus;

pub trait Listener: Sync + 'static + Send {
    fn cast<E>(&self, event: &dyn Any) -> Option<&E> {
        return event.downcast_ref::<E>();
    }
    fn handle(&self, event: Box<&dyn Any>) {
        println!("Hi from event dispatcher: {:?}", event);
    }
}

pub struct EventDispatcher {
    event_map: Arc<Mutex<HashMap<String, Vec<Box<dyn Listener>>>>>,
}

impl EventDispatcher {
    pub fn new() -> Self {
        EventDispatcher {
            event_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn add_event<E>(&mut self, event_map: Vec<Box<dyn Listener>>) -> &mut Self
    where
        E: 'static + Send,
    {
        let s = type_name::<E>().to_owned();
        let key = s.split("::").last().unwrap_or_default().to_owned();

        self.event_map
            .lock()
            .unwrap()
            .entry(key)
            .or_insert_with(Vec::new)
            .extend(event_map);

        self.listen_to_event::<E>();

        return self;
    }
    pub fn add_subscriber() {}
    fn listen_to_event<E>(&self)
    where
        E: 'static + Send,
    {
        let mut event_bus = SharedEventBus::get_instance().event_bus.lock().unwrap();

        let clone = Arc::clone(&self.event_map);
        event_bus.listen::<E>(move |event: &E| {
            let s = type_name::<E>().to_owned();
            let key = s.split("::").last().unwrap_or_default().to_owned();

            if let Some(listeners) = clone.lock().unwrap().get(&key) {
                for listener in listeners {
                    listener.handle(Box::new(event));
                }
            }
        })
    }
}
