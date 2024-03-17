#[derive(Debug, sqlx::FromRow)]
pub struct Job {
    pub id: i32,
    pub payload: String,
    pub status: String,
}

impl Job {
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
    pub fn set_status(&mut self, job_status: JobStatus) -> &Self {
        self.status = job_status.to_string();

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
