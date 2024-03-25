use std::{any::Any, default};

use dotenvy::dotenv;
use rust_queue::models::{
    application::Application,
    event_bus::SharedEventBus,
    event_dispatcher::{EventDispatcher, Listener},
};

#[derive(Debug)]
struct MyEvent {
    data: i32,
}

#[derive(Default)]
struct MyListener {}
impl Listener for MyListener {
    fn handle(&self, event: Box<&dyn Any>) {
        let e = event.downcast_ref::<MyEvent>();
        println!("Hi from MyListener: {:?}", e);
    }
}

#[derive(Default)]
struct MySecondListener {}
impl Listener for MySecondListener {
    fn handle(&self, event: Box<&dyn Any>) {
        let e = event.downcast_ref::<MyEvent>();
        println!("Hi from MySecondListener, {:?}", e);
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
        .add_event::<MyEvent>(vec![Box::new(MySecondListener::default())]);

    SharedEventBus::emit(&MyEvent { data: 1 });
}
