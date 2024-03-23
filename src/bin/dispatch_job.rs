use std::thread;

use dotenvy::dotenv;
use rust_queue::{
    dispatch,
    models::{app_state::AppStateManager, data_connection::DatabaseConnection, job::Job},
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

#[tokio::main]
async fn main() {
    dotenv().ok();

    let connection = DatabaseConnection::create().await;
    AppStateManager::get_instance()
        .initialize()
        .set_connection(connection);

    let mut handles = vec![];

    for _ in 0..5 {
        let rt = Runtime::new().unwrap();
        let handle = thread::spawn(move || {
            rt.block_on(async {
                run_job_1().await;
            });
        });
        handles.push(handle);
    }

    for _ in 0..5 {
        let rt = Runtime::new().unwrap();
        let handle = thread::spawn(move || {
            rt.block_on(async {
                run_job_2().await;
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
    println!("Dispatch job");
    dispatch!(job);
}

async fn run_job_2() {
    let job = MultipleValueJob { value: 2 };
    println!("Dispatch job");
    dispatch!(job);
}
