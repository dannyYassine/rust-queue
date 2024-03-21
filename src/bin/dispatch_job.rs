use dotenvy::dotenv;
use rust_queue::{
    dispatch,
    models::{custom_job::PrintToConsoleJob, job::Job},
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    for _ in 0..5 {
        let print = PrintToConsoleJob {};
        println!("Dispatch job");
        dispatch!(print);
    }
}
