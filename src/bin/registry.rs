use rust_queue::models::registry::{HashableRegistry, Registry};

struct MyStruct {
    name: String,
}
impl HashableRegistry for MyStruct {}

fn main() {
    let registry = Registry::get_instance();
    let st = MyStruct {
        name: String::from("value"),
    };
    registry.set("My", st);

    let name = registry.get::<MyStruct, String>(
        "My",
        Box::new(|object: Option<&MyStruct>| {
            let new = object.unwrap();
            println!("{}", new.name);

            return new.name.to_owned();
        }),
    );

    registry.get::<MyStruct, _>(
        "My",
        Box::new(|object: Option<&MyStruct>| {
            let new = object.unwrap();
            println!("{}", new.name);
        }),
    );

    println!("{}", name);
}
