use std::thread;

use dotenvy::dotenv;
use rust_queue::{dispatch, models::job::Job};
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

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
    let rt = Runtime::new().unwrap();

    let mut handles = vec![];

    let handle = thread::spawn(move || {
        rt.block_on(async {
            for _ in 0..2 {
                let job = PrintToConsoleJob {
                    name: "this is my job".to_string(),
                };
                println!("Dispatch job");
                dispatch!(job);
            }
        })
    });

    handles.push(handle);

    let rt = Runtime::new().unwrap();

    let handle = thread::spawn(move || {
        rt.block_on(async {
            for _ in 0..2 {
                let job = MultipleValueJob { value: 2 };
                println!("Dispatch job");
                dispatch!(job);
            }
        });
    });
    handles.push(handle);

    for handle in handles {
        handle.join().unwrap();
    }
}
