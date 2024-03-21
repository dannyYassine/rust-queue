use dotenvy::dotenv;
use rust_queue::{
    dispatch,
    models::{custom_job::PrintToConsoleJob, job::Job},
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    for _ in 0..5 {
        let print = PrintToConsoleJob {
            name: "this is my job".to_string(),
        };
        println!("Dispatch job");
        dispatch!(print);
    }
}
