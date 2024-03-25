use std::rc::Rc;

use rust_queue::models::{
    app_state::AppStateManager, application::Application, event_bus::EventBus,
};

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
