use std::sync::{Arc, Mutex};

use rust_queue::models::event_bus::EventBus;

mod common;
use common::set_up;

struct MyCustomEventWithData {
    data: String,
}

#[tokio::test]
async fn it_should_listen_to_typed_event() {
    set_up();

    let mut event_bus = EventBus::default();

    let custom_event = MyCustomEventWithData {
        data: String::from("value"),
    };

    let rc = Arc::new(custom_event);
    let another_rc = Arc::clone(&rc);
    event_bus.listen::<MyCustomEventWithData>(move |event: &MyCustomEventWithData| {
        assert_eq!(rc.data, event.data);
    });

    event_bus.emit::<MyCustomEventWithData>(&another_rc);
}

#[tokio::test]
async fn it_should_clear_all_listeners() {
    set_up();

    struct Data {
        did_call: bool,
    }

    let data = Data { did_call: false };

    let mut event_bus = EventBus::default();

    let custom_event = MyCustomEventWithData {
        data: String::from("value"),
    };

    let did_call = Arc::new(Mutex::new(data));
    let did_call_clone = Arc::clone(&did_call);

    event_bus.listen::<MyCustomEventWithData>(move |_| {
        let mut value = did_call.lock().unwrap();
        value.did_call = true;
    });

    event_bus.clear();

    event_bus.emit::<MyCustomEventWithData>(&custom_event);

    assert_eq!(did_call_clone.lock().unwrap().did_call, false);
}
