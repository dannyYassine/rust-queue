use std::any::type_name;

// Trait for the associated constant `NAME`
pub trait JobName: 'static {
    fn name() -> String {
        let s = type_name::<Self>().to_string();
        let word = s.split("::").last().unwrap_or_default();

        return word.to_owned();
    }
}

// Trait for the method `handle`
pub trait JobHandle: 'static {
    fn handle(&self);
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Job {
    pub id: i32,
    pub payload: String,
    pub status: String,
    pub model_type: String,
    pub data: String,
}

impl Job {
    pub fn handle(&self) {
        //
    }
    pub fn new(payload: String, status: String, model_type: String, data: String) -> Self {
        Job {
            id: 0,
            payload,
            status,
            model_type,
            data,
        }
    }
    pub fn set_status_as_pending(&mut self) -> &Self {
        self.status = JobStatus::Pending.to_string();

        return self;
    }
    pub fn set_status_as_running(&mut self) -> &Self {
        self.status = JobStatus::Running.to_string();

        return self;
    }
    pub fn set_status_as_completed(&mut self) -> &Self {
        self.status = JobStatus::Completed.to_string();

        return self;
    }
}

#[derive(Debug)]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
}

impl JobStatus {
    pub fn to_string(&self) -> String {
        return match self {
            JobStatus::Pending => String::from("pending"),
            JobStatus::Running => String::from("running"),
            JobStatus::Completed => String::from("completed"),
        };
    }
}
