use rust_queue::models::job::{JobHandle, JobName};
use serde::{Deserialize, Serialize};

impl JobName for PrintToConsoleJob {}
impl JobHandle for PrintToConsoleJob {
    fn handle(&self) {
        println!("running PrintToConsoleJob");
    }
}

impl JobName for MultipleValueJob {}
impl JobHandle for MultipleValueJob {
    fn handle(&self) {
        println!("running MultipleValueJob: {}", self.value * self.value);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrintToConsoleJob {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultipleValueJob {
    pub value: i32,
}
