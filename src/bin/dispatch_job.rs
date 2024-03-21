use dotenvy::dotenv;
use rust_queue::{
    dispatch,
    models::{
        custom_job::{MultipleValueJob, PrintToConsoleJob},
        job::Job,
    },
};

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
