use serde::{Deserialize, Serialize};

use super::job::CanHandleJob;

#[derive(Debug, Serialize, Deserialize)]
pub struct PrintToConsoleJob {
    pub name: String,
}

impl CanHandleJob for PrintToConsoleJob {
    const NAME: &'static str = "PrintToConsoleJob";

    fn handle(&self) {
        println!("Running PrintToConsoleJob");
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultipleValueJob {
    pub value: i32,
}

impl CanHandleJob for MultipleValueJob {
    const NAME: &'static str = "MultipleValueJob";

    fn handle(&self) {
        println!("Multiple value: {}", self.value * self.value);
    }
}
