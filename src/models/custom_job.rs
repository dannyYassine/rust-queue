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
