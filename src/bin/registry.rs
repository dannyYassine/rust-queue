use std::{fmt::Debug, sync::Arc};

use rust_queue::models::registry::Registry;

trait MyTrait: Debug {}

#[derive(Clone, Debug)]
struct MyStruct {
    name: String,
    another_struct: Arc<Box<AnotherStruct>>,
}

#[derive(Clone, Debug)]
struct AnotherStruct {
    name: String,
}

fn main() {
    let registry = Registry::get_instance();
    // let st = MyStruct {
    //     name: String::from("value"),
    // };
    // registry.set("My", st);

    // let name = registry.get::<MyStruct, String>(
    //     "My",
    //     Box::new(|object: Option<&MyStruct>| {
    //         let new = object.unwrap();
    //         println!("{}", new.name);

    //         return new.name.to_owned();
    //     }),
    // );

    // registry.get::<MyStruct, _>(
    //     "My",
    //     Box::new(|object: Option<&MyStruct>| {
    //         let new = object.unwrap();
    //         println!("{}", new.name);
    //     }),
    // );

    // println!("{}", name);

    registry.register::<AnotherStruct>(Box::new(|_| {
        Box::new(AnotherStruct {
            name: "DI2".to_string(),
        })
    }));

    registry.register::<MyStruct>(Box::new(|r: &Registry| {
        Box::new(MyStruct {
            name: "DI".to_string(),
            another_struct: r.get_type::<AnotherStruct>(),
        })
    }));

    let m = registry.get_type::<AnotherStruct>();
    println!("{:?}", m);
    println!("{:?}", m.as_ref().name);

    let m = registry.get_type::<MyStruct>();
    println!("{:?}", m);
    println!("{:?}", m.as_ref().name);
    println!("{:?}", m.as_ref().another_struct);
}
