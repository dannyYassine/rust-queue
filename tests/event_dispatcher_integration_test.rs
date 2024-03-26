use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
use rust_queue::models::{
    application::Application,
    event_bus::SharedEventBus,
    event_dispatcher::{CanHandleEvent, Event, EventDispatcher, EventType, Listener, Subscriber},
    resolve::Resolvable,
};

mod common;
use common::set_up;

#[derive(Debug, Default)]
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

#[derive(Default)]
struct MyListener();
impl Listener for MyListener {
    fn get_event(&self) -> String {
        MyEvent::name()
    }
}

#[derive(Default)]
struct MySubscriber {}
impl Subscriber for MySubscriber {
    fn get_events(&self) -> Vec<String> {
        return vec![MyEvent::name(), MyOtherEvent::name()];
    }
}
impl CanHandleEvent for MySubscriber {
    fn handle(&self, event: EventType) {
        if let Some(_) = event.cast::<MyEvent>() {
            let mut l = SHARED.data.lock().unwrap();
            *l = true;
        }
        if let Some(_) = event.cast::<MyOtherEvent>() {
            let mut l = SHARED.data.lock().unwrap();
            *l = true;
        }
    }
}

struct TestData {
    pub data: Arc<Mutex<bool>>,
}

lazy_static! {
    static ref SHARED: TestData = TestData {
        data: Arc::new(Mutex::new(false))
    };
}

#[tokio::test]
async fn it_should_bind_listener_to_trigger_handle_method() {
    set_up();
    Application::bootstrap().await;

    impl CanHandleEvent for MyListener {
        fn handle(&self, event: EventType) {
            if let Some(_) = event.cast::<MyEvent>() {
                let mut l = SHARED.data.lock().unwrap();
                *l = true;
            }
        }
    }

    EventDispatcher::new().bind_listener::<MyListener>();

    SharedEventBus::emit(&MyEvent { data: 1 });

    assert_eq!(*SHARED.data.lock().unwrap(), true);
}

fn tear_down() {
    *SHARED.data.lock().unwrap() = false;
}

struct EventNotRegisteredToo;

#[tokio::test]
async fn it_should_bind_subcriber_to_trigger_handle_method() {
    set_up();
    Application::bootstrap().await;

    EventDispatcher::new().bind_subscriber::<MySubscriber>();

    SharedEventBus::emit(&EventNotRegisteredToo {});

    assert_eq!(*SHARED.data.lock().unwrap(), true);

    tear_down();
}

#[tokio::test]
async fn it_should_bind_listener_and_not_trigger_event_not_registered_to() {
    set_up();
    Application::bootstrap().await;

    EventDispatcher::new().bind_listener::<MyListener>();

    SharedEventBus::emit(&EventNotRegisteredToo {});

    assert_eq!(*SHARED.data.lock().unwrap(), false);

    tear_down();
}

#[tokio::test]
async fn it_should_bind_subcriber_for_multiple_events_to_trigger_handle_method() {
    set_up();
    Application::bootstrap().await;

    EventDispatcher::new().bind_subscriber::<MySubscriber>();

    SharedEventBus::emit(&MyOtherEvent { data: 1 });

    assert_eq!(*SHARED.data.lock().unwrap(), true);

    tear_down();
}

#[tokio::test]
async fn it_should_bind_subcriber_and_not_trigger_events_not_registered_to() {
    set_up();
    Application::bootstrap().await;

    EventDispatcher::new().bind_subscriber::<MySubscriber>();

    SharedEventBus::emit(&EventNotRegisteredToo {});

    assert_eq!(*SHARED.data.lock().unwrap(), false);

    tear_down();
}
