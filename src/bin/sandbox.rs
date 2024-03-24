use serde::{Deserialize, Serialize};
use std::{any::type_name, collections::HashMap};

// Trait for the method `handle`
trait JobHandle: 'static {
    fn handle(&self);
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

// Implementing trait `JobHandle` for `MyJob`
impl JobHandle for AnotherJob {
    fn handle(&self) {
        println!("{:?}", self.value)
    }
}

// Function to register a job in the JobMap
fn register<J>(map: &mut JobMap)
where
    J: JobHandle + Serialize + for<'de> Deserialize<'de>,
{
    let s = type_name::<J>().to_owned();
    let job_key = s.split("::").last().unwrap_or_default();

    map.insert(
        job_key.to_owned(),
        Box::new(move |json_value: String| {
            Box::new(serde_json::from_str::<J>(json_value.as_str()).unwrap()) as Box<dyn JobHandle>
        }),
    );
}

fn get_job<J>(map: &mut JobMap, json: String) -> Box<dyn JobHandle> {
    let model_type = {
        let s = type_name::<J>().to_owned();
        s.split("::").last().unwrap_or_default().to_owned()
    };
    let func = map.get(&model_type).unwrap();

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
    let job_to_run = get_job::<MyJob>(&mut map, json);
    job_to_run.handle();

    let my_job = AnotherJob { value: 1 };
    // saves struct in DB
    let json = serde_json::to_string::<AnotherJob>(&my_job).unwrap();
    // retrieve job info from DB, using json and model_type
    let job_to_run = get_job::<AnotherJob>(&mut map, json);
    job_to_run.handle();
}
