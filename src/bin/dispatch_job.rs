use dotenvy::dotenv;
use rust_queue::{dispatch, models::job::Job};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct PrintToConsoleJob {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MultipleValueJob {
    value: i32,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    for _ in 0..2 {
        let job = PrintToConsoleJob {
            name: "this is my job".to_string(),
        };
        println!("Dispatch job");
        dispatch!(job);
    }

    for _ in 0..2 {
        let job = MultipleValueJob { value: 2 };
        println!("Dispatch job");
        dispatch!(job);
    }
}
