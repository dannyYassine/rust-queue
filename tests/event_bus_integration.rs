use std::{cell::RefCell, rc::Rc};

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

    let rc = Rc::new(custom_event);
    let another_rc = Rc::clone(&rc);
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

    let did_call = Rc::new(RefCell::new(data));
    let did_call_clone = Rc::clone(&did_call);

    event_bus.listen::<MyCustomEventWithData>(move |_| {
        did_call.borrow_mut().did_call = true;
    });

    event_bus.clear();

    event_bus.emit::<MyCustomEventWithData>(&custom_event);

    assert_eq!(did_call_clone.borrow().did_call, false);
}
