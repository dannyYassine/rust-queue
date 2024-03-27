use std::{thread, time::Duration};

use rust_queue::{
    dispatch,
    models::{
        application::Application,
        event_bus::SharedEventBus,
        event_dispatcher::{
            CanHandleEvent, Event, EventDispatcher, EventType, Listener, Subscriber,
        },
        job::JobHandle,
        resolve::{resolve, Resolvable},
    },
};
use serde::{Deserialize, Serialize};
use tokio::{runtime::Runtime, time::sleep};

#[derive(Debug, Default, Serialize, Clone, Copy)]
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
struct SendEmailUseCase;
impl Resolvable for SendEmailUseCase {
    fn resolve() -> Self {
        SendEmailUseCase {}
    }
}
impl SendEmailUseCase {
    #[allow(unused)]
    fn execute(&self, data: i32) {
        println!("Email sent!");
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct MyListener {
    name: &'static str,
}
impl Listener for MyListener {
    fn get_event(&self) -> String {
        MyEvent::name()
    }
}
impl CanHandleEvent for MyListener {
    fn handle(&self, event: EventType) {
        if let Some(event) = event.cast::<MyEvent>() {
            let e = event.clone();
            let rt = Runtime::new().unwrap();
            let handle = thread::spawn(move || {
                rt.block_on(async {
                    dispatch!(CustomJob(e));
                });
            });
            handle.join().unwrap();
            print!("Handle finished");
        }
    }
}
#[derive(Serialize)]
struct CustomJob(MyEvent);
impl JobHandle for CustomJob {
    fn handle(&self) {
        resolve::<SendEmailUseCase>().execute(self.0.data);
    }
}
#[derive(Default, Serialize)]
struct MySecondListener {}
impl Listener for MySecondListener {
    fn get_event(&self) -> String {
        MyEvent::name()
    }
}
impl CanHandleEvent for MySecondListener {
    fn handle(&self, event: EventType) {
        let e = event.cast::<MyEvent>();
        println!("Hi from MySecondListener, {:?}", e);
    }
}

#[derive(Default, Debug, Serialize)]
struct MySubscriber {}
impl Subscriber for MySubscriber {
    fn get_events(&self) -> Vec<String> {
        return vec![MyEvent::name(), MyOtherEvent::name()];
    }
}
impl CanHandleEvent for MySubscriber {
    fn handle(&self, event: EventType) {
        let mut data = None;
        if let Some(event) = event.cast::<MyEvent>() {
            println!("Hi from MySubscriber MyEvent, {:?}", event);
            data = Some(event.data);
        } else if let Some(event) = event.cast::<MyOtherEvent>() {
            println!("Hi from MySubscriber MyOtherEvent, {:?}", event);
            data = Some(event.data);
        }

        if let Some(data) = data {
            resolve::<SendEmailUseCase>().execute(data);
        }
    }
}

#[tokio::main]
async fn main() {
    Application::bootstrap().await;

    EventDispatcher::new()
        .bind_listener::<MyListener>()
        .bind_subscriber::<MySubscriber>();

    SharedEventBus::emit(&MyEvent { data: 1 });
    SharedEventBus::emit(&MyOtherEvent { data: 2 });

    let _ = sleep(Duration::from_secs(10));
}
