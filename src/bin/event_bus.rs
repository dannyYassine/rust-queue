use rust_queue::models::event_bus::EventBus;

struct Event {
    data: String,
}

fn main() {
    let mut event_bus = EventBus::default();

    event_bus.listen_with_key(
        "func".to_string(),
        Box::new(|_| {
            println!("Hi!");
        }),
    );

    event_bus.emit_with_key("func".to_string(), None);

    let event = Event {
        data: String::from("value"),
    };

    event_bus.listen::<Event>(Box::new(|e| {
        println!("Hi event {:?}!", e);
    }));

    event_bus.emit(&event);
}
