use rust_queue::models::event_bus::EventBus;

struct Event {
    data: String,
}

fn main() {
    let mut event_bus = EventBus::default();

    let event = Event {
        data: String::from("value"),
    };

    event_bus.listen::<Event>(Box::new(|e: &Event| {
        println!("Hi event {:?}!", e.data);
    }));

    event_bus.emit(&event);
}
