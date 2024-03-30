use rust_queue::models::type_map::TypeMap;

fn main() {
    let mut type_map = TypeMap::new();
    type_map.set::<String>(String::from("value"));

    println!("{:?}", type_map.get_mut::<String>());
}
