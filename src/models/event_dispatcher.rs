use std::{
    any::{type_name, Any},
    collections::HashMap,
    fmt::Debug,
    sync::{Arc, Mutex},
};

use super::event_bus::SharedEventBus;

#[derive(Debug, Clone)]
pub struct EventType<'a>(Box<&'a dyn Any>);
impl<'a> EventType<'a> {
    pub fn cast<T: 'static>(&self) -> Option<&'a T> {
        self.0.downcast_ref::<T>()
    }
}

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
    fn handle(&self, event: EventType) {
        println!("Hi from event dispatcher: {:?}", event);
    }
    fn should_queue(&self) -> bool {
        return false;
    }
}

pub trait Listener: Sync + 'static + Send + CanHandleEvent {
    fn get_event(&self) -> String;
}

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
    pub fn register_event<E>(&mut self) -> &mut Self
    where
        E: 'static + Send,
    {
        self.bind_event::<E>(vec![]);

        return self;
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
    pub fn bind_listener<L>(&mut self) -> &mut Self
    where
        L: Listener + CanHandleEvent + Default + Debug,
    {
        let listener = L::default();

        let key = listener.get_event();

        let event_map: Vec<Box<dyn CanHandleEvent>> = vec![Box::new(listener)]
            .into_iter()
            .map(|s| s as Box<dyn CanHandleEvent>)
            .collect();

        self.event_map
            .lock()
            .unwrap()
            .entry(key.to_owned())
            .or_insert_with(Vec::new)
            .extend(event_map);

        self.listen_to_event_with_key(key);

        return self;
    }
    pub fn bind_subscriber<S>(&mut self) -> &mut Self
    where
        S: Subscriber + CanHandleEvent + Default + Debug,
    {
        let subscriber = S::default();
        let events = subscriber.get_events();

        for event in events {
            let subscriber = S::default();
            let event_map: Vec<Box<dyn CanHandleEvent>> = vec![Box::new(subscriber)]
                .into_iter()
                .map(|s| s as Box<dyn CanHandleEvent>)
                .collect();

            self.event_map
                .lock()
                .unwrap()
                .entry(event.clone())
                .or_insert_with(Vec::new)
                .extend(event_map);

            self.listen_to_event_with_key(event);
        }

        return self;
    }
    fn listen_to_event_with_key(&self, key: String) {
        let mut event_bus = SharedEventBus::get_instance().event_bus.lock().unwrap();

        if event_bus.has_key(&key) {
            return;
        }

        let clone = Arc::clone(&self.event_map);
        let owned_key = key.to_owned();
        event_bus.listen_with_key(&key, move |event: Box<&dyn Any>| {
            if let Some(listeners) = clone.lock().unwrap().get(&owned_key) {
                for listener in listeners {
                    listener.handle(EventType(event.clone()));
                }
            }
        })
    }
    fn listen_to_event<E>(&self)
    where
        E: 'static + Send,
    {
        let mut event_bus = SharedEventBus::get_instance().event_bus.lock().unwrap();

        if event_bus.has::<E>() {
            return;
        }

        let clone = Arc::clone(&self.event_map);
        event_bus.listen::<E>(move |event: &E| {
            let s = type_name::<E>().to_owned();
            let key = s.split("::").last().unwrap_or_default().to_owned();

            if let Some(listeners) = clone.lock().unwrap().get(&key) {
                for listener in listeners {
                    listener.handle(EventType(Box::new(event)));
                }
            }
        })
    }
}
