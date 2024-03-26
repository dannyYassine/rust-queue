use std::any::Any;

use rust_queue::models::{
    application::Application,
    event_bus::SharedEventBus,
    event_dispatcher::{CanHandleEvent, Event, EventDispatcher, Listener, Subscriber},
};

#[derive(Debug)]
#[allow(dead_code)]
struct MyEvent {
    data: i32,
}
impl Event for MyEvent {}

#[derive(Debug)]
#[allow(dead_code)]
struct MyOtherEvent {
    data: i32,
}
impl Event for MyOtherEvent {}

#[derive(Default)]
struct MyListener {}
impl CanHandleEvent for MyListener {
    fn handle(&self, event: Box<&dyn Any>) {
        let e = event.downcast_ref::<MyEvent>();
        println!("Hi from MyListener: {:?}", e);
    }
}
impl Listener for MyListener {}

#[derive(Default)]
struct MySecondListener {}
impl CanHandleEvent for MySecondListener {
    fn handle(&self, event: Box<&dyn Any>) {
        let e = event.downcast_ref::<MyEvent>();
        println!("Hi from MySecondListener, {:?}", e);
    }
}

#[derive(Default)]
struct MySubscriber {}
impl CanHandleEvent for MySubscriber {
    fn handle(&self, event: Box<&dyn Any>) {
        let e = event.downcast_ref::<MyEvent>();
        println!("Hi from MySubscriber, {:?}", e);
    }
}
impl Subscriber for MySubscriber {
    fn get_events(&self) -> Vec<String> {
        return vec![MyEvent::name(), MyOtherEvent::name()];
    }
}

#[tokio::main]
async fn main() {
    Application::bootstrap().await;

    EventDispatcher::new()
        .add_event::<MyEvent>(vec![
            Box::new(MyListener::default()),
            Box::new(MySecondListener::default()),
        ])
        .add_event::<MyEvent>(vec![Box::new(MyListener::default())])
        .add_event::<MyEvent>(vec![Box::new(MySecondListener::default())])
        .add_subscriber(MySubscriber::default());

    SharedEventBus::emit(&MyEvent { data: 1 });
}
