use std::vec;

use rust_queue::{
    json,
    models::resource::{Resource, ResourceArray},
};

pub struct User {
    pub name: String,
}

#[derive(Default, Debug)]
struct UserResource;

impl Resource<User> for UserResource {
    fn to_array(&self, data: User) -> ResourceArray {
        json! {
            "name" => data.name,
            "count" => 1
        }
    }
}

fn main() {
    let user = User {
        name: "Yo".to_owned(),
    };

    println!("{:?}", UserResource::make(user));

    let users = vec![
        User {
            name: "Hello".to_owned(),
        },
        User {
            name: "World".to_owned(),
        },
    ];

    println!("{:?}", UserResource::make_collection(users));
}
