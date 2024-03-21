pub trait CanHandleJob {
    const NAME: &'static str;
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

// impl CanHandleJob for Job {
//     async fn handle(&self) {
//         //
//     }
// }

impl Job {
    pub fn handle(&self) {
        //
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
