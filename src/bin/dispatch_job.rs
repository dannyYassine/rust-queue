use std::thread;

use rust_queue::{
    dispatch,
    models::{application::Application, job::Dispatchable},
};
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
impl Dispatchable for MultipleValueJob {}

#[tokio::main]
async fn main() {
    Application::bootstrap().await;

    let mut handles = vec![];

    for i in 0..5 {
        println!("Dispatch second job {}", i);
        let rt = Runtime::new().unwrap();
        let handle = thread::spawn(move || {
            rt.block_on(async {
                run_job_2().await;
                println!("Dispatch second job {} finished", i);
            });
        });
        handles.push(handle);
    }

    for i in 0..5 {
        println!("Dispatch first job {}", i);
        let rt = Runtime::new().unwrap();
        let handle = thread::spawn(move || {
            rt.block_on(async {
                run_job_1().await;
                println!("Dispatch first job {} finished", i);
            });
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

async fn run_job_1() {
    let job = PrintToConsoleJob {
        name: "this is my job".to_string(),
    };
    dispatch!(job);
}

async fn run_job_2() {
    MultipleValueJob { value: 2 }.dispatch().await;
}
