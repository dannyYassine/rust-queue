use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Trait for the associated constant `NAME`
trait JobName: 'static {
    const NAME: &'static str;
}

// Trait for the method `handle`
trait JobHandle: 'static {
    fn handle(&self);
}

// Implementing trait `JobName` for `MyJob`
impl JobName for MyJob {
    const NAME: &'static str = "MyJob";
}

// Implementing trait `JobHandle` for `MyJob`
impl JobHandle for MyJob {
    fn handle(&self) {
        println!("{:?}", self.name)
    }
}

// Type alias for the closure used in the JobMap
type JobClosure = Box<dyn Fn(String) -> Box<dyn JobHandle>>;

// HashMap for the JobMap
type JobMap = HashMap<String, JobClosure>;

// Struct for the job
#[derive(Serialize, Deserialize, Debug)]
struct MyJob {
    name: String,
}

// Struct for the job
#[derive(Serialize, Deserialize, Debug)]
struct AnotherJob {
    value: i32,
}

// Implementing trait `JobName` for `MyJob`
impl JobName for AnotherJob {
    const NAME: &'static str = "AnotherJob";
}

// Implementing trait `JobHandle` for `MyJob`
impl JobHandle for AnotherJob {
    fn handle(&self) {
        println!("{:?}", self.value)
    }
}

// Function to register a job in the JobMap
fn register<J>(map: &mut JobMap)
where
    J: JobName + JobHandle + Serialize + for<'de> Deserialize<'de>,
{
    map.insert(
        J::NAME.to_owned(),
        Box::new(move |json_value: String| {
            Box::new(serde_json::from_str::<J>(json_value.as_str()).unwrap()) as Box<dyn JobHandle>
        }),
    );
}

fn get_job(map: &mut JobMap, json: String, model_type: &str) -> Box<dyn JobHandle> {
    let func = map.get(model_type).unwrap();

    return func(json);
}

fn main() {
    let mut map: JobMap = HashMap::new();
    register::<MyJob>(&mut map);
    register::<AnotherJob>(&mut map);

    let my_job = MyJob {
        name: "MyJob is running".to_string(),
    };
    // saves struct in DB
    let json = serde_json::to_string::<MyJob>(&my_job).unwrap();
    // retrieve job info from DB, using json and model_type
    let job_to_run = get_job(&mut map, json, MyJob::NAME);
    job_to_run.handle();

    let my_job = AnotherJob { value: 1 };
    // saves struct in DB
    let json = serde_json::to_string::<AnotherJob>(&my_job).unwrap();
    // retrieve job info from DB, using json and model_type
    let job_to_run = get_job(&mut map, json, AnotherJob::NAME);
    job_to_run.handle();
}
