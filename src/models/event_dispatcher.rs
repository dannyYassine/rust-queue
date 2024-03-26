use std::{
    any::{type_name, Any},
    collections::HashMap,
    sync::{Arc, Mutex},
};

use super::event_bus::SharedEventBus;

pub trait Event {
    fn name() -> String {
        return type_name::<Self>()
            .split("::")
            .last()
            .unwrap_or_default()
            .to_owned();
    }
}

pub trait CanHandleEvent: Sync + 'static + Send {
    fn handle(&self, event: Box<&dyn Any>) {
        println!("Hi from event dispatcher: {:?}", event);
    }
}

pub trait Listener: Sync + 'static + Send + CanHandleEvent {}

pub trait Subscriber: CanHandleEvent + Sync + 'static + Send {
    fn get_events(&self) -> Vec<String>;
}

pub struct EventDispatcher {
    event_map: Arc<Mutex<HashMap<String, Vec<Box<dyn CanHandleEvent>>>>>,
}

impl EventDispatcher {
    pub fn new() -> Self {
        EventDispatcher {
            event_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn bind_event<E>(&mut self, event_map: Vec<Box<dyn CanHandleEvent>>) -> &mut Self
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
    pub fn bind_subscriber<S>(&self)
    where
        S: Subscriber + CanHandleEvent + Default,
    {
        let subscriber = S::default();
        let events = subscriber.get_events();

        for event in events {
            let subscriber = S::default();
            let event_map = vec![subscriber];

            self.event_map
                .lock()
                .unwrap()
                .entry(event.to_owned())
                .or_insert_with(Vec::new)
                .extend(
                    event_map
                        .into_iter()
                        .map(|s| Box::new(s) as Box<dyn CanHandleEvent>),
                );
        }
    }
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
